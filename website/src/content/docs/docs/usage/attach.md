---
title: Attach
description: Connect to remote tmux sessions on the Mac mini.
---

Attach to the default session:

```bash
eternalMac attach
```

Attach to an existing named session:

```bash
eternalMac attach agent-night-shift
```

Create a new named session and attach immediately:

```bash
eternalMac attach -n agent-night-shift
```

Named sessions keep working after your laptop disconnects or shuts down. Reattach later with the same session name.

