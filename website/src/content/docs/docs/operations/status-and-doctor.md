---
title: Status and Doctor
description: Check local eternalMac health.
---

Use `status` for a compact health snapshot:

```bash
eternalMac status
```

Use `doctor` when something feels wrong:

```bash
eternalMac doctor
```

`status` reports the current role, daemon heartbeat, Tailscale state, paired server, and sync health.

`doctor` reports actionable local issues such as missing config, stale daemon state, role mismatch, or degraded sync state.

