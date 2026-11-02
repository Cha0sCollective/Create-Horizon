# C0e1c: retained native client Session

Status: PROPOSED, no source authority. Accepted predecessor
909452923f9f6b83224714eefdc139e665c902ac, tree
8f8a30586ee25d3f1703ad72ec18de95595f39f6, different FINAL6b494df3.
Retained LAN and close-only protocol capabilities are accepted, not global close.
S1/G1 remain open; no ordinary startup, C1, live S2 or release activation.

## Exact bounded source

ONE path: crates/horizon-client/src/net.rs, including inline tests. No protocol,
LAN, Game, Remote, control, Cargo or lock changes. Existing Unix rustix development
feature supports native fixture socket controls. Revise accepted scope first if
concrete preparation discovers an unavoidable additional path.

Add separate opaque RetainedSession, preserving all ordinary Session constructors,
legacy run/disconnect and B-required negotiation behavior. No mutable ordinary
Session, Game, raw writer, join handle or cookie-Arc escape. A private fixed sink
adapter or narrowly scoped retained helpers may share existing codecs; do not
introduce a general transport framework or duplicate packet formats.

Validate offline name/target/settings before starting workers or helpers. Exclude
AuthSession and friend: relay/tunnel inputs before acquisition; reject unsupported
targets rather than downgrade. Constructor starts no worker, then start operates
on the SAME retained owner. Memory target already contains an original native
Pipe: validation refusal must preserve/return that original input, or validate
primitive config before infallibly accepting it. Do not drop an acquired endpoint
and describe the constructor as having acquired nothing. Close-before-start must
also own and interrupt any already supplied endpoint.

Fixed API shape: new_offline/start, poll_events(max), send, send_pack_status,
send_custom_click, begin_close, poll_close, retry_close, observe, try_closed.
Sends return explicit admission/refusal, with a declared finite queue budget that
accounts for queued plus active bytes; no silent success on overflow/sealed input.
No actual socket write or worker wait on caller/UI paths. Pure packet encoding and
short locks do not imply a hard per-call latency guarantee.

## Original acquisition and worker custody

Install pending acquisition before spawning setup/reader, retain its original
JoinHandle before releasing a start latch. DNS/SRV/connect stays on that same
worker; no detached resolver/friend/auth helper. During DNS/connect no socket may
exist: deadline can report Blocked, but cannot claim cancellation or termination.
Retain the same worker across Retry and its eventual success/error/panic.

Immediately after actual Connection acquisition, register its close capability
before the first handshake send under one permanent stop/registration fence. Late
registration after close requests shutdown and refuses further protocol work.
Keep login packet/compression/known-pack/profile behavior from existing codecs.
Offline-only profile must not acquire online authentication helpers if a peer
requests encryption; preserve that refusal and the original native owner.

After login, independently register the actual writer worker and its start latch
before configuration processing. Registry custody survives reader unwinding; a
local writer guard that drops/detaches its handle is insufficient. Keep original
active/queued bodies, packet accounting, events and full terminal history across
native write error, spawn failure and panic. Use honest Receive for ambiguous
baseline error text, distinguish encoding before native send_contents IO, retain
known server disconnect/EOF/queue failures without false delivery claims.

Read/write exit seals outgoing admission and interrupts the actual transport;
one worker's exit does not stand for the other one's completion. Close invokes no
shared writer mutex or synchronous join. Poll takes and joins only positively
finished original handles, once, outside registry locks. Deadline is set once;
only explicit Retry renews it/reissues interruption, never starts replacements,
reopens admission or erases failures. Quiescence requires actual original workers
and acquisition outcomes, not Disconnected or an absent installed writer.

## Ordered configuration and actual acknowledgments

The private outbound queue owns a PLANNED wire phase and FIFO admission under one
short lock. StartConfiguration atomically queues its play acknowledgment then
changes planned phase to Configuration. FinishConfiguration atomically queues its
configuration acknowledgment then changes planned phase to Play. Every play send,
resource-pack response and custom-click chooses the correct phase and enters FIFO
under that same ordering. AtomicBool plus independent channel is insufficient.

Native acknowledgment completion is separate from planned phase. Checked per-ack
write receipts gate Joined/Reconfigured publication on the READER worker, never
the UI. Close/writer panic/failure wakes that receipt wait and retains its outcome.
No queue lock is held during socket IO or waiting. Observation reports planned
phase and actual ack completion distinctly; no early success publication.

Keep automatic KeepAlive replies network-side, while play Ping remains a game
event so Pong cannot overtake prior teleport/world handling. Resource-pack and
custom-click replies must respect both initial config and reconfiguration order.

Transfer events preserve the old worker, original events and cookie custody; no
replacement inside this component. ClosedSession may supply typed target/cookie
snapshot handoff only after real native quiescence. New construction revalidates
exclusions. Cookies remain owned and snapshot-based, no mutable Arc handoff.

## Genuine native qualification

Root serially runs tooling/native processes; authors native scoped TEXT only.
Fresh exact SOURCE/tool CLEAR before formatter/compiler/runtime, different FINAL
afterward. All one-path source and packet-prefix artifacts preserved before each
correction. No compiler/Java/Python children inside Rust tests, no playable data.

Use real TCP/Pipe peers and actual login/configuration/phase codecs, fresh native
owners per case. Minimal protocol registry fixtures suffice for this component;
do not claim full game/resource conformance or silently require prepared worlds.
Required fixture/setup failures fail, not skip. Selected existing default-login,
reconfiguration, cookie/transfer and Ping/KeepAlive regressions cover shared edits.

Defining cases: close during stalled pre-writer TCP/Pipe login/partial read; close
before/after late acquisition registration and deadline with the SAME setup actor;
actual client write backpressure with send/status/close independent of native IO;
initial/reconfiguration phase races with play/pack/click and actual ack publication;
automatic reply and teleport/Ping order; reader/writer spawn failure/panic custody;
active/queued write failure and finite budget refusal; transfer/cookies in both
phases; constructor exclusion/input retention; repeated poll/Retry/events.

For blocked IO, use the accepted pattern of verified actual socket buffers, bytes
from the same identified active frame at its native peer and original unfinished
writer BEFORE interruption. A fixed debug/test-only constructor may accept an
original connected TCP endpoint solely to set safe native fixture buffers before
Connection wrapping; it must use the SAME real setup/reader/writer/phase custody,
own all supplied endpoints and expose no production socket or callback escape.
That bounded case is not DNS/connect proof; separate actual address-target cases
exercise production acquisition. Close/reap every native fixture clone on failure.
Finite IO backstops and a direct-process watchdog cannot supply passing evidence.

Plan roughly12-20 defining cases plus the affected legacy regressions; final exact
selection needs SOURCE/tool review, no inherited full save/Game battery. Targeted
format/all-target client Clippy, relevant feature-off/type configurations and public
source checks; reuse actual unchanged dependency coverage under its real pins.
One ignored unresolved-owner fatal case runs separately with exact marker/diagnostic,
SIGABRT/core0 and reaping. No client performance or Windows-runtime promise.

## Stop condition

Stop at real retained client component FINAL, clean private checkpoint and reviewed
public sync. Ordinary Session stays default until a later accepted wiring packet.
C0e2/3 still own window/control/Remote decoder/LOD/Friends/integrated/dedicated and
original Box<Game>/NativeReady composition. No global CloseCommit, Game/Guest
durability, connected G1, C1 admission, live S2, PR, merge, tag or release follows.
