# Retained native remote-chunk decoder prerequisite

Status: HELD PROPOSAL ONLY. No engine source, execution or activation authority.
The preceding C0e2c window packet has different FINAL acceptance at exact engine
60899d4fbff3908cff243bc9581cdf3c39ab8a6f, tree991c6c4c, receipt a30abfdd.
Its reviewed public branch is synced; the immutable evidence/private checkpoint
must be recorded before this proposal's fresh design review. This proposal still
grants no source work until accepted assignment/checkpoint/root gate.

## Concrete gap and one-path scope

Read-only inspection used engine HEAD bd0647a6aef9efc2484629fb36a954167b738b30,
branch codex/s1-window-close, with eight dirty C0e2c paths. The two relevant files
were unchanged from HEAD:

| Input | Git blob | Inspected raw SHA256 |
| --- | --- | --- |
| game/remote_chunks.rs | 47f47461904061e4e9d3dc663d81511a73c85e0e | b6938f8b8952a230882a6949b9b9b4d8225de202eaa0fbb425c8bda412823366 |
| game/remote.rs | 15f8dc65f059ceb46c4ee934c0c687967d992060 | bac0947e447ff1ed10f29d00c07d161ad6b7315ed5849483433dc8977827a55c |

Paths above are relative to crates/horizon-client/src. Raw hashes identify the
inspected checkout bytes; they are not new execution or Git-normalization proof.
Both files remain unchanged at accepted60899d4, the proposed exact source parent.

ChunkDecoder::new currently spawns a native decoder and discards its JoinHandle
(remote_chunks.rs:180-207). Its channels do not establish checked native completion.
The worker moves each original format/packet into its unwind scope; next reports
Lost after disconnection, with a comment that the columns are gone (:214-219).
remote.rs:469 constructs this decoder unconditionally. Its polling loop (:808-857)
holds ordered Received::Chunk markers and skips a missing column after Lost.
This cannot supply retained original client ownership during integrated shutdown.

Proposed source budget: ONE path, game/remote_chunks.rs, including inline tests.
Add a separate paired-packages RetainedChunkDecoder and opaque ClosedDecoder.
Keep legacy ChunkDecoder, Next, format decoding and all existing callers unchanged.
No remote.rs/Game/streaming/net/LAN/renderer/dependency/lock edits or caller
activation in this packet. Any required wider edit returns to scope review first.

This is a useful prerequisite for a later original remote-client constructor.
It does not fix the separate remote mesh-worker construction, Session adapter,
retained LAN-to-Hub bridge, server-thread owner or actual integrated composition.

## Fixed original native ownership

prepare creates an inert non-Clone owner and its private state, with no native
thread. start explicitly acquires the one native decoder. Store its actual handle
before releasing a fixed start latch; preserve the prepared owner and input on
spawn refusal. Never adopt a legacy decoder or infer ownership from empty queues.

Use fixed methods for owned admission, ordered result handoff, observation,
begin_close, exact-attempt Retry, poll_close and consuming try_closed. Expose no
worker handle, mutable decoder state, callback/executor or restart operation.
Refusal returns the original format Arc and packet Box intact with a reason.
Only this decoder's ordinary successful owned handoff transfers custody to its
caller; observations and counters cannot transfer or certify an applied column.

The original format allocation and packet must remain registered outside the
worker's unwind stack during actual ChunkFormat::column execution. A separate
actor payload owner may be locked by the native decoder, but UI-facing admission,
status, close and polls must not wait for that payload lock or an unfinished
thread. Busy fixed operations return Pending/refusal with input intact. Keep
queued inputs, the active input, ready results and a result arriving after close
in that same registered owner. Register a completed result before any test fault
or subsequent fallible worker action. No worker panic may silently lose an
original packet, format, already stored result or queued input.

Preserve the actual decoding algorithm and format association. A malformed chunk
returning Err is an owned decode result, distinct from a native actor panic or
spawn refusal. This component does not claim snapshots of parser temporaries
inside an interrupted pure decode, successful application to Game, or retention
of results already transferred successfully to the caller.

Provisional fixed admission: eight outstanding decode slots, counting queued,
active and completed results awaiting handoff together. A new submission must
reserve a slot before moving ownership; full/closed/not-running refusal returns
the original input. A slot retires only on successful owned handoff or remains
in ClosedDecoder. The count bounds admitted work, not total bytes, wire allocation,
derived column size or the surrounding Session/backlog. No whole-client memory
or latency guarantee follows; later caller activation needs its own backpressure
and input-budget review. No configurable scheduler or general queue framework.

## Close, failure and terminal result

begin_close permanently fences admission and preserves the first deadline.
Wake the actual worker without waiting for a UI payload lock. An already active
indivisible decode may finish and publish its original late result; no further
queued input starts after the close fence. Pending queued work remains owned,
explicitly unprocessed, rather than being relabelled decoded or dropped. A
close/admission/start race must resolve against registered original ownership.

poll_close only joins the positively finished original handle, exactly once,
and checks its actual outcome. It never joins in Drop or waits for active decode.
An expired deadline returns Blocked with the same owner and native work retained.
Exact-attempt Retry may extend this wait; it never reopens admission, reruns a
packet, replaces the thread or clears the original failure. Repeated close is
idempotent and does not renew the first deadline.

Once the actual actor is checked terminal, try_closed may consume the original
owner into ClosedDecoder, keeping original unprocessed/failed inputs and stored
results. A checked native panic or spawn refusal is a visible FAILURE handoff,
not successful decoding. Do not require an impossible Retry to repair an already
finished failed actor. A prepared unstarted owner can close without spawning.
ClosedDecoder supplies fixed borrowed observations and owned result disposition;
no reopening or decoder recovery. Keeping remaining original payloads inside the
closed owner until its caller deliberately disposes it is an explicit cancellation
disposition, not proof that Game received them.

Unresolved RetainedChunkDecoder destruction is process-fatal before its fields
could detach a handle or discard active ownership. A separate real subprocess
must demonstrate that backstop. Stop requests, elapsed time, channel disconnection
and test markers are not native completion. Claim no hard bound on CPU decoding,
allocation, indivisible mutex operations or OS scheduling. No native writer, save,
LOD, Session, renderer or complete-client closure witness is fabricated here.

## Focused defining checks

Use the actual production constructor and native worker with small, explicit wire
chunk packets. Existing inline packet construction supplies independent expected
coordinates, block states and biome mapping; no game-resource bundle, Java, visible
window or compiler spawned by a native test is needed for these decoder claims.
Fixed test controls may hold the actual actor before decoding, after storing its
real result and before native exit. They must not replace ChunkFormat::column
with an echo decoder or let the UI declare the worker finished.

Select roughly six defining normal scenarios, grouping related assertions:

1. Prepared/unstarted closure and controlled spawn refusal preserve the original
   owner, perform no replacement spawn and produce the correct typed outcome.
2. Real ordered multi-column decoding keeps each original format allocation,
   packet and independently expected column contents; a format change does not
   substitute the newest format for an older queued packet.
3. Eight-slot saturation counts queued/active/completed ownership; full refusal
   returns the actual input, successful handoff frees only its own slot, and
   closing permanently refuses later admission.
4. Close with an actual held active decode, queued packets and a stored result
   reaches Blocked, preserves custody, accepts exact Retry, retains a real late
   result and checks the original finished handle before ClosedDecoder.
5. Malformed real chunk input produces an owned Err; controlled actual-worker
   unwind preserves active/queued/stored payloads and gives a checked native
   failure handoff without replay or legacy Lost fallback.
6. Admission/start/close/result races retain every accepted or refused input,
   enforce one original handle and distinguish handoff from still-owned work.

Add one separate unresolved-live-owner fatal subprocess with an actual held
worker, expected entered marker and fatal diagnostic, core dumps disabled and
actual process reaping. Driver/watchdog expiry is failure, never completion.
Keep the existing the_decoder_returns_columns_in_order as the focused legacy
regression. These are estimates, not a numerical cap; add only uncovered defining
behavior identified by SOURCE review. Do not replay old Game/network/save/LOD or
window batteries when this dormant helper does not affect their declared inputs.

Run appropriate formatting, paired all-target client compilation/lints, feature-off
compatibility and public-source checks on the exact candidate, using the existing
pinned toolchain and warm target belonging to this physical checkout. Bind the
actual Cargo artifact/binary and source inputs for native tests. Reuse predecessor
results only at their actual accepted pins. No broad experiment framework or new
resource preparation is proposed.

## Release and stop conditions

First finish C0e2c and record its accepted exact parent. Then obtain fresh read-only
independent Sol High acceptance of this proposal and a bounded assignment, sync
the private checkpoint and record a root release gate before source work. Preserve
the one source and packet-prefix artifacts before corrections; preserve all older
immutable evidence and follow the adopted mutable-source archive layout. Fresh
exact SOURCE/tool review precedes any formatter/compiler/runtime; a different
FINAL precedes consumption or a reviewed public engine push with normal hooks and
fresh repository visibility/origin checks. Existing permissions are not expanded.

Stop this packet at retained-decoder FINAL, private checkpoint and separately
reviewed public branch sync. Legacy/default activation remains unchanged. No claim
of an actual integrated window, ordinary startup/close, client mesh/Session closure,
machine lifecycle, two-client dedicated behavior, G1, live S2, PR, merge, tag,
upstream contribution or release follows.
