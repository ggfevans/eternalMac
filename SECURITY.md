# Security Policy

## Reporting a Vulnerability

**Please do not file public issues for security vulnerabilities.**

Report them privately by opening a [GitHub Security Advisory](https://github.com/eternalMac/eternalMac/security/advisories/new).

Include as much of the following as you can:

- Affected version and command
- Steps to reproduce
- Impact assessment (what an attacker could do)
- Any potential fix ideas

You should receive an acknowledgement within **72 hours** and an initial assessment within **7 days**. If you don't hear back within those timeframes, please follow up — this is a volunteer-maintained project and things can slip through.

## Supported Versions

| Version | Supported |
| ------- | --------- |
| Latest release | ✅ |
| Older releases | ❌ |
| Development branches | ❌ |

Only the most recent release receives security fixes. Please update before reporting.

## Scope

### In Scope

- The eternalMac Rust codebase and CLI commands
- Shell scripts and launchd integrations shipped with eternalMac
- The integration layer between eternalMac and the tools it wraps (session setup, Mutagen sync configuration, Tailscale connectivity)

### Out of Scope

- **Dependencies:** Report vulnerabilities in dependencies (Rust crates, etc.) to the upstream maintainers. eternalMac will update dependency versions once fixes are available.
- **Wrapped tools:** Eternal Terminal, tmux, Mutagen, Tailscale, and SSH each have their own security policies. Report vulnerabilities in those projects directly.
- **User misconfiguration:** Issues arising from custom SSH configs, firewall rules, or network setups that are outside eternalMac's control.

## Disclosure Policy

We follow **coordinated disclosure**:

- We ask that reporters give us **90 days** to address a vulnerability before public disclosure.
- We're happy to coordinate on disclosure timing — if the fix takes longer, we'd rather work with you than against you.
- We will credit reporters in advisories unless they prefer to remain anonymous.
