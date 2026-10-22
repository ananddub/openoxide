//! AI-generated Dockerfile build strategy.
//!
//! Reads the repo's (noise-filtered, depth-capped) file tree + key manifests,
//! asks the configured AI provider to produce a nested-aware Dockerfile, writes
//! it as `Dockerfile.openoxide`, builds it, and on failure feeds the build
//! error + current Dockerfile back to the model to revise and retry (cap 3).
//!
//! It NEVER edits repo source — only the generated Dockerfile and build-args.

use crate::utils::ai::{AiClient, AiProviderConfig};
use crate::utils::exec::CommandExecutor;
use crate::utils::os::OsCli;

/// Directories that never help the model and would bloat the prompt.
const IGNORE_DIRS: &[&str] = &[
    "node_modules",
    ".git",
    "target",
    "dist",
    "build",
    "vendor",
    ".next",
    ".cache",
    "__pycache__",
    ".venv",
    "venv",
    ".idea",
    ".gradle",
];

/// Manifests whose contents strongly signal the language/build. Read (small)
/// so the model can pick the right base image, install, and start commands.
const MANIFEST_NAMES: &[&str] = &[
    "package.json",
    "package-lock.json",
    "pnpm-lock.yaml",
    "yarn.lock",
    "bun.lockb",
    "go.mod",
    "requirements.txt",
    "pyproject.toml",
    "Pipfile",
    "Cargo.toml",
    "pom.xml",
    "build.gradle",
    "Gemfile",
    "composer.json",
    "mix.exs",
    "Procfile",
    ".nvmrc",
    ".python-version",
    "Dockerfile",
    "docker-compose.yml",
    "tsconfig.json",
    "next.config.js",
    "vite.config.ts",
    "nest-cli.json",
];

const MAX_DEPTH: u32 = 4;
const MAX_TREE_LINES: usize = 400;
const MAX_MANIFEST_BYTES: usize = 4_000;

/// A compact repo summary for the model: a filtered file tree + key manifest
/// contents, enough to detect nested/monorepo layouts.
pub struct RepoContext {
    pub tree: String,
    pub manifests: String,
    /// Stable hash of the manifest set (names + contents) for caching.
    pub signature: String,
}

/// Walk the work directory and gather a filtered tree + manifest contents.
pub async fn read_repo_context(
    executor: &CommandExecutor,
    work_directory: &str,
) -> Result<RepoContext, String> {
    let os = OsCli::new(executor);

    // 1. File listing (relative paths), depth-capped, type=file.
    let walk = os
        .dir(work_directory)
        .walk()
        .max_depth(MAX_DEPTH)
        .type_file()
        .ignore_errors()
        .run()
        .await
        .map_err(|e| format!("could not read repo tree: {e}"))?;

    let prefix = format!("{}/", work_directory.trim_end_matches('/'));
    let mut rel_paths: Vec<String> = walk
        .stdout
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.strip_prefix(&prefix).unwrap_or(l).to_string())
        .filter(|p| {
            !IGNORE_DIRS
                .iter()
                .any(|d| p.split('/').any(|seg| seg == *d))
        })
        .collect();
    rel_paths.sort();
    rel_paths.truncate(MAX_TREE_LINES);
    let tree = rel_paths.join("\n");

    // 2. Read key manifests (anywhere in the tree, nested included).
    let mut manifests = String::new();
    let mut sig_input = String::new();
    let mut read = 0;
    for rel in &rel_paths {
        let base = rel.rsplit('/').next().unwrap_or(rel);
        if !MANIFEST_NAMES.contains(&base) {
            continue;
        }
        if read >= 12 {
            break; // cap number of manifests
        }
        let abs = format!("{}/{}", work_directory.trim_end_matches('/'), rel);
        if let Ok(out) = os.file(abs).read().execute().await {
            let mut content = out.stdout;
            if content.len() > MAX_MANIFEST_BYTES {
                content.truncate(MAX_MANIFEST_BYTES);
                content.push_str("\n…(truncated)");
            }
            manifests.push_str(&format!("\n### {rel}\n```\n{content}\n```\n"));
            sig_input.push_str(rel);
            sig_input.push_str(&content);
            read += 1;
        }
    }

    let signature = simple_hash(&sig_input);
    Ok(RepoContext {
        tree,
        manifests,
        signature,
    })
}

/// Ask the model to generate (or fix) a Dockerfile for this repo.
/// `previous` + `error` are set on retry so the model can correct itself.
pub async fn generate_dockerfile(
    config: &AiProviderConfig,
    repo: &RepoContext,
    previous: Option<&str>,
    error: Option<&str>,
) -> Result<String, String> {
    let mut prompt = String::new();
    prompt.push_str(
        "You are an expert DevOps engineer. Produce a single, production-ready \
         Dockerfile that builds and runs this repository. Rules:\n\
         - Output ONLY the Dockerfile content — no prose, no markdown fences.\n\
         - Detect the language/framework from the manifests below.\n\
         - Handle NESTED / MONOREPO layouts: if the app lives in a subfolder \
           (e.g. backend/, apps/web/), set WORKDIR and COPY paths accordingly; \
           the build context is the repo root.\n\
         - Use a sensible, slim official base image and a multi-stage build when \
           it reduces image size.\n\
         - Install dependencies, build if needed, then define the correct CMD/\
           ENTRYPOINT to start the app. EXPOSE the port the app listens on.\n\
         - Do not assume files that aren't listed. Do not modify app source.\n\n",
    );
    prompt.push_str(&format!("## Repo file tree\n{}\n", repo.tree));
    prompt.push_str(&format!("\n## Key manifests{}\n", repo.manifests));

    if let (Some(prev), Some(err)) = (previous, error) {
        prompt.push_str(&format!(
            "\n## The previous Dockerfile FAILED to build.\n\
             Fix it. Previous Dockerfile:\n```\n{prev}\n```\n\n\
             Build error:\n```\n{}\n```\n\n\
             Return a corrected Dockerfile (content only).",
            tail(err, 3_000)
        ));
    }

    let raw = AiClient
        .complete_text(config, &prompt)
        .await
        .map_err(|e| format!("AI Dockerfile generation failed: {e}"))?;
    Ok(strip_fences(&raw))
}

/// Keep only the last `n` chars of a (possibly huge) build error.
fn tail(s: &str, n: usize) -> &str {
    if s.len() <= n { s } else { &s[s.len() - n..] }
}

/// Remove ```dockerfile / ``` fences if the model added them despite instructions.
fn strip_fences(text: &str) -> String {
    let t = text.trim();
    let t = t
        .strip_prefix("```dockerfile")
        .or_else(|| t.strip_prefix("```Dockerfile"))
        .or_else(|| t.strip_prefix("```"))
        .unwrap_or(t);
    let t = t.strip_suffix("```").unwrap_or(t);
    t.trim().to_string()
}

/// Small, stable, dependency-free hash (FNV-1a) for the cache key.
fn simple_hash(s: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}")
}

/// Return a cached working Dockerfile for this repo signature, if any.
pub async fn cache_lookup(signature: &str) -> Option<String> {
    let pool = auto_di::resolve::<sqlx::SqlitePool>().await.ok()?;
    sqlx::query_scalar::<_, String>(
        "SELECT dockerfile FROM ai_dockerfile_cache WHERE signature = ?",
    )
    .bind(signature)
    .fetch_optional(&*pool)
    .await
    .ok()
    .flatten()
}

/// Store a working Dockerfile for this repo signature (upsert). Best-effort.
pub async fn cache_store(signature: &str, dockerfile: &str) {
    let Ok(pool) = auto_di::resolve::<sqlx::SqlitePool>().await else {
        return;
    };
    let now = chrono::Utc::now().timestamp();
    let _ = sqlx::query(
        "INSERT INTO ai_dockerfile_cache (signature, dockerfile, updated_at) \
         VALUES (?, ?, ?) \
         ON CONFLICT(signature) DO UPDATE SET dockerfile = excluded.dockerfile, \
         updated_at = excluded.updated_at",
    )
    .bind(signature)
    .bind(dockerfile)
    .bind(now)
    .execute(&*pool)
    .await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_fences_removes_markdown() {
        assert_eq!(
            strip_fences("```dockerfile\nFROM node:20\n```"),
            "FROM node:20"
        );
        assert_eq!(strip_fences("FROM go:1.22"), "FROM go:1.22");
    }

    #[test]
    fn hash_is_stable_and_differs() {
        assert_eq!(simple_hash("a"), simple_hash("a"));
        assert_ne!(simple_hash("a"), simple_hash("b"));
    }

    #[test]
    fn tail_trims_long_errors() {
        let s = "x".repeat(5000);
        assert_eq!(tail(&s, 100).len(), 100);
        assert_eq!(tail("short", 100), "short");
    }
}
