# Original Hub adoption of retained offline transport

Status: HELD for fresh independent DESIGN review. No source release, activation
or C0 composite acceptance. Research is at clean engine
`11c640c55975f609cb172f5ffd373da29747e244`, tree
`d6008569f27581abdeca48d682e95b022d38e835`. Its original-client FINAL/private
checkpoint/public sync must complete before this successor source assignment.
Prefix `s1hro1` was checked unused. S1/G1 remain open.

## Outcome and concrete dependency

Use the original `lan::hub::Hub`, Guest and native encoder with the original
RetainedLan for genuine offline MemoryPipe and modern TCP admission, play packet
processing and checked transport closure. Keep the actual Hub/guests/inboxes and
original payloads through close; do not manufacture a legacy Lan or copy the Hub
simulation into a new mock. This is the server transport-to-Hub component needed
by original server Game startup, not a claim to complete that startup or saving.

At the research pin, Hub owns a public concrete Lan (`hub.rs:204`). Ordinary poll
removes guests on Left (`1179-1204`). Staged recovery has a separate retained
roster and must stay operational. Most LAN metadata uses are within hub.rs and
hub/chat.rs; a small number of Game/recovery and LAN bot callers access the
public Lan directly. RetainedLan is opaque and cannot furnish a mutable legacy
Lan/Shared. Neither a placeholder Lan nor Deref to an invented inner owner is
acceptable.

There is a specific additional dependency in `lan/conn.rs:511-536`:
RetainedOutbound::enqueue_internal currently returns after consuming and dropping
new bodies when closing/sealed/disconnecting or over MAX_PENDING. The ordinary
PlayerSender API is void. The actual Hub needs an owned, visible failure bridge
before it can use this path. Counts for previously accepted bodies alone do not
account for the newly refused originals. Change this concrete queue alongside
Hub adoption; do not create a general transport framework.

## Ownership representation and compatibility

Replace Hub's concrete public Lan field with an opaque concrete transport owner
(for example HubTransport), containing exactly Legacy(Lan) or a retained branch
with active/closed original RetainedLan slots. No trait-object executor, generic
runtime, raw registry/handle/Shared escape or owner replacement. Hub::new keeps
its signature and selects Legacy; new retained construction selects the retained
branch directly. Keep all ordinary/default and staged recovery semantics.

Expose narrow metadata through Hub/transport methods: actual port, maximum
players, online count, MOTD/favicon, online-mode/security and native chat language
where applicable. The retained profile is offline and has no online/chat key
fetch/log callback. Legacy-only operations and identity-sensitive recovery can
use explicit Option/Result legacy access, with existing factory provenance
checked at each call. No general Deref. Migrate the enumerated callers
mechanically; do not change their algorithms or turn new retained access into
an unchecked assertion. Retained methods must refuse legacy-only access.

Construct all fallible SyncData/ChunkEncoder/Hub configuration inputs before
native acquisition. Verify the SAME SyncData owner is used by RetainedLan and
Hub; do not accept merely equal report bytes. Invalid/mismatched construction
returns every original input/owner on refusal. Prefer constructing the inert
RetainedLan within the retained Hub factory from the supplied original Arc and
primitive RetainedOfflineConfig; then listen/connect_local mutate that same
owner and retain partial native startup on failure. No constructor should return
through `?` while discarding a partially acquired original transport.

## Original outbound refusal custody

Keep the default and staged PlayerSender branches unchanged. In retained queue
admission, preserve the original supplied Vec bodies and their order on every
refusal, including permanent close latch, sealed/disconnecting and size overflow.
Do not re-encode, replay, send or silently count them as accepted/written.
An explicit immutable failure/diagnostic view includes refusal reason/count/bytes;
all raw refused inputs stay owned by the same queue through ClosedLan. An error
bridge checked by the Hub before and after a bounded turn fences subsequent Hub
dispatch and retains the original failure. A void producer is acceptable only
with this owned bridge. Later calls must also retain their originals rather than
erase the first failure. Preserve held/queued/active/written ordering and the
existing actual native writer/outer ownership; no POST helper acquisition.

Retained bodies and output history have no new global memory bound. Admission
limit and retained-error storage are different facts. Native allocation/encoding,
locks and IO calls remain indivisible; do not claim hard UI latency or try-only
locking. Actual server Game callers will need their own freeze before invoking
unrestricted gameplay methods; this component does not establish that gate.

## Retained Hub dispatch and close

Use a distinct retained poll/turn API that handles a bounded number of original
NetEvents in native order. Keep original not-yet-processed events in the Hub when
a turn is exhausted or failure is discovered. Joined creates the actual Guest
only after validating actual retained connection/sender identity; no synthetic
UUID/claimed ownership witness. Refused Joined must remain owned before entity-id
or guest mutation. Guard checked entity-id exhaustion before mutation.

Process the actual existing place_new_player, packet/inbox and terrain encoder
operations for the admitted fixed offline profile. Retained transport senders
must use their existing owned branch. Check failures before the next event/turn.
Do not mark received/queued native bytes as client delivery. Existing ordinary
poll/tick remain legacy-only entry points or explicitly dispatch to the retained
route under its exact gates; do not let a retained Hub accidentally execute the
unbounded legacy event drain or staged recovery path.

Retained close is nonconsuming begin/poll/retry. Fence Hub admissions and all
dispatch first, keep guests/inboxes/player payloads, then request the original
RetainedLan. During Closing/Blocked, poll only original native close progress and
status. Never dispatch Left, kick/drop guests, run ticks, drain arbitrary gameplay
or start replacements to achieve closure. Exact attempts preserve the original
deadline; explicit Retry applies only to the current blocked attempt. Checked
quiescence/finished joins win over elapsed deadlines. Preserve terminal/error
outcomes, not just a quiescent Boolean.

ClosedHub is an opaque consuming completion containing the ORIGINAL Hub, guests,
pending/refused inputs and ClosedLan. Failed consuming completion restores the
whole original Hub. No ordinary Drop join/detach, owner substitution or releasing
world/player storage is part of completion. ClosedHub exposes bounded/read-only
observations needed by a later server owner, not mutable actor or gameplay access.
External sender aliases may still own queue Arcs; checked actor closure is not a
claim that arbitrary external producers ceased to exist. The original server
composition must fence its actual producers before using this completion.

## Maximum source scope

Under crates/horizon-client, this proposed whitelist is 14 paths:

1. src/lan/mod.rs: narrow retained metadata/actual input identity/connection
   validation and refusal observations; preserve native acquisition algorithms.
2. src/lan/conn.rs: original refused-body custody and observation; focused tests.
3. src/lan/hub.rs: concrete owner, actual dispatch/close and legacy migration.
4. src/lan/hub/chat.rs: narrow metadata access, unchanged legacy chat rules.
5. NEW src/lan/hub/retained.rs: actual retained owner/dispatch/close implementation
   if separated from hub.rs; no duplicated native gameplay implementation.
6. NEW src/lan/hub/retained_tests.rs: genuine native defining tests.
7. src/game/lan.rs: mechanical legacy identity/port access only.
8. src/game/integrated.rs: mechanical legacy connect/listen/stop access only.
9. src/game/dedicated.rs: mechanical maximum-player metadata only.
10. src/game/dedicated/status.rs: same metadata access only.
11. src/game/lan/connected_recovery.rs: explicit genuine staged legacy access.
12. src/game/lan/connected_recovery_tests.rs: corresponding fixture access only.
13. src/game/levels/session_checkpoint_tests.rs: actual legacy Arc identity checks.
14. tests/lan_bot.rs: mechanical actual port accessor only.

No Game startup/constructor, native save/close, integrated worker, LOD, protocol,
Session, decoder, configuration admission, dependency/lock, mod domain or public
documentation change. Stop for scope amendment if the real caller inventory
requires another path or algorithm change. Exact source assignment follows review.

## Focused qualification and stop

Plan a few grouped defining resource tests, not a replay of all accepted batteries:

- Genuine MemoryPipe original Hub admission/Login/placement and meaningful play
  packet/inbox progress; original RetainedSession/transport checked joins.
- Genuine modern TCP original Hub, real encoded terrain and receipt by a native
  client peer; same owner close, partial startup, terminal event held across freeze,
  blocked exact attempt/Retry and guest/refused custody through completion.
- Original sender refusal bodies for each admission refusal branch, held/flush
  ordering, no POST acquisition and original first-error visibility.
- Smallest existing staged-memory/recovery regression and ordinary LAN bot flow
  affected by mechanical field migration; default PlayerSender regression if its
  shared code changes. Reuse valid transport/library proofs at their actual pins.

Root alone runs serial formatting, paired all-target lint, feature-off compatibility,
matching bin/lib builds and the frozen exact selected tests. Preserve sources and
packet artifacts before corrections under the existing operating rules. Review
source/tool before execution; source, recipe, actual binary/resources and native
owner/peer observations bind results. Missing fixture, deadline or nonzero result
fails; no sleep/count replaces real joins. A later original server Game/window
smoke is a separate composition requirement, not supplied by these library tests.

Fresh independent Sol High DESIGN, accepted original-client checkpoint and private
design sync precede source release. Renew SOURCE/tool/formatted binding and obtain
a DIFFERENT FINAL/privacy before reviewed public sync with fresh visibility/origin
and normal hooks. Stop this component at clean reviewed checkpoint; then continue
the roadmap under the owner's standing instruction. No paired guard relaxation,
C0 composite/G1/S2 decision, PR/merge/tag/release or asset redistribution.
