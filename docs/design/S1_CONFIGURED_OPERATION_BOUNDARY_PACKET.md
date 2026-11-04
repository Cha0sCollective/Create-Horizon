# Configured operation boundary for the S1 slot adapter

Status: PROPOSED bounded design. No source authority until independent acceptance,
private checkpoint/sync, untouched baseline and serial gate. No ordinary filled
slot, replay-service, departure/reconnect, C0/C1/G1/S1 or release acceptance.

## Concrete gap and reuse

At clean accepted/public engine9f7d5ecb7abee68b0f3af6363fbdb3e6b0547219,
treeb9386e81c795c9bf0273acf4a97d56c4b24a4b39, ConfiguredAdmission::is_live is an
atomic snapshot. RetainedHubIdentity binds original Hub/Guest/sender/configuration,
but exposes no held operation boundary. A token after a Boolean check would retain
the check-then-act race. H06d already moves one actual native inventory cell
synchronously; retained Hub already keeps original Guest/GuestPlay after terminal.
Reuse those owners. Do not create a second departing-player inventory.

This packet supplies only the missing original-connection ordering primitive and
its diagnostic composition. Actual intent validation/finite replay storage/native
input and serialized departure transitions follow separately. The stale
S1_PLAYER_SLOT_INTENT_PROPOSAL is research, not source authority; its old parent and
five-path budget do not apply. No new wire/schema/menu/profile/framework.

## Small concrete API

Replace the private original Live.open flag with a finite atomic state: open=0,
operation-held=1, closed=2, held+closed=3. Existing actual constructors/transfer
retain their provenance. All Live invalidators set closed while preserving held;
permit Drop clears held while preserving closed. Never reopen, spin, wait or nest
an operation. Unpaired ordinary behavior and negotiation/configuration semantics
remain unchanged; only the paired configured operation factory can set held.

A nonclone opaque ConfiguredOperation owns the original configured allocation.
Its nonblocking factory performs one CAS open/unheld->open/held. Closed refuses;
held returns Busy. Check the original monotonic runtime-open flag AFTER successful
CAS and before exposing the permit; refusal releases held. Use a documented common
sequentially consistent ordering for CAS, Live invalidation/release and the
runtime close/postcheck. Runtime close preceding CAS must be caught by that later
check. A racing later close may conservatively refuse before returning a permit.
No new mutex, condition variable, task, queue, scheduler or generic lease framework.

The CAS is the successful admitted operation's ordering point. Native invalidation
before it refuses; admitted-first may finish despite subsequent invalidation.
'Termination' here means that native invalidation transition, not instantaneous
observation of a kernel/socket event. Shutdown remains free to run. Retained Game/
Hub serialize inventory/departure work, so original player custody is retained
through the synchronous callback even when the network actor has terminated.

Inventory all actual invalidators: NegotiationConnection::close (including IO,
refusal and non-transferred Drop), ConfiguredOwner::invalidate/Drop, callers from
RetainedContext::interrupt, configured transfer during close, RetainedRegistry::
request_close, and RequiredNegotiation::close/legacy Lan::stop. Do not change their
shutdown, original-error, native-owner or quiescence semantics.

Expose acquisition through the actual original configured Hub identity/connection
entry. Check original Hub allocation, open Hub phase, actual Guest, original sender
allocation, UUID and configured admission. The owned opaque result binds these
allocations; an equal numeric conn/UUID, another Hub, client-side admission or fresh
same-profile connection cannot construct or substitute it. No public raw constructor
or Clone. A narrow matching predicate may compare the original allocations without
requiring them to remain open after successful admission. This is transport
permission only, not world/machine/player authorization.

The acquisition path must not harvest terminals, call retained_turn_open or acquire
the registry mutex. Native interrupt/close already use that mutex; extending it
across gameplay risks inversion. Only direct bounded allocation comparisons and
atomic operations belong in acquisition/release. The later Game adapter must retain
the permit through complete actual actor/machine validation, H06d publication and
preallocated terminal result assignment. For a guest, cache that result inside the
as_guest closure immediately after transfer, before leave_guest does further work.
Existing GuestSwap panic limits are preserved; no unwind-safety improvement claim.

## Six existing paths, maximum

1. crates/horizon-client/src/lan/login_negotiation.rs: private finite state,
   original invalidators and opaque configured operation API only.
2. crates/horizon-client/src/lan/login_negotiation_tests.rs: focused state/runtime
   ordering, Busy, release/invalidation and allocation controls.
3. crates/horizon-client/src/lan/hub/retained.rs: original configured Hub factory
   and allocation matching only; no poll/Left/roster/close policy change.
4. crates/horizon-client/src/lan/hub/retained_tests.rs: actual configured native
   Memory/TCP producers and original Guest custody controls.
5. crates/horizon-client/src/lan/mod.rs: ONLY narrowly needed cfg(test)+feature
   native synchronization controls, if existing controls do not suffice. No new
   production departure, polling or transport semantics here.
6. crates/horizon-client/src/game/retained_server_tests.rs: synchronous test-local
   actual Game/H06d composition. No supervisor/production Game/worker hook changes.

No Game.rs, retained_server.rs, slot_player.rs, lan_actions.rs, SDK, protocol,
persistence, renderer/H11 or unrelated formatter edits. If actual APIs cannot fit,
stop authoring for the smallest independently accepted amendment BEFORE edits.

## Focused qualification and honest limits

Mechanism cases prove all four finite states; invalidation-first refusal; held-first
release preserving closed; second-operation Busy; monotonic runtime close before/
between CAS/postcheck; original allocation mismatch and fresh admission rejection.
Place any test barrier OUTSIDE registry locks; never wait in production entry.

Actual configured native Hub families use real Memory and two-client TCP. Exercise
outer/writer terminal invalidation before Hub processes Left, same-connection Busy,
held-first lifetime and new-connection allocation mismatch. Retain originals and
reap real actors. No fabricated Joined, Guest, live flag or successful permit.

For actual Game composition, add a test-local synchronous constructor using existing
OrdinaryBundle::new/input, ordinary_own_selection, OrdinaryPreparation::prepare,
Game::new_with_world/adopt_activated_startup and install_original_retained_host;
retain the Box<Game> locally instead of Supervisor/run_game. Service existing
retained_server_settle_step/tick_lan with actual paired RetainedSession producers.
Do not turn View into a mutating RPC or add Startup.ordinary/control callbacks.

The existing OrdinaryControl::bootstrap may be called directly as explicitly
AUTHORED DIAGNOSTIC machine setup while its real bootstrap grant is unused, before
ordinary counter arming can consume that state. It must retain captured registered
binding/evaluator/native world ownership; it is not ordinary placement evidence.
Diagnostic cell seeds after genuine admission must be labeled setup and counted
before/after. Then the real as_guest+H06d transfer under the real permit moves only
the actual current cell, with the exact outcome retained before leave_guest.
Native terminal-first must refuse without transfer; permit-first can retain the
exact result and original inventories through frozen coherent close, including the
actual nonempty slot where supported. Do not weaken an actual save/refusal to force
that composition. If the constructor/setup cannot satisfy existing policy, obtain
a bounded amendment rather than inventing body/roster/OWN/grant/lease authority.
This proves diagnostic service composition, not ordinary filled ingress/replay.

Run pinned formatting, all-target lint and relevant off/mods-only/paired/full
compilation. Use exact focused library and Game families with pinned prepared
resources/captured pair; missing inputs fail visibly. Reuse unchanged counter/
materialization and H06d evidence with attribution; no historical whole battery,
new optimized GUI build or all-mode render replay for this dormant service.
Different independent FINAL/privacy and normal publication hooks remain required.
Preserve original FALSE/HOLD/raw sources/worlds. Subsequent source must use a new
prefix/assignment after this packet's checkpoint; S1 work continues under owner
instruction until complete or genuine blocker. No gate promotion from this primitive.
