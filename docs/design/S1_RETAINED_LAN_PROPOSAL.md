# C0e1b: retained native LAN transport ownership

Status: PROPOSED, no source authority. Predecessor engine906f91b10683224532f8b300dd6a4a654acbe88a,
tree9a349b3dd986c9718cb8ac0f9b769b3136e32891, different FINAL866c4acf. This builds
on the accepted close-only capability; no ordinary launch/admission activation.
S1/G1 and global close remain open.

## Exact scope

Three production paths in crates/horizon-client/src: lan/mod.rs, lan/conn.rs,
lan/discovery.rs. Focused inline tests in those same files. No protocol, Game,
Hub, Remote, Session, configuration-data, control or dependency changes. No new
generic executor, global worker, polling service or arbitrary callback API.
If concrete prep needs another source path, revise this assignment before edits.

Implement a separate paired-packages retained API. Ordinary Lan, Pinger, B-required
negotiation and staged memory constructors keep their existing behavior. Retained
construction must not run them and later claim their untracked workers absent.

RetainedLan owns private native listener/connection/writer/announcement custody;
no mutable Lan/Shared or raw join handle escape. New RetainedOfflineConfig contains
only primitive owned settings needed for bind/MOTD/player limit/compression/status
and announcement. It cannot contain NetConfig's online/SecureChat/resource_pack,
gate/bypass/log callbacks or Friends owners. Do not accept and then drop a rejected
NetConfig: its last SecureChat Arc may already own an active fetch worker.

RetainedLan::new(sync: Arc<SyncData>, config: RetainedOfflineConfig) creates no
worker or listener. Subsequent listen/connect_local methods mutate this same
owner and return errors without losing partially started native owners. Bind and
spawn failures remain recorded. New config refuses invalid limits before starting
anything. No implicit online-to-offline conversion or support claim.

The retained TCP path is modern protocol777 only. Construct Connection and register
its close capability before ANY native read/write. Call the shared modern packet
pipeline directly, bypassing ordinary serve's legacy raw TcpStream::peek. Modern
status may use its existing packet codec; legacy pre-Netty status is explicitly
unsupported in this new profile. Ordinary legacy status remains unchanged.

## Actual owner and race rules

Use one small private registry with permanent closing latch, attempt identities
allocated BEFORE spawn, original close capabilities, actual native JoinHandles,
terminal outcomes/history and per-attempt admitted UUID binding. Register native
ownership before allowing the corresponding actor to perform IO. A start latch is
permitted where spawn/handle installation otherwise races. Never hold the registry
lock across native IO, shutdown, join, large packet encoding or a user callback.

Stop/accept/local-connect/capability/writer registration share the same admission
linearization. Once closing is latched, reject new work or register the already
acquired owner and immediately request shutdown outside the lock. Accepted sockets
and partial listener/Pinger startup remain accounted for even if closing or spawn
failure wins. Absence of a published play writer is not absence of a connection.

Retain every successfully spawned native outer actor AND play writer in registry
custody that survives an outer panic. A local guard that drops a writer JoinHandle
while unwinding is insufficient. Each handle is taken only after is_finished,
joined exactly once, and its actual return/panic retained. Terminal event or EOF
alone never substitutes for join. Listener and Pinger have the same rule.

Reuse the ordinary login/configuration codecs, registry order and packet rules;
do not duplicate synchronization data or invent abbreviated login. The retained
profile owns no resource-pack scoped keepalive worker or online authenticator.
Native write failures, queue send failures, spawn failures, malformed IO, EOF and
panics have distinct retained outcomes. Explicit shutdown-caused termination may
be classified as such, but must not erase the actual outcome or claim delivery.

Admission/release binds the exact UUID to the original attempt, including failures
between reservation and play. Refuse a retained duplicate atomically; do not enter
the ordinary duplicate replacement/wait loop. A stale attempt cannot erase another
attempt's reservation/online row. Ordinary Shared::try_admit/release behavior is
unchanged. Closed admission cannot be reopened by retry.

Retained PlayerSender must bypass the detached global POST worker in EVERY send,
send_many, Holding/flush, flush_all and deliver path. Preserve per-connection packet
order with a private direct queue and sealed admission. Its public compatibility
tx is disconnected, as staged senders already do. Never perform native socket IO
on the game/coordinator sender path. Retain/report actual in-flight/queued outcome
accounting on writer error or panic; no queue-empty or pending-byte count proves
completion or delivery. Do not silently clear held/queued bodies as close proof.

Queued Joined/Packet/Left payloads and terminal history are separate: event polling
cannot erase custody evidence, and terminal completion cannot erase unconsumed
events. A closed result retains/returns those events and native failure history.
This component neither disposes a Game/Guest nor proves guest data durable.

Pinger uses its actual native UDP announcement worker, with a retained mode that
records bind/spawn/send/panic outcomes, explicit stop and checked finished-only
harvest. Preserve ordinary Pinger::start/Drop. Retained Drop must neither join an
unfinished worker nor detach it. Nonblocking UDP is allowed for this new mode;
no hard latency guarantee follows from synchronous native calls or short locks.

## Fixed API and close semantics

Proposed public shape (typed fixed DTOs, no arbitrary task closures):
RetainedLan::{new,listen,connect_local,poll_events,begin_close,poll_close,
retry_close,observe,try_closed}. poll_events takes a maximum count. All original
native handles remain inside this same owner. Deadline is set by begin_close and
renewed only by explicit retry_close; observation/polling never renews it.

begin_close seals producers/admission and requests actual socket/Pipe/announcement/
listener interruption without waiting for a writer mutex or joining. poll_close
harvests finished handles only and reports Pending, Blocked with preserved state,
or Quiescent with all actual outcomes. A timeout keeps the same owner/handles.
retry_close only renews the deadline/reissues interruption to existing owners;
it cannot start replacement workers or repeat admission. Panics/errors remain
observable even once all workers are terminal. try_closed consumes only a proved
quiescent owner and returns ClosedLan containing unconsumed events/outcomes, never
a global CloseCommit. Unresolved retained Drop fails loudly rather than joining
or silently detaching; exercise it in a separately controlled fatal process.

## Defining validation, not a full game replay

Root alone serially runs tools/runtime. Authors do scoped native text edits only.
Fresh SOURCE/tool CLEAR before compiler/runtime, different FINAL afterward. Use
actual TCP/Pipe and native listener/writer/Pinger, fresh isolated process paths.
Tests needing SyncData load the pinned real prepared resources; missing inputs
refuse qualification. Reuse existing data encoders without changing their inputs.
No compiler/Java/Python/test child launched inside Rust tests. Fatal case executed
by root directly with exact marker/diagnostic/exit, core0 and checked reaping.

Focused controls: silent accepted TCP and local partial-frame close before login;
actual full login/configuration with queued unconsumed Joined over TCP AND Pipe;
observed blocked native writer interrupted without coordinator IO; stop racing
accept/registration/local start; writer/outer/listener/Pinger spawn failure/panic
with original handles retained; duplicate UUID exact owner release; close deadline,
repeated poll and explicit retry; sender/flush ordering and no POST acquisition;
announcement native start/stop; actual event/terminal history survives polling.
Test-only deterministic controls attach to actual acquisition/IO paths, never
replace real native workers with stand-ins. Every fixture cleans up/reaps before
failing assertions. Plan roughly12-20 defining tests, expand only for actual risks.

Static selection: targeted fmt; all-target client Clippy under paired runtime
features and no-default-features; feature-off client typecheck for changed shared
types/constructors; public-tree/privacy. Reuse the accepted20 protocol tests at906f
unless this packet changes protocol assumptions. Select existing PlayerSender/
staged/login-negotiation regressions that touch shared branches, not prior complete
native-save/game batteries. Exact case list/tool/source pins frozen before running.

## Stop and follow-on

Stop at actual retained LAN ownership FINAL, clean private checkpoint and reviewed
public sync. C0e1c still needs Session DNS/connect/reader/outbound custody and must
handle or pre-acquisition refuse auth/Friends/transfer helpers. C0e2/3 still need
Remote decoder, LOD, application/control/integrated/dedicated owners and coherent
native close composition. No ordinary activation, C1, G1, live S2 or release.
