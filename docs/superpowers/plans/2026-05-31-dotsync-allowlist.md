# DotSync Allowlist Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add optional DotSync setup support that detects curated AI-agent dotfiles, prompts the user, and creates safe Mutagen sync roots with exclusions.

**Architecture:** Add a focused `dotsync` module that owns target metadata, detection, selection defaults, path expansion, and conversion into sync roots. Extend existing sync models and Mutagen argument generation to carry ignore patterns and DotSync labels without changing the ordinary sync workflow. Wire DotSync into client setup after normal sync roots are collected, then surface DotSync targets in status output.

**Tech Stack:** Rust, Clap, Dialoguer, Serde, Mutagen CLI, existing EternalMac config/state store and fake-runner tests.

---

## File Structure

- Create `src/dotsync/mod.rs`: module entry point.
- Create `src/dotsync/allowlist.rs`: DotSync target definitions, standard exclusions, detection, path expansion, and sync-root conversion.
- Modify `src/lib.rs`: export the `dotsync` module.
- Modify `src/model/config.rs`: add backward-compatible sync metadata fields.
- Modify `src/model/state.rs`: add backward-compatible sync metadata fields for status rendering.
- Modify `src/setup/client.rs`: carry ignore paths and DotSync metadata through setup and Mutagen creation.
- Modify `src/setup/prompts.rs`: add DotSync enable and per-target prompts.
- Modify `src/commands/setup.rs`: collect DotSync setup choices and append generated roots to normal roots.
- Modify `src/tooling/mutagen.rs`: add `--ignore` support while preserving the existing no-ignore helper.
- Modify `src/status/service.rs`: render DotSync targets separately from ordinary sync roots.
- Create `tests/dotsync.rs`: cover allowlist, detection, expansion, exclusions, and sync-root conversion.
- Modify `tests/setup.rs`, `tests/sync.rs`, and `tests/status.rs`: cover integration behavior and model changes.
- Modify `README.md`: add a lean DotSync mention under setup/usage.

---

### Task 1: Add DotSync Allowlist Module

**Files:**
- Create: `src/dotsync/mod.rs`
- Create: `src/dotsync/allowlist.rs`
- Modify: `src/lib.rs`
- Test: `tests/dotsync.rs`

- [ ] **Step 1: Write failing allowlist tests**

Create `tests/dotsync.rs`:

```rust
use std::fs;

use eternalmac::dotsync::allowlist::{
    build_dotsync_root, detect_existing_targets, expand_home_path, standard_exclusions, targets,
    DotSyncRisk,
};

#[test]
fn allowlist_has_unique_stable_ids() {
    let mut ids = targets().iter().map(|target| target.id).collect::<Vec<_>>();
    ids.sort_unstable();
    ids.dedup();

    assert_eq!(ids.len(), targets().len());
    assert!(ids.contains(&"claude"));
    assert!(ids.contains(&"codex"));
    assert!(ids.contains(&"opencode"));
    assert!(ids.contains(&"goose"));
    assert!(ids.contains(&"gemini"));
    assert!(ids.contains(&"qwen"));
    assert!(ids.contains(&"pi"));
    assert!(ids.contains(&"amp"));
}

#[test]
fn safe_defaults_and_caution_targets_match_design() {
    let safe_defaults = targets()
        .iter()
        .filter(|target| target.default_selected)
        .map(|target| target.id)
        .collect::<Vec<_>>();
    let caution = targets()
        .iter()
        .filter(|target| target.risk == DotSyncRisk::Caution)
        .map(|target| target.id)
        .collect::<Vec<_>>();

    assert_eq!(
        safe_defaults,
        vec!["claude", "codex", "opencode", "goose", "gemini", "qwen", "pi", "amp"]
    );
    assert_eq!(caution, vec!["continue", "aider", "cline", "roo"]);
}

#[test]
fn standard_exclusions_include_credentials_machine_identity_and_noise() {
    let exclusions = standard_exclusions();

    assert!(exclusions.contains(&".DS_Store"));
    assert!(exclusions.contains(&"auth.json"));
    assert!(exclusions.contains(&"credentials.json"));
    assert!(exclusions.contains(&"token.json"));
    assert!(exclusions.contains(&"tokens.json"));
    assert!(exclusions.contains(&"installation_id"));
    assert!(exclusions.contains(&"cache/"));
    assert!(exclusions.contains(&"tmp/"));
    assert!(exclusions.contains(&"telemetry/"));
    assert!(exclusions.contains(&"*.sqlite-wal"));
    assert!(exclusions.contains(&"*.sqlite-shm"));
}

#[test]
fn expands_home_relative_paths_without_touching_user_home() {
    let home = tempfile::tempdir().unwrap();

    assert_eq!(expand_home_path(home.path(), "~/.claude"), home.path().join(".claude"));
    assert_eq!(
        expand_home_path(home.path(), "~/.config/opencode"),
        home.path().join(".config/opencode")
    );
}

#[test]
fn detects_only_existing_allowlisted_targets() {
    let home = tempfile::tempdir().unwrap();
    fs::create_dir_all(home.path().join(".claude")).unwrap();
    fs::create_dir_all(home.path().join(".config/opencode")).unwrap();
    fs::create_dir_all(home.path().join(".ssh")).unwrap();

    let detected = detect_existing_targets(home.path());
    let ids = detected
        .iter()
        .map(|target| target.target.id)
        .collect::<Vec<_>>();

    assert_eq!(ids, vec!["claude", "opencode"]);
    assert!(ids.iter().all(|id| *id != "ssh"));
}

#[test]
fn builds_dotsync_root_with_remote_home_path_and_metadata() {
    let home = tempfile::tempdir().unwrap();
    let target = targets()
        .iter()
        .find(|target| target.id == "claude")
        .unwrap();

    fs::create_dir_all(home.path().join(".claude")).unwrap();
    let detected = detect_existing_targets(home.path())
        .into_iter()
        .find(|detected| detected.target.id == target.id)
        .unwrap();

    let root = build_dotsync_root(&detected, "devuser", "mac-mini.example.ts.net");

    assert_eq!(root.name, "dotsync-claude");
    assert_eq!(root.local, home.path().join(".claude").display().to_string());
    assert_eq!(root.remote, "devuser@mac-mini.example.ts.net:~/.claude");
    assert_eq!(root.kind.as_deref(), Some("dotsync"));
    assert_eq!(root.label.as_deref(), Some("Claude Code"));
    assert!(root.ignore_paths.contains(&"auth.json".to_string()));
    assert!(root.ignore_paths.contains(&".claude.json".to_string()));
}
```

- [ ] **Step 2: Run the failing tests**

Run: `cargo test --test dotsync`

Expected: FAIL because `eternalmac::dotsync` does not exist.

- [ ] **Step 3: Create the allowlist module**

Create `src/dotsync/mod.rs`:

```rust
pub mod allowlist;
```

Create `src/dotsync/allowlist.rs`:

```rust
use std::path::{Path, PathBuf};

use crate::setup::client::SyncRootInput;
use crate::tooling::ssh::build_sync_destination;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DotSyncRisk {
    SafeDefault,
    Caution,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DotSyncTarget {
    pub id: &'static str,
    pub label: &'static str,
    pub include_paths: &'static [&'static str],
    pub default_selected: bool,
    pub risk: DotSyncRisk,
    pub risk_note: &'static str,
    pub target_exclusions: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedDotSyncTarget {
    pub target: &'static DotSyncTarget,
    pub include_path: String,
    pub local_path: PathBuf,
}

const STANDARD_EXCLUSIONS: &[&str] = &[
    ".DS_Store",
    "cache/",
    ".cache/",
    "*cache*",
    "tmp/",
    ".tmp/",
    "*tmp*",
    "logs/",
    "*log*",
    "*.sqlite-wal",
    "*.sqlite-shm",
    "telemetry/",
    "auth.json",
    "credentials.json",
    "token.json",
    "tokens.json",
    "installation_id",
];

const CLAUDE_EXCLUSIONS: &[&str] = &[".claude.json"];
const NO_TARGET_EXCLUSIONS: &[&str] = &[];

const CLAUDE_INCLUDES: &[&str] = &["~/.claude"];
const CODEX_INCLUDES: &[&str] = &["~/.codex"];
const OPENCODE_INCLUDES: &[&str] = &["~/.config/opencode"];
const GOOSE_INCLUDES: &[&str] = &["~/.config/goose"];
const GEMINI_INCLUDES: &[&str] = &["~/.gemini"];
const QWEN_INCLUDES: &[&str] = &["~/.qwen"];
const PI_INCLUDES: &[&str] = &["~/.pi"];
const AMP_INCLUDES: &[&str] = &["~/.config/amp"];
const CONTINUE_INCLUDES: &[&str] = &["~/.continue"];
const AIDER_INCLUDES: &[&str] = &["~/.aider.conf.yml"];
const CLINE_INCLUDES: &[&str] = &["~/.cline"];
const ROO_INCLUDES: &[&str] = &["~/.roo/rules", "~/.roo/rules-*"];

const TARGETS: &[DotSyncTarget] = &[
    DotSyncTarget {
        id: "claude",
        label: "Claude Code",
        include_paths: CLAUDE_INCLUDES,
        default_selected: true,
        risk: DotSyncRisk::SafeDefault,
        risk_note: "",
        target_exclusions: CLAUDE_EXCLUSIONS,
    },
    DotSyncTarget {
        id: "codex",
        label: "Codex",
        include_paths: CODEX_INCLUDES,
        default_selected: true,
        risk: DotSyncRisk::SafeDefault,
        risk_note: "",
        target_exclusions: NO_TARGET_EXCLUSIONS,
    },
    DotSyncTarget {
        id: "opencode",
        label: "OpenCode",
        include_paths: OPENCODE_INCLUDES,
        default_selected: true,
        risk: DotSyncRisk::SafeDefault,
        risk_note: "",
        target_exclusions: NO_TARGET_EXCLUSIONS,
    },
    DotSyncTarget {
        id: "goose",
        label: "Goose",
        include_paths: GOOSE_INCLUDES,
        default_selected: true,
        risk: DotSyncRisk::SafeDefault,
        risk_note: "",
        target_exclusions: NO_TARGET_EXCLUSIONS,
    },
    DotSyncTarget {
        id: "gemini",
        label: "Gemini CLI",
        include_paths: GEMINI_INCLUDES,
        default_selected: true,
        risk: DotSyncRisk::SafeDefault,
        risk_note: "",
        target_exclusions: NO_TARGET_EXCLUSIONS,
    },
    DotSyncTarget {
        id: "qwen",
        label: "Qwen Code",
        include_paths: QWEN_INCLUDES,
        default_selected: true,
        risk: DotSyncRisk::SafeDefault,
        risk_note: "",
        target_exclusions: NO_TARGET_EXCLUSIONS,
    },
    DotSyncTarget {
        id: "pi",
        label: "Pi Coding Agent",
        include_paths: PI_INCLUDES,
        default_selected: true,
        risk: DotSyncRisk::SafeDefault,
        risk_note: "",
        target_exclusions: NO_TARGET_EXCLUSIONS,
    },
    DotSyncTarget {
        id: "amp",
        label: "Amp",
        include_paths: AMP_INCLUDES,
        default_selected: true,
        risk: DotSyncRisk::SafeDefault,
        risk_note: "",
        target_exclusions: NO_TARGET_EXCLUSIONS,
    },
    DotSyncTarget {
        id: "continue",
        label: "Continue",
        include_paths: CONTINUE_INCLUDES,
        default_selected: false,
        risk: DotSyncRisk::Caution,
        risk_note: "may contain model/provider credentials",
        target_exclusions: NO_TARGET_EXCLUSIONS,
    },
    DotSyncTarget {
        id: "aider",
        label: "Aider",
        include_paths: AIDER_INCLUDES,
        default_selected: false,
        risk: DotSyncRisk::Caution,
        risk_note: "may reference API key configuration",
        target_exclusions: NO_TARGET_EXCLUSIONS,
    },
    DotSyncTarget {
        id: "cline",
        label: "Cline",
        include_paths: CLINE_INCLUDES,
        default_selected: false,
        risk: DotSyncRisk::Caution,
        risk_note: "may contain API provider secrets",
        target_exclusions: &["data/settings/providers.json"],
    },
    DotSyncTarget {
        id: "roo",
        label: "Roo Code",
        include_paths: ROO_INCLUDES,
        default_selected: false,
        risk: DotSyncRisk::Caution,
        risk_note: "rules only; whole ~/.roo is not synced",
        target_exclusions: NO_TARGET_EXCLUSIONS,
    },
];

pub fn standard_exclusions() -> &'static [&'static str] {
    STANDARD_EXCLUSIONS
}

pub fn targets() -> &'static [DotSyncTarget] {
    TARGETS
}

pub fn expand_home_path(home: &Path, path: &str) -> PathBuf {
    if let Some(relative) = path.strip_prefix("~/") {
        return home.join(relative);
    }

    if path == "~" {
        return home.to_path_buf();
    }

    PathBuf::from(path)
}

fn detect_include_path(home: &Path, include_path: &str) -> Vec<(String, PathBuf)> {
    let Some(prefix_pattern) = include_path.strip_suffix('*') else {
        let local_path = expand_home_path(home, include_path);
        return local_path.exists().then_some((include_path.to_string(), local_path)).into_iter().collect();
    };

    let prefix_path = expand_home_path(home, prefix_pattern);
    let Some(parent) = prefix_path.parent() else {
        return vec![];
    };
    let Some(prefix_name) = prefix_path.file_name().and_then(|value| value.to_str()) else {
        return vec![];
    };

    let Ok(entries) = std::fs::read_dir(parent) else {
        return vec![];
    };

    entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let name = entry.file_name();
            let name = name.to_str()?;
            name.starts_with(prefix_name).then(|| {
                let matched_include = include_path
                    .trim_end_matches('*')
                    .to_string()
                    + &name[prefix_name.len()..];
                (matched_include, entry.path())
            })
        })
        .collect()
}

pub fn detect_existing_targets(home: &Path) -> Vec<DetectedDotSyncTarget> {
    targets()
        .iter()
        .flat_map(|target| {
            target.include_paths.iter().flat_map(move |include_path| {
                detect_include_path(home, include_path)
                    .into_iter()
                    .map(move |(matched_include_path, local_path)| DetectedDotSyncTarget {
                        target,
                        include_path: matched_include_path,
                        local_path,
                    })
            })
        })
        .collect()
}

pub fn combined_exclusions(target: &DotSyncTarget) -> Vec<String> {
    standard_exclusions()
        .iter()
        .chain(target.target_exclusions.iter())
        .map(|value| (*value).to_string())
        .collect()
}

pub fn build_dotsync_root(
    detected: &DetectedDotSyncTarget,
    server_ssh_user: &str,
    server_dns: &str,
) -> SyncRootInput {
    SyncRootInput {
        name: dotsync_root_name(detected),
        local: detected.local_path.display().to_string(),
        remote: build_sync_destination(server_ssh_user, server_dns, &detected.include_path),
        ignore_paths: combined_exclusions(detected.target),
        kind: Some("dotsync".into()),
        label: Some(detected.target.label.into()),
    }
}

fn dotsync_root_name(detected: &DetectedDotSyncTarget) -> String {
    if detected.target.include_paths.len() == 1 {
        return format!("dotsync-{}", detected.target.id);
    }

    let suffix = detected
        .include_path
        .trim_start_matches("~/")
        .replace(['/', '.', '*'], "-")
        .trim_matches('-')
        .to_string();
    format!("dotsync-{}-{}", detected.target.id, suffix)
}
```

Modify `src/lib.rs`:

```rust
pub mod dotsync {
    pub mod allowlist;
}
```

- [ ] **Step 4: Run DotSync tests**

Run: `cargo test --test dotsync`

Expected: FAIL because `SyncRootInput` does not have `ignore_paths`, `kind`, or `label`.

- [ ] **Step 5: Commit**

Do not commit in this task because Task 2 completes the model fields required by this module.

---

### Task 2: Add Sync Metadata and Mutagen Ignore Support

**Files:**
- Modify: `src/model/config.rs`
- Modify: `src/model/state.rs`
- Modify: `src/setup/client.rs`
- Modify: `src/sync/service.rs`
- Modify: `src/tooling/mutagen.rs`
- Test: `tests/sync.rs`
- Test: `tests/setup.rs`

- [ ] **Step 1: Write failing Mutagen ignore test**

Add to `tests/sync.rs`:

```rust
use eternalmac::tooling::mutagen::build_create_args_with_ignores;

#[test]
fn mutagen_create_args_include_ignore_paths_before_endpoints() {
    let args = build_create_args_with_ignores(
        "dotsync-claude",
        "/Users/me/.claude",
        "devuser@mac-mini:~/.claude",
        &["auth.json".to_string(), ".DS_Store".to_string()],
    );

    assert_eq!(
        args,
        vec![
            "sync",
            "create",
            "--name",
            "dotsync-claude",
            "--sync-mode",
            "two-way-resolved",
            "--ignore",
            "auth.json",
            "--ignore",
            ".DS_Store",
            "/Users/me/.claude",
            "devuser@mac-mini:~/.claude",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>()
    );
}
```

- [ ] **Step 2: Run the failing Mutagen test**

Run: `cargo test --test sync mutagen_create_args_include_ignore_paths_before_endpoints`

Expected: FAIL because `build_create_args_with_ignores` does not exist.

- [ ] **Step 3: Implement ignore-aware Mutagen args**

Modify `src/tooling/mutagen.rs`:

```rust
pub fn build_create_args(name: &str, local: &str, remote: &str) -> Vec<String> {
    build_create_args_with_ignores(name, local, remote, &[])
}

pub fn build_create_args_with_ignores(
    name: &str,
    local: &str,
    remote: &str,
    ignore_paths: &[String],
) -> Vec<String> {
    let mut args = vec![
        "sync".into(),
        "create".into(),
        "--name".into(),
        name.into(),
        "--sync-mode".into(),
        SYNC_MODE_TWO_WAY_RESOLVED.into(),
    ];

    for ignore_path in ignore_paths {
        args.push("--ignore".into());
        args.push(ignore_path.clone());
    }

    args.push(local.into());
    args.push(remote.into());
    args
}
```

- [ ] **Step 4: Run the Mutagen tests**

Run: `cargo test --test sync mutagen_create_args`

Expected: PASS for existing and new Mutagen argument tests.

- [ ] **Step 5: Add backward-compatible sync metadata fields**

Modify `src/model/config.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPairConfig {
    pub name: String,
    pub local: String,
    pub remote: String,
    pub mode: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ignore_paths: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
```

Modify `src/model/state.rs`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPairState {
    pub name: String,
    pub local: String,
    pub remote: String,
    pub mode: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ignore_paths: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
```

- [ ] **Step 6: Extend setup input and state creation**

Modify `src/setup/client.rs`:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncRootInput {
    pub name: String,
    pub local: String,
    pub remote: String,
    pub ignore_paths: Vec<String>,
    pub kind: Option<String>,
    pub label: Option<String>,
}
```

When building `SyncPairConfig` inside `apply_client_setup_with_preflight`, include:

```rust
ignore_paths: root.ignore_paths.clone(),
kind: root.kind.clone(),
label: root.label.clone(),
```

When building `SyncPairState`, include:

```rust
ignore_paths: pair.ignore_paths.clone(),
kind: pair.kind.clone(),
label: pair.label.clone(),
```

When creating Mutagen sessions, replace:

```rust
let args = build_create_args(&root.name, &root.local, &root.remote);
```

with:

```rust
let args = build_create_args_with_ignores(
    &root.name,
    &root.local,
    &root.remote,
    &root.ignore_paths,
);
```

Update the import from:

```rust
use crate::tooling::mutagen::{
    build_create_args, list_args as mutagen_list_args, parse_list_output, ListedSession,
    SYNC_MODE_TWO_WAY_RESOLVED,
};
```

to:

```rust
use crate::tooling::mutagen::{
    build_create_args_with_ignores, list_args as mutagen_list_args, parse_list_output,
    ListedSession, SYNC_MODE_TWO_WAY_RESOLVED,
};
```

- [ ] **Step 7: Update ordinary sync builder defaults**

Modify `src/sync/service.rs` when creating `SyncPairConfig`:

```rust
let sync_pair = SyncPairConfig {
    name: pair.name,
    local: pair.local,
    remote: pair.remote,
    mode: pair.mode,
    ignore_paths: vec![],
    kind: None,
    label: None,
};
```

- [ ] **Step 8: Update test literals**

For every `SyncRootInput` literal in `tests/setup.rs` and `src/commands/setup.rs` tests, add:

```rust
ignore_paths: vec![],
kind: None,
label: None,
```

For every `SyncPairConfig` literal in tests, add:

```rust
ignore_paths: vec![],
kind: None,
label: None,
```

For every `SyncPairState` literal in source or tests, add:

```rust
ignore_paths: vec![],
kind: None,
label: None,
```

- [ ] **Step 9: Run focused tests**

Run: `cargo test --test sync --test setup --test dotsync`

Expected: PASS.

- [ ] **Step 10: Commit**

```bash
git add src/model/config.rs src/model/state.rs src/setup/client.rs src/sync/service.rs src/tooling/mutagen.rs src/dotsync src/lib.rs tests/dotsync.rs tests/sync.rs tests/setup.rs
git commit -m "feat: add dotsync allowlist and sync metadata"
```

---

### Task 3: Wire DotSync Into Client Setup Prompts

**Files:**
- Modify: `src/setup/prompts.rs`
- Modify: `src/commands/setup.rs`
- Test: `tests/setup.rs`

- [ ] **Step 1: Write failing setup collection test**

Add to the test module in `src/commands/setup.rs`:

```rust
#[test]
fn client_setup_request_appends_dotsync_roots_after_regular_syncs() {
    let calls = RefCell::new(Vec::new());

    let (_preflight, input) = collect_client_setup_request(
        Some("override.ts.net".into()),
        Some("devuser".into()),
        || {
            calls.borrow_mut().push("preflight");
            Ok("ready")
        },
        |server_override| {
            calls.borrow_mut().push("prompt-dns");
            assert_eq!(server_override.as_deref(), Some("override.ts.net"));
            Ok("mac-mini.example.ts.net".into())
        },
        |user_override| {
            calls.borrow_mut().push("prompt-user");
            assert_eq!(user_override.as_deref(), Some("devuser"));
            Ok("devuser".into())
        },
        |server_user, server_dns| {
            calls.borrow_mut().push("prompt-syncs");
            assert_eq!(server_user, "devuser");
            assert_eq!(server_dns, "mac-mini.example.ts.net");
            Ok(vec![SyncRootInput {
                name: "project".into(),
                local: "/Users/me/project".into(),
                remote: "devuser@mac-mini.example.ts.net:~/project".into(),
                ignore_paths: vec![],
                kind: None,
                label: None,
            }])
        },
        |server_user, server_dns| {
            calls.borrow_mut().push("prompt-dotsync");
            assert_eq!(server_user, "devuser");
            assert_eq!(server_dns, "mac-mini.example.ts.net");
            Ok(vec![SyncRootInput {
                name: "dotsync-claude".into(),
                local: "/Users/me/.claude".into(),
                remote: "devuser@mac-mini.example.ts.net:~/.claude".into(),
                ignore_paths: vec!["auth.json".into()],
                kind: Some("dotsync".into()),
                label: Some("Claude Code".into()),
            }])
        },
    )
    .unwrap();

    assert_eq!(
        calls.into_inner(),
        vec![
            "preflight",
            "prompt-dns",
            "prompt-user",
            "prompt-syncs",
            "prompt-dotsync"
        ]
    );
    assert_eq!(input.sync_roots.len(), 2);
    assert_eq!(input.sync_roots[0].name, "project");
    assert_eq!(input.sync_roots[1].name, "dotsync-claude");
}
```

- [ ] **Step 2: Run the failing setup collection test**

Run: `cargo test --lib client_setup_request_appends_dotsync_roots_after_regular_syncs`

Expected: FAIL because `collect_client_setup_request` has no DotSync prompt closure.

- [ ] **Step 3: Add DotSync prompt helpers**

Modify `src/setup/prompts.rs` imports:

```rust
use crate::dotsync::allowlist::{
    build_dotsync_root, detect_existing_targets, DetectedDotSyncTarget, DotSyncRisk,
};
```

Add:

```rust
pub fn prompt_enable_dotsync() -> Result<bool> {
    Ok(Confirm::new()
        .with_prompt("Enable DotSync for AI-agent dotfiles?")
        .default(false)
        .interact()?)
}

fn prompt_dotsync_target(target: &DetectedDotSyncTarget) -> Result<bool> {
    let mut prompt = format!(
        "DotSync {} ({})?",
        target.target.label, target.include_path
    );
    if target.target.risk == DotSyncRisk::Caution && !target.target.risk_note.is_empty() {
        prompt.push_str(&format!(" - {}", target.target.risk_note));
    }

    Ok(Confirm::new()
        .with_prompt(prompt)
        .default(target.target.default_selected)
        .interact()?)
}

pub fn prompt_dotsync_roots(server_ssh_user: &str, server_dns: &str) -> Result<Vec<SyncRootInput>> {
    if !prompt_enable_dotsync()? {
        return Ok(vec![]);
    }

    let home = std::env::var("HOME")?;
    let detected = detect_existing_targets(std::path::Path::new(&home));
    if detected.is_empty() {
        println!("No supported AI-agent dotfiles found for DotSync.");
        return Ok(vec![]);
    }

    let mut roots = Vec::new();
    for detected_target in detected {
        if prompt_dotsync_target(&detected_target)? {
            roots.push(build_dotsync_root(
                &detected_target,
                server_ssh_user,
                server_dns,
            ));
        }
    }

    Ok(roots)
}
```

- [ ] **Step 4: Wire setup collection**

Modify `src/commands/setup.rs` import:

```rust
use crate::setup::prompts::{
    prompt_dotsync_roots, prompt_server_dns, prompt_server_ssh_user, prompt_sync_roots,
};
```

Modify `run_client` call:

```rust
let (preflight, input) = collect_client_setup_request(
    server_override,
    std::env::var("USER").ok(),
    || preflight_client_setup(&context.runner),
    prompt_server_dns,
    prompt_server_ssh_user,
    prompt_sync_roots,
    prompt_dotsync_roots,
)?;
```

Modify `collect_client_setup_request` signature:

```rust
fn collect_client_setup_request<PF, PD, PR, DD, T>(
    server_override: Option<String>,
    ssh_user_prefill: Option<String>,
    preflight: PF,
    prompt_dns: PD,
    prompt_user: impl FnOnce(Option<String>) -> Result<String>,
    prompt_roots: PR,
    prompt_dotsync: DD,
) -> Result<(T, ClientSetupInput)>
where
    PF: FnOnce() -> Result<T>,
    PD: FnOnce(Option<String>) -> Result<String>,
    PR: FnOnce(&str, &str) -> Result<Vec<SyncRootInput>>,
    DD: FnOnce(&str, &str) -> Result<Vec<SyncRootInput>>,
{
    let preflight = preflight()?;
    let paired_server = prompt_dns(server_override)?;
    let server_ssh_user = prompt_user(ssh_user_prefill)?;
    let mut sync_roots = prompt_roots(&server_ssh_user, &paired_server)?;
    sync_roots.extend(prompt_dotsync(&server_ssh_user, &paired_server)?);

    Ok((
        preflight,
        ClientSetupInput {
            paired_server,
            server_ssh_user,
            sync_roots,
        },
    ))
}
```

Update existing tests in `src/commands/setup.rs` to pass a final closure:

```rust
|_server_user, _server_dns| Ok(vec![])
```

- [ ] **Step 5: Run setup command tests**

Run: `cargo test --lib setup::`

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/setup/prompts.rs src/commands/setup.rs
git commit -m "feat: prompt for dotsync during client setup"
```

---

### Task 4: Apply DotSync Metadata During Client Setup

**Files:**
- Modify: `tests/setup.rs`
- Modify: `src/setup/client.rs`

- [ ] **Step 1: Write failing client setup integration test**

Add to `tests/setup.rs`:

```rust
#[test]
fn client_setup_creates_dotsync_mutagen_session_with_ignores_and_metadata() {
    let tempdir = tempfile::tempdir().unwrap();
    let paths = Paths::new(tempdir.path().to_path_buf());
    let store = Store::new(paths.clone());
    let runner = FakeRunner::default();

    let summary = apply_client_setup(
        &paths,
        &store,
        &runner,
        ClientSetupInput {
            paired_server: "mac-mini.example.ts.net".into(),
            server_ssh_user: "devuser".into(),
            sync_roots: vec![SyncRootInput {
                name: "dotsync-claude".into(),
                local: "/Users/me/.claude".into(),
                remote: "devuser@mac-mini.example.ts.net:~/.claude".into(),
                ignore_paths: vec!["auth.json".into(), ".DS_Store".into()],
                kind: Some("dotsync".into()),
                label: Some("Claude Code".into()),
            }],
        },
    )
    .unwrap();

    assert_eq!(summary.sync_names, vec!["dotsync-claude"]);

    let config = store.load_config().unwrap();
    let saved = &config.client.as_ref().unwrap().sync_pairs[0];
    assert_eq!(saved.kind.as_deref(), Some("dotsync"));
    assert_eq!(saved.label.as_deref(), Some("Claude Code"));
    assert_eq!(saved.ignore_paths, vec!["auth.json", ".DS_Store"]);

    let state = store.load_state().unwrap();
    let sync_state = &state.syncs[0];
    assert_eq!(sync_state.kind.as_deref(), Some("dotsync"));
    assert_eq!(sync_state.label.as_deref(), Some("Claude Code"));
    assert_eq!(sync_state.ignore_paths, vec!["auth.json", ".DS_Store"]);

    let calls = runner.calls.borrow();
    assert!(calls.iter().any(|(program, args)| {
        program == "mutagen"
            && args
                == &vec![
                    "sync".to_string(),
                    "create".to_string(),
                    "--name".to_string(),
                    "dotsync-claude".to_string(),
                    "--sync-mode".to_string(),
                    "two-way-resolved".to_string(),
                    "--ignore".to_string(),
                    "auth.json".to_string(),
                    "--ignore".to_string(),
                    ".DS_Store".to_string(),
                    "/Users/me/.claude".to_string(),
                    "devuser@mac-mini.example.ts.net:~/.claude".to_string(),
                ]
    }));
}
```

- [ ] **Step 2: Run the failing integration test**

Run: `cargo test --test setup client_setup_creates_dotsync_mutagen_session_with_ignores_and_metadata`

Expected: FAIL if Task 2 did not fully propagate metadata to config, state, and Mutagen args.

- [ ] **Step 3: Complete propagation in client setup**

Ensure `src/setup/client.rs` uses `build_create_args_with_ignores` and copies these fields into both `SyncPairConfig` and `SyncPairState`:

```rust
ignore_paths: root.ignore_paths.clone(),
kind: root.kind.clone(),
label: root.label.clone(),
```

Ensure every state save uses the cloned `sync_states` vector that contains those fields.

- [ ] **Step 4: Run setup tests**

Run: `cargo test --test setup`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/setup/client.rs tests/setup.rs
git commit -m "feat: create dotsync sessions with exclusions"
```

---

### Task 5: Render DotSync Status Separately

**Files:**
- Modify: `src/status/service.rs`
- Modify: `tests/status.rs`

- [ ] **Step 1: Write failing status test**

Add to `tests/status.rs`:

```rust
#[test]
fn client_status_renders_dotsync_targets_separately() {
    let snapshot = StatusSnapshot {
        config: Config {
            role: Role::Client,
            server: None,
            client: Some(ClientConfig {
                paired_server: "mac-mini".into(),
                server_ssh_user: Some("devuser".into()),
                server_etterminal_path: None,
                pinned: vec![],
                sync_pairs: vec![],
            }),
            session: SessionConfig { auto_attach: true },
        },
        state: State {
            role: Role::Client,
            tailscale_ok: true,
            server_reachable: true,
            healthy: true,
            summary: "client setup complete; runtime health pending".into(),
            tailscale_dns: None,
            daemon_healthy: false,
            daemon_heartbeat_unix: 0,
            default_session_present: false,
            known_sessions: vec![],
            syncs: vec![
                SyncPairState {
                    name: "project".into(),
                    local: "/Users/me/project".into(),
                    remote: "devuser@mac-mini:~/project".into(),
                    mode: "two-way-resolved".into(),
                    status: "created".into(),
                    ignore_paths: vec![],
                    kind: None,
                    label: None,
                },
                SyncPairState {
                    name: "dotsync-claude".into(),
                    local: "/Users/me/.claude".into(),
                    remote: "devuser@mac-mini:~/.claude".into(),
                    mode: "two-way-resolved".into(),
                    status: "created".into(),
                    ignore_paths: vec!["auth.json".into()],
                    kind: Some("dotsync".into()),
                    label: Some("Claude Code".into()),
                },
            ],
        },
    };

    let rendered = render_summary(&snapshot);

    assert!(rendered.contains("syncs: project:created"));
    assert!(rendered.contains("dotsync: Claude Code:created"));
}
```

- [ ] **Step 2: Run the failing status test**

Run: `cargo test --test status client_status_renders_dotsync_targets_separately`

Expected: FAIL because status currently renders all syncs together.

- [ ] **Step 3: Render ordinary syncs and DotSync separately**

In `src/status/service.rs`, replace the client sync rendering block with:

```rust
let ordinary_syncs = snapshot
    .state
    .syncs
    .iter()
    .filter(|sync| sync.kind.as_deref() != Some("dotsync"))
    .collect::<Vec<_>>();
let dotsyncs = snapshot
    .state
    .syncs
    .iter()
    .filter(|sync| sync.kind.as_deref() == Some("dotsync"))
    .collect::<Vec<_>>();

let sync_summary = if ordinary_syncs.is_empty() {
    "none".to_string()
} else {
    ordinary_syncs
        .iter()
        .map(|sync| format!("{}:{}", sync.name, sync.status))
        .collect::<Vec<_>>()
        .join(", ")
};
lines.push(format!("syncs: {sync_summary}"));

let dotsync_summary = if dotsyncs.is_empty() {
    "none".to_string()
} else {
    dotsyncs
        .iter()
        .map(|sync| {
            let label = sync.label.as_deref().unwrap_or(sync.name.as_str());
            format!("{label}:{}", sync.status)
        })
        .collect::<Vec<_>>()
        .join(", ")
};
lines.push(format!("dotsync: {dotsync_summary}"));
```

- [ ] **Step 4: Run status tests**

Run: `cargo test --test status`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/status/service.rs tests/status.rs
git commit -m "feat: show dotsync status separately"
```

---

### Task 6: Document DotSync and Run Full Verification

**Files:**
- Modify: `README.md`
- Optional modify: `docs/superpowers/specs/2026-05-31-dotsync-allowlist-design.md` only if implementation proves the approved design needs wording correction.

- [ ] **Step 1: Add README DotSync section**

Add this lean section under the client setup or usage area in `README.md`:

```markdown
### DotSync

During `eternalMac setup client`, you can optionally enable DotSync. DotSync detects supported AI-agent dotfiles such as Claude Code, Codex, OpenCode, Goose, Gemini CLI, Qwen Code, Pi, and Amp, then creates normal EternalMac sync roots for the targets you approve.

DotSync is off by default. It uses a curated allowlist instead of syncing every hidden file in your home directory, and it excludes common auth, cache, log, telemetry, and machine-identity files.
```

- [ ] **Step 2: Run formatting**

Run: `cargo fmt`

Expected: no command failure.

- [ ] **Step 3: Run full tests**

Run: `cargo test`

Expected: PASS.

- [ ] **Step 4: Run CLI smoke help**

Run: `cargo run -- --help`

Expected: command succeeds and prints EternalMac CLI help.

- [ ] **Step 5: Check worktree**

Run: `git status --short`

Expected: only intended README or formatting changes remain.

- [ ] **Step 6: Commit**

```bash
git add README.md
git commit -m "docs: describe dotsync setup"
```

If `cargo fmt` changed Rust files, include those files in the same commit:

```bash
git add README.md src tests
git commit -m "docs: describe dotsync setup"
```

---

## Self-Review

- Spec coverage: Tasks cover curated allowlist constants, detection, safe defaults, caution defaults, standard exclusions, setup prompts, Mutagen ignore rules, config/state persistence, status rendering, and documentation.
- Scope check: This plan does not add a menu-bar UI, new conflict resolution, credential portability, or wildcard dotfile sync.
- Type consistency: `ignore_paths`, `kind`, and `label` are used consistently on `SyncRootInput`, `SyncPairConfig`, and `SyncPairState`.
- Verification: Each implementation task has a focused test command, and the final task runs `cargo fmt`, `cargo test`, and CLI help.
