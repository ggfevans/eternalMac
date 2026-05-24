---
title: Troubleshooting
description: Common setup and runtime issues.
---

## Tailscale is not connected

Open Tailscale, sign in, and rerun setup.

## SSH is unreachable

Enable Remote Login on the Mac mini:

```text
System Settings -> General -> Sharing -> Remote Login
```

## ET cannot find etterminal

Rerun client setup. It discovers and stores the remote `etterminal` path used by Eternal Terminal.

```bash
eternalMac setup client --server <mac-mini-tailscale-dns>
```

## Sync is degraded

Run:

```bash
eternalMac sync status
eternalMac doctor
```

If duplicate Mutagen sessions exist, manual cleanup may be required until first-class sync removal lands.

