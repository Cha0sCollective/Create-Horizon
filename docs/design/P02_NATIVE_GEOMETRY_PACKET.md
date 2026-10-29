# P02 native static geometry packet

Accepted bounded design after a DIFFERENT fresh independent read-only Sol High
review with no blocking findings. Reviewed draft SHA256:
ffa3cc42ddcc1905e94f810bdc3b2e34abbdbd4909e10fd978a65181793e6340.
This private successor proceeds
from G0; it does not enable live S2/P03, accept G1/G2, or replace the unfinished
source-arbitration work.

## Objective and inspected baseline

Add one small, pure adapter from a complete static dry kinetic fixture to the
existing declared `Node`/`Edge` topology. Keep `solve` and its arithmetic/error
contract unchanged. The missing behavior is discovering supported physical
connections from positions and block orientations; another graph solver or
propagation state machine is unnecessary for this packet.

Inspected private branch `codex/h02-smoke-h03-notes`, HEAD
`b60fae1fb67635dbd72c1308a4e6410f1a990c4f`. The coordinator's CURRENT_WORK and H08
notes were already dirty/untracked; they are outside this authorship assignment.
Create is clean and detached at `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`, matching
workspace.toml: Create 6.0.11, Minecraft 1.21.1, NeoForge 21.1.219. Preserve that
snapshot. This is personal, source-informed reference work, with no planned
Create contributions or independently established clean-room claim.

P02_RESULTS.md and P02_WASM_RESULTS.md qualify settled single-source arithmetic
on declared topology. P02_HORIZONTAL_RESULTS.md accepts source candidate
`4dbc0923cbfbc8b1f6f70f1c32cb1c79f91db337` and two matching fresh captures:
`b2362bee-0d81-42f6-a67a-ca24b665dd12` and
`0a48e0f9-ee92-47d1-b6c2-20e7b4e36f15`. Each has four fixtures, 240 samples and
960 machine observations. Those captures establish their exact X/Z layouts,
EAST/NORTH facing, callback order and settled values. They establish neither
arbitrary-axis closure nor a native geometry implementation.

Actual seams inspected: `crates/create-core/src/lib.rs`, `tests/kinetics.rs`,
`examples/kinetic_probe.rs`, `tools/kinetic_probe_bridge.rs`, `tools/compare_core.py`,
`tools/run_wasm_probe.py` and both `tools/wasm-probe` sources. The native example
and actual Wasm guest already share the bridge. The library has no runtime
dependencies; keep serialization development-only. No execution, compilation,
network, reference edits or production edits occurred in drafting this packet.

## Deliberately bounded input contract

Propose `geometry::derive_topology(blocks)` with a small typed block record:
unique u32 ID, integer `[i32; 3]` position and one of creative motor, shaft, small
cog or large cog. Passive records carry axis X or Z. Motors carry facing EAST or
NORTH, existing `MotorSetting` and explicit capacity-per-RPM. A typed axis/facing
representation may include other values solely to return a specific unsupported
error. No position-dependent registry IDs or generic block-property map in the
library. All four kinds have zero impact in this packet; passive sources are None.

Inputs describe dry, present blocks in ONE complete, finite, closed coordinate
frame with no omitted kinetic neighbors. This is a caller contract for isolated
fixtures, not a host snapshot or loaded-chunk contract. The development decoder
must refuse unknown kinds/properties, waterlogged=true, unsupported axes/facings,
wrong/missing fields, duplicate IDs and duplicate positions. No fluid/waterlogged
property is representable in the supported dry input. An absent record
means absent topology, not an inferred unloaded/air block. There is no dimension,
generation, lifetime, previous solution, edit event, tick or storage state here.

Motor source RPM is setting RPM for EAST and its negation for NORTH. Source
capacity remains an explicit fact checked by existing core validation, not a
new global constant. Reference comparisons author 16384 SU/RPM and magnitudes
16. Normal player settings remain nonzero 1..256; do not add a zero setting to
represent stopping. Removing a record/edge on a new complete invocation is an
independent recomputation, not a modeled Java removal event.

Return sorted nodes plus one oriented edge per unordered connected pair. Shaft
edges point from lower ID to higher ID. A gear-size edge always points LARGE to
SMALL with `Transmission::LargeToSmall`; existing reverse traversal supplies
-0.5 without a second edge. Sort edges by `(min ID, max ID)` regardless of input
order. Pairwise inspection is sufficient for these small fixtures; no spatial
index, graph cache, generic rule registry or performance claim is needed.

Use widened i64 coordinate differences before subtraction/absolute value, so
extreme legal i32 inputs cannot overflow or wrap into neighbors. Errors return
no partial topology/rotation result. Stable validation precedence is: record
identity/shape and supported domain, legal cog placement, unsupported physical
couplings, then topology and existing `solve` validation. Exact error type names
can follow local style; tests must distinguish invalid placement, unsupported
coupling and existing SolveError rather than accepting any error.

## Proposed connection rules and evidence limits

The following X/Z rules are a source-derived implementation proposal. Java
comparison acceptance is narrower: ONLY the four captured layouts at ticks
20/50/60. Other signed directions, translations, ID permutations, opposite
large/small drive and diagonal quadrants get independently authored software
checks; they must be reported as source-derived checks, not additional measured
Create equivalence. No new Java capture is required by this packet.

For each unordered pair, apply mutual shaft connection first:

1. Positions differ by exactly one unit along one coordinate and zero along the
   other two. Each block must expose a shaft toward the other. A motor exposes
   only its facing side; shaft/cogs expose both directions of their AXIS.
   A valid mutual pair yields `Shaft` (+1), including coaxial cogs. Touching
   motors/shafts/cogs on other faces or across a gap supplies no shaft edge.
2. Otherwise a LARGE/SMALL pair meshes only if axes are equal, displacement
   along that axis is zero, and absolute displacement on BOTH perpendicular
   axes is one. Yield LARGE->SMALL ratio -2. Axis mismatch or failure of that
   exact diagonal rule supplies no gear edge. Because records allow X/Z only,
   this does not implement Y geometry. Both signs of each perpendicular unit
   displacement are intentionally source-derived support; only +Y/+Z for X
   and +X/+Y for Z have horizontal capture equality.
3. Detect physically relevant excluded couplings and refuse the complete
   fixture. Equal-axis SMALL/SMALL face neighbors perpendicular to their axis
   would mesh at -1 in Create; return unsupported rather than silently omit
   that edge. Noncoaxial LARGE/LARGE candidates with different axes, absolute
   unit displacement along each of those axes and zero on the third would
   couple in Create; return unsupported. Their sign-dependent +/-1 rule is
   outside this packet. Same-axis diagonal LARGE/LARGE does not mesh. Pairs
   with no connection under these stated supported/excluded rules yield no
   edge, not a guess at an unsupported transmission.

Before deriving edges, reject illegal cog placement within the complete static
supported fixture. For each cog, examine the six face neighbors excluding its
own axis: ignore a neighbor with AXIS equal to that face direction's axis;
otherwise reject if that neighbor is a large cog, or if this cog is large and
the neighbor is small. This is the bounded `isValidCogwheelPosition` rule for
these block kinds. It prevents forced, invalid states from becoming accepted
native fixtures. It does not simulate placement callbacks, drops or terrain.
Unknown kinetic kinds are unsupported; inert surroundings supply no additional
cog exclusion under this source rule.

Source seams under the pinned `content/kinetics/` directory:

- `motor/CreativeMotorBlock.java:43-51`: facing-side shaft and facing-axis.
  `CreativeMotorBlockEntity.java:74-77` with
  `base/KineticBlockEntity.java:506-508`: setting-to-direction sign conversion.
- `simpleRelays/AbstractShaftBlock.java:41-48`, inherited by CogWheelBlock:
  AXIS shaft faces. `CogWheelBlock.java:130-143`: legal cog placement.
- `RotationPropagator.java:54-81`: shaft connection and +1 for these plain
  blocks; `100-120,172-186`: size ratios, small/small rule and exact diagonal.
  The perpendicular large/large branch at `89-97,143-160` is excluded explicitly.
- `RotationPropagator.java:382-444`,
  `base/KineticBlockEntity.java:556-572,588-590` and
  `simpleRelays/SimpleKineticBlockEntity.java:25-37`: face-neighbor enumeration,
  small-cog planar diagonals and large-cog distance-squared-two candidates.
  Do not take a modifier formula alone as permission to connect distant pairs.

Y assessment: UP/Y direct-shaft and size-cog cases in the older corpus could be
covered by adding Y/UP and selecting just those checkpoints, without another
geometry algorithm. They need a wider input/fixture-set contract and extra
comparison cases, not a new solver. This draft deliberately defers that small
expansion to keep its new native reference claim tied to one accepted set; it
is not a technical blocker. Covering ALL older selected cases would additionally
require small/small geometry and hand crank/millstone source/load facts. Preserve
all old declared-topology comparisons. A reviewer/coordinator may explicitly
replace this X/Z-only choice before implementation, but Y must not appear as an
unrecorded side effect of using a generic axis formula. Existing algebra retains
its four ratios regardless.

## Unchanged solver and unqualified behavior

`solve` continues to accept at most one Source record per component, including
inactive Source records. Multiple sources return MultipleSourcesUnsupported;
contradictory cycles return ConflictingCycleUnsupported; source or conveyed
speeds beyond +/-256 return OverspeedUnsupported. No geometry adapter catches
these and substitutes an arbitrary survivor, clamps speed, removes a block,
merges capacity or returns a partial result. Consistent cycles retain the
existing solver semantics. Preserve finite/nonnegative fact validation and
numerical overflow/underflow checks.

The accepted arbitration traces remain observations, not an implemented
multi-source algorithm. Source-parent selection, flicker, transition/rebuild
latency, destruction/drop outcomes, unloaded contributions, encased parts,
perpendicular large cogs, equal-cog meshing, other directions/Y, fluids,
millstones, item processing, retained rendering and live world ownership remain
outside the new adapter's supported contract.

## Bounded implementation surface after design acceptance

Private files only; no new crate, dependency, lock or engine change:

- NEW `crates/create-core/src/geometry.rs` and a module export in `src/lib.rs`;
  do not modify the existing `solve`, motor-setting or transmission arithmetic.
- NEW `crates/create-core/tests/geometry.rs` with independently authored inputs
  and literal expected edges/results; leave existing kinetics tests intact.
- `tools/kinetic_probe_bridge.rs`: add an explicit geometry case form alongside
  the unchanged legacy nodes/edges form. Geometry requests contain block records
  and facts, never caller-declared edges, captured speeds, parents or networks.
  Use exactly {case, geometry:{blocks}} for the new envelope. Passive JSON
  records have exactly id/position/kind/axis; motor records exactly
  id/position/kind/facing/setting/capacity_per_rpm. Parse positions as i32,
  IDs as u32 and setting through MotorSetting. Reject mixed/extra forms.
  Geometry replies add derived canonical edges beside the
  existing rotations; legacy replies keep their exact current shape.
- NEW `tools/compare_geometry.py`: use existing strict `load_run`/`decode` and
  validation helpers, requiring complete `p02-horizontal-1` inputs. Keep
  `compare_core.py` and the Java/strict-loader contracts unchanged.
- `tools/run_wasm_probe.py`: an explicit geometry mode selects the new comparator
  and its control while retaining the existing declared mode. Include new
  comparator and all changed source files in before/after hashes; existing src
  glob covers geometry.rs. Reject mismatched fixture sets. Preserve sandbox
  manifest/capabilities, identity/side-effect checks and artifact/engine drift
  receipts. The guest/example already share the bridge; change them only if a
  demonstrated compile/interface need arises, with review of that delta.

Source-review correction: the development host dispatches a batch as individual
guest calls, so the shared bridge alone cannot enforce unique geometry case names
across that batch. Add ONLY `tools/wasm-probe/host/src/main.rs` to the bounded file
scope for a borrowed batch geometry-name preflight before dispatch. Preserve the
legacy duplicate-name policy, manifest/capabilities, reply checks and one-case
dispatch. Verify duplicate valid geometry case names refuse through BOTH actual
native and sandbox probes; include a focused host helper test. The runner must
hash ALL changed source files before/after, including tests/geometry.rs and this
host source. This demonstrated interface correction changes no geometry/solver
rule or host permission. A different independent read-only Sol High design delta
review accepts this correction with no findings (reviewed document SHA256:
e101d1a1cf59ac814869eb35ff1516a1066ceac6f953193c5c978f0de821cc02).
Corrected source and actual execution still require independent qualification.

Any additional file/dependency/host API need requires a stated packet correction
before scope grows. Do not edit qualified game-smoke drivers. The coordinator
serializes Cargo and chooses a frozen reviewed engine source identity for the
sandbox receipt; active unrelated engine work must not drift during execution.

## Required validation and acceptance

Design review is complete. Implement, freeze the source candidate and obtain
fresh independent source review before execution.
The coordinator runs pinned offline tools and retains commands, exits, sources,
artifacts and failures outside Git. This draft author performed none of them.

Native tests must exercise actual derive_topology, literal canonical edges and
solve together where applicable:

- Four authored horizontal layouts, both signs, exact motor settings and zero
  passive impact; X/Z motor face/rear/side, gaps, coaxial shaft/cog +1 and
  connected/mismatched size diagonals. Direct edge assertions catch a wrong
  topology even if unpowered outputs happen to match.
- Source-derived four diagonal quadrants for each supported axis; wrong
  perpendicular offset (0 or 2), nonzero axial offset, wrong axis and distant
  collinear pairs. Small-driven large uses existing reverse traversal -0.5.
  Include translations, reversed pair/input order and permuted IDs without
  changing the literal expected physical result. Extreme i32 coordinates must
  neither overflow nor create false local neighbors.
- Duplicate position/ID, invalid cog placement, unsupported Y/facing/kind,
  small/small mesh and perpendicular large/large coupling must specifically
  refuse. Preserve all existing malformed-fact and solver regressions.
- A supported two-source geometry must reach MultipleSourcesUnsupported (for
  example two parallel EAST motor/shaft lines driving diagonal large/small
  cogs); a 256 RPM large-driven small must reach OverspeedUnsupported. Do not
  alter old conflicting-cycle tests or claim Java arbitration from these errors.

The new reference comparator independently authors geometry from the accepted
fixture schedule, not measured speeds/parents or predeclared edges. IDs 0..3
map M/S/L/C. EAST settings are +16 at20 and -16 at50/60; NORTH settings invert
those settings. Use exact coordinates/axes in P02_NEXT_KERNEL_AUDIT.md and the
Java fixture. Require literal edges 0->1 shaft, 1->2 shaft, plus 2->3
large_to_small ONLY in connected cases. Verify all returned IDs, fields, types,
finite values and no duplicates/missing cases. Compare theoretical/effective
RPM, stress/capacity and derived operating state with the independently loaded
Java rows. Also compare the native motor rotation with the observed motor
generated RPM and authored converted setting, and exact connected membership.
Membership is the observed-node network equivalence only; do not compare a
native invented parent tree or reuse Java network IDs as inputs.

Select exactly ticks20/50/60 from all four fixtures: 12 checkpoints and 48
machine comparisons per accepted run. Execute BOTH original accepted captures
against the native example and actual freshly packaged Wasm guest using the
same frozen source. Compare native/Wasm canonical edges and rotations as well.
Keep full-trace timing, source-parent selection and drop semantics outside the
native claim; the strict reference loader still validates the complete captures.

Require useful calibration: changing authored C axis or motor facing within the
supported domain must produce an exit-1 difference on connected cases. A
software wrapper that changes a reported edge ratio while retaining rotations
must also fail, proving edge comparison is active. Malformed geometry/probe
controls (missing/duplicate identity, wrong types including Boolean numeric,
nonfinite values, unknown transmission/fields) refuse with exit2; incomplete
reference evidence already refuses through load_run. Controls are software
checks, not additional captures or alternate native implementations.

Run the unchanged native/core and Wasm selected E/F comparisons and their
existing wrong-ratio control; compare legacy reference corpus pairs unchanged.
Run appropriate workspace tests, rustfmt and Clippy and the Wasm compile/check
under pinned Rust1.99.1, offline/locked. No broad live Game smoke is justified
by this pure private addition. No new Java runs or dependency resolution.

Stop on an unexplained mismatch, unsupported-case silent omission, invalid
placement, mutable/incomplete receipt, unexpected dependency/engine need or
source drift. Do not adjust expected values from captured results. Any scope or
rule correction needs source explanation and independent delta review before
a new frozen validation run. Final independent evidence review precedes the
coordinator's authored result note and acceptance. Acceptance may qualify this
bounded static adapter and selected sandbox equality only; full P02, S1/G1,
source arbitration and live S2/P03 remain open.
