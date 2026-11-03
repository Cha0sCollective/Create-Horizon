# Integrated owner and joint application lifetime

PROPOSED DESIGN. Within adopted deliverable 1; no source release yet. Parent is
accepted/public47fc752f0695c3435488e4607185d7d0e7717418; private6fdb4b7.
Prefix s1int1 is reserved after checking absence. Open to LAN remains deferred.

## Outcome and implementation order

Reuse the original server Game, original client Game, Memory Pipe, native owner
mapping, retained server supervisor and existing window application. No second
transport, renderer, loader, journal or general executor. The combined outcome is
an actual rendered Memory client with genuine integrated player storage and one
application retaining both sides until native close and server worker join.

Implement in two bounded steps within this design, with separate honest evidence:
A. Distinct integrated-owner cold/admission/native-close adapter with real Memory
protocol playback and native owner readback; B. original client Game/Memory and
joint window/server lifetime with actual application smoke. A alone is component
evidence, not integrated/default/G1 acceptance. Paired configuration and ordinary
selected-pair startup follow this C0 connection; guards remain until C1 is complete.

## Step A: exact initial source budget

Only five existing engine paths under crates/horizon-client/src:
game.rs, game/world_loading.rs, game/retained_server.rs, game/lan.rs and
game/retained_server_tests.rs. Inspect whether paired_close.rs needs more than its
existing server completion predicate before release; do not silently enlarge scope.

Add a private typed integrated-server cold profile, distinct from DedicatedServer
and InFrameClient, with the fixed existing small fresh TestTerrain/no selected
mods/no LOD/audio/controllers profile. NativeCloseAuthority binds the exact original
native save/world/origin and profile. ServerGameState retains its profile and
selected integrated offline owner. The profile uses ExecutionRole::IntegratedServer,
one original Hub with integrated:true, a Memory connection, no TCP listener and no
new Serving/task executor. Reuse existing integrated Owner and singleplayer_owner
semantics; default integrated pause/application attachment is Step B.

Select the owner's offline profile from a validated player name. The native world
player UUID is separately issued once by the native Level factory before authority
issuance; never substitute the connection UUID. Bind both identities for lifetime
and close. Host.owner retains an actual saved Level until successful installation;
preserve it on failed body preparation or JSON read. Reject a different Memory
profile before body mutation while retaining its actual Guest/events for closure.

NativeAdmission must cache the actual owner Level source and mapped stats/
advancement reads. For external guests preserve all old behavior. Owner records
read/write under the world's native player UUID, not its offline profile. A fresh
owner without position takes the actual spawn path; do not synthesize [0,0,0]
from an empty player_tag. On success apply the prepared body and consume saved
owner only after all fallible preparation; failed/partial admission remains owned.

Native population permits exactly the genuine integrated owner for this profile.
Installed rows use ParticipantStorage::IntegratedOwner with the bound world UUID;
dedicated rows remain External. Untouched or incomplete admission must retain
pending owner storage and never fabricate an installed row. Existing coherent
session readiness predicates remain unchanged.

Reuse NativeParticipantsInput and Level's integrated-owner writer. Extend
project_native_owner only as needed to use the existing RetainedNative body
projection (including native Creative reach modifiers), and to retain the saved
owner's native player fields when closing before successful admission. Preserve
world metadata projection. Prevent project_sync_level's empty server inventory
from overwriting the pending owner's actual inventory. Do not discard current
owner state or add a new save codec. Keep original Hub within Game through native
metadata/writer/producer completion and actual worker join.

Step A acceptance: real Memory login/terrain/play/acknowledgments, actual native
world clock, distinct profile/world player UUIDs, nonzero owner inventory and
movement/rotation saved in Level's native player record, no external-profile
player file or JSON alias. Also close before admission with original owner items
unchanged; genuine mapped JSON read/write failure with explicit Retry preserves
the same owner; wrong profile refuses without overwriting it. Dedicated saved/
fresh Memory+TCP counterparts are targeted regressions. No broad historical matrix.

## Step B: joint client/server lifetime

Expected existing seams: game.rs, game/integrated.rs, game/window_close.rs,
game/retained_server.rs; inspect needed test paths and fix exact budget before
release. Do not treat these estimates as source authority.

Prepare complete client resources/settings/name and event loop before offering
the actual Pipe. A widened Game constructor must preserve the original Memory
target on refusal, as RetainedSession already does; no map_err that discards it.
No fallible error return may abandon a running supervisor or acquired endpoint.
Reuse the existing failed Offer reply retention; add no duplicate transport owner.

The application owns the original server supervisor through startup, play,
failure/Retry and shutdown. Client transport/decoder/streamer completion alone
cannot seal final control or exit while server native state remains unjoined.
Preserve both actual errors; do not relabel EOF/ENOTCONN as clean shutdown.
Use existing control/status/Retry handling rather than build a general manager.
Use native owner permissions and pause semantics; deferral of Open to LAN stays.

An explicit qualification entry may establish this component, but ordinary
default integrated selected-pair startup remains an explicit deliverable-1 gate.
Actual rendered Game/application, native owner readback, live movement/terrain,
successful and failed-save/Retry joint close are required. Preserve original TCP
and native in-frame behavior; do not claim factory-only or protocol-only evidence
as Step B acceptance. Any broad redesign/profile expansion returns to the owner.
