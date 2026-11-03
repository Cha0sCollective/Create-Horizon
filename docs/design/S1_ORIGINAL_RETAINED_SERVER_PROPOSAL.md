# Original headless server Game and checked lifetime

Status: HELD for fresh independent DESIGN review. No source or launch release.
Parent is accepted/publicly synced engine
`e722bcf58ae236ebd0418e410ba15c9e9747c32f`, tree
`3bb3a290c91660c1f2315a3add1e107853ee9a9c`; private results `e0279995` synced.
Prefix `s1srv1` was checked unused. S1/G1 remain open.

Revision3 also includes hub.rs solely to re-export the new opaque identity type
from its private retained module. Revision2 retains accepted revision1 and adds the native-sender
identity predicate below. Revision1 addresses independent DESIGN HOLD on f80d9d5f,
preserved at .local/s1srv1-design-attempt0-proposal.md. This revision is still
HELD and needs renewed review; no source follows from editing this document.

## Concrete outcome

Start the ORIGINAL headless server Game from retained cold world construction,
install its actual retained Hub before accepting clients, run original server
ticks and actual client play, then freeze, join original transports, complete
native player/world metadata and storage closure and join the original outer
server worker. Preserve original owners on partial startup, failure and Retry.
This supplies the missing server Game composition, not another library-only
transport wrapper. Do not call ordinary server_game and replace title workers,
borrow legacy dedicated console/shutdown, or substitute a mock world/server.

First supported profile: fresh identified TestTerrain Overworld, fixed small
view/simulation distances, offline/no selected mods or pairs, no online services,
Friends, publication, extra dimensions, native console-input worker, renderer,
window, audio/controllers or LOD. Actual MemoryPipe and loopback TCP are required.
This explicit headless route does not enable default integrated/window startup,
paired configuration, coherent archive recovery, C0 composite or G1. Those remain
required later outcomes; no unsupported negative case substitutes for them.

## Constructor-bound server authority

ColdPreparation and NativeCloseAuthority currently authorize client/in-frame
construction only. Add a DISTINCT typed server construction profile to the same
original PrepareOwner/ColdPreparedWorld flow. Preserve the exact old client gate;
do not add an allow-LAN Boolean or let a caller assemble authority from status,
UUID/path claims, a legacy Game or an arbitrary already running Hub.

The server authority carries the existing exact original SaveDir/WorldSession,
identified world/root/dimension and native origin binding. Bind its actual freshly
constructed retained Hub identity once, BEFORE listen/pipe offer. Require the
same original Game/host/Hub and fixed profile at native close/disposal. Refusal
returns all original inputs; an acquired native producer/preparer never falls
through an ordinary error return or disposal thread.

Select an explicit Linux headless route before ordinary heartbeat/mod/extras/menu/
dedicated setup. Validate fresh absolute isolated paths, resources, offline inputs,
feature availability and unsupported environment/arguments before acquisition.
Preload complete actual CPU/resource/GUI/font/registry/encoder inputs synchronously.
Use the original Game::new_with_world cold transfer directly with no local player,
renderer or title/native replacement. Keep SaveDir/Streaming and all partially
prepared owners in the actual startup owner until checked transfer or close.

Start one concrete server worker behind an acquisition/registration latch. The
parent owns the original JoinHandle before that worker can acquire native world
owners. Spawn refusal returns the original inert resource inputs. Readiness is
published only from the actual worker's constructed Game and original Hub, never
from a successful thread spawn or an offered pipe alone. Resources/pack allocations
must remain alive until every dependent actor and the original worker are checked
finished/joined. No self-referential Game or arbitrary executor is needed.

## Actual Game dispatch and partial admission

Use the original tick_server_timed/tick_world/Hub and lan_guest_joined/placement
algorithms for the supported profile. Propagate a retained fence/result through
tick_lan and its TWO production callers before any later simulation, input batch,
dimension work or scheduling. Keep returned-but-undispatched HostEvents in the
same owned server state if failure wins. Stop before unsupported enter_level/
open_level acquisition. Settling is an owner-retaining bounded sequence of real
server steps, not ordinary settle_spawn's blocking loop or a fake ready flag.

An admitted Hub Guest is not yet a loaded native player. Distinguish the untouched original admitted Guest with its pending Joined event,
an incomplete owned native-installation operation, and the fully installed actual
Guest/GuestPlay/native player state; network placement remains a separate fact. The worker observes
stop commands only at owned turn boundaries. An untouched pending Guest can stay
owned with the original event through close, WITHOUT fabricating placement or
saving empty data over existing player files. Check all native Guest initial-body
and no-GuestPlay/no-load invariants and its actual sender/Hub identity before this
disposition; preserve existing native player files unchanged.

Complete an already started join operation without simulation, or retain its
explicit partial data/error in a blocked startup/close owner. Never silently label
a partly mutated Guest untouched. Existing fallible load/save results must be
checked, not log-and-continue. A retry may complete the exact retained native
operation after repair, solely to close; it cannot resume gameplay or replay
admitted actions. Test both the untouched stop boundary and an actual controlled
partial failure. Panic/unrecoverable unwind keeps the existing fatal backstop;
planned timeout is never an unwind/discard policy.

Choose explicit single-read checked admission. Add a feature-gated checked reader
in save.rs that delegates its existing read_dat_native codec/error policy;
ordinary read_dat remains unchanged. Read each player NBT/stats/advancement file
once per successful stage, caching original successful inputs across later-stage
failure. Only NotFound means absent/default. Preserve IO/decode/JSON errors and
original read inputs, reject an unsupported player-root/body before mutation,
and reuse Level::read_player, playerdata::apply and Progress::from_json. Add a
small shared pure stats JSON decoder in game/stats.rs, used by the unchanged
legacy read policy and the checked adapter; do not copy its interpretation rules
or read the file again through a bool/Option/logging wrapper.

Prepare and validate the first native PlayerMotion before consuming saved body,
absorption or logs. Implement this checked retained helper in lan.rs using the
same existing native constructor/application operations. Complete all fallible
construction before committing the actual Guest/GuestPlay/body. Preserve original
inputs and any partial body on failure. The retained guest must never enter the
lossy legacy fresh-body branch in lan_actions.rs. Ordinary behavior is unchanged;
no lan_actions.rs source change is proposed.

For close, an in-progress admission retains its successful cached reads and exact
failed stage. Explicit Retry may complete that stage after repair and finish the
checked native installation solely for saving. It performs no simulation, replay,
network placement or reopened Hub admission. Track native installation separately
from network placement: a close-only installed player can have a real body/records
and actual native installation receipt while the original Hub placed/loaded/ack
fields remain false. Never set those fields just to satisfy a checkpoint predicate.

## Participant identity and native save

ParticipantRoster currently stores legacy Shared identity. Add a concrete opaque
retained identity branch, issued by the actual retained Hub constructor. Match
the same original Hub/Guest connection and sender allocation during projection,
including after original native handles have closed. Expose no registry/Shared,
raw native handle or claims-only authority. Keep legacy identity checks exact.
Reuse existing native participant row packing and checked writes; do not introduce
a save format, journal or new inventory codec.

Capture each original validated Joined sender in the private retained Hub owner
before admitting its actual Guest. Keep that original sender through native close
and ClosedHub. Add one narrow predicate in lan/conn.rs comparing the PRIVATE
RetainedOutbound Arc of two retained PlayerSenders. Public tx/pending fields and
heap addresses are not connection proof. Hub's opaque identity can then check
the constructor-issued Hub origin, conn/UUID and actual current Guest sender
against its captured original even after RetainedLan becomes ClosedLan. Expose
only the predicate/result, not the queue/registry/Arc/worker. No change to native
acquisition, IO, queue, default/staged sending or failure/close algorithms.

Define a DISTINCT retained-server NativeWorld population/projection; do not call
or relax preflight_session_checkpoint for it. Untouched admissions retain their
original Guest/event without a native row or file overwrite. Fully native-installed
participants require the constructor-bound installation receipt, same actual
Guest/sender/body/GuestPlay and valid native item/body/record ownership. They can
close while awaiting teleport/PlayerLoaded acknowledgements and with queued/
unprocessed input: serialize authoritative native state and
keep those original payloads/ack fields unchanged, not processed or invented.
An incomplete native installation blocks until its exact checked stage completes.

Preserve all existing coherent-session ownership/idle predicates. The new native
projection must retain necessary item/owner/generation/data-validity checks; it
must not broadly bypass them to admit network-transient states. The first fixed
profile refuses unsupported inventory/menu/respawn/dimension transitions before
mutation, while supporting genuine normal login, terrain, movement and the chosen
non-mutating play probes. It cannot refuse ordinary initial acknowledgements as
a shortcut or substitute an unsupported negative for its positive native close.

Freeze actual Game producers and retain its host, guests, GuestPlay, native locks,
worlds and resources. Request the original Hub close and poll finished-only native
joins while serving status/Retry. Keep the SAME Hub nonconsumingly at Ready inside
Game while the existing CloseOwner projects native participants and writes native
metadata/storage. Do not remove Game.lan or consume ClosedHub before projection.
The constructor-bound server authority must distinguish this from a foreign Hub,
unjoined transport, arbitrary legacy host or client/window native authority.

After all original native writers/producers/metadata helpers complete and the
native profile is Ready, a DISTINCT server disposal transition consumes the actual
Hub into ClosedHub and then the original Game/native owner. Failed consuming
transitions restore the entire original host/Game. Do not weaken dormant/window
disposal or infer world durability from transport quiescence. Preserve raw native
operation outcomes and the original application error through final return.

## Worker, commands and parent lifetime

Use one concrete original worker handle and fixed typed commands for actual
status, memory-pipe offer, close and exact-attempt Retry. No FnOnce(Game) tasks,
generic command executor, untracked stdin reader or hidden Drop join/detach.
Pipe offer failure retains the exact original unsent Pipe/response input and
closes its actual connection; no dropped reply stands for client completion.

The worker remains alive owning Game through Closing/Blocked, polling only the
concrete retained close work and commands. Observer/channel loss cannot drop,
detach or silently release it. One close creates one absolute attempt; repeated
close/status/quit preserves it. Explicit Retry accepts only the current blocked
attempt; actual finished quiescence wins over deadline expiry. Failed save/read/
drain keeps the same owners and lock until explicit supported retry resolves it.

The parent retains its actual supervisor/control endpoint through worker/native
failure and Blocked. It joins only an actually finished original worker and checks
its returned result. Keep original error/result data until that checked completion.
Only then finish required final control responses and seal the actual endpoint;
no later fallible native ownership stage follows control sealing. Process return
is Ok/Err only after checked disposal/joins. Signals request the same close path;
they do not detach workers or grant a forced-discard operation.

An existing retained control service can supply the fixed status/close/Retry
grammar if it can honestly represent a headless server. Do not fabricate a window,
renderer or NativeWindowReady witness. Parent control, server world durability
and actual worker completion remain separate facts. CPU encoding, allocations,
locks, native IO and surface-free resource work are indivisible; no hard per-call,
UI-latency or global memory/performance claim is introduced.

## Proposed maximum source scope

Twelve paths under crates/horizon-client/src:

1. game.rs: early explicit route, direct cold factory/typed server owner and tick
   fence propagation; preserve ordinary entry and Game algorithms.
2. game/world_loading.rs: distinct cold server profile and constructor-bound
   authority; preserve existing original preparation/producer/lock custody.
3. NEW game/retained_server.rs: concrete startup/run/freeze/transport/native-close,
   actual worker/supervisor/control lifetime and partial-admission custody.
4. NEW game/retained_server_tests.rs: actual defining resource/transport/storage
   tests, declared only by the new module.
5. game/lan.rs: real retained Game dispatch, original join/participant identity and
   checked native row reuse, exact final original-host handoff.
6. game/levels.rs: propagate fence at its caller and refuse unsupported dimension
   acquisition before any replacement/open_level mutation.
7. game/paired_close.rs: distinct original server authority/disposal; preserve
   client/window/dormant gates and original native helpers.
8. lan/hub/retained.rs: narrow opaque identity and original closed-owner binding
   needed by native participant projection, no registry/actor escape.
9. save.rs: ONLY a feature-gated checked read wrapper delegating the existing
   read_dat_native codec/error policy; preserve all native writer/close algorithms.
10. game/stats.rs: shared pure native JSON decoder/checked adapter support;
    preserve ordinary read fallback, interpretation and native write semantics.
11. lan/conn.rs: ONLY the private-queue identity predicate for two actual retained
    senders; no outbound/acquisition/IO/close behavior change.
12. lan/hub.rs: ONLY public re-export of the new opaque identity type from its
    existing private retained module. No Hub dispatch/constructor/close changes.

No ordinary integrated/dedicated startup/shutdown, new client Memory factory,
protocol/Session/streamer library/native codec/control library/dependency/lock,
mod-domain or public-doc edits. If concrete preparation finds another required
path, revise this exact proposal before source; this is not a wildcard release.

## Defining checks and follow-on

Require genuine original server Game startup and world-clock progress; actual
MemoryPipe AND TCP Login, terrain and meaningful play through its original Hub;
actual loaded native players with nonzero inventory custody; and native player/
world readback after checked transport, storage-helper and outer-worker joining.
Author native-codec player-file fixtures in a separate isolated staging directory
before launch. A fixed test-only startup seam installs them AFTER successful new
identity/cold world acquisition, under its original lock and BEFORE listener/pipe
admission. Do not pre-populate the target world root: CreateNewIdentity correctly
refuses that layout. No arbitrary worker task or reopen claim. Require actual
login/load and nonzero-item save/readback; a fake player mutation is insufficient.

Exercise partial cold startup, untouched Joined stop and controlled partial join
failure, late frozen events, native writer/metadata failure or stall, exact refused
attempt/explicit Retry, original-error retention and failed consuming restoration.
Use actual native actors and existing deterministic IO controls. Include the
smallest existing cold client authority and participant/coherent-close regressions
affected by shared authority/identity changes. Reuse unchanged Hub proofs at e722;
do not replay the old matrix or rebuild Java/reference worlds.

Root alone runs serial pinned formatting, paired all-target lint, feature-off
compatibility, matching test builds and exact selected cases. Preserve every
packet source/prefix before corrections. SOURCE/tool precedes execution; freeze
actual source/recipe/binaries/resources/native observations. Missing fixtures,
timeouts/nonzero/missing passes fail. No sleep/count replaces actual native joins.

Fresh independent DESIGN, private adoption checkpoint/sync and exact root release
precede source. Different FINAL/privacy and fresh repository visibility/origin/
normal hooks precede reviewed public sync. Stop at a clean reviewed component
checkpoint, then continue the owner's roadmap instruction. Original Memory client
attachment, default integrated/window activation, shared LOD, coherent archive
server reopen, C0 composite, paired C1, G1 and S2 remain separate required outcomes.
No PR, merge, tag, release, public assets or permission expansion.
