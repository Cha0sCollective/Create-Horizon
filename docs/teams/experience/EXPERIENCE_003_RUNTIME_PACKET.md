# Experience 003 — one focused Goggles runtime milestone

Status: prepared only. No game, Cargo or Java run in the Experience lane. Root
owns the shared build/runtime reservation, final frozen source/binary metadata,
isolated worlds and process lifecycle. The source-only producer/consumer
composition is a candidate, not a gameplay pass. This packet combines the
[supplied-sheet craft/equip checklist](EXPERIENCE_003_CRAFT_EQUIP_CHECKLIST.md)
with the owner-approved selected HUD field contract; record those results
separately.

## Inputs and owned setup

Use `native/create/tools/experience003_milestone.py` against already running
root-owned sessions. It accepts a root-written JSON `--metadata`, writes to a
packet-owned `--output` directory outside Git, and never launches, builds,
restarts, or edits resources. The metadata must state `source_commit`,
`binary_sha256`, `resource_key`, `world_id`, `mode` (`integrated` or
`dedicated`), `ctl`, `actor_socket`, `actor_log`, and `scene`. Dedicated also
requires `peer_socket`. `binary_sha256` maps process roles to full hashes;
`world_id` must match each client's `state.world`. The hashes are the frozen
root receipt, not inferred by the driver. Root verifies binaries and prepared resources before
launch; keep the actual command/env, server and client logs, process exits,
source/native content hash and cache key with that receipt. Supply no obsolete
physical `.11` path in the packet or driver.

`scene` contains positions `[x,y,z]`: `table`, `craft_stand`,
`actor_view_stand`, `peer_stand`, `actor_head_target`, `millstone`,
`millstone_stand`, `wheel`, `wheel_stand`,
`shaft`, `shaft_stand`, `speedometer`, and `speedometer_stand`. Stands must
make the target visible and within the native eight-block Goggles raycast.
The actor starts in Survival with empty head/inventory and first-person camera;
the dedicated peer starts in Survival with no Goggles. Enable
`CREATE_HORIZON_TRACE=1` on each rendered client and supply its log path.
Grant the actor operator rights only in the disposable world so controller
fixture grants, marked `/data` queries and camera teleports work. Use an
ordinary crafting table and a real connected selected small wheel/millstone
line; the root may reuse its accepted S4 line and selected fixture setup.
Do not directly write network NBT or the new solver-speed field.

## Run order

1. In dedicated play, run `--step craft`. It grants exactly one golden sheet,
   two plain glass blocks and one string, then uses the normal table slots
   101/103/104/105 and result 200. It asserts baseline and final inventory,
   exact 1→0/2→0/1→0 ingredient changes, one crafted Goggles, empty grid,
   ordinary right-click head equip, authoritative `equipment.head` item/count
   query, ordinary slot-500
   unequip/reequip and constant total item count. It captures actor and peer
   images for each equipment state, cycling all three actor camera modes. A peer screenshot file alone is pending
   visual review; root must inspect the rendered head model and its removal.
2. In the integrated world, run `--step equip-fixture` first to supply one
   labeled Goggles fixture and equip it ordinarily; no integrated crafting
   result follows from this grant. With the selected line running, run
   `--step hud --target millstone
   --status Running`, then the wheel equivalent, in integrated and dedicated
   play. The driver queries authoritative block NBT with marked chat commands,
   computes `4 × abs(theoretical RPM)` for the millstone and the Java
   generated-speed adjustment for a small wheel (32 SU/RPM, generated RPM from
   `FlowScore` clamped to -1..1 then ×8). It checks a fresh client
   `goggles_hud` trace for provider, theoretical speed and SU value, and saves
   the actual overlay frame for separate text/layout review. In dedicated
   play the peer may then receive one explicitly labeled **Goggles fixture**
   using `--step equip-fixture` with peer-as-actor metadata, followed by the
   same two HUD observations from that client. This peer fixture is not a
   second crafting pass.
3. While the selected line is running, run `--step negatives` on a wearer.
   It starts from a positive millstone trace, then checks absent fresh HUD
   traces and captures frames with Goggles removed, F1, spectator mode and a
   plain shaft. It restores Survival/Goggles, checks that the preexisting
   Speedometer still emits its legacy gauge trace with server-matched RPM,
   and captures that gauge frame. Root inspects all negative images; traces
   alone do not prove absence of a drawn panel.
4. Root creates a genuine `Idle` state by removing the wheel's water source,
   waits for the server status/Float-zero field, then runs both `--step hud`
   targets with `--status Idle`. Root creates a genuine `Overstressed` state
   using ordinary connected load, without editing NBT; run both targets with
   `--status Overstressed`. The driver requires effective `Speed=0` for the
   latter and requires a nonzero theoretical field before comparing the client
   value.
   Record any unsupported/Byte tombstone seen in a server query as unavailable,
   not a successful selected HUD case. No injected runtime corruption loop is
   needed; focused producer and merge tests cover invalidation.
5. After restoring a selected running state, root normally shuts down and
   cold reopens the integrated world and dedicated server/two clients. Use a
   fresh output subdirectory. Run `--step reopen` to check worn Goggles,
   server equipment and actor/peer head images; the dedicated crafted actor's
   ingredient counts must remain consumed, while the integrated actor retains
   an explicitly supplied Goggles fixture. Rerun
   both Running HUD targets and compare freshly recomputed server field/status
   with client traces and actual overlay frames. Preserve normal exit codes
   and world identity. A warm reconnect or server query does not substitute
   for these client views.

Invoke each step as `python3 native/create/tools/experience003_milestone.py
--metadata <root-frozen-session.json> --output <packet-run/phase> --step
<craft|equip-fixture|hud|negatives|reopen>` plus `--target` and `--status` for
HUD. Use a distinct output subdirectory for integrated, dedicated, peer and
post-reopen observations. Each step writes a JSON report and full controller
transcript even on failure. `passed_assertions` covers only numeric/protocol
checks; every saved frame is listed in `visual_review_pending` until root
inspects it. Root records a separate visual verdict per frame. Do not alter a
failed report or rerun into the same output directory.

## Boundary

This is one selected both-mode milestone, not a broad regression battery.
Retain prior S4 power/recipe, gauge, Saw, Tank, Funnel and audio evidence with
its own limits. A pass can support supplied-sheet Goggles craft/equip/head
render and the selected millstone/small-wheel HUD under the observed states.
It cannot establish gold-sheet obtainability, complete Java tooltip parity,
all kinetic providers, full Create progression, or S6 completion.
