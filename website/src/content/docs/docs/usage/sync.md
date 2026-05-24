---
title: Sync
description: Add and inspect project folder sync pairs.
---

Add a sync pair:

```bash
eternalMac sync add project \
  --local ~/project \
  --remote kindshadow@dhruvils-mac-mini.example.ts.net:~/project
```

List configured sync pairs:

```bash
eternalMac sync list
```

Inspect Mutagen sync state:

```bash
eternalMac sync status
```

The MVP uses Mutagen two-way resolved sync. Conflict handling is intentionally simple for the first release.

