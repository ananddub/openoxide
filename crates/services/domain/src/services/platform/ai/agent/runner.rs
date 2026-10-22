//! The assistant's reasoning loop: drive rig completions by hand with the search_actions / invoke / ask_user tools until the model answers in plain text.

use rig_core::OneOrMany;
use rig_core::completion::{AssistantContent, CompletionModel, ToolDefinition};
use rig_core::message::{Message, UserContent};
use serde_json::json;

use super::catalog;
use super::invoke::{InvokeOutcome, UserContext, invoke};

const MAX_STEPS: usize = 14;
const TOKEN_BUDGET: u64 = 4096;

/// Provider-compatibility flags toggled on when a request is rejected for a known OpenAI-family quirk.
#[derive(Clone, Copy, Default)]
struct CompatFlags {
    /// Send `max_completion_tokens` instead of `max_tokens` (newer reasoning models).
    completion_tokens: bool,
    /// Send `reasoning_effort: "none"` so function tools are allowed on chat/completions for reasoning models.
    reasoning_none: bool,
}

impl CompatFlags {
    /// Flip the flag the error asks for; false if nothing new applies, so the caller stops retrying.
    fn adapt(&mut self, err: &str) -> bool {
        let mut changed = false;
        if !self.completion_tokens && wants_completion_tokens(err) {
            self.completion_tokens = true;
            changed = true;
        }
        if !self.reasoning_none && wants_reasoning_none(err) {
            self.reasoning_none = true;
            changed = true;
        }
        changed
    }
}

/// True if the provider rejected `max_tokens` and wants `max_completion_tokens`.
fn wants_completion_tokens(err: &str) -> bool {
    let e = err.to_ascii_lowercase();
    e.contains("max_completion_tokens") || (e.contains("max_tokens") && e.contains("unsupported"))
}

/// True if the provider rejected function tools at default reasoning effort and wants `reasoning_effort: "none"`.
fn wants_reasoning_none(err: &str) -> bool {
    let e = err.to_ascii_lowercase();
    e.contains("reasoning_effort") && (e.contains("'none'") || e.contains("function tools"))
}

/// Run one completion turn, shaping request params per the compat flags.
async fn complete_once<M: CompletionModel>(
    model: &M,
    prompt: &Message,
    prior: &[Message],
    defs: &[ToolDefinition],
    flags: CompatFlags,
) -> Result<rig_core::completion::CompletionResponse<M::Response>, String> {
    let mut builder = model
        .completion_request(prompt.clone())
        .preamble(preamble())
        .tools(defs.to_vec())
        .temperature(0.1);

    let mut extra = serde_json::Map::new();
    if flags.completion_tokens {
        extra.insert("max_completion_tokens".into(), json!(TOKEN_BUDGET));
    } else {
        builder = builder.max_tokens(TOKEN_BUDGET);
    }
    if flags.reasoning_none {
        extra.insert("reasoning_effort".into(), json!("none"));
    }
    if !extra.is_empty() {
        builder = builder.additional_params(serde_json::Value::Object(extra));
    }
    if !prior.is_empty() {
        builder = builder.messages(prior.to_vec());
    }
    model
        .completion(builder.build())
        .await
        .map_err(|e| e.to_string())
}

/// What the assistant produced for the user.
pub struct AgentReply {
    /// Natural-language reply shown to the user.
    pub text: String,
    /// Optional structured interaction the UI renders as controls instead of expecting free text.
    pub interaction: Option<Interaction>,
}

/// A structured request for user input, rendered by the UI as controls.
#[derive(Clone, Debug)]
pub struct Interaction {
    pub kind: InteractionKind,
    /// The question/summary to show above the controls.
    pub question: String,
    /// Selectable options (CHOICE only; empty for CONFIRM).
    pub options: Vec<ChoiceOption>,
    /// For CONFIRM: JSON `[{"action":..,"params":..}]` of the actions to run on "yes", carried back by the client so the next turn executes them deterministically.
    pub pending_actions: String,
}

#[derive(Clone, Copy, Debug)]
pub enum InteractionKind {
    /// Yes/No confirmation of a (usually destructive) action.
    Confirm,
    /// Pick one of several options (projects, environments, servers, ...).
    Choice,
}

#[derive(Clone, Debug)]
pub struct ChoiceOption {
    /// Stable value sent back when chosen (e.g. an id or canonical name).
    pub id: String,
    /// Human label shown on the control.
    pub label: String,
    /// Optional secondary line.
    pub description: String,
}

fn tool_definitions() -> Vec<ToolDefinition> {
    vec![
        ToolDefinition {
            name: "search_actions".into(),
            description: "Find the actions relevant to what you need to do by keyword (e.g. \"template\", \"deploy compose\", \"ssh key\", \"domain\", \"environment\"). Returns the matching action names + their params. There are hundreds of actions, so ALWAYS search for the few you need instead of guessing names.".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "space-separated keywords describing the action you want" }
                },
                "required": ["query"]
            }),
        },
        ToolDefinition {
            name: "invoke".into(),
            description: "Run one action on the user's resources. Provide the action name and its JSON params. For destructive actions, set confirmed=true only after the user has confirmed.".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "action": { "type": "string", "description": "action name from search_actions" },
                    "params": { "type": "object", "description": "action parameters" },
                    "confirmed": { "type": "boolean", "description": "true only after explicit user confirmation of a destructive action" }
                },
                "required": ["action"]
            }),
        },
        ToolDefinition {
            name: "ask_user".into(),
            description: "Ask the user a clarifying free-text question when information is missing or ambiguous. Do not guess.".into(),
            parameters: json!({
                "type": "object",
                "properties": { "question": { "type": "string" } },
                "required": ["question"]
            }),
        },
        ToolDefinition {
            name: "confirm_action".into(),
            description: "Ask the user to confirm destructive/important action(s) with Yes/No buttons. Call this BEFORE any stop/delete/reset/prune. ALWAYS include the exact action(s) to run in `actions` (each = an invoke action name + params). When the user confirms, those actions are executed automatically — you do NOT need to call invoke again yourself.".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "summary": { "type": "string", "description": "Plain-language description of exactly what will happen if confirmed." },
                    "actions": {
                        "type": "array",
                        "description": "The invoke action(s) to run on confirmation (in order). For 'delete all 4 tags', include all four here.",
                        "items": {
                            "type": "object",
                            "properties": {
                                "action": { "type": "string", "description": "invoke action name from search_actions" },
                                "params": { "type": "object", "description": "action parameters" }
                            },
                            "required": ["action"]
                        }
                    }
                },
                "required": ["summary", "actions"]
            }),
        },
        ToolDefinition {
            name: "choose".into(),
            description: "Ask the user to pick one option when a name is ambiguous or there are multiple candidates (projects, environments, servers, resources). The UI shows selectable cards. Prefer this over ask_user whenever you can enumerate the choices.".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "question": { "type": "string", "description": "What the user is choosing." },
                    "options": {
                        "type": "array",
                        "description": "The selectable options.",
                        "items": {
                            "type": "object",
                            "properties": {
                                "id": { "type": "string", "description": "Stable value to act on (id or canonical name)." },
                                "label": { "type": "string", "description": "Human label." },
                                "description": { "type": "string", "description": "Optional secondary line." }
                            },
                            "required": ["id", "label"]
                        }
                    }
                },
                "required": ["question", "options"]
            }),
        },
        ToolDefinition {
            name: "list_templates".into(),
            description: "List the available one-click app templates (id, name, description) from the template registry. Call this when the user asks to deploy a known app by name (e.g. 'AList', 'Cloudflare tunnel') so you can find the exact template id. NEVER invent template content yourself.".into(),
            parameters: json!({ "type": "object", "properties": {}, "required": [] }),
        },
        ToolDefinition {
            name: "install_template".into(),
            description: "Install a registry template into an environment. The server fetches the real compose + config for the given template_id — you must NOT pass any compose YAML. Find the id first via list_templates.".into(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "template_id": { "type": "string", "description": "Template id from list_templates (e.g. 'alist')." },
                    "environment_id": { "type": "number", "description": "Target environment id (resolve via list actions)." },
                    "name": { "type": "string", "description": "Optional name for the created compose; defaults to the template." },
                    "server_id": { "type": "number", "description": "Optional server id; omit for the local server." }
                },
                "required": ["template_id", "environment_id"]
            }),
        },
    ]
}

fn preamble() -> String {
    "You are OpenOxide's deploy assistant. You help the user manage their \
         projects by calling actions on their behalf. The conversation history \
         is provided — always use it for context.\n\
         There are hundreds of actions; you do NOT see them up front.\n\
         RULES:\n\
         - FIRST decide: is this a request to DO something, or just talk? If the \
           user is only chatting, greeting, or asking a general/how-does-it-work \
           / what-can-you-do / conceptual question, just ANSWER in plain text — \
           do NOT call any tools (no search_actions, no invoke). Only touch \
           tools when the user actually wants you to look up or change THEIR \
           resources (deploy, list my X, create, stop, status of my services, \
           etc.). When in doubt and nothing needs to change, answer directly \
           and offer to do it if they want.\n\
         - VAGUE / BEGINNER requests: if the intent is unclear or open-ended \
           (e.g. 'make blinko work', 'fix it', 'set it up', 'deploy my app' \
           with no app), do NOT guess-chain many actions. First identify the \
           resource (one or two list calls), then use `choose` to offer the \
           likely intents as options (e.g. for 'make blinko work' on a compose: \
           'Start it', 'Redeploy it', 'View logs') or ask_user one short \
           question. Prefer asking over trial-and-error — the user may be a \
           first-timer. Never burn steps on exploratory actions that might fail.\n\
         - To do anything, first call `search_actions` with keywords (e.g. \
           \"list projects\", \"deploy compose\", \"create domain\", \"environment\") \
           to find the exact action name + params, then `invoke` it. Search for \
           just what the current step needs — don't try to enumerate everything.\n\
         - PARAMS: each action's params show a marker — `name!` is REQUIRED, \
           `name?` is optional. You MUST provide every required (!) param when \
           you invoke. Resolve ids yourself via list actions; if a required \
           value is genuinely unknown and can't be defaulted, ask_user (or \
           `choose`) for it instead of guessing or omitting it. Only send \
           optional (?) params when the user asked for that specific value.\n\
         - Only CREATE, DEPLOY, or DELETE something when the user explicitly \
           asked for it in their message. Do not create or deploy anything on \
           your own initiative; if unsure whether they want you to create it, \
           ask_user first.\n\
         - NEVER ask the user for a numeric id. Ids are your job: when the user \
           names a resource (e.g. 'blinko', 'test project', 'production'), \
           discover it yourself (search for + invoke the relevant list actions: \
           projects → environments → applications/compose/databases) and match \
           the name case-insensitively. Only ask_user if, after listing, the \
           name truly matches nothing or is ambiguous between multiple results.\n\
         - If there is exactly ONE project, environment, or server, use it \
           automatically — do NOT ask the user to pick when there is only one \
           choice. Only ask when there are multiple and the user did not say which.\n\
         - When you DO need the user to pick among multiple candidates \
           (projects, environments, servers, matching resources), call `choose` \
           with the enumerated options — never list them as plain text.\n\
         - CONFIRMATIONS: before a destructive action (stop/delete/reset/prune), \
           FIRST resolve every target to its real id (call the list actions as \
           needed), THEN call `confirm_action` with a clear summary and the \
           `actions` array fully populated — each entry must have the exact \
           invoke action name AND its real params (e.g. {\"action\":\"tag_delete_tag\",\
           \"params\":{\"id\":42}}). Put ALL targets in one confirm_action (for \
           'delete all 4 tags', include all four with their ids). Do NOT emit \
           empty params. After the user confirms, those actions run \
           automatically — you will NOT be called again, so they must be \
           complete and correct now.\n\
         - TEMPLATES / one-click apps (e.g. 'AList', 'Cloudflare tunnel', \
           'redis template'): call `list_templates` to find the matching id, \
           then call `install_template` with that template_id + the \
           environment_id. The server pulls the real compose — NEVER pass or \
           invent compose YAML yourself, and never use the raw compose actions \
           for a named template. If the name matches multiple templates, use \
           `choose`. install_template already seeds the template's required env \
           vars with sensible defaults — do NOT run a separate env-update after \
           it unless the user gave specific values to set.\n\
         - ENV UPDATES: to change a compose's env, call compose_update_compose \
           with the compose id AND env_var as plain .env text (one KEY=VALUE per \
           line, e.g. \"FOO=bar\\nBAZ=qux\") — never a JSON object. Always include \
           the id.\n\
         - NAMING: when creating a resource (template install, app, database), \
           give it a meaningful name (e.g. the template/app name like 'alist', \
           not the generic 'compose'). If the user clearly wants to name it \
           themselves, use ask_user to get the name.\n\
         - 'create X and deploy' for a plain resource (e.g. a redis database): \
           search for the create action, invoke it with sensible defaults, then \
           deploy/start the created resource using the id it returns.\n\
         - If any required parameter OTHER THAN an id is missing and cannot be \
           sensibly defaulted, call ask_user — never guess those.\n\
         - ENUM PARAMS: for a param shown as enum(A|B|C), the value must be \
           EXACTLY one of those literals (e.g. \"SSH_KEY_TYPE_ED25519\"); never \
           invent spellings. If the user did NOT clearly specify which one, do \
           NOT pick a default yourself — call `choose` first so the user selects \
           (one option per enum value, with a short human label like 'Ed25519' \
           or 'RSA'). Only proceed with invoke once a value is chosen. If the \
           user already named the value in their message, skip the choose and \
           use it directly.\n\
         - Stay strictly within the user's organization and permissions. If an \
           action returns a permission error, tell the user plainly.\n\
         - Reply in the SAME language/script the user used in their latest \
           message (Hinglish stays Hinglish). Keep replies short."
        .to_string()
}

/// Execute a non-terminating tool call, returning text for the model; terminating tools end the turn in the loop.
async fn run_tool(
    ctx: &UserContext,
    name: &str,
    args: &serde_json::Value,
    registered: &std::collections::HashSet<String>,
) -> String {
    match name {
        "search_actions" => {
            let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
            catalog::search(query, registered, 25)
        }
        "list_templates" => match super::templates::list().await {
            Ok(list) => {
                let lines: Vec<String> = list
                    .iter()
                    .map(|t| {
                        format!(
                            "{} — {}{}",
                            t.id,
                            if t.name.is_empty() { &t.id } else { &t.name },
                            if t.description.is_empty() {
                                String::new()
                            } else {
                                format!(": {}", t.description)
                            }
                        )
                    })
                    .collect();
                format!("Available templates (use the id):\n{}", lines.join("\n"))
            }
            Err(e) => format!("Could not list templates: {e}"),
        },
        "install_template" => {
            let template_id = args
                .get("template_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let environment_id = args.get("environment_id").and_then(|v| v.as_i64());
            if template_id.is_empty() || environment_id.is_none() {
                return "install_template needs template_id and environment_id.".into();
            }
            let files = match super::templates::files(template_id).await {
                Ok(f) => f,
                Err(e) => return e,
            };
            let mut params = json!({
                "template": files.compose,
                "environment_id": environment_id,
            });
            // Default the compose name to the template so it never falls back to the generic "compose".
            let name = args
                .get("name")
                .and_then(|v| v.as_str())
                .filter(|s| !s.trim().is_empty())
                .unwrap_or(template_id);
            params["name"] = json!(name);
            if let Some(server_id) = args.get("server_id").and_then(|v| v.as_i64()) {
                params["server_id"] = json!(server_id);
            }
            if !files.toml.is_empty() {
                params["env_var"] = json!(files.toml);
            }
            match invoke(ctx, "compose_install_template", params, false).await {
                InvokeOutcome::Done { body } => body,
                InvokeOutcome::UnknownAction { name } => {
                    format!("No such action '{name}'.")
                }
                InvokeOutcome::NeedsConfirmation { action, summary } => {
                    format!("CONFIRMATION_REQUIRED for {action}: {summary}")
                }
            }
        }
        "invoke" => {
            let action = args.get("action").and_then(|v| v.as_str()).unwrap_or("");
            let params = args.get("params").cloned().unwrap_or(json!({}));
            let confirmed = args
                .get("confirmed")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            match invoke(ctx, action, params, confirmed).await {
                InvokeOutcome::Done { body } => body,
                InvokeOutcome::UnknownAction { name } => {
                    format!("No such action '{name}'. Use search_actions to find valid names.")
                }
                InvokeOutcome::NeedsConfirmation { action, summary } => {
                    // The model skipped confirm_action; tell it to confirm first.
                    format!(
                        "CONFIRMATION_REQUIRED for {action}: {summary}. Call confirm_action \
                         with this summary, then invoke with confirmed=true after the user agrees."
                    )
                }
            }
        }
        other => format!("Unknown tool '{other}'."),
    }
}

/// Parse a `choose` tool-call's options array into ChoiceOptions.
fn parse_options(args: &serde_json::Value) -> Vec<ChoiceOption> {
    let mut options: Vec<ChoiceOption> = args
        .get("options")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|o| {
                    let id = o.get("id").and_then(|v| v.as_str())?;
                    let label = o.get("label").and_then(|v| v.as_str()).unwrap_or(id);
                    Some(ChoiceOption {
                        id: id.to_string(),
                        label: label.to_string(),
                        description: o
                            .get("description")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    // Always offer a way out so the user is never stuck on a choice.
    if !options.is_empty() && !options.iter().any(|o| o.id.eq_ignore_ascii_case("cancel")) {
        options.push(ChoiceOption {
            id: "cancel".into(),
            label: "Cancel".into(),
            description: String::new(),
        });
    }
    options
}

/// True if the latest user message is an affirmative confirmation.
fn user_affirmed(prompt: &str) -> bool {
    let p = prompt.trim().to_ascii_lowercase();
    const YES: &[&str] = &[
        "yes",
        "y",
        "haan",
        "ha",
        "ok",
        "okay",
        "sure",
        "confirm",
        "confirmed",
        "kardo",
        "kar do",
        "karo",
        "do it",
        "proceed",
        "go ahead",
        "yep",
        "yeah",
    ];
    YES.iter()
        .any(|w| p == *w || p.starts_with(&format!("{w} ")) || p.starts_with(&format!("{w},")))
}

/// Compact a dispatch body for a one-line outcome summary.
fn summarize(body: &str) -> String {
    let t = body.trim();
    if t.is_empty() {
        return "ok".into();
    }
    if t.len() > 160 {
        format!("{}…", &t[..160])
    } else {
        t.to_string()
    }
}

/// One live event emitted as the agent works. The controller forwards each
/// to the gRPC server-stream so the browser sees progress in real time.
#[derive(Clone, Debug)]
pub enum AgentEvent {
    /// A transient status/thinking line (e.g. "Searching actions…").
    Thinking(String),
    /// An action is being invoked.
    Status(String),
    /// An action finished; short result summary.
    ActionResult(String),
    /// The model's final text reply.
    Reply(String),
    /// A structured confirm / choose interaction.
    Interaction(Interaction),
    /// All done (terminal).
    Done,
}

/// Human-friendly one-line outcome for a confirmed action, so the final reply
/// never dumps raw response JSON (which overflowed the chat card). Pulls a
/// resource name + new status out of the response when present.
fn clean_outcome(action: &str, body: &str) -> String {
    let verb = action
        .rsplit('_')
        .next()
        .map(|v| match v {
            "stop" => "stopped",
            "start" => "started",
            "deploy" => "deploying",
            "redeploy" => "redeploying",
            "delete" | "tag" => "deleted",
            "create" => "created",
            other => other,
        })
        .unwrap_or("done");
    // Try to read name/status from the JSON response (compose/app/db share this shape).
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(body) {
        let obj = v
            .get("compose")
            .or_else(|| v.get("application"))
            .or_else(|| v.get("database"))
            .or_else(|| v.get("tag"))
            .unwrap_or(&v);
        let name = obj.get("name").and_then(|n| n.as_str());
        let status = obj.get("status").and_then(|s| s.as_str());
        return match (name, status) {
            (Some(n), Some(s)) => format!("{n}: {verb} ({s})"),
            (Some(n), None) => format!("{n}: {verb}"),
            _ => format!("{action}: {verb}"),
        };
    }
    // Non-JSON (e.g. an error string) — surface it, trimmed.
    let t = body.trim();
    if t.is_empty() {
        format!("{action}: {verb}")
    } else {
        format!("{action}: {}", &t[..t.len().min(100)])
    }
}

/// Human-friendly status line for a tool call (shown as a live progress step).
fn friendly_tool_label(name: &str, args: &serde_json::Value) -> String {
    match name {
        "search_actions" => {
            let q = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
            format!("🔍 Searching actions for \"{q}\"…")
        }
        "invoke" => {
            let a = args
                .get("action")
                .and_then(|v| v.as_str())
                .unwrap_or("action");
            format!("⚡ Running {a}…")
        }
        "list_templates" => "📦 Loading templates…".into(),
        "install_template" => {
            let t = args
                .get("template_id")
                .and_then(|v| v.as_str())
                .unwrap_or("template");
            format!("📦 Installing {t}…")
        }
        other => format!("⚙️ {other}…"),
    }
}

/// Drive the completion loop against a concrete provider model, emitting
/// live events to `tx` as it works (thinking, status, results, reply).
pub async fn run<M: CompletionModel>(
    model: M,
    ctx: &UserContext,
    prior_turns: &[(String, String)],
    pending_actions: &str,
    user_prompt: &str,
    tx: Option<tokio::sync::mpsc::Sender<AgentEvent>>,
) -> Result<AgentReply, String> {
    // Best-effort event emission; never blocks or fails the loop if the consumer went away.
    macro_rules! emit {
        ($event:expr) => {
            if let Some(tx) = &tx {
                let _ = tx.send($event).await;
            }
        };
    }

    // Deterministic confirmation: run the actions the previous turn proposed instead of re-consulting the model.
    if user_affirmed(user_prompt) && !pending_actions.trim().is_empty() {
        if let Ok(actions) = serde_json::from_str::<Vec<serde_json::Value>>(pending_actions) {
            if !actions.is_empty() {
                emit!(AgentEvent::Thinking(
                    "Confirmed — running the action(s)…".into()
                ));
                let mut outcomes = Vec::new();
                for a in &actions {
                    let action = a.get("action").and_then(|v| v.as_str()).unwrap_or("");
                    let params = a.get("params").cloned().unwrap_or_else(|| json!({}));
                    emit!(AgentEvent::Status(format!("Running {action}…")));
                    match invoke(ctx, action, params, true).await {
                        InvokeOutcome::Done { body } => {
                            let line = clean_outcome(action, &body);
                            emit!(AgentEvent::ActionResult(line.clone()));
                            outcomes.push(line)
                        }
                        InvokeOutcome::UnknownAction { name } => {
                            outcomes.push(format!("{name}: not available"))
                        }
                        InvokeOutcome::NeedsConfirmation { action, .. } => {
                            outcomes.push(format!("{action}: still needs confirmation"))
                        }
                    }
                }
                let reply_text = if outcomes.len() == 1 {
                    format!("Done. {}", outcomes[0])
                } else {
                    format!(
                        "Done ({} actions):\n- {}",
                        outcomes.len(),
                        outcomes.join("\n- ")
                    )
                };
                emit!(AgentEvent::Reply(reply_text.clone()));
                emit!(AgentEvent::Done);
                return Ok(AgentReply {
                    text: reply_text,
                    interaction: None,
                });
            }
        }
    }
    let defs = tool_definitions();
    // Registered route set: used to hide phantom proto-only RPCs from search.
    let registered = super::invoke::registered_paths().await;
    // One ordered transcript: rig takes the last message as the prompt and the rest as history, so each tool result sits right after the assistant turn that called it.
    let mut history: Vec<Message> = Vec::with_capacity(prior_turns.len() + 1);
    for (role, content) in prior_turns {
        if content.trim().is_empty() {
            continue;
        }
        history.push(if role.eq_ignore_ascii_case("assistant") {
            Message::assistant(content)
        } else {
            Message::user(content)
        });
    }
    history.push(Message::user(user_prompt));
    let mut flags = CompatFlags::default();

    for step in 0..MAX_STEPS {
        emit!(AgentEvent::Thinking(if step == 0 {
            "Thinking…".into()
        } else {
            "Processing…".into()
        }));
        let (prompt, prior) = history.split_last().expect("history is never empty");
        // Retry the same turn with the corrective flag the rejection asks for; each flag flips at most once.
        let response = loop {
            match complete_once(&model, prompt, prior, &defs, flags).await {
                Ok(r) => break r,
                Err(e) if flags.adapt(&e) => continue,
                Err(e) => return Err(format!("AI completion failed: {e}")),
            }
        };

        // Collect text + tool calls from the model's turn.
        let mut text_parts = Vec::new();
        let mut tool_calls = Vec::new();
        for content in response.choice.iter() {
            match content {
                AssistantContent::Text(t) => text_parts.push(t.text.clone()),
                AssistantContent::ToolCall(tc) => tool_calls.push(tc.clone()),
                _ => {}
            }
        }

        // No tool calls → the model answered. Done.
        if tool_calls.is_empty() {
            let text = text_parts.join("\n").trim().to_string();
            let reply_text = if text.is_empty() {
                "Done.".to_string()
            } else {
                text
            };
            emit!(AgentEvent::Reply(reply_text.clone()));
            emit!(AgentEvent::Done);
            return Ok(AgentReply {
                text: reply_text,
                interaction: None,
            });
        }

        // Terminating interaction tools hand control back to the user with structured UI: honor the first and stop.
        for tc in &tool_calls {
            let args = &tc.function.arguments;
            match tc.function.name.as_str() {
                "ask_user" => {
                    let q = args
                        .get("question")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Could you clarify?")
                        .to_string();
                    emit!(AgentEvent::Reply(q.clone()));
                    emit!(AgentEvent::Done);
                    return Ok(AgentReply {
                        text: q,
                        interaction: None,
                    });
                }
                "confirm_action" => {
                    let summary = args
                        .get("summary")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Confirm this action?")
                        .to_string();
                    let pending_actions = args
                        .get("actions")
                        .map(|a| a.to_string())
                        .unwrap_or_else(|| "[]".to_string());
                    let interaction = Interaction {
                        kind: InteractionKind::Confirm,
                        question: summary.clone(),
                        options: Vec::new(),
                        pending_actions,
                    };
                    emit!(AgentEvent::Interaction(interaction.clone()));
                    emit!(AgentEvent::Done);
                    return Ok(AgentReply {
                        text: summary,
                        interaction: Some(interaction),
                    });
                }
                "choose" => {
                    let question = args
                        .get("question")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Please choose:")
                        .to_string();
                    let interaction = Interaction {
                        kind: InteractionKind::Choice,
                        question: question.clone(),
                        options: parse_options(args),
                        pending_actions: String::new(),
                    };
                    emit!(AgentEvent::Interaction(interaction.clone()));
                    emit!(AgentEvent::Done);
                    return Ok(AgentReply {
                        text: question,
                        interaction: Some(interaction),
                    });
                }
                _ => {}
            }
        }

        // Record the assistant's tool-call turn as returned, so tool-call ids line up with the results appended next.
        history.push(Message::Assistant {
            id: None,
            content: OneOrMany::many(response.choice.iter().cloned().collect::<Vec<_>>())
                .unwrap_or_else(|_| {
                    OneOrMany::one(AssistantContent::Text(text_parts.join("\n").into()))
                }),
        });

        let mut results: Vec<UserContent> = Vec::new();
        for tc in &tool_calls {
            let label = friendly_tool_label(&tc.function.name, &tc.function.arguments);
            emit!(AgentEvent::Status(label));
            let out = run_tool(ctx, &tc.function.name, &tc.function.arguments, &registered).await;
            emit!(AgentEvent::ActionResult(summarize(&out)));
            results.push(UserContent::tool_result(
                tc.id.clone(),
                OneOrMany::one(rig_core::message::ToolResultContent::text(out)),
            ));
        }

        history.push(Message::User {
            content: OneOrMany::many(results).map_err(|_| "empty tool results".to_string())?,
        });
    }

    let fallback =
        "I couldn't complete that in a reasonable number of steps. Please narrow the request.";
    emit!(AgentEvent::Reply(fallback.into()));
    emit!(AgentEvent::Done);
    Ok(AgentReply {
        text: fallback.into(),
        interaction: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rig_core::completion::{CompletionError, CompletionRequest, CompletionResponse, Usage};
    use rig_core::message::{AssistantContent, Message, Text, ToolCall, ToolFunction, UserContent};
    use std::sync::{Arc, Mutex};

    #[test]
    fn user_affirmed_detects_yes_variants() {
        for yes in [
            "yes",
            "Yes",
            "haan",
            "ok",
            "confirm",
            "kardo",
            "kar do",
            "yes please",
            "sure, go",
        ] {
            assert!(user_affirmed(yes), "{yes:?} should be affirmative");
        }
        for no in [
            "no",
            "cancel",
            "wait",
            "not now",
            "stop",
            "nahi",
            "what else",
        ] {
            assert!(!user_affirmed(no), "{no:?} should NOT be affirmative");
        }
    }

    #[test]
    fn compat_detectors_match_real_errors() {
        // gpt-6-sol: function tools + default reasoning effort on chat/completions.
        let reasoning = "Function tools with reasoning_effort are not supported for \
                         gpt-6-sol in /v1/chat/completions. To use function tools, use \
                         /v1/responses or set reasoning_effort to 'none'.";
        assert!(wants_reasoning_none(reasoning));
        assert!(!wants_completion_tokens(reasoning));

        // newer reasoning models rejecting max_tokens.
        let tokens = "Unsupported parameter: 'max_tokens' is not supported with this \
                      model. Use 'max_completion_tokens' instead.";
        assert!(wants_completion_tokens(tokens));
        assert!(!wants_reasoning_none(tokens));

        // adapt() flips the right flag and reports change only once.
        let mut f = CompatFlags::default();
        assert!(f.adapt(reasoning) && f.reasoning_none && !f.completion_tokens);
        assert!(!f.adapt(reasoning)); // already set → no new change
        assert!(f.adapt(tokens) && f.completion_tokens);
        assert!(!f.adapt("rate limit exceeded"));
    }

    /// Mock model: `search_actions` tool call on the first turn, plain text after, recording each request's history.
    #[derive(Clone)]
    struct MockModel {
        calls: Arc<Mutex<usize>>,
        seen: Arc<Mutex<Vec<Vec<Message>>>>,
    }

    impl CompletionModel for MockModel {
        type Response = ();
        type StreamingResponse = ();
        type Client = ();

        fn make(_client: &Self::Client, _model: impl Into<String>) -> Self {
            unreachable!("mock is constructed directly")
        }

        async fn completion(
            &self,
            request: CompletionRequest,
        ) -> Result<CompletionResponse<Self::Response>, CompletionError> {
            self.seen
                .lock()
                .unwrap()
                .push(request.chat_history.into_iter().collect());
            let mut calls = self.calls.lock().unwrap();
            *calls += 1;
            let first = *calls == 1;
            let choice = if first {
                OneOrMany::one(AssistantContent::ToolCall(ToolCall::new(
                    "call_1".into(),
                    ToolFunction::new("search_actions".into(), json!({"query":"projects"})),
                )))
            } else {
                OneOrMany::one(AssistantContent::Text(Text::new("All done.")))
            };
            Ok(CompletionResponse {
                choice,
                usage: Usage::new(),
                raw_response: (),
                message_id: None,
            })
        }

        async fn stream(
            &self,
            _request: CompletionRequest,
        ) -> Result<
            rig_core::streaming::StreamingCompletionResponse<Self::StreamingResponse>,
            CompletionError,
        > {
            unreachable!("streaming is not used by the agent")
        }
    }

    /// Mock that fails the first two attempts with the max_tokens and reasoning_effort errors, then succeeds, recording each attempt's additional_params.
    #[derive(Clone)]
    struct RetryMock {
        attempts: Arc<Mutex<usize>>,
        params: Arc<Mutex<Vec<Option<serde_json::Value>>>>,
    }

    impl CompletionModel for RetryMock {
        type Response = ();
        type StreamingResponse = ();
        type Client = ();

        fn make(_c: &Self::Client, _m: impl Into<String>) -> Self {
            unreachable!()
        }

        async fn completion(
            &self,
            request: CompletionRequest,
        ) -> Result<CompletionResponse<Self::Response>, CompletionError> {
            self.params
                .lock()
                .unwrap()
                .push(request.additional_params.clone());
            let mut n = self.attempts.lock().unwrap();
            *n += 1;
            match *n {
                // First attempt: classic max_tokens, no reasoning_effort.
                1 => Err(CompletionError::ProviderError(
                    "Unsupported parameter: 'max_tokens' is not supported with this \
                     model. Use 'max_completion_tokens' instead."
                        .into(),
                )),
                2 => Err(CompletionError::ProviderError(
                    "Function tools with reasoning_effort are not supported for \
                     gpt-6-sol in /v1/chat/completions. Set reasoning_effort to 'none'."
                        .into(),
                )),
                _ => Ok(CompletionResponse {
                    choice: OneOrMany::one(AssistantContent::Text(Text::new("ok"))),
                    usage: Usage::new(),
                    raw_response: (),
                    message_id: None,
                }),
            }
        }

        async fn stream(
            &self,
            _r: CompletionRequest,
        ) -> Result<
            rig_core::streaming::StreamingCompletionResponse<Self::StreamingResponse>,
            CompletionError,
        > {
            unreachable!()
        }
    }

    #[tokio::test]
    async fn retries_adapt_token_and_reasoning_params() {
        let model = RetryMock {
            attempts: Arc::new(Mutex::new(0)),
            params: Arc::new(Mutex::new(Vec::new())),
        };
        let ctx = UserContext {
            token: "t".into(),
            organization_id: 1,
            user_id: 1,
        };
        let reply = run(model.clone(), &ctx, &[], "", "hi", None)
            .await
            .expect("should recover");
        assert_eq!(reply.text, "ok");

        let params = model.params.lock().unwrap();
        assert_eq!(params.len(), 3, "expected two failed attempts then success");
        // Attempt 1: no additional_params (classic max_tokens path).
        assert!(params[0].is_none());
        // Attempt 2: max_completion_tokens added after the first rejection.
        let p2 = params[1].as_ref().expect("attempt 2 has params");
        assert_eq!(p2["max_completion_tokens"], json!(TOKEN_BUDGET));
        assert!(p2.get("reasoning_effort").is_none());
        // Attempt 3: both flags now present.
        let p3 = params[2].as_ref().expect("attempt 3 has params");
        assert_eq!(p3["max_completion_tokens"], json!(TOKEN_BUDGET));
        assert_eq!(p3["reasoning_effort"], json!("none"));
    }

    /// True if no `tool` result appears without an immediately-preceding assistant tool-call turn.
    fn tool_results_are_well_ordered(history: &[Message]) -> bool {
        for (i, msg) in history.iter().enumerate() {
            let is_tool_result = matches!(msg, Message::User { content }
                if content.iter().any(|c| matches!(c, UserContent::ToolResult(_))));
            if !is_tool_result {
                continue;
            }
            let Some(prev) = i.checked_sub(1).and_then(|j| history.get(j)) else {
                return false;
            };
            let prev_has_tool_call = matches!(prev, Message::Assistant { content, .. }
                if content.iter().any(|c| matches!(c, AssistantContent::ToolCall(_))));
            if !prev_has_tool_call {
                return false;
            }
        }
        true
    }

    #[tokio::test]
    async fn tool_result_follows_assistant_tool_call_turn() {
        let model = MockModel {
            calls: Arc::new(Mutex::new(0)),
            seen: Arc::new(Mutex::new(Vec::new())),
        };
        let ctx = UserContext {
            token: "t".into(),
            organization_id: 1,
            user_id: 1,
        };

        let reply = run(model.clone(), &ctx, &[], "", "what can you do?", None)
            .await
            .expect("agent loop should complete");
        assert_eq!(reply.text, "All done.");

        // The model was called twice (tool turn → final text).
        assert_eq!(*model.calls.lock().unwrap(), 2);

        // Every transcript the provider would have received is well-ordered.
        let seen = model.seen.lock().unwrap();
        assert_eq!(seen.len(), 2);
        for history in seen.iter() {
            assert!(
                tool_results_are_well_ordered(history),
                "tool result not preceded by an assistant tool-call turn: {history:?}"
            );
        }
        // The second request must keep the assistant tool-call turn followed by its tool result.
        let second = &seen[1];
        assert!(
            second
                .iter()
                .any(|m| matches!(m, Message::Assistant { content, .. }
                if content.iter().any(|c| matches!(c, AssistantContent::ToolCall(_))))),
            "second request lost the assistant tool-call turn: {second:?}"
        );
    }

    /// Mock that immediately calls the `choose` tool with two options.
    #[derive(Clone)]
    struct ChooseMock {
        seen: Arc<Mutex<Vec<Vec<Message>>>>,
    }

    impl CompletionModel for ChooseMock {
        type Response = ();
        type StreamingResponse = ();
        type Client = ();
        fn make(_c: &Self::Client, _m: impl Into<String>) -> Self {
            unreachable!()
        }
        async fn completion(
            &self,
            request: CompletionRequest,
        ) -> Result<CompletionResponse<Self::Response>, CompletionError> {
            self.seen
                .lock()
                .unwrap()
                .push(request.chat_history.into_iter().collect());
            Ok(CompletionResponse {
                choice: OneOrMany::one(AssistantContent::ToolCall(ToolCall::new(
                    "c1".into(),
                    ToolFunction::new(
                        "choose".into(),
                        json!({
                            "question": "Which project?",
                            "options": [
                                { "id": "1", "label": "test", "description": "#work" },
                                { "id": "2", "label": "prod" }
                            ]
                        }),
                    ),
                ))),
                usage: Usage::new(),
                raw_response: (),
                message_id: None,
            })
        }
        async fn stream(
            &self,
            _r: CompletionRequest,
        ) -> Result<
            rig_core::streaming::StreamingCompletionResponse<Self::StreamingResponse>,
            CompletionError,
        > {
            unreachable!()
        }
    }

    #[tokio::test]
    async fn choose_tool_returns_choice_interaction_and_history_is_seeded() {
        let model = ChooseMock {
            seen: Arc::new(Mutex::new(Vec::new())),
        };
        let ctx = UserContext {
            token: "t".into(),
            organization_id: 1,
            user_id: 1,
        };
        let prior = vec![
            ("user".to_string(), "deploy something".to_string()),
            ("assistant".to_string(), "which one?".to_string()),
        ];
        let reply = run(model.clone(), &ctx, &prior, "", "the app", None)
            .await
            .expect("should return an interaction");

        // A CHOICE interaction with the two options (+ an auto Cancel) is surfaced.
        let it = reply.interaction.expect("expected an interaction");
        assert!(matches!(it.kind, InteractionKind::Choice));
        assert_eq!(it.question, "Which project?");
        assert_eq!(it.options.len(), 3);
        assert_eq!(it.options[0].id, "1");
        assert_eq!(it.options[0].label, "test");
        assert_eq!(it.options[0].description, "#work");
        // The last option is the auto-appended Cancel.
        assert_eq!(it.options.last().unwrap().id, "cancel");

        // Prior turns are seeded ahead of the latest prompt (rig adds the preamble as a system message).
        let seen = model.seen.lock().unwrap();
        let non_system = seen[0]
            .iter()
            .filter(|m| !matches!(m, Message::System { .. }))
            .count();
        assert_eq!(non_system, 3, "2 prior turns + the latest prompt");
    }
}
