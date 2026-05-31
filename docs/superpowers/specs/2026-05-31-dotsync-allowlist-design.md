# DotSync Curated Allowlist Design

Date: 2026-05-31
Status: Approved for planning

## Context

DotSync is a proposed EternalMac setup option that syncs selected AI-agent dotfiles between the client laptop and the Mac mini devserver. The goal is to make AI-agent configuration, rules, and session-adjacent state available on both machines without asking users to remember hidden folders such as `~/.claude` or `~/.codex`.

DotSync must not sync every dotfile under `~/.*`. That would create avoidable security and reliability risk by copying credentials, SSH keys, shell history, package-manager caches, and machine-specific state.

## Goals

- Provide a separate DotSync setup flow that is off by default.
- Use a curated allowlist of popular AI-agent paths.
- Detect which allowlisted paths exist locally before offering them.
- Sync only selected allowlisted targets through the existing EternalMac sync mechanism.
- Keep the allowlist in one constants module so new agent tools can be added without changing DotSync orchestration logic.
- Avoid syncing known sensitive or noisy files by default.

## Non-Goals

- DotSync will not wildcard-sync all hidden files or folders in the home directory.
- DotSync will not solve credential portability for every vendor-specific auth flow.
- DotSync will not add deep conflict-resolution logic beyond the existing MVP sync behavior.
- DotSync will not sync SSH keys, cloud credentials, Kubernetes config, Docker state, shell histories, or package-manager caches.

## User Experience

During setup, EternalMac prompts:

```text
Enable DotSync for AI-agent dotfiles? [y/N]
```

If enabled, EternalMac scans the curated allowlist and presents detected targets. Safe-default entries are selected by default. Riskier entries are shown but not selected by default, with a short warning.

Example:

```text
Detected DotSync targets:
  [x] Claude Code (~/.claude)
  [x] Codex (~/.codex)
  [x] OpenCode (~/.config/opencode)
  [x] Goose (~/.config/goose)
  [x] Gemini CLI (~/.gemini)
  [x] Qwen Code (~/.qwen)
  [x] Pi Coding Agent (~/.pi)
  [x] Amp (~/.config/amp)
  [ ] Cline (~/.cline) - may contain API provider secrets
  [ ] Continue (~/.continue) - may contain model/provider credentials
```

For the first implementation, if a full checklist UI is too much, the CLI can use a simple per-target yes/no prompt while preserving the same default-selection rules.

## Allowlist Model

The allowlist should be represented as data, not scattered conditionals. Each entry should include:

- Stable tool ID, such as `claude`, `codex`, or `opencode`.
- Display label.
- One or more home-relative include paths.
- Optional home-relative exclude paths or glob-like patterns.
- Optional standard exclusions inherited by all targets.
- Whether the target is selected by default when DotSync is enabled.
- Risk level, such as `safe_default`, `caution`, or `blocked`.
- Short user-facing risk note.

This structure allows DotSync setup, status, docs, and future menu-bar UI to read from the same source of truth.

## Standard Exclusions

Every DotSync target should inherit these exclusions unless a future design explicitly overrides them:

- `.DS_Store`
- `cache/`, `.cache/`, and files with `cache` in the filename
- `tmp/`, `.tmp/`, and files with `tmp` in the filename
- `logs/`, files with `log` in the filename, and SQLite WAL/SHM sidecar files created for logging
- `telemetry/`
- Files named `auth.json`, `credentials.json`, `token.json`, or `tokens.json`
- Files named `installation_id` or equivalent machine identity markers

These defaults keep DotSync focused on portable configuration, rules, skills, and session-adjacent state while avoiding credentials, machine identity, logs, and transient caches.

## Initial Default-Selected Targets

These targets should be selected by default when DotSync is enabled and the path exists:

| Tool | Include Path | Target-Specific Exclusions Beyond Standard | Notes |
| --- | --- | --- | --- |
| Claude Code | `~/.claude/` | Exclude `~/.claude.json` | Claude docs identify `~/.claude/` as user scope. `~/.claude.json` can contain OAuth/session state and should not be included by default. |
| Codex | `~/.codex/` | None initially | OpenAI docs identify `~/.codex/config.toml` as user config. Local inspection confirms Codex also stores auth and machine identity files under `~/.codex/`; those are covered by standard exclusions. |
| OpenCode | `~/.config/opencode/` | None initially | OpenCode docs identify this as the global config directory. |
| Goose | `~/.config/goose/` | None initially | Goose docs identify `~/.config/goose/config.yaml` as primary config. |
| Gemini CLI | `~/.gemini/` | None initially | Gemini CLI docs identify `~/.gemini/settings.json` as user settings. |
| Qwen Code | `~/.qwen/` | None initially | Qwen Code docs identify `~/.qwen/settings.json` as user settings. |
| Pi Coding Agent | `~/.pi/` | None initially | Pi docs identify `.pi` as the default config directory. |
| Amp | `~/.config/amp/` | Exclude `~/.amp/oauth/` | Amp docs identify `~/.config/amp/settings.json` and user-wide skills/checks/plugins under `~/.config/amp`. OAuth state is outside that directory and must remain excluded. |

## Initial Caution Targets

These targets are supported by the allowlist but should not be selected by default:

| Tool | Include Path | Reason |
| --- | --- | --- |
| Continue | `~/.continue/` | Local config can contain model/provider configuration. |
| Aider | `~/.aider.conf.yml` | Useful config file, but less relevant to session continuity and may reference API key configuration. |
| Cline | `~/.cline/` | Cline docs explicitly show provider/API-key configuration under `~/.cline/data/settings/providers.json`. |
| Roo Code | `~/.roo/rules/`, `~/.roo/rules-*` | Rules are safe to support, but the whole `~/.roo` directory should not be assumed safe. |

## Blocked Targets

These paths must not be part of the MVP allowlist:

- `~/.ssh`
- `~/.gnupg`
- `~/.aws`
- `~/.azure`
- `~/.config/gcloud`
- `~/.kube`
- `~/.docker`
- Shell histories such as `~/.zsh_history` and `~/.bash_history`
- Broad shell startup files such as `~/.zshrc`, `~/.bashrc`, and `~/.profile`
- Package-manager state such as Homebrew, npm, pnpm, cargo, and pip caches

These can be revisited only as explicit future features with targeted UX and warnings.

## Data Flow

1. Setup asks whether DotSync should be enabled.
2. EternalMac loads the curated DotSync target constants.
3. EternalMac expands each include path relative to the user's home directory.
4. EternalMac detects which include paths exist on the local machine.
5. EternalMac prompts the user to confirm selected targets.
6. EternalMac creates normal EternalMac sync roots for each confirmed target.
7. Status and doctor report DotSync targets as named sync roots, using the allowlist metadata for labels and risk notes.

## Error Handling

- If DotSync is disabled, setup continues without creating DotSync sync roots.
- If no allowlisted paths exist, setup reports that no supported AI-agent dotfiles were found and continues.
- If a selected path does not exist by the time sync is created, skip it with a warning instead of failing setup.
- If a target contains an excluded path, EternalMac must ensure the underlying sync layer receives an exclusion rule before the sync root starts.
- If the sync backend cannot represent a requested exclusion, the target should be downgraded to caution or blocked rather than synced unsafely.
- If a conflict occurs, the existing MVP "most recent version wins" behavior applies.

## Testing

Manual tests should cover:

- DotSync disabled by default.
- DotSync enabled with no allowlisted paths present.
- DotSync enabled with one safe-default target present.
- DotSync enabled with multiple safe-default targets present.
- Caution targets shown but not selected by default.
- Excluded paths are not synced.
- Re-running setup does not duplicate existing DotSync roots.
- Removing and re-adding a DotSync target works through the normal sync root lifecycle.

Automated tests should cover:

- Allowlist constants contain stable unique IDs.
- Home-relative paths expand correctly.
- Detection only returns existing allowlisted paths.
- Default selection follows each target's metadata.
- Blocked paths are never returned as selectable targets.
- Exclusion metadata is preserved when converting DotSync targets into sync root definitions.

## References

- Claude Code settings: https://docs.anthropic.com/en/docs/claude-code/settings
- Codex config basics: https://developers.openai.com/codex/config-basic
- OpenCode config: https://opencode.ai/docs/config/
- Goose config files: https://goose-docs.ai/docs/guides/config-files/
- Gemini CLI configuration: https://google-gemini.github.io/gemini-cli/docs/get-started/configuration.html
- Qwen Code configuration: https://qwenlm.github.io/qwen-code-docs/en/users/configuration/settings/
- Continue configuration: https://docs.continue.dev/customize/deep-dives/configuration
- Aider config file: https://aider.chat/docs/config/aider_conf.html
- Cline config: https://docs.cline.bot/getting-started/config
- Roo Code custom instructions: https://roocodeinc.github.io/Roo-Code/features/custom-instructions
- Amp manual: https://ampcode.com/manual
- Pi Coding Agent docs: https://pi.dev/docs/latest/development
