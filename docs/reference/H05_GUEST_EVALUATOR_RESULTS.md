# H05b1 import-free evaluator results

Local engine commit `fc6811a0066a495f441d4891473477356c0dec68` adds the optional
diagnostic evaluator, without World or Game wiring. Public construction takes a
checked pair and selects its captured server Component and package identity.
The literal exported instance is `counter-trial`; its synchronous request/proposal
types are local experimental bindings, with no shared SDK WIT/API-line change.

Zero actual component imports and exact structural types are checked before any
Store/start. Every call creates a fresh Store containing only limits, with an empty
linker and one fuel allocation before instantiation. Pinned Wasmtime48's call API
completes lowering, body, lifting and canonical post-return before success. The
Store is dropped before validating echoed identity/time/counter and checked +1.
No host State, world, storage, log, output, clock, WASI or ordinary mod callback is
available. Request identifiers are bounded at 256 UTF-8 bytes each; engine error
text uses the existing 512-character clipping helper. These are diagnostic limits.

Fresh independent read-only Sol High design, source and focused fix reviews find
no blocking issue. Final key source SHA256:

| File | SHA256 |
| --- | --- |
| runtime/counter_trial.rs | `ecaa53c18fe6a480beed053ce136318ab92cc7cb6131e51ab70263006b2d8237` |
| runtime/counter_trial/tests.rs | `9411a4bfa053b92dd6e8b50d95799ea77b3177b7ca8ecb6eb7774624f27035fa` |
| runtime.rs | `4ca880a931db6195cf8b717a97f0e52c14c72d1218aafda84696f15475143267` |
| horizon-mods/Cargo.toml | `ee21752ef48f3809da36eb1ed5d7d45a36c038eeb67406e38ac7fcb0e7935f84` |
| Cargo.lock | `abdaf2e6adc423aa731744320e65ce7ab38b2caa879cd22f009d9b692768a266` |

Eight actual Wasm groups pass: request projection/fresh globals after success and
failure; import/missing/wrong export refusal before trapping start; current-engine
async refusal; start/body/realloc/post-return traps/loops and malformed lifting;
one shared start/body fuel budget; memory/table/instance limits and boundaries;
invalid scalar/rejection/overflow behavior; and request/error bounds. Async coverage
is engine compilation refusal, not execution of an async-enabled guard.

The complete mod-runtime suite passes 116 tests, with one existing ignored test
and zero doc tests: 54 library, 4 API-reference, 15 paired, 6 paired activation,
5 paired types and 32 sandbox cases. This full-suite execution predates only a
test assertion's let-else lint fix; it is reused unchanged production/fixture
coverage. All eight evaluator groups and feature-on/off Clippy ran again afterward.
Runtime-only/paired-only checks, formatting, public-tree and normal commit hook pass.

Preserved failures remain excluded: initial compile needed Clone/Copy for the
canonical enum lowering derive; the first runtime run passed seven groups and
failed malformed instance-limit fixture generation (replace matched a declaration
and a reference); then Clippy rejected err().expect in the async test. The fixes
are narrow derives, first-match fixture insertion and equivalent let-else assertion.
Raw logs/statuses remain .local/h05b1-*; final native status is zero.

The commit is five files/+606, including concrete Wasm fixtures, beyond the soft
500-line guideline. No PR or public push is proposed. Bare-component tests do not
prove real Rust SDK component buildability, the public pair factory's execution,
ordinary activation or Game callbacks. Private Rust fixture source passes fresh review;
no purity is inferred from capabilities alone. Combined broad/native-game checks
with host commit83649b8 pass: 362/347 native counter checks, 369 Python, 2,384
workspace, two doc, five content-root and 197 launcher tests plus formatting/lint.
See H05_HOST_PHASE_RESULTS for exact frozen identity, runs, refusals, scope warning
and skipped parity. Final independent read-only receipt review finds no issue,
including reused coverage and excluded failures. No persistence, inventory,
stable ABI, performance or G1 claim follows.
