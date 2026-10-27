# Remaining P02: observe source arbitration before extending the solver

Status: proposed next reference packet, not an accepted result or implemented
multi-source solver. Source inspected at unchanged Create commit
`7ff2760e4924d35d4bc2e22c4b19923f0178cd16`. Keep this research private and
source-informed. It does not imply any contribution to Create.

## Why the current boundary matters

The native kernel intentionally rejects multiple sources and contradictory
cycles. Merely choosing the largest absolute RPM and summing every source at that
driven RPM would not reproduce the pinned reference. Its propagation also keeps
source-parent links, handles removal/reactivation, and can destroy a block.
Exact survivors and transition ticks still require actual reference captures.

Source reading identifies the following observation targets; these are not newly
measured conformance claims. Paths are relative to `../references/create/`.

| Source seam | Consequence for the next experiment |
| --- | --- |
| `RotationPropagator.java:219` in `src/main/java/com/simibubi/create/content/kinetics/` | Propagation compares conveyed speed in both directions. Opposing nonzero signs, configured overspeed, or excessive flicker can destroy the currently visited block. Equal/faster merges depend on network identity and existing parent links. |
| Same file, lines 268–273 | An already shared network uses a relative epsilon when rejecting a speed-increasing cycle. A generic exact-equality cycle error is only the current kernel's unsupported boundary. |
| Same file, `propagateMissingSource`, line 339 | Removal traverses dependent source links, clears them and tries candidate replacement sources; this is not equivalent to discarding all remaining generators. |
| `base/GeneratingKineticBlockEntity.java:31–53, 123–164` under that directory | Generator source changes can schedule reactivation, retain an overpowering connection, switch source or destroy a conflicting generator. Capture both generated and driven RPM. |
| `KineticNetwork.java:133–174` under that directory | Capacity uses each source's generated RPM; load uses each member's theoretical driven RPM. Both include retained unloaded contributions. Do not substitute one network RPM for both. |
| `base/KineticBlockEntity.java:125–151` under that directory | Source validation returns early for an unloaded source position. Unloaded does not mean air or removed. |

## Bounded fixture proposal

Extend the existing real GameTest adapter and its strict local evidence loader;
do not create another experiment framework. Reuse the pinned classpath/config
capture and two fresh-world repeat requirement. Keep the existing accepted E/F
captures unchanged.

For merge cases place upward motor A at relative `(2,2,2)` and downward motor B at
`(2,4,2)`. Their shaft faces point toward initially empty `(2,3,2)`. Set both
requested values at tick 10 while disconnected, and verify settings and generated
RPM at tick 20. For A facing up, setting equals generated RPM; for B facing down,
negate the target RPM when setting its value behavior. This conversion follows
`CreativeMotorBlockEntity::getGeneratedSpeed` and
`KineticBlockEntity::convertToDirection` (line 506). Equal scroll values on these
opposite-facing motors would actually request opposing generated rotation.
Insert a Y-axis shaft at tick 30, after sampling. This avoids briefly
joining a newly placed motor at its default +16 before applying another setting.
Use one scheduled callback per fixture tick, sampling before the action/assertion,
as in the accepted adapter. Record placement and mutation order explicitly.

| Case | A/B target generated RPM | Further action; final observation |
| --- | --- | --- |
| Equal merge | +16 / +16 | None; sample through tick 120, including individual generated speeds, parent links and total capacity |
| Unequal merge | +16 / +32 | None; sample through 120 to distinguish driven RPM from the weaker source's own generated capacity |
| Opposite merge | +16 / −16 | None; sample through 120, retaining air/destroyed-node observations and item drops rather than assuming which block survives |
| Dominant removal | +16 / +32 | At tick 70 remove B through the real world operation, then sample through 160 for A's reactivation and stable network recovery |
| Subordinate reversal | +16 / +32 | At tick 70 request A = −16 through its real speed behavior, then sample through 160 for its survival/removal and the remaining network |

Repeat the asymmetric cases with the +32 source at A rather than B as a second
bounded follow-up if the first captures show an order-sensitive survivor or
transition. Do not infer order independence from a single arrangement.

A separate overspeed case uses the accepted Y-axis large-to-small layout: motor
at `(2,2,2)`, large cog at `(2,3,2)`, small cog at `(3,3,3)`. Start connected at
+16, request +256 at tick 30 after sampling, and capture through 120. The nominal
conveyed small-cog speed has magnitude 512 RPM; first verify the captured server
configuration's speed ceiling. Record actual destruction and drops, not a made-up
clamped RPM. This observation does not authorize changing the configured ceiling.

## Evidence and stop condition

Add narrowly needed source-parent, generated-RPM and flicker observations alongside
the existing block states, effective/theoretical RPM, stress/capacity and item
accounting. Normalize positions to fixture coordinates and network equivalence
groups to stable fixture members; do not compare raw runtime IDs. Preserve absent
nodes as explicit air. Include every action and its ordering in the adapter hash
and authored schedule. Never call private propagation routines or machine ticks
to manufacture a settled result.

First use captures to establish exact outcomes. Then turn them into meaningful
assertions and rerun twice with matching complete traces and successful GameTest
summaries. Review both assertions and source-state extraction independently.
Unexplained nondeterminism or incomplete evidence is a stop, not an approximate
pass. Do not change the core's current unsupported errors during this observation
packet. A later bounded implementation can consume the accepted outcomes.

Chunk unloading, arbitrary-axis geometry, speed-changing cycles and flicker-limit
thresholds need separate controlled observations. None is established by these
merge fixtures, and full P02/G2 remains open.

The proposed packet received a fresh strictly read-only Sol High source review
in candidate tree `6abf8e94acdf013b739ef7c4c3b9893489f55bee`, with no actionable
source-correctness findings. The reviewer confirmed that extraction/strict-loader
changes, explicit removal/drop observation semantics and reviewed assertions are
prerequisites for future acceptance. No new reference fixture ran in this packet.
