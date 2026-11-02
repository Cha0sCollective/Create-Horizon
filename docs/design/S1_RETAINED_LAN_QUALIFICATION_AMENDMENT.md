# C0e1b: truthful failures and native writer qualification

Status: PROPOSED. No correction authority until exact independent design acceptance
and a private checkpoint. Original proposal6da81085/assignment85c660d4 stay preserved.
SOURCE43b617f7 is NOT cleared: independent review found three concrete blockers;
no compiler/runtime or formatter has executed on that candidate.

## Exact revised scope

Keep client src/lan/{mod.rs,conn.rs,discovery.rs}; add exactly
crates/horizon-client/Cargo.toml for a Unix development-only dependency entry:

```toml
[target.'cfg(unix)'.dev-dependencies]
rustix = { version = "1", default-features = false, features = ["std", "net"] }
```

The existing Unix normal dependency rustix std/time remains unchanged. Reuse locked
rustix1.1.5 and its safe native socket option API; no new crate, unsafe code, protocol
change or production networking feature. Cargo.lock is outside scope: use --locked
on compile/build commands; refuse an unexpected lock change rather than accept it.
Root preserves ALL FOUR source paths and prefix artifacts before each correction.
These four paths replace the original three-path source budget for this packet.

## Error evidence correction

Existing horizon-protocol::Error holds only text. Stop classifying an unrecognized
receive message as native IO. Use an honest Receive(String) outcome for ambiguous
frame/decompression/native-read errors, preserving actual text. Known EOF may be
recorded explicitly only when supported by the concrete baseline error path.
Packet decode errors at known decoder call sites remain Protocol outcomes.

Separate outbound frame encoding/compression preparation from ConnWriter::
send_contents. Preparation failure is Encode/Protocol, not native socket IO;
send_contents returns errors from its actual native write_all and may be labeled
IO. Reuse existing compression/shared-frame encoding, encryption, counters and
ordering. Ordinary/staged/B paths keep their existing behavior. No protocol edits
or string heuristic pretending to recover a missing error type.

Add one focused case covering actual malformed compressed input and a rejected
oversized outbound body. Both preserve the original error and original owner,
must not falsely label codec failure as native IO, and rejoin their native workers.

## Pinger acquisition evidence

Return a typed retained startup failure with exact phase Bind, SocketSetup or
Spawn and original kind/raw-OS/message. Parent must not label bind/setup as Spawn.
No worker may be lost when an error returns. Add one native acquisition case with
an actual occupied UDP bind and actual Builder spawn refusal (Linux MAX/2 stack
request), retaining each correct phase. Test controls remain fixed debug/test-only
variants at the original acquisition sites; they do not return synthetic failures.
Ordinary Pinger API/behavior stays unchanged.

## Strong native blocked-write precondition

Replace elapsed scheduling silence as proof. A fixed Linux-only debug/test fixture
sets the actual accepted socket's SO_SNDBUF before Connection construction and
the native client's SO_RCVBUF before sending the defining frame. Use safe rustix
socket options and read back the effective values. These test-only settings and
helpers expose no production/raw-socket escape or general callback API.

Send a first, uniquely identifiable legal uncompressed frame substantially larger
than the verified combined socket buffers plus the small peer observation. The
peer must read its correct length/id/marker and a small body prefix from that SAME
active frame. Require original writer entered=1/completed=0, the same retained
active frame identity, actual byte progress and an unfinished original writer
before requesting native interruption. No writer pause control in this case.
Capacity mismatch, no identified progress or prior write completion FAILS the
precondition. Never let cleanup produce the passing observation.

The fixture may retain a separate clone of its own client socket only for bounded
native reads/observation. It owns and closes every clone on all paths, uses finite
read/write backstops, interrupts/reaps original workers before assertions, and
makes no delivery or hard-latency claim. Production connection custody is unchanged.

## Qualification and gates

Proposed focused selection32normal: original19 defining+11 affected regressions,
plus the two failure-classification/acquisition cases above. The two additions
directly address review findings; this is the30-case planning prompt, not a cap.
Keep two separate exact-marker fatal processes. No Game/Java/native-save battery.
Protocol20 remains accepted/reused at906f91b under its actual earlier evidence.

Root updates the local qualifier guards to FOUR exact source paths and accepted
amendment/checkpoint pins, adds --locked compilation and exact case names, then
obtains refreshed SOURCE/tool CLEAR before formatter/compiler/runtime. Follow-up
review covers all correction effects, socket/dependency API and feature gates.
Different FINAL, clean successor/private checkpoint and reviewed public sync still
required. No ordinary activation, C1/G1/S2, PR, merge, tag or release.
