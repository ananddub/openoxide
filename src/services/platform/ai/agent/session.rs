//! Server-side conversation session store backed by one markdown file per
//! session under `/app/data/ai-sessions/{session_id}.md`. Each turn is
//! appended as a `## role` section. On read the turns are parsed back into
//! `(role, content)` pairs for the runner's `prior_turns`. Compaction
//! rewrites old turns into a `## summary` block when the file exceeds a
//! token-budget, keeping only a sliding window of recent turns.

use std::path::{Path, PathBuf};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

const BASE_DIR: &str = "/app/data/ai-sessions";
/// Approximate chars-to-tokens ratio (1 token ≈ 4 chars).
const CHARS_PER_TOKEN: usize = 4;
/// Max tokens fed to the LLM from history (leaves room for the preamble + prompt + tools).
const TOKEN_BUDGET: usize = 60_000;
/// Sliding window: always keep the last N turns regardless of budget.
const KEEP_RECENT: usize = 15;
/// Sessions older than this are eligible for cleanup.
const TTL_SECS: u64 = 7 * 24 * 3600; // 7 days

fn session_path(session_id: &str) -> PathBuf {
    Path::new(BASE_DIR).join(format!("{session_id}.md"))
}

/// Create a new session, returning its id. Does NOT write a file yet (first
/// append will).
pub fn create_session_id() -> String {
    Uuid::new_v4().to_string()
}

/// Append one turn to the session file (fire-and-forget friendly).
pub async fn append_turn(session_id: &str, role: &str, content: &str) -> std::io::Result<()> {
    let path = session_path(session_id);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }
    let block = format!("\n## {role}\n{content}\n");
    fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .await?
        .write_all(block.as_bytes())
        .await?;
    Ok(())
}

/// Read the session file and return parsed turns `(role, content)`. Returns
/// an empty vec if the file doesn't exist.
pub async fn read_turns(session_id: &str) -> Vec<(String, String)> {
    let path = session_path(session_id);
    let text = match fs::read_to_string(&path).await {
        Ok(t) => t,
        Err(_) => return Vec::new(),
    };
    parse_turns(&text)
}

fn parse_turns(text: &str) -> Vec<(String, String)> {
    let mut turns = Vec::new();
    let mut role = String::new();
    let mut content = String::new();
    for line in text.lines() {
        if let Some(r) = line.strip_prefix("## ") {
            if !role.is_empty() {
                turns.push((role.clone(), content.trim().to_string()));
            }
            role = r.trim().to_lowercase();
            content.clear();
        } else {
            content.push_str(line);
            content.push('\n');
        }
    }
    if !role.is_empty() && !content.trim().is_empty() {
        turns.push((role, content.trim().to_string()));
    }
    turns
}

/// Return the turns that should be fed to the LLM (within token budget).
/// Older turns beyond the budget are dropped (the summary block, if present,
/// is always kept). Always keeps the last `KEEP_RECENT` turns.
pub fn windowed_turns(turns: &[(String, String)]) -> Vec<(String, String)> {
    if turns.is_empty() {
        return Vec::new();
    }
    let total_chars: usize = turns.iter().map(|(_, c)| c.len()).sum();
    let budget_chars = TOKEN_BUDGET * CHARS_PER_TOKEN;
    if total_chars <= budget_chars {
        return turns.to_vec();
    }
    // Always include the first turn if it's a summary block.
    let mut result = Vec::new();
    let start = if turns[0].0 == "summary" { 1 } else { 0 };
    if start == 1 {
        result.push(turns[0].clone());
    }
    // Then the last KEEP_RECENT turns (or fewer if not enough).
    let recent_start = turns.len().saturating_sub(KEEP_RECENT).max(start);
    result.extend_from_slice(&turns[recent_start..]);
    result
}

/// Check whether the session should be compacted (older turns → one summary).
/// Returns true if total chars exceed the budget and there are enough turns
/// beyond the window to be worth summarizing.
pub fn needs_compaction(turns: &[(String, String)]) -> bool {
    let total_chars: usize = turns.iter().map(|(_, c)| c.len()).sum();
    let budget_chars = TOKEN_BUDGET * CHARS_PER_TOKEN;
    total_chars > budget_chars && turns.len() > KEEP_RECENT + 2
}

/// Compact the session: summarize old turns into a `## summary` block and
/// keep only the recent window. `summary_text` is produced by the caller
/// (an LLM call). Rewrites the file.
pub async fn compact(
    session_id: &str,
    turns: &[(String, String)],
    summary_text: &str,
) -> std::io::Result<()> {
    let recent_start = turns.len().saturating_sub(KEEP_RECENT);
    let mut out = format!("## summary\n{summary_text}\n");
    for (role, content) in &turns[recent_start..] {
        out.push_str(&format!("\n## {role}\n{content}\n"));
    }
    let path = session_path(session_id);
    fs::write(&path, out.as_bytes()).await
}

/// Delete sessions older than TTL_SECS. Call from a background task.
pub async fn cleanup_old_sessions() {
    let Ok(mut entries) = fs::read_dir(BASE_DIR).await else {
        return;
    };
    let cutoff = std::time::SystemTime::now()
        .checked_sub(std::time::Duration::from_secs(TTL_SECS))
        .unwrap_or(std::time::SystemTime::UNIX_EPOCH);
    while let Ok(Some(entry)) = entries.next_entry().await {
        if let Ok(meta) = entry.metadata().await {
            if let Ok(modified) = meta.modified() {
                if modified < cutoff {
                    let _ = fs::remove_file(entry.path()).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_turns_round_trip() {
        let md = "## user\nhello\n\n## assistant\nhi there\n\n## user\ndeploy blinko\n";
        let turns = parse_turns(md);
        assert_eq!(turns.len(), 3);
        assert_eq!(turns[0], ("user".into(), "hello".into()));
        assert_eq!(turns[1], ("assistant".into(), "hi there".into()));
        assert_eq!(turns[2], ("user".into(), "deploy blinko".into()));
    }

    #[test]
    fn windowed_trims_to_budget() {
        let big: Vec<(String, String)> = (0..100)
            .map(|i| ("user".into(), "x".repeat(5000)))
            .collect();
        let w = windowed_turns(&big);
        assert!(w.len() <= KEEP_RECENT + 1);
    }

    #[test]
    fn empty_session_returns_empty() {
        assert!(parse_turns("").is_empty());
        assert!(windowed_turns(&[]).is_empty());
    }
}
