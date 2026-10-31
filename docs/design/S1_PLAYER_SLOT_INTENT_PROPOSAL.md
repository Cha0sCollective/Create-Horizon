# Bounded authoritative player-to-machine slot intents

Status: PROPOSED DESIGN ONLY. No source authority, ordinary activation, new client
wire, publication or S1/G1 acceptance. This is one generic counter and one owned
slot, not a general inventory framework or Create machine behavior.

## Prerequisites and exact bounded source budget

Require independent FINAL for login B at clean
5b41f44fe83b53fc6b7c4dfe550cc0bc1aca5b83, dropped-item D, registered counter binding,
and an independently accepted clean combined successor with exact parent/tree and
private checkpoint. Research used D's worktree based on retirement
ab239890cd62595889f7f0302cd1396a5e2c547c. Refresh every actual API at the clean pin;
no current dirty overlay or proposed registered factory is an implementation pin.

True transport liveness requires the accepted C0e1/C1 original connection allocation
and terminal service. Guest.sender.pending's actual Arc can distinguish a native
sender allocation but is NOT a socket-closure latch. Do not substitute that Arc,
placed=true, connection ID or a test Boolean for a live configured connection.
The bounded guest service source waits for these actual accepted APIs; this draft
neither invents their constructors nor widens engine scope to implement them.

Exactly FIVE engine paths under crates/horizon-client/src:

1. game/slot_player.rs: retain H06d synchronous transfer and expose only narrow
   fresh-view/preflight helpers actually needed by the intent adapter.
2. NEW game/slot_intent.rs: concrete actor/view/request/result types, bounded
   same-session replay ownership, authorization and synchronous application.
3. game.rs: module declaration, one dormant SlotIntentState field and constructor
   initialization; internal authoritative local-owner service seam. No use_hand,
   UI event, network startup, simulation or ordinary close activation.
4. game/lan_actions.rs: ONLY widen existing under_spawn_protection visibility to
   pub(super), preserving its actual predicate. No duplicate authority rule,
   packet route, GuestPlay field, GuestSwap behavior or normal action change.
5. NEW game/slot_intent_tests.rs: actual-Game service proof, declared explicitly
   from slot_intent.rs with the correct sibling path.

Earlier research suggested four paths; the coordinator explicitly authorized this
FIVE-path DESIGN budget so existing spawn-protection policy is reused rather than
copied. Independent exact design acceptance is still required before source work.
No new SDK, native codec, packet/channel, persistence field, player ABA ledger,
menu/cursor/offhand interaction or arbitrary item mutation API.

Ordinary activation additionally requires retained departure custody and complete
C0 retained close/C1 admission. It needs a separately reviewed packet for actual
client send/receive and native gameplay callers, full integrated/dedicated/two-client
and in-frame qualification. This component cannot silently enable those paths.

## Existing contract and synchronous player freshness

HOST_CONTRACT.md:76-86 requires genuine world/dimension/generation/record version,
player session, reach, permission, held item and operation checks. Lines128-144
require fresh transaction read-set checks and replay key
(session_epoch, request_sequence), returning the committed result including
overflow on replay. These experimental design requirements remain in force.

H06d's actual Game helper transfer_world_slot_inventory_cell lends ONE real cell
to World::prepare_world_slot_transfer and immediately consumes its plan in
commit_world_slot_transfer. The CellLend guard restores that SAME cell on every
Result branch. Native limits, root/count representation and raw recursive metadata
checks already exist. No projection/plan survives this synchronous call.

H06_PLAYER_CELL_ADAPTER_PACKET.md:101-106 explicitly excludes a global player
revision ledger and ABA detection across calls. This proposal does not add one.
The request never claims a player ItemStack, client player revision or prior player
snapshot. Its semantics are: act on this actor's ACTUAL CURRENT selected ordinary
cell at application, with the validated machine view/revision. No stale detached
player plan can commit because none crosses a tick or call boundary. Player A/B/A
between two independent requests is not promised to refuse merely because of an
old client observation. A future contract needing that comparison requires its own
mutation-coverage design; native inventory hash/menu state is no substitute.

## Genuine actor and machine views

Proposed internal factories obtain actors only from actual Game/native owners.
No public from_claims, raw UUID/session constructor or test-only successful authority
constructor. Data fields and peer-supplied identifiers cannot mint an actor.

A local in-frame ActorLease is issued only from this authoritative Game with an
actual current local player body, identified world/session and supported machine
owner. It binds a private same-Game actor allocation plus world lifetime. The
integrated FRONTEND and every Remote client refuse local application; their later
ordinary caller sends to the actual server instead.

A guest ActorLease is captured from THIS actual LanHost/Hub's live placed Guest,
its original accepted C1 configured connection allocation/terminal owner, profile
UUID and current world owner. The actual integrated memory owner is a guest through
this same route, not a privileged local shortcut. Reconnect or equal-value new
Game/host/connection has a different allocation and epoch. The factory verifies
native producer provenance and liveness; no test inserts a fabricated Guest/Joined
and calls that genuine accepted transport authority.

Keep genuine allocation authority through application and replay response. A new
request MUST obtain the accepted C1 allocation's real held liveness gate/lease
BEFORE its full read-set validation, and retain that SAME lease through preparation,
transfer publication and terminal result cache assignment. A check-then-act alive
Boolean, terminal snapshot or sender Arc is insufficient. Native terminal transition
and this application boundary must be ordered by that concrete accepted service:
closure linearized before the lease/application refuses without mutation, even
before Hub processes Left; application linearized first may commit, retaining its
exact cached result and original player custody for subsequent departure handling.
No claim of instantaneous detection of an unobserved kernel/socket event follows.

A busy gate returns nonmutating Busy under the actual accepted API; no owner-loop
blocking wait, new ad-hoc mutex or synthetic successful lease is introduced. The
lease covers the whole actual read-set/prepare/publication boundary, not merely
its final assignment. Native close completion and retained departure must use the
SAME allocation service, so neither can release the actor inventory midway.
Before source authoring refresh the genuine accepted API and prove this held
operation is supported. If unavailable in the five-path budget, STOP and obtain
an amended bounded design/source scope; do not manufacture an authority guard.
A new same-profile connection cannot reuse the old cache/lease.
Private actor allocation identity plus actual live transport owner, not a guessed
u32 connection ID or UUID alone, identify its ledger. Native next_id currently
uses fetch_add; do not assume it cannot wrap or confer allocation authority.

A MachineIntentView is captured by the server from the actual accepted registered
binding/catalog and current persistent domain. It retains the genuine opaque
RuntimeCounterKey/SlotKey plus world ID, dimension, physical target/type, machine
and slot generations, residency, healthy status and slot revision. No peer numeric
ID/string creates those opaque keys. Wire-facing projection remains future work.
The view is data used with a genuine actor, not independent player permission.

At application compare the same actual Game/world, retained registered binding,
counter placement generation and SlotKey allocation, current active/resident state
and read_world_slot(key).slot.revision. Both opaque identities AND their observed
values matter; copied numbers from a restored/new allocation do not match. A view
becomes stale after retirement/readmission/replacement, fault/detach,
slot change or changed current dimension. Never generate/load a missing column
implicitly to satisfy an intent. While unloaded, application refuses. Matching
unload/reload may preserve the same key/revision: application after arrival may
succeed when ALL actual current facts still match. No separate residency-ABA
ledger or rule that every unload permanently invalidates a view is introduced.

## Request, replay and finite ownership

An internal request contains the host-issued actor epoch, contiguous u64 request
sequence, host-issued machine view reference, selected ordinary cell index,
Direction and requested count. Index must equal the actual current selected
hotbar cell, lie0..9 and within the actual ordinary inventory vector; armor,
offhand, cursor, menus and another player's inventory are outside this profile.
Bound count to0..99; zero is a validated NoMove, not a bypass of authority checks.
Only explicit IntoMachine/IntoPlayer are supported; no implicit block/item action.

Use one concrete ActorIntentLedger per live actor allocation. Allocate capacity
BEFORE its first mutation, with at most1024 accepted requests per actor epoch and
at most native MAX_PLAYERS plus one local actor ledger. Retain every accepted
request's exact finite canonical payload and terminal result until that actor's
actual session ends. Reclaiming a proven-ended session's data cannot authorize old
requests against a fresh session. No live-session eviction or sliding-window
reinterpretation of the replay contract. When1024 rows are used, refuse ALL new
sequences before mutation; retained replays still work. A fresh actual connection/
world actor epoch, not a client reset/ledger-clear call, provides a fresh budget.
This is an explicit bounded experimental limit, not unlimited gameplay capacity.

For a valid epoch, require next sequence exactly; future/gapped sequence refuses
without mutation. For an already accepted sequence, compare its exact original
request payload. Equal payload returns the EXACT cached committed/NoMove/refusal
result without re-running a transfer. Different payload with the same sequence
refuses conflict and cannot replace the original result. Old unknown/expired epoch
never executes anew. Sequence/capacity exhaustion refuses before touching cells.

Reserve and validate all ledger storage, actor/view facts and reply representation
BEFORE invoking transfer. The reserved row becomes a concrete Pending execution
state; reentry/replay of Pending returns Busy and never executes a second time.
No external callback, filesystem action, network send, allocation or fallible
operation is allowed between the first transfer publication and terminal result
cache assignment. Existing H06d constructs its committed result before publication
and restores the player by assignments; preserve that boundary.

Terminal response retains the actual TransferOutcome and exact moved/no-move/
remaining counts and machine revision, plus request identity. H06d does not spawn
a remainder: an untransferred amount remains with its original source. Explicitly
record that outcome; overflow/spawned remainder is0 in this bounded profile.
Do not copy an ItemStack into the replay cache as a second authoritative item owner.
If future overflow semantics are added, the SAME exact committed result including
its original overflow must be retained; it cannot be recreated on replay.
H06d's fresh player_projection revision is local-operation data, not a persistent
player-cell revision or client authority token.

Expected new-request authorization/precondition refusals also consume/cache their
accepted sequence, so replay after permissions/world state change returns that
original refusal instead of opportunistically executing later. Malformed epoch,
out-of-order/conflicting sequence or exhausted budget are protocol-level refusals,
not accepted ledger rows. A reply delivery failure cannot undo/repeat a committed
move; later same-session replay obtains its retained result. Wire IO is deferred.

## Actual authorization and atomic application

For a NEW request perform ALL checks before lending the real cell:

- This Game is authoritative, retained machine profile is supported, not Closing/
  Blocked/recovering/frozen-ineligible, and the actual actor/connection is current.
  Do not infer Running permission from diagnostic capture success. Refresh the
  accepted real state APIs before source; no new Boolean grants authority.
- Actual body exists, alive/placed where applicable, correct dimension, no removed/
  respawning actor or invalid selected ordinary cell. The selected item is whatever
  the server owns NOW; H06d validates actual native root/count/components/limits.
- Use native mode policy for the current creative S1 profile: require actual
  Creative and may_build, refuse Spectator/Adventure/Survival for this initial
  ordinary-intent service. This does not narrow H06d's existing diagnostic modes
  or claim future survival progression. Do not use caller-supplied operator bits.
- Reuse the actual under_spawn_protection(host,guest_index,target) predicate after
  its visibility-only widening; local in-frame owner uses native local protection
  semantics. Read actual Hub ops/radius/worldspawn, not a copied request policy.
- Derive eye/rotation/range from the actual actor body, check finite coordinates,
  native reach and a native raycast to the same registered full-cube target. Do
  not trust the client's hit/cursor and do not call pick while forced_hit bypasses
  the raycast. Native Game::clip_ray treats absent world blocks as air: target
  residency alone is INSUFFICIENT. A resident-aware use of the SAME native voxel
  traversal must check actual native present/resident evidence for EVERY traversed
  cell/column, from actor eye through the hit, before using its block/outline fact.
  Any missing/unavailable traversed cell or column yields Unavailable/refusal
  before lending or mutation; unknown space cannot establish visibility. Preserve
  native occlusion/outline/reach rules for fully resident paths, not a separate
  approximate target-distance visibility test. Do not load/generate an absent
  intermediate column or replace its facts with air. The game.rs path already in
  scope may provide this narrow resident-aware traversal service; default native
  pick/clip behavior is unchanged. Reject occluded/out-of-range/wrong-target hits.
- Recheck genuine machine allocation/generation/status/residency/current slot
  revision and actual cell native data. A stale revision/generation refuses the
  whole operation even if current stack values compare equal.

For a guest, the service resolves actor from the actual LanHost and uses the
existing as_guest normal Result path to run H06d on that guest's original cell.
Capture actual host protection before lending; verify correct Game dimension,
actual selected cell and source/body after the swap. Other guests and local owner
remain unchanged. No unsafe borrowed GuestSwap, new unwind-restoration algorithm
or swallowed panic is added. H06d's CellLend remains unwind-safe locally, while the
existing broader as_guest swap is NOT claimed panic-safe. An unexpected panic is
a fault/backstop case, never a normal recoverable abort proof in this packet.

All expected Result failures and NoMove restore normal guest/player ownership.
Commit has no guest/Wasm callback, IO or asynchronous player plan. Raw metadata,
native max-stack resolution, machine slot revision and genuine persistent
retirement checks reuse H06d/World APIs. No success clears pending retirement,
reconciles a machine or takes its items implicitly. That lifecycle remains a
separate host action with its existing checked receiver.

## Actual-Game qualification and honest limits

Root alone performs serial isolated build/fmt/runtime/Git. New test path is exactly
crates/horizon-client/src/game/slot_intent_tests.rs. Use pinned actual resources,
genuine materialized registered binding/catalog and real native WorldSession,
Game/body/Hub/connection owners. No fake Joined/Guest or raw nonce factory counts
as a positive actor proof; use the real memory/TCP admission service and harvest
its actual producer. Cross-allocation/forged data are NEGATIVE controls only.
Native Rust tests launch no Cargo/Python/Java/test children.

Required defining matrices:

1. Actual authoritative local in-frame actor and actual serving Game with native
   memory-owner/guest actors; both transfer directions, partial/exact moves and
   same raw recursive metadata, registered native limits and precise source
   remainder. Another guest/local owner inventory stays unchanged.
2. Exact replay returns byte/value-identical terminal result and zero second
   move, including NoMove/refusal and retained remainder outcome. Same sequence
   different payload, future/gapped sequence, stale epoch, equal-value new actor/
   host/Game/connection and full1024-request budget refuse without mutation.
3. Current selected-cell changes and current stack/component changes before a
   NEW request apply server-current semantics. No client ItemStack/revision is
   accepted. A/B/A across separate calls has no rejection promise; stale machine
   slot revision STILL refuses even if the machine's value returns to A.
4. Dead/unplaced/disconnected-before-Left/wrong dimension, frontend/Remote,
   close/freeze-ineligible state, native Creative/may_build and actual spawn
   protection/ops changes. Replays return prior result without a new effect;
   new requests reevaluate actual permissions and liveness. Use genuine fixed
   controls to delay native closure versus the held C1 application gate at read-set,
   prepare and publication boundaries. Closure-first refuses with both owners
   unchanged; application-first commits once, caches its exact result and retains
   the original departing player custody. Close-before-Left, lease contention and
   reconnect cannot pass through a check-then-act race. Controls release/reap actual
   helpers before teardown; no fake latch or Boolean result is a positive proof.
5. Exact native reach boundary, nonfinite pose, occlusion, changed rotation,
   client hit spoof and forced_hit control. Include an ACTUALLY missing intermediate
   ray column/cell with a still-resident target: request must return Unavailable
   without mutation and without loading/generating that column. After genuine
   native arrival, resident unobstructed path succeeds; a resident occluder refuses
   using native outline rules. Inspect every visited column, not just target or
   endpoint columns. Confirm no implicit column loads.
6. Current persistent machine generation/SlotKey/revision, retirement pending,
   take/readmission, same-state writes, block replacement, chunk unload/reload,
   fault/detach and checkpoint/restored equal-value new runtime identities.
   Rich items remain with their actual machine/player/retirement receiver owner.
7. Unsupported stack/count/native limit, empty/full/incompatible cells, zero
   request and reserved-metadata errors. All expected Result failures conserve
   widened totals and exact raw data; ledger row/preallocation failure occurs
   before mutation. No fallible serialization/IO is injected after publication.
8. Existing H06d actual-cell/GuestSwap expected-Result tests, H06 slot transfer,
   persistent retirement/D native item checkpoints and feature-off defaults.
   Appropriately scoped lint/fmt/public-tree/broad checks with honest calendar
   and parity limits; no ordinary two-client activation claim from this service.

Freeze exact source/tool/input manifests, preserve failed attempts before changes,
qualify clean local pin and obtain DIFFERENT FINAL/private result checkpoint.
No ordinary player hook or client wire is authorized by that FINAL.

## Deferred ordinary wire and departure custody dependencies

A later bounded ordinary caller must route actual native use/input into a finite
paired request/response using the accepted C1 original connection, not overload
native UseItemOn acknowledgement as deduplication. Hub::ack only keeps max sequence;
ContainerClick/native menu state wraps15bits and corrects prediction rather than
checking this machine read-set. No new protocol/channel is approved here. That
later proposal must explicitly scope client game/remote.rs and server routing,
input semantics, authoritative tracking/machine view, response/replay behavior,
connection closure and actual default-integrated/dedicated TWO exact clients plus
in-frame comparison. No generic menu GUI is required for this diagnostic slot.

Departure is a real unresolved custody dependency BEFORE ordinary activation.
Hub::poll moves a departing Guest into HostEvent::Left. game/lan.rs currently calls
save_guest, logs NBT/JSON errors, removes GuestPlay/models and relinquishes the
Guest. A successful machine-to-player move cannot safely rely on that logging path.

The separate departure packet needs an owned RetainedDepartingPlayer in the actual
LanHost/Game, retaining the SAME boxed Guest, removed GuestPlay, profile UUID,
original connection allocation/epoch, dimension, native save projection and
unfinished native metadata IO/receipts. Invalidate new intent authority immediately
on transport termination, preserving completed results as data while the actual
session/custody rule permits. Keep the original departing player inventory as its
sole authoritative owner; do not reconstruct from replay summaries or copy a new
inventoried Guest for reconnect.

Native player NBT plus stats/advancement save inputs must use the accepted C0
metadata IO/ack services or their independently reviewed bounded extension.
Expected IO failure retains the player and original dirty/snapshot owners through
explicit Retry; no logged-success/drop. Block conflicting same-profile admission
until the old player custody is resolved. Coherent checkpoint membership must
include that retained owner OR refuse while departure is unresolved; never omit
its inventory because it left Hub.guests. Exact integration with managed authority,
recovery roster, world close and once-only crafting/carried-item settlement needs
its OWN bounded path/API review before source. This appendix grants none.

Actual later qualification must disconnect before/after transfer commit and reply,
fail each player NBT/JSON write, attempt same-profile reconnect, repair and Retry,
then verify native reread/coherent recapture and both clients' item totals/metadata.
No departure proof is inferred from H06d, this service's replay cache or a successful
native writer close. Full S1/G1 remains open until all ordinary combined evidence.