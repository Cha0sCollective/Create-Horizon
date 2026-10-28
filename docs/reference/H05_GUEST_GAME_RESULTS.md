# H05d authoritative guest counter results

Local engine commit `d2816f6d7f03cec0ab326ec9ad6f1b5cd4a0ed40`, tree
`2fbffbd7e6bea5d85a00f6b378809f342f43d250`, connects the import-free evaluator to
the authoritative counter phase. Explicit guest-on selects the exact captured
primary owner/server code. Each Entry caches compiled immutable code only; every
call rechecks captured identity, Running status and an actual server Store.
Fresh phase preparation/evaluation/commit keeps the host record authoritative.
Unavailable code and evaluator errors latch finite faults without native fallback.

Fresh independent read-only Sol High design/source and equivalent Clippy-delta
reviews find no issue. Native JSON/control behavior remains unchanged; guest
snapshots expose the backend owner/digest. Invocation receipts are explicitly
opted in with HORIZON_COUNTER_GUEST_RECEIPTS=1, bounded to 8 KiB and generated only
for an evaluate call, including failure before the step body. Receipt output is
diagnostic, not a delivery guarantee. Primary namespace only remains a limitation.

Final source SHA256:

| File | SHA256 |
| --- | --- |
| game/counter_control.rs | `e3002b536cca1b63244f9ea1fad6dfc9a9e930da0f7362c4920d8e50929b67ea` |
| mods/runtime.rs | `eddcc3371f7c2f896f61e546812241d47310ba172d86f193d0df77c98707ff68` |
| client/counter_trial.rs | `a9dfe1a312abafd90658349640dd0a8f84087a1e48b06ec364faa39693c5bb66` |
| mods/runtime/counter_trial/tests.rs | `23db7739977f9529d32bce26d8e6a433a2e4f8081134f7a2a9b55addd36b3880` |

All173 binary tests pass (four ignored), plus explicit actual-Rust selector and
captured-token/native-snapshot tests, one each. All13 host tests and116 mod-runtime
tests pass (two ignored); unchanged host/runtime coverage is reused after the
controller's tuple-only then_some and equivalent integer-bound lint fixes.
Final binary/captured tests, runtime-on/no-runtime/off Clippy, mods all-target lint,
format/public-tree and normal commit hook pass. Initial Clippy failure is retained.

Frozen optimized game SHA256
`6e335b872662e05bfb9085241b62454d6a27dfe6ab0c14a535ecd4369e4c4107`
passes actual guest trials with driver SHA256
`c1b2a0b1875029c69264a12cbfa871f02a48fad87bd4e875297f8b3c69a48926`:

| Variant | Mode | Run | Checks | Initial invocation receipts |
| --- | --- | --- | --- | --- |
| Normal | Integrated | `d69a2d0a-b0ad-455d-b333-380a5c8817dc` | 138 | 14 |
| Normal | In-frame | `36c7a25a-99dc-4586-a5b5-bd4e3542bc44` | 133 | 15 |
| Trap | Integrated | `5c4f3c0a-c1fe-4b4a-b1e5-e1ef951676a9` | 91 | 4 |
| Trap | In-frame | `4cf26ec7-a81f-49a0-86fd-35fbd25aa5cc` | 84 | 4 |
| Reject | Integrated | `45fc5c31-0589-4faf-8fd8-ca1df29f1ae6` | 91 | 4 |
| Reject | In-frame | `6a8b8c86-73a9-4b1c-9669-3c5d34f52fc7` | 84 | 4 |
| Invalid +2 | Integrated | `dbfc36de-ee10-4e66-925e-1ae8a8fcea01` | 91 | 4 |
| Invalid +2 | In-frame | `b97ffda8-68bc-4363-8636-58d0400ba01b` | 84 | 4 |

Normal runs prove exact phase/+1 with native comparison, replica disabled, no
invocations during pause/freeze/loaded-ineligible/dormant phases, no-catch-up
reload, replacement/stale keys, dimension lifetime/retirement and restart disabled.
Each fault run commits0->1->2->3, then records one Evaluation fault at input3,
preserves counter/last committed phase, makes no more calls, permits normal block
removal/replacement and restores no backend on restart. All eight runs have null
failure/empty cleanup errors; all initial/restart processes naturally exit0 with
forced=false and null Game errors. No fault is claimed as a clean successful step.

Fault components are independently built reviewed Cargo features, never a runtime
switch or imported capability. Frozen server hashes:

| Variant | SHA256 |
| --- | --- |
| Trap | `7bb3aa0c060e50a1cebe182ce593b29899daed702468c83dc19dc710012c1755` |
| Reject | `0d95eec4c5e806a52d71ccbcb077fa5353abedc8e0f01ba9d8cb11afbebfffb7` |
| Invalid | `4f2495f39417ebafa8ad20569f878996868fdd252610aa4bdac94bcb51e4a8f8` |

Each standard checker reports zero imports/commands/output. The frozen native
default consumer intentionally fails its counter9 request with the expected
variant-specific evaluation error, after its import/type/activation checks; this
is separate expected-negative evidence, not another passing default fixture run.
Shared quiet client and positive server hashes remain in H05_RUST_COUNTER_FIXTURE_RESULTS.

Unchanged full native counter driver also passes: integrated
`323b8434-66f4-45eb-bc9d-780018694423`362/final66; in-frame
`16565144-48fe-4f28-a6aa-9945363c5879`347/final25. Normal initial/restart exits0/null;
refusals1 preserve bytes. Only integrated missing-package cleanup races an already
exited process; the other refusal cleanup records are null.

Broad quick checks pass369 Python/1431 client+mods/zero doc/five content-root/197
launcher tests plus format/lint/public-tree. Cargo.lock is unchanged. The four-file
+551/-9 scope warning is disclosed; parity is explicitly skipped. CI update records
d2816f6 while the footer prints its merge base. No PR/public push is proposed.

Two driver-review findings were fixed BEFORE execution: scratch TMPDIR/TMP/TEMP
are captured/checked, and receipts must remain unchanged across frozen reload plus
its quiet window. Qualified helpers stay immutable. Preserve original wrapper
failures: fresh HOME first hid prepared components; the next nested HOME exceeded
Unix socket length. Short fresh homes with copies of only the approved component
inputs pass. No successful native gameplay was claimed for those failed setups.
Original composite stays counter1/guest-normal0/quick0; separate native rerun2=0
and fault statuses trap/reject/invalid=0 establish the final results.

Raw records remain .local/h05d-* and named game-run folders. Wrappers clear
HORIZON_SESSION_FILE and preserve the display-service runtime directory. Final
independent functional and driver receipt reviews find no issues. Reviewers
reconcile archived inputs, control/console receipts and all sixteen clean exits;
they do not rerun games or independently rehash the Linux game executable.
This transient inventory-free diagnostic
does not establish slots, durable records, external peer replication, retained
animation, performance, coherent recovery or G1. It is not Create conformance.
