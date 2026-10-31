# Managed connected worker bridge results

Status: FINAL ACCEPTED. The exact clean local candidate passes312 normal
cases, two separate verified exceptions, formatting, three feature checks,
optimized compilation and the public-tree audit. Broad369 Python/1375 client tests
pass under an explicitly verified NON-HALLOWEEN child calendar. The native Oct31
baseline fixture failure remains unresolved and preserved. S1/G1 is OPEN.
No public publication is authorized.

## Scope and source identity

Worktree: `.local/worktrees/s1-managed-connected`.
Branch: `codex/s1-managed-connected`.
Parent: `c95ba7ec7ccdaba06e4a23cea5fdbb4e4ddeb076` (FINAL connected recovery).
The previous acceptance and private checkpoint30306db satisfied the design's start
conditions. The current private progress checkpoint is e95dc29.

Accepted design input: `026d3415869a27f7b1726f16b1724adac1d51cc1a6a0749cd680ab1e46432f42`,
preserved in `.local/s1cw1-proposal2.md`.

Current six-path source manifest: `.local/s1cw1-source-sha256.json`, SHA256
`17432ff70f5e771c61ea4fe23cdcbdcbbd6059cffa6fe89ed25c477a31d792b7`.
Expected suite: `.local/s1cw1-test-expected.json`, SHA256
`62d07c34fbd28be1d058ac9fa9cc8ef4a8fa7684f663ae29daae43a06cf3c2a5`.

Paths under `crates/horizon-client/src/`:

| Path | Current raw SHA256 |
| --- | --- |
| game/integrated/managed_stop.rs | 574a404602264cab69c6d9257e6194a860c27b43b943d908fe0e2bd0cbc59487 |
| game/integrated/managed_connected_tests.rs | c4787befc8fe59f5f52e5700e15fe615474f77272f55b30725382007f7b07269 |
| game/lan.rs | ee6c1efe84f1dab245706abcff289e02934eb6db69d7b6952490db1f5501f3f4 |
| game/lan/connected_recovery.rs | c95dedf199adc7eec42e7957c967ba586afc956035532048f44d95399c62a7aa |
| game/lan/connected_recovery_tests.rs | 088496d5cc29caea09701306b42eff6b609a0224c61add424f80c013c42626b5 |
| game/integrated/managed_stop_tests.rs | 3b4e47e68516a5b2687baa2ff5dd61e08dabf3a81959b9bf3e90f91f9e4e3a39 |

Independent Sol High source reviewer `s1cw_source_review` cleared the initial
candidate and the exact current delta. Tool reviewer `s1cw_tool_review` cleared
the tools and current manifest/suite pins. Different final reviewer
`s1cw_final_review` has cleared preflight and the prior candidate's runtime
receipts; FINAL is withheld until all current evidence and commit pins exist.
All reviewers used procedural native read-only checks in a write-capable runtime.
Root alone formats, compiles, executes tests/games and commits, serially.

## Implemented bounded behavior

The existing connected-recovery coordinator runs wholly on the managed worker.
It retains the actual native Game, Guest, GuestPlay and authority leases. The
caller-thread driver receives immutable coalesced status, non-owning typed
controls and one moved pair of memory endpoints. No Game or transport owner
escapes. The legacy closed-profile entry remains supported.

The bridge uses one-slot commands and monotonic attempt IDs. A stale Ready or
older failed-attempt publication cannot reopen admission while a newer request is
in flight. Status carries bounded scalar observations and at most256 UTF-8 bytes
of diagnostics; the supervisor retains full frame/player observations only in tests. Pipe offers
are nonblocking, delivered once and never replayed to replacement observers.

A stop succeeds only after the connected coordinator confirms its checkpoint,
closes and joins native connection threads, and the outer supervisor observes
worker completion, explicitly joins it and validates its matching receipt.
Copied/Promoted failures retain the actual owners until an explicit matching retry.
Pre-Ready refusal is a distinct non-success result after finite cleanup. Late
post-Ready departure remains CleanupBlocked with retained owners and leases.
Unexpected supervisor unwind with a live worker is process-fatal; Drop never joins.

## Current candidate validation

Fresh debug build15859 completed successfully. Frozen directory:
`/home/arbiter/.cache/create-horizon/bin/s1cw1-tests-brs9gevt`.

| Executable | SHA256 |
| --- | --- |
| bin-tests | a63c0f2cececc1a56ec7c10aaea5f960931357aaaa969e7319d8412c029caef3 |
| lib-tests | b9702811c003ea3e62d02cebcd55538220d9ecfd32b778bec1f89552700cae87 |
| protocol-tests | 9716ea008728bfe358d7f6588dcffec41e8fbd97a90ed1ac910fb255b8aa5058 |

Build profile has debug assertions enabled, default features disabled, and exact
features `mods,paired-packages,native-writer-test-control`. Build script SHA256 is
`146d8e968bdaaea45147238243d4248a3691e7ac798cb507a8ec18efb8cf7b18`.
Current full runtime run16638 completed successfully:312 normal cases across eight groups (203 ordinary,103 protocol, four actual connected cases, two legacy resource regressions). Receipt SHA256 aed4139982241ad499a2181d78a53e72be4e64120e93a98bc1845e8ae288241b. Current build receipt SHA256 be5275226d79cbe812e6f48df8debcb3916323b73b2df16a79dfbc6045f92094. Current Ready supervisor exception31548 verified actual SIGABRT(-6), required marker and reaped process in94.05s; receipt c4d10224011dc2e9369cca6b2f9d8184ea7fb9ebbc7c8eafb076f57cfaab041a. Current late-departure exception17325 verified retained2Guest/2GuestPlay, rows/core/keys/clocks/exclusions and refused close across ten coalesced supervisor checks, then explicit root-owned SIGTERM(-15), reaped in93.03s; receipt d5a7e8a3bc86a9faa6343d21dc1dc2b513988e938948513c04570fae442b4da3. Both current source epoch1 full member inventories were verified. These are separate exception observations, not normal passes.

Formatting, three feature Clippy checks and public-tree audit passed in run91335.
The feature combinations are runtime+paired, paired without runtime, and neither.
Validation receipt SHA256:
`3b89d1b97fb8b48875dad56ab47865c7af60d74029cb8f5b99c2ff4e9764086a`.
Current exception reruns and optimized TYPECHECK95906 passed. Native and separately conditioned broad results are recorded below.

## Preserved earlier evidence and correction

The initial source manifest was3d64c6ce and its test file was300d7f1b. Frozen bin
00838b4f belonged to that candidate. Its full suite passed312 normal cases across
eight groups:203 ordinary,103 protocol, four new actual resource cases and two
legacy closed-profile resource regressions. Ignored cases were not counted.
Receipt SHA256:
`92623a9d06dc5e60f735961abf5293c14d0d3ac4820742eb2bf522e8cace24bf`.
The positive/Copied/Promoted cases genuinely loaded their saved continuations after
joining, with restored stone3/5, one offer, frozen clocks and receipt epochs1/2/2.
The pre-Ready case joined after actual endpoint loss and released exclusions.

Two separate exceptions also verified on that earlier candidate:

| Case | Observation | Receipt SHA256 |
| --- | --- | --- |
| Ready supervisor unwind | Actual SIGABRT(-6), required fatal marker, process reaped;95.38s | dba976dc098522c8f7434bf1249495236454145371fe26ff213c108b8d147a39 |
| Late client departure | Retained2Guest/2GuestPlay, rows/core/keys/clocks/locks and refused close; explicit root-owned SIGTERM(-15), reaped;106.69s | 4befefca0e1d9417c28278bd79e2fe718f4364eb3daa0cbf21dfa6c4c57eb810 |

Exception receipts/logs are preserved in
`.local/s1cw1-first-source-exceptions/` and the full prefix snapshot at
`.local/s1cw1-attempt7/evidence/`. Neither exception is an ordinary libtest pass.
Ten retained observations mean repeated supervisor checks of coalesced latest
status, not ten distinct worker publications. Full source epoch1 image inventories
and member hashes were checked by the root exception runner.

Validation63621 then failed five collapsible-if style checks in the new test file.
Before correction, root preserved all six sources and all prefix artifacts in
attempt7. Exactly five equivalent nested conditions became let-chains; short-
circuit order, borrows, assertions, client behavior and exception functions stayed
the same. The other five source hashes are unchanged. Independent review confirmed
Rust1.88 stabilization is within the existing MSRV1.90. Old runtime evidence was
never repinned to the corrected candidate. The corrected candidate was rebuilt
and requalified explicitly.

Earlier ignored snapshots attempt0 throughattempt6 also preserve sources and
prefix artifacts before changes. One tool correction cleared inherited fatal flags;
another removed a calendar-build timeout that could leave compiler children alive
while restoring the checkout. Those corrections were independently reviewed.

## Limits and following work

This qualifies only the fixed two offline profiles on memory transport, a frozen
Survival source with real stone3/5, and an empty machine slot. It adds no Running,
ordinary UI, machine-slot ingress, TCP/dedicated/in-frame qualification,
cryptographic offline authentication, crash/power-loss durability or total-memory
bound. Native pipe buffers remain unbounded; typed publication faults are not OS
I/O or power-loss tests. Existing source-fixture ordinary-LAN reader limitations
remain as recorded in the connected-recovery result.

The prior connected packet reproduced an unchanged native Oct31 Halloween drowned
fixture failure. That baseline qualification remains unresolved. Any conditioned
broad pass must be reported explicitly with fresh current-versus-parent calendar
proof; no source/global clock/global timezone change or test skip is authorized.

The following application-holder design is independently accepted at input
cb1f1496b9404120bd44f734cd015a91b5fa5fffa2bc1345ed8af12b0c156db9,
preserved in `.local/s1app1-proposal2.md`. It remains conditional on this packet's
FINAL acceptance, clean engine commit and private checkpoint. Its pollable-owner
then real frozen-window sequence does not establish gameplay, world rendering,
retained animation or G1. Full S1 continuation remains the owner's instruction.

## Local commit and complete broad qualification

Clean LOCAL commit: `7a08dc25c0e3135e3a0e0a4eaae7c11af5e1d99e`.
Tree: `90791f7e6705732bd85852cd9149cfc41555464e`.
Sole parent: `c95ba7ec7ccdaba06e4a23cea5fdbb4e4ddeb076`.
Six files,1135 insertions/50 deletions. Normal native Windows Git hook PublicTree
passed. A restricted-shell startup failure occurred before the first commit;
authorized native retry preserved source, index and normal hooks. No public push.

Optimized TYPECHECK95906 passed opt3/debug assertions false/tests_executed false.
Receipt50f03db11b4aac4ceefd324716b33c510a6e5a4b74b9cd080521bc88366d117f;
loge9ff454e2ff6df3c4e937328a2f71a73af6809c6568468a503d91e98368b63ad.
This is compilation only, not optimized runtime or performance qualification.

Native broad9654 FAILED:369 Python tests, formatting, workspace Clippy and doc
checks passed; client lib1172 passed/1 failed/51 ignored. The sole failure was
`vanillagen::structure_mobs::tests::ocean_ruin_drowned_match_vanilla`, showing the
Halloween jack_o_lantern/RNG difference. Native failure is preserved unchanged:

| Artifact | SHA256 |
| --- | --- |
| s1cw1-pr-ready.log | c8deed21eecc199476b7b1145251988f2e111a0007e8589622dc36550997f306 |
| s1cw1-broad-status.json | 40f8534036b39f5d3f15239a7a28b9392736bf12da6cd4d13b96474d7491ac3c |

All six sources and all prefix artifacts were preserved in attempt9 before the
calendar comparison. An initial nested-shell path quoting failure occurred before
calendar-script entry; direct environment/argument-vector invocation corrected it
without executing cells or changing the checkout in that failed launch.

Fresh four-cell calendar56755 verified candidate7a08 and parentc95. Both built the
same frozen library executable74c43ea3b6d6bac2a1e5e43cd5d3f549dca09b4571523bcfdf973091ab69dc3e.
Candidate and parent each failed the exact one test with the same assertion vectors
under observed UTC Oct31, and each passed the exact one test under observed
Honolulu Oct30. Local/UTC/TZ were recorded before and after every fresh process;
expected failures are not normal passing tests. Clean7a08 was restored.
Receipt2d51116d96d98d288fd7fbc9253ee53a96c20a7ea607e67bdff940d3abfd928f.
No source, system clock or global timezone change occurred.

Separate NON-HALLOWEEN-CALENDAR-CONDITIONED broad44686 PASSED:369 Python tests,
1375 client tests, formatting, workspace Clippy, doc tests(0), public tree and
unchanged Cargo.lock. Reverse-consumer checks are compilation only; `--quick`
parity is unexecuted. This is not a native-Halloween or broad default-stack claim.
Child TZ was Pacific/Honolulu; observed local Oct30 20:32:47-1000 through
20:39:10-1000, UTC Oct31 06:32:47Z through06:39:10Z. The wrapper pins the fresh
four-cell receipt and verifies non-Halloween before and after. Native Oct31
fixture qualification remains explicitly UNRESOLVED.

| Artifact | SHA256 |
| --- | --- |
| s1cw1-conditioned-pr-ready.log | e45750764956fdec9955cbb239eaafcc2e202bbe9759917d55f17576a164a94a |
| s1cw1-conditioned-broad-status.json | 08be19adae60705633dd76d0fe2252f16c54213c4e9131dbd27b1c0004ff6018 |
| Native and conditioned source comparisons | 3ed1be52725651248ba69d9a30b8bebf7085aafda9ad89c35697b8df071276a8 |

Both comparisons show identical before/after source maps. Four scoped files match
raw/Git/Linux bytes exactly; lan.rs has3965 CRLF and managed_stop_tests.rs420 CRLF.
Those two differ ONLY by exact CRLF-to-LF normalization. The tested raw manifest
17432ff7 is preserved; the committed/Linux identity is independently recorded.
Clean branch7a08 and all raw hashes remained unchanged through validation.

All dirty-parent s1cw1 source-guard/build/validate/test/exception/fast scripts MUST
NOT execute after this commit. The broad route used its separate exact-clean guard.
Final independent review must bind this complete authored result to its exact SHA.
The following application's source remains gated on FINAL, clean successor pin and
private evidence checkpoint; this packet milestone does not end full S1 work.

Independent FINAL ACCEPT: reviewer s1cw_final_review accepted exact authored input
44cfad2ad4dbfb905090ad1a38318486c5cbda5b06093892b701555dbc0049c3, preserved as
.local/s1cw1-final-input2.md. Exact clean7a08/source17432; no unresolved actionable
findings. The native Halloween baseline qualification remains explicitly unresolved.
This acceptance covers the bounded bridge only; full S1 continuation remains active.
