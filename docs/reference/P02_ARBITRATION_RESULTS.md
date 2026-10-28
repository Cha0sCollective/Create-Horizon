# P02 source-arbitration reference observations

This bounded packet uses the unchanged Create reference at
`7ff2760e4924d35d4bc2e22c4b19923f0178cd16`, Minecraft 1.21.1,
NeoForge 21.1.219 and pinned OpenJDK 21. It is private, source-informed personal
port research. No native multi-source behavior or full P02/G2 result is implied.

## Authored schedule and observation contract

Select exactly the six cases with `--fixture-set p02-arbitration-1`. The runner's
`--workspace` locates the shared pinned reference and `.local/` runtime outputs
when this adapter is run from an isolated worktree. Its default remains the
repository parent. The selected GameTest namespace is
`create_horizon_arbitration`; existing S0 fixtures retain their own namespace,
schema and accepted captures. All outputs use new UUID directories and fresh
worlds. Gradle uses two workers and a 2 GiB heap; these are functional reference
checks, with no performance claim.

Each fixture places a stone floor at relative y=0 to retain item drops inside
its 16x8x16 observation bounds. Placement order is floor, motor A facing up at
(2,2,2), motor B facing down at (2,4,2). The join at (2,3,2) is initially air.
One scheduled callback per tick samples first, then performs the following
ordered actions/assertions. No private propagation routine or manual entity
tick is invoked.

| Case | Tick 10, disconnected | Tick 30, after sample | Tick 70, after sample | Last sample |
| --- | --- | --- | --- | --- |
| Equal merge | Request A +16, then B +16 generated RPM | Insert Y shaft | None | 120 |
| Unequal merge | Request A +16, then B +32 | Insert Y shaft | None | 120 |
| Opposite merge | Request A +16, then B -16 | Insert Y shaft | None | 120 |
| Dominant removal | Request A +16, then B +32 | Insert Y shaft | `ServerLevel.destroyBlock(B, true)` | 160 |
| Subordinate reversal | Request A +16, then B +32 | Insert Y shaft | Request A -16 | 160 |

At tick 20 both disconnected motor inputs are verified. B's downward facing
negates the generated RPM, so its scroll setting is the negative of the requested
generated value. Tick 10 requests use the real `generatedSpeed.setValue` behavior.
Removal explicitly requests normal world drops; no external item inventory is
introduced or cleared.

The overspeed case places the floor, upward motor A at (2,2,2), Y large cog at
(2,3,2), then Y small cog at (3,3,3). It verifies the captured default maximum
of 256 RPM, starts at +16 motor RPM, verifies small cog -32 at tick 30, then
requests motor +256 after that tick's sample. Samples continue through tick 120.
The nominal conveyed magnitude is 512 RPM; no speed ceiling is changed.

Schema 3 includes each explicit block/air state, fixture-relative position,
block properties, generated/theoretical/effective RPM, motor scroll setting,
source-parent position (empty array means none), flicker score and network
stress/capacity. Network equivalence is the ordered list of observed fixture
members sharing that network, with no raw network IDs. The item observation is
the complete live item registry/count map in fixture bounds; empty means no live
drops. Counts are aggregated independently of random item motion/UUIDs. Schema 1
and 2 extraction and accepted E/F captures remain unchanged.

## Status

Initial setup attempt `5b0e503c-3786-413a-bd58-2432ef9520aa` compiled successfully
and discovered exactly six tests, but failed before the first fixture sample:
NeoForge prepended the holder namespace to an already qualified template name.
The adapter now uses `template="empty"` and creates the authored empty structure
under the selected holder namespace. The failed setup run is not evidence of any
arbitration outcome.

Calibration `51e02410-9002-4247-b7bf-e047e68619d7` exited 0 with all six GameTests
passing and 800 samples, but is not accepted: the initial source-parent extractor
used GameTestHelper.relativePos and emitted wrong X/Z coordinates, and the new
strict loader still needed its exact fixture layouts. Both were corrected before
asserted captures. Parents now match actual world BlockPos values against each
declared observed position transformed with helper.absolutePos; an unknown parent
fails explicitly. Missing/air positions stay explicit.

The calibrated stable outcomes used to author assertions are:

| Case | Observed result after action |
| --- | --- |
| Equal merge | All three +16 RPM, A root; total capacity 524,288 SU |
| Unequal merge | All three driven +32 RPM, B root; A still generates +16; total capacity 786,432 SU |
| Opposite merge | Joining shaft destroyed, one shaft drop; motors remain separate at +16/-16, each capacity 262,144 SU |
| Dominant removal | B destroyed through the requested drop-producing world operation; A and shaft recover +16, capacity 262,144 SU; one motor drop |
| Subordinate reversal | A destroyed; shaft/B remain +32, capacity 524,288 SU; one motor drop |
| Overspeed | Large cog destroyed; motor remains +256 with capacity 4,194,304 SU; small cog stops without a network; one large-cog drop |

Assertions check surviving blocks, theoretical/effective/generated RPM, actual
world source parents, network presence/capacity/zero stress and the entire live
drop map at tick 40, tick 80 where applicable and the final sample. Sampling still
occurs before actions/assertions. These are fixture-specific observations, not
general order independence or an implemented Rust arbitration policy.

The runner archives its original generated empty template per run and verifies
the identical stable template input used by Gradle. Gradle's copy specification
selects only that exact relative resource path; other cache files are not inputs.
This avoids changing the
resource input path for every fresh world, which previously forced a full resource
copy. Schema 3 records/checks the archived template hash. S0 selection excludes the
new arbitration Java class from compilation as well as its adapter identity;
arbitration captures identify both compiled observer files. No shared game world
or configuration is reused. Assertions/adapter inputs remain frozen during each
final pair; two complete matching successful captures and independent review are
still required before acceptance. First asserted capture
`a88a86bd-11aa-4d67-b74f-1510f61ee5a4` passed six tests, exited 0 and passes strict
loading with 800 samples. It precedes the narrowed template copy specification,
so the final accepted pair will use two new matching captures of the final adapter.

## Accepted final pair

Both fresh worlds exited 0 with all six required tests passed and 800 unique,
complete samples per run:

- `bf96ed53-99ee-43e1-8365-8b90c8103044`
- `0e8d3404-3f21-44f3-9345-c453697a71f7`

The strict comparison exits 0: all rows, source/JDK/adapter identity, 211 dependency
records and 15 configuration files match. Only the generated Properties date
comment is excluded from cross-run configuration comparison. A synthetic extra
template-cache file was absent from processed resources after both runs, proving
the added copy specification selects the hashed template alone. The second run's
resource step was up to date; its world and generated configuration were fresh.

Fresh Sol High review found no actionable findings in source tree
`608d4a1426465557dfb7a5f768c83058459380e3`; a strictly read-only follow-up cleared
the narrowed template-copy delta in `43b8fe5a62b5ada412bcf2d20f2aef1c6918b2d5`.
The reviewer independently read both final receipts, all 800 corresponding rows,
adapter/dependency/configuration/template hashes, excluded control output and the
clean pinned source. The coordinator executed the strict loader/comparator.

Eight explicitly derived software controls are recorded outside Git at parent
workspace `.local/s0/arbitration-controls/b735f5d5-3b6d-45f6-b882-7fd053534e10/results.json`.
Missing samples, Boolean zero generated RPM on a passive shaft, unknown parents,
broken reciprocal network groups, kinetic fields on air, changed template bytes
and duplicate JSON keys all return invalid-evidence exit 2. A legal-shaped changed
flicker value produces a useful exit-1 difference. These derived copies are not
new game captures. Both older accepted corpora still load and compare: six cases /
730 samples and fifteen cases / 1,730 samples. Python syntax checks pass.

Raw final-pair summary is `.local/s0/logs/arbitration-final-pair.json`; per-run raw
evidence remains under `.local/s0/reference-runs/<uuid>/`. Existing failed and
calibration runs remain separately identified. The accepted outcomes cover these
specific Y-axis layouts, placement order and scheduled-callback observations.
They do not establish arbitrary source ordering, unloaded contributions, axis
geometry, conflicting cycles or a native multi-source implementation. The Rust
core's unsupported boundaries and full P02/G2 remain unchanged.
