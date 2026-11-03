# Ordinary paired startup: remaining deliverable 1 connections

Investigation at accepted/public engine `21471ef835da8a3d462186de29be20a5d916b9fd`
and private checkpoint `72ce4d11c5e33cbf71a2967458a1dbb4c82466a2`.
This map releases no source paths and does not accept the older full C1 proposal.
The owner-adopted four-deliverable plan and deferred Open to LAN remain unchanged.

## Avoid an unnecessary transport prerequisite

Fresh independent read-only `c0_clean_stop_design_review` traced the raw EOF to
actual blocking receive interrupted by native shutdown. Parent call reordering
cannot guarantee a clean actor result. Relabeling an error after checking a stop
latch could hide a racing real peer failure and remains forbidden.

The adopted close contract requires actual native completion, retained failure
custody, explicit Retry and finished-only joins. It preserves the original error
through completion and ReturnServer(original_result). It does not require every
safe close to return Ok or eliminate error acknowledgment. Keep the existing
diagnostics and defer error-free/no-ack Quit polish. New transport API budget for
that polish is zero. This does not complete ordinary paired C0 ownership wiring.

## Existing services and actual missing callers

| Area | Existing Horizon/accepted service | Required connection |
| --- | --- | --- |
| Selected packages and resources | Packages::read, content_root, actual startup-owned prepared bundles, frozen host inputs and exact local wire contract | Classify selected paired startup before legacy exit handling; retain actual final content/registry owners on both endpoints |
| Normal world entry | WorldRequest, host identity, ordinary integrated/in-frame factories; retained cold preparation and refusal custody | Use actual normal entry/world identity and supported native provenance, including our own reopen/recovery; do not widen cold authority with a Boolean |
| Integrated application | Accepted original Game/Memory/native owner, frontend preparation, actual supervisor/Offer lifetime and joint close | Connect ordinary selected-pair entry to those owners; preserve real native integrated owner versus in-frame local player and dedicated external players |
| Normal controls | mapped_key/key_pressed, creative search/click/wheel, inventory, chat and remote synchronization already exist | Reuse them inside retained_remote_input; keep window lifetime/error/exit in Application. Do not call the whole Game::window_event handler |
| Native gameplay | Existing remote mouse and server native player handlers | Current retained_server::supported_packet permits mainly movement and refuses hotbar, creative slot, Swing, placement, chat and inventory actions. Admit only the precise supported ordinary operations with their real native/checkpoint custody; no blanket guard removal |
| Paired configuration | Exact login negotiation, retained SyncData/assets/packages and native configuration codecs | Continue the same live connection/deadline through exact configuration and consume a concrete configured handoff before registry/player publication. Agreement data is insufficient |
| Player departure/rejoin | Ordinary native player serialization and accepted custody/recovery services | Retained Hub currently refuses Left and keeps the guest. Connect save/custody/removal by actual connection owner before permitting departure/rejoin; do not drop the retained guest |
| World close and recovery | NativeWorld/CoherentSession close owners, actual native writers, complete checkpoint/recovery formats | Select actual native versus persistent-domain close and preserve every reachable participant. Keep filled machine ingress closed until this connection is complete |

The controls gap is concrete: a hotbar digit can already make the normal remote
handler send SetCarriedItem, which the current retained server rejects. A frontend
input-only patch cannot establish usable placement. Compose controls with the
ordinary startup/gameplay admission work instead of adding another diagnostic
endpoint. Existing remote code already sends edited creative slots and placement;
no creative UI, item transport, renderer or hand subsystem needs reimplementation.

## Input and lifecycle boundaries to preserve

close_world already recognizes retained remote ownership and records a leave
request. Existing pause/death exits reuse this safe boundary. Whole window_event
also contains legacy save, replacement and event-loop exits, so it is not a safe
delegation boundary. Escape must close inventory/search/chat before pause/leave;
integrated repeat suppression, input clearing and timing resets remain required.
If additional native pausing screens become reachable, use pauses_game consistently
for frontend timing and backend pause forwarding.

Existing lan_open, lan_start and native host-builder guards already refuse retained
frontends even when unpaired. Preserve them. Do not change external_server merely
to improve the pause label: that would expose host WorldOptions whose current
apply path can mutate the frontend before a later LAN refusal. Retain the current
remote presentation until actual backend world-options service is in scope.

## Next bounded design

Inventory the smallest strict paired configuration service and actual ownership
handoff first, using current APIs rather than the older proposed 43-path budget.
A dormant component must preserve all ordinary paired guards; it cannot count as
full C1 or bypass the ordinary C0 composite activation gate. Then specify the
exact normal startup/native authority/gameplay/custody integration assignment.
No source release follows from this investigation alone.

The defining composed game scenario uses the ordinary application: E opens the
existing creative screen, search/select an admitted item, assign a hotbar slot,
close the screen, select by digit/wheel, place by normal right-click, observe the
authoritative block and returned client rendering, then save/reopen/close using
actual retained owners. Use a native block for the input seam; the registered
counter becomes the positive after exact C1 and its lifecycle are connected.
Reuse unchanged Step B evidence; do not replay its full battery for this map.

No new transport framework, wire schema/channel, task executor, loader, rendering
system, arbitrary world import, general machine/menu framework or later-stage
gameplay is proposed. Consult the owner before a broader redesign, new supported
profile or acceptance change. No user decision is currently needed.
