# Transport-003 placement dependency

Status: production source work stopped before edits on November 5, 2026.
The project lead allocated the down-facing extracting Andesite Funnel packet
from `transport-003-proposal.md`, with ordinary chest-click placement required
and an instruction to return a concrete dependency before changing architecture
or the acceptance fixture. Transport lane `9674d95e9f2ddec02e4413bc6d189509b00ce264`
remains clean; accepted native Create tree
`fc71cb5f0fb0c038853902a7d2f775085b34a0f1` is unchanged.

## Exact gap

Pinned Create Java 6.0.11 `FunnelItem.java:22-25` suppresses use of the chest
clicked with the held Funnel item. Without sneaking, ordinary placement then
chooses `extracting=true` in `FunnelBlock.java:52-62`; looking up at the
chest underside must produce a standing `facing=down` funnel.

In accepted host `9c7f9b2215dfed497aba5522c44f38926628648d`, the
server's `game.rs:6554-6563` calls `native_right_click_block` before chest use,
but setting `canceled` stops **both** block and item, marks use failed and does
not place. `game.rs:6565-6745` then gives the chest its block-use turn.
`native_use_item_on` runs only afterward (`game.rs:6765-6779`), and
`game/native_hooks.rs:6360-6375` explicitly says a chest still opens before
that hook. The existing private `use_on.rs` and `placement_runtime.rs` can
customize item placement only after the chest passes. The `place_block` hook
observes a successful placement and cannot change this priority.

The remote client independently predicts chest reaction in
`game/remote.rs:7456-7475,7573-7584`; its Shift+held-item path passes the
block in `:7823-7832`. Custom placing inside the server-only early cancel
hook would leave a failed use and a wrong client prediction. It is not a
valid substitute for the selected ordinary item action.

## Decision needed before production

- Preserve selected unsneaked chest-click behavior: scope a narrow **private**
  interaction override on both server and client that skips chest block use
  only for the held Andesite Funnel, then lets the ordinary BlockItem placement
  path run. A targeted mixin into `use_hand` plus matching remote prediction
  is a plausible route, but the existing hooks alone cannot do it and the
  mixin has no proven local example. This needs lead acceptance and fresh
  independent Sol High source review after implementation. A general host API
  is a separate architectural decision.
- Change the selected interaction to Shift-click followed by the Java wrench
  toggle from collection to extraction. Existing hooks can support Shift-click,
  but Java deliberately sets sneaking placement to `extracting=false`; the
  extra action changes the accepted fixture and must be chosen explicitly.

No backing-block fixture, source adapter, generated content, build or game
run was substituted. The downward extraction, redstone and item custody work
remains described in `transport-003-proposal.md` and can resume after the
placement decision. Read-only specialists `/root/trace` and
`/root/java_logistics` verified the host and pinned Java paths and closed
their phases.
