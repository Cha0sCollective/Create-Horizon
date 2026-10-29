# P02 vertical and equal-cog static geometry packet

Accepted bounded design after a DIFFERENT fresh independent read-only Sol High
review with no blocking findings. Reviewed draft SHA256:
21d1d040a1ed84fdaf8a7f39a66fb2ec983aaccd146b1b9625adbd37418e119f.
This is a private pure
static successor from G0; full P02, S1/G1 and live S2/P03 remain open.

## Baseline and useful bounded next step

Inspected clean private branch `codex/h02-smoke-h03-notes` at
`0ba28189ed2c1ac1b7e4cb86a84f4269dc9c7d84`. Accepted implementation candidate
`c821d21d42639d699a1fe5248b09426b7bd35760` is qualified in
P02_STATIC_GEOMETRY_RESULTS.md: X/Z passive axes, EAST/NORTH motors, adjacent
mutual shafts and equal-axis large/small diagonals, with exact horizontal
native/actual-sandbox comparisons and strict controls. Preserve those results
and their limits. The existing solver and legacy comparison still remain the
single-source declared-topology baseline.

Choose BOTH Y/UP support and small/small meshing. Together they permit direct
geometry comparison of four already captured fixtures with no new source kind,
state machine, dependency or Java experiment. Adding only Y would leave the
simplest captured equal-cog fixture unsupported; adding only equal cogs would
lack a matching X/Z Java small/small fixture. The implementation is two local
extensions of the accepted adapter, rather than another solver.

Read private AGENTS/CURRENT_WORK, workspace.toml, current geometry/core/probe and
comparators, PortReferenceTests.java, strict schema-2 loading, and BOTH original
E/F console selections/environment receipts. Create remains clean detached at
`7ff2760e4924d35d4bc2e22c4b19923f0178cd16`, matching the pin (6.0.11, Minecraft
1.21.1, NeoForge21.1.219). Preserve it and the source snapshots. Personal reference
use is source-informed, with no Create contribution or clean-room claim. No
engine access, execution, imports, WSL, compilation, scripts, test/build/format,
network, commit or production edit occurred in this authorship assignment.

## Exact measured selection and independent authored inputs

Use the existing accepted complete `s0-edges-1` captures only:

- E: `b531ce69-28ce-4a5a-a8a2-e5ea3b10ebe2`
- F: `d0528c63-86c7-416b-9dbf-5ee33c1e6a50`

Raw originals are workspace-root `.local/s0/reference-runs/<id>/`. Each complete
capture contains1730rows, not just the proposed selected rows. Retain the existing
full strict loader and all receipt/config/dependency checks unchanged. The stored
older adapter hashes agree between E/F; the current Java helper has since gained
other fixture extraction. Its selected schedules/coordinates remain consistent
with both retained captures. Do not present the current Java file hash as the
historical capture's adapter identity.

For these inputs use dry Y-axis passive blocks and UP motor0 at M=(2,2,2), with
capacity16384 SU/RPM and zero impact for every record. IDs are the authored
observation slots, including the gap after removal. Do not renumber remaining
records or inspect measured outputs to decide record existence, setting or edges.

| Fixture/tick | Authored present records after M0 | Motor setting | Literal canonical edges |
| --- | --- | --- | --- |
| direct_stop_reverse/010 | shaft1=(2,3,2), shaft2=(2,4,2) | +16 | 0->1 shaft;1->2 shaft |
| direct_stop_reverse/030 | same | -16 | 0->1 shaft;1->2 shaft |
| direct_stop_reverse/070 | shaft2=(2,4,2); record1 absent | -16 | none |
| small_to_small/020 | small1=(2,3,2), small2=(3,3,2) | +16 | 0->1 shaft;1->2 equal_cogs |
| large_to_small/020 | large1=(2,3,2), small2=(3,3,3) | +16 | 0->1 shaft;1->2 large_to_small |
| small_to_large/020 | small1=(2,3,2), large2=(3,3,3) | +16 | 0->1 shaft;2->1 large_to_small |

Exactly6checkpoints/17PRESENT machine comparisons per capture:3+3+2+3+3+3.
At direct/070 reference slot1 must be the explicit air observation with empty
properties and no kinetic fields; native input/output IDs are `[0,2]`, not
`[0,1]` or a fabricated air node. Other selected cases use `[0,1,2]`.

Independent schedule expectations come from `PortReferenceTests.java:140-189`:
sample first at every scheduled callback; default setting+16; the tick10 action
sets-16 AFTER that sample; tick30 removes DRIVE AFTER that sample; tick70 samples
the disconnected final snapshot. Gear cases place motor then driver then target
and sample before asserting the settled tick20 speed. Default `axial` setsY;
`motor` facesUP. Small/small target offset is(+1,0,0); both size cases use(+1,0,+1).
The strict loader's `expected_machines` independently fixes the same identities,
positions and tick>30 air boundary. Do not infer timing or an event algorithm
from recomputing these independent settled snapshots.

Source-predicted theoretical/effective speeds at the selected checkpoints are
respectively direct(+16,+16,+16), direct(-16,-16,-16), gap(-16,0 for ID2),
small/small(+16,+16,-16), large/small(+16,+16,-32), small/large(+16,+16,-8).
All present nodes have stress0; powered nodes have capacity262144. Gap node2 has
capacity0 and unpowered state; its motor remains running. Both retained original
row selections agree with these authored predictions. Outputs remain the measured
comparison side, never input facts or the method for generating expectations.

Schema2 limits MATTER: these rows have no setting, generated_rpm, source_parent,
network_members, item-drop map or actual absolute rotation-basis receipt. Compare
only the recorded fields plus exact authored topology. Require expected observed
network presence (all powered=true, gap node2=false), but make NO Java network-
equivalence/parent or generated-speed observation claim. The older loader checks
facing/axis with `.get` rather than exact property-key sets; the new selected-row
comparator must additionally require motor `{'facing':'up'}`, passive
`{'axis':'y','waterlogged':'false'}`, exact machine/network field sets, and the
explicit gap-air form. Preserve the older loader; do not weaken or globally
change it to accommodate this packet.

Positions are the fixture's authored/helper coordinate frame. The old capture
does not establish an absolute unrotated world basis; its all-Y/UP geometry and
quadrant-independent ratio formula do not justify an absolute-world-quadrant
claim. No new rotation normalization or missing schema3 facts may be fabricated.

## Two source-derived geometry extensions

Keep the complete, dry, static fixture contract and integer widening, legal cog
placement, unique ID/position checks, canonical edge ordering and no-partial-
result behavior. Add Y to passive axes and UP to motor facings. UP exposes only
(0,+1,0); its generated source RPM equals MotorSetting. NORTH retains sign
inversion, EAST retains its existing sign. DOWN/WEST/SOUTH remain unsupported;
unknown kinds/properties, wet input and invalid numeric/setting fields refuse.
The existing typed bridge already decodesY/UP; no bridge/host/schema change is
needed merely to allow these values in the domain adapter.

After the existing mutual-shaft check, an equal-axis SMALL/SMALL pair separated
by exactly one face step perpendicular to that axis yields `EqualCogs`(-1),
from lower ID to higher ID. A coaxial small/small pair remains `Shaft`(+1) by
precedence. Unequal axes, a face gap>1, planar diagonal or a non-face displacement
supplies no small/small mesh. Do not treat a potential diagonal neighbor alone
as a connection. Existing same-axis large/small rules then apply onY as well,
with canonical LARGE->SMALL -2 and reverse traversal -0.5; no duplicate reverse
edge. The specific source branch order may stay as currently implemented so
long as mutual shafts retain precedence and size kinds are disjoint.

Source seams under unchanged `content/kinetics/`: CreativeMotorBlock.java:43-51
(facing shaft/axis), CreativeMotorBlockEntity.java:74-77 with
base/KineticBlockEntity.java:506-508 (sign), AbstractShaftBlock.java:41-48 (AXIS
shaft faces), RotationPropagator.java:54-81,100-120,172-186 (shaft,+size,-1 mesh),
CogWheelBlock.java:130-143 (legal placement), and the previously inspected actual
neighbor enumeration in RotationPropagator/KineticBlockEntity/SimpleKineticBlockEntity.
Perpendicular LARGE/LARGE couplings still explicitly refuse for all relevant
axis pairs, now includingY; their sign-dependent +/-1 rule is not implemented.
Illegal large-cog face placements must still fail before deriving topology.

The broader new support (small/small onX/Z, both perpendicular signs, translated
and permuted layouts, Y size-cog quadrants beyond the one captured) is explicitly
SOURCE-DERIVED software support, not additional measured Java conformance. Native
unit tests must cover it. Measured equality is restricted to the6selected Y
snapshots and previously accepted12horizontal snapshots. No arbitrary-axis,
all-direction, lifecycle or ordering closure follows.

## Four-file implementation surface after design acceptance

1. `crates/create-core/src/geometry.rs`: allowY/UP, add the EqualCogs physical
   edge and update only the now-obsolete refusal paths/error variants/docs.
   Preserve the entire existing solver, MotorSetting and Transmission arithmetic.
2. `crates/create-core/tests/geometry.rs`: replace formerY/small-mesh refusal
   expectations with positive and boundary regressions. Update only corresponding
   malformed-decoder controls (Y is now valid); keep missing/unknown axis,
   unsupported facing/kind, duplicate, placement and numerical controls. Retain
   old measured X/Z cases and all other meaningful checks.
3. NEW `tools/compare_vertical_geometry.py`: one dedicated fixed6checkpoint
   comparator using existing load_run/decode/finite helpers. Its input table is
   the authored table above. Validate exact per-case PRESENT IDs, not range(4),
   exact ordered edges, typed finite rotations and strict selected reference
   shape. Keep the geometry JSON form/results unchanged. Do not import a generic
   topology builder, read expected edges/speeds from native outputs or invent
   network-membership fields. Save canonical outputs under coordinator .local.
4. `tools/run_wasm_probe.py`: one explicit `--vertical-equal` mode, mutually
   exclusive with existing `--geometry`; default remains legacy declared mode.
   Select only the new comparator for this mode and s0-edges-1 inputs; retain
   existing horizontal/legacy commands and controls. Reuse existing native/
   sandbox execution and canonical equality receipts through a small explicit
   branch, not a profile/registry framework. Add the new comparator to before/
   after source hashes. Preserve geometry-test hashes, reference/artifact/source/
   engine drift checks, untracked-engine refusal and cleared session environment.

NO lib/export, bridge, native example, guest, host, Java collector, reference
loader, existing comparators, engine, lock or dependency change is needed. Report
any demonstrated additional interface need BEFORE scope grows. No new fixture
format, source kind, host API, world registration or game-smoke driver changes.

## Acceptance and stop conditions

Different fresh independent design review precedes implementation; independent
source review of the frozen candidate precedes execution. Root serializes all
Cargo, selects a frozen reviewed engine identity for actual sandbox receipts and
owns formatting/runtime/commits. Preserve all failed attempts accurately.

Native tests must assert literal canonical edges AND solve outcomes for the six
selected Y snapshots, including ID gap[0,2]; UP front/rear/side/gap; all passive
axes; small/small contacts in both signs on each perpendicular axis; coaxial+1,
wrong axis, face distance2 and diagonal no-mesh; Y size transmission in both
drive directions; translated/permuted IDs and extreme i32 coordinates. Retain
specific invalid-placement and perpendicular-large errors. DOWN/WEST/SOUTH,
unknown/wet records, multiple sources, conflicting cycles, >256RPM and numerical
range behavior remain explicit refusals from the appropriate layer. Existing
solver tests remain untouched; do not implement arbitration or destruction.

Compare BOTH original complete E/F captures with fresh native and actual sandbox
probes at exactly6checkpoints/17present machines each. Save and require byte-equal
canonical edges/rotations across runtimes, bind source/test/compiler/locks/
reference/engine/artifact hashes and before/after drift checks. Match recorded
speed/stress/capacity/state plus expected observed network presence; no source-
parent/tick/network-equivalence claim. No new Java execution is required.

New comparator calibration uses the actual probes: change target small-cog axis
in small_to_small/020 (Y->X), or its motor facing (UP->EAST), to produce useful
exit1 differences within the supported domain. A software wrapper changing the
reported equal_cogs edge to shaft while leaving rotations intact must also
exit1. Duplicate a fully valid geometry case through BOTH real probes: require
actual exit2, duplicate-name diagnostic and empty stdout; setup/timeouts or
comparator-only rejection cannot satisfy that control. Preserve the existing
three horizontal semantic controls and duplicate-batch controls unchanged.

Derived malformed-output controls must reject missing/duplicate case or PRESENT
node, fabricated gap node1, Boolean ID/numeric, nonfinite/string RPM, unknown
transmission and extra fields with exit2. A valid-shaped changed RPM must exit1.
Validate wrong/missing selected reference properties and incomplete full captures
as malformed evidence; schema2 limitations are not permission to accept extra
orientation/fluid state. These copies test software validation, not new captures.

Run appropriate pinned Rust1.99.1 offline/locked workspace tests, both workspace
formatting and core/host Clippy checks, Wasm compile check and bounded Python
compilation. Re-run accepted horizontal12/48 native/sandbox comparisons against
both horizontal originals, unchanged E/F legacy16/62 native/sandbox comparisons
and controls, and retained strict reference-pair regressions. No live Game smoke,
Java capture or new dependency resolution is justified by this pure addition.
Final different independent evidence review precedes a separate authored result
note and coordinator acceptance.

Stop on unexplained source/reference mismatch, illegal forced placement, silent
unsupported coupling, missing/changed receipt, artifact drift or a demonstrated
need beyond these four files. Do not fix expectations from observed outputs.
This packet neither accepts multi-source arbitration, propagation/destruction,
processing, persistent machines, rendering, peers, performance nor G1/G2; live
S2/P03 still waits for G1.
