---
title: Installation
description: Install eternalMac from the public Homebrew tap.
---

Install from the public Homebrew tap in one command:

```bash
brew install eternalmac/eternalmac/eternalmac
```

You do not need to run `brew tap` first when using the fully qualified formula name above. Homebrew taps `eternalmac/eternalmac` automatically.

If you want the shorter command for future installs or upgrades:

```bash
brew tap eternalmac/eternalmac
brew install eternalmac
```

For local development, build from source:

```bash
git clone https://github.com/eternalMac/eternalMac
cd eternalMac
cargo build
```

`eternalMac` installs or verifies these runtime tools during setup:

- Eternal Terminal
- tmux
- Mutagen
- Tailscale
- launchd agents

The MVP is macOS-only and assumes Homebrew-managed dependencies.
