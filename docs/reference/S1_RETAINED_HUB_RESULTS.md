# Original Hub with retained transport

FINAL/CLEAN `e722bcf58ae236ebd0418e410ba15c9e9747c32f`, tree
`3bb3a290c91660c1f2315a3add1e107853ee9a9c`, parent `11c640c5`, branch
`codex/s1-retained-hub`. Different independent FINAL/privacy `ff8c6f11` accepts
this component. Normal-hook public push passed and exact remote HEAD was verified;
public sync `3b95ca87`. S1/G1 remain open.

## Behavior

The original Hub now directly owns the original retained offline MemoryPipe/TCP
transport through an opaque concrete owner. Ordinary Hub construction and staged
recovery retain their legacy transport and use explicit accessors. No placeholder
Lan or duplicated Hub simulation is introduced.

Retained admission validates actual connection/sender identity before changing
entity IDs or creating a Guest. Bounded native events and inbox processing retain
undispatched and applying originals. Departure, duplicate or transport failure
freezes the original roster instead of removing guests. Newly refused outbound
vectors remain in the original queue across closing/seal/disconnect/capacity;
their allocation, ordering and first failure are preserved without replay.

Exact close/Retry polls finished original actors and preserves guests, inboxes,
pending/refused inputs and raw outcomes through opaque ClosedHub completion.
Retained terrain uses the same original synchronous encoder without submitting
to the detached global warm pool. Legacy/staged warming stays unchanged. A pool
acquired by another legacy path may still exist.

## Qualification

Accepted proposal `149ef6dc`, assignment `9f88ca08`, private checkpoint `e64fd1d7`
and root release `e33524b6` bind 14 source paths. Independent SOURCE/tool and the
warm-pool refinement review cover the actual scope. Successful results bind source
`f26be673`, qualifier `dc5df61b`, definitions `a0dddf4f` and actual matching bin
`0b618404`, library `3601249e` and LAN bot `e6568c52`.

| Check | Result |
| --- | --- |
| Formatting, paired all-target Clippy, feature-off compile, public tree | Pass, 64.27 seconds |
| Matching three test executables | Pass, 107.31 seconds |
| Four defining Hub/refusal groups and two sender regressions | Pass, 1.51 seconds |
| Actual staged two-client nonzero-item recovery and coherent close | Pass, 109.27 seconds |
| Ordinary LAN status/login/terrain/placement/broadcast | Pass, 42.91 seconds |

All eight runtime groups passed on their first attempt; no older result was
presented as new execution. The defining positive test uses actual original
Session/Hub MemoryPipe and loopback TCP login, independently expected flat terrain
section values and meaningful recipe-book changes through native packets/inboxes.
This proves encoded terrain, not window rendering or global-palette behavior.

Negative/close cases retain the actual refused Joined allocation before mutation,
the actual departing Guest, original raw refusal vectors and the same guest/inbox
allocation through Blocked and explicit Retry. The writer fixture pauses the
original actor before its native write; this is not a blocked-native-IO claim.
Existing direct sender tests verify held ordering and no POST helper acquisition.
The staged regression restores inventories of three and five items without
recovery ticks, and completes the native coherent close.

The exact 28,176-entry resource inventory and prebuilt pair are checked before and
after. Fresh HOME/game/XDG/temp/working directories isolate each run. A checked
resource symlink supplies the ordinary bot's compiled fixture path, preventing
its missing-resource skip. Build uses pinned Rust, opt-level zero, line-table
debug information, assertions/overflow checks and exact mods,
native-writer-test-control,paired-packages features with no default features.

## Integrity and remaining composition

Two earlier compile checks remain distinctly failed. The first found a missing
test import and one multiline legacy staged caller; the second found only an
equivalent test let-chain style issue. Both passed formatting, but neither reached
build/runtime. Attempts0-5 preserve all source/prefix states and moved failure
outputs. Corrected source was independently reviewed before retry.

Final input index `f1c1ac72` freezes 261 inputs. Source map `42066005` binds 42
live/archive pairs: 14 changed files match committed Git raw-exact, 21 unchanged
parent/current baselines compare explicitly after CRLF canonicalization, and seven
private inputs are excluded from engine Git. Baseline copies were taken after
qualification. Guarded dirty-parent execution was then committed exactly; no
archive or clean-commit execution is claimed. The qualifier is retired by its
parent guard. Fifteen historical indices and 4,904 entries remain unchanged.

The different reviewer independently rehashed all 261 frozen inputs and all 4,904
historical entries initially/finally, and recomputed all 42 source-map bindings,
with zero mismatches. It inspected the complete diff and actual guarded code,
assertions, receipts and logs. Native read-only review did not reopen live Linux
binaries/resources; those facts rely on the inspected execution guards. No
correctness, compatibility, failure-path or privacy findings remained.

Actual retained server Game startup, integrated worker ownership, player/world
storage closure and shared LOD ordering remain separate. Later Game composition
must fence its actual producers before using Hub completion. No C0 composite,
default/C1/G1/S2, whole-client, memory/hard-latency/performance, PR/merge/tag/release
or asset redistribution acceptance follows from this component.
