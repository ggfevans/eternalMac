---
title: Sessions
description: List, create, pin, and unpin named tmux sessions.
---

List sessions on the Mac mini:

```bash
eternalMac session list
```

Create a session without attaching:

```bash
eternalMac session new build-watch
```

Pin and unpin sessions:

```bash
eternalMac session pin build-watch
eternalMac session unpin build-watch
```

Use descriptive names for long-running work: `agent-night-shift`, `api-refactor`, `release-build`, or `openclaw-watch`.

