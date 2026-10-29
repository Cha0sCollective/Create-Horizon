# P02 declared arbitration event trial

DESIGN ACCEPTED by a DIFFERENT fresh independent read-only Sol High review of
draft SHA256 e3fffc5418e7f56d3f704b4609248a0942dd7119626c95387f84d8f40ef2a0d5.
Coordinator assignment controls implementation release; actual source/native/Wasm
qualification is unperformed. The earlier isolated-classifier
recommendation in P02_NATIVE_ARBITRATION_AUDIT.md was NOT ADOPTED; preserve that
audit as historical proposed source analysis. This packet instead constructs and
evolves the merged state and its two continuations from explicit events.

Private source-informed personal port work, no Create contribution/public release.
Existing solve, source/geometry contracts and accepted evidence remain unchanged.
No live S2/P03, complete P02, G1/G2, persistence or performance claim follows.

## Inspected baseline and ordering evidence

Private branch codex/h02-smoke-h03-notes is at
`c1224d23a2e3b01e4e5ec8c17fa58e336d0f6abb`; coordinator CURRENT_WORK is dirty and
the prior audit is untracked. Neither is edited here. Read private AGENTS,
CURRENT_WORK, accepted arbitration results/fixtures and current core/probe seams.
The unchanged Create reference remains pinned at
`7ff2760e4924d35d4bc2e22c4b19923f0178cd16`, with the accepted 6.0.11/MC1.21.1/
NeoForge21.1.219/JDK21 environment. Source and selected original observations
were inspected in the preceding audit; no source-derived endpoint is inferred
from a reference speed. This proposal adds no Java capture or engine work.

RotationPropagator.getPotentialNeighbourLocations iterates Iterate.directions.
Local cache inspection found the EXACT accepted Ponder1.0.85+mc1.21.1 JAR,
SHA256 `df0ed5e5d0827eadc3871f14be49e0045c33a13d1c7904dd885c7eb140c5f05b`,
matching the original dependency receipt. It contains
net/createmod/catnip/data/Iterate.class, but no Iterate.java or locally discovered
source JAR. No Java/javap execution or download was performed. This inspection
does NOT establish that field's order. Nor do the captures establish Minecraft's
complete BE/callback order. The native orders below are explicit experiment
assumptions, not claimed Java scheduler observations.

Later coordinator static inspection closes ONLY the dependency-initializer gap:
JDK21 javap on the exact captured Ponder JAR above shows Iterate.directions takes
Direction.values(). The matching captured NeoForge/Minecraft class JAR
SHA256 c871db1044c933f1898ece89c86b5b34776d699c157704b97efb75ce29f18b53
shows Direction's values in DOWN,UP,NORTH,SOUTH,WEST,EAST order. Thus the declared
vertical JOIN neighbors LOWER then UPPER match that static enumeration. This
changes no proposed event input or model behavior and establishes no global BE,
world callback, propagation interleaving or transient timing claim. No class was
executed and no new Java capture was run. Raw logs and exact hashes remain in
workspace-root .local/p02a1-direction-order-inspection.json and the two named
javap logs; neither class archive nor bytecode output enters Git.

## Bounded model and typed interface

Implement one private experimental event model for a CLOSED THREE-SLOT path:
LOWER generator -- JOIN shaft -- UPPER generator. All transmissions are+1; all
blocks are present or absent records. IDs are arbitrary distinct u32 values,
not hardcoded0/1/2, registry IDs or node priority. No fixture name may select a
rule, endpoint, parent, speed, capacity, absent node or destruction result.

Proposed dependency-free entrypoint:

- `TrialSpec { lower:NodeId, join:NodeId, upper:NodeId,
  capacity_per_rpm:u32, join_neighbor_order:[NodeId;2], be_order:[NodeId;3] }`.
- `Event::{ PlaceGenerator{node,generated:MotorSetting}, PlaceJoin,
  SetGenerated{node,generated:MotorSetting}, DestroyGenerator{node},
  Settle{label} }`.
- `run_declared_trial(&TrialSpec, &[Event]) -> Result<TrialTrace, TrialError>`.

The spec declares the two possible physical edges; only present endpoints
participate. Require capacity16384 SU/RPM, zero impacts, maximum magnitude256,
unique IDs, exact endpoint permutations in the two order lists and <=16events.
Both source types are creative motors in this trial. Generation is NORMALIZED
signed RPM supplied as data; no facing/geometry inference or DOWN-motor expansion.
Unknown kinds, extra topology, ratios, loads, unloaded state and config overrides
refuse. The existing general solver is never relaxed or invoked with fake reduced
sources to hide this model's separate contract.

State belongs only to one invocation, and survives its events: each present node
has own generated RPM (zero for JOIN), theoretical/effective RPM, parent, symbolic
network token, flicker, needs-attach, reActivateSource and network-dirty state.
Symbolic tokens identify the native source tree, NOT Java's raw network IDs.
Absent nodes have no kinetic fields. Own generation remains separate from driven
speed even while a generator is parented. Keep an ordered destruction-effect
ledger. Return labeled checkpoints and cumulative domain effects, not a lasting
host object or a generic scheduler API. Labels/case strings are metadata ONLY.

## Exact native event phases

All external commands execute in list order, without an implicit BE round between
them. Immediate recursive propagation drains synchronously where stated.
`Settle` alone runs deferred native BE rounds and then emits a checkpoint. This
phase contract is authored; its rounds have NO mapping to Minecraft tick numbers.
Errors return no partial TrialTrace. Trial validation/rollback is an in-memory
software boundary, not a claim that Java world mutations are transactional.

1. PlaceGenerator: require an absent endpoint and absent JOIN. Set declared own
   generation; invoke the generator's initial generated-rotation update from
   speed0, creating its own network and attaching. Each endpoint is disconnected
   at this point. PlaceJoin requires absent JOIN, two present settled independent
   root generators with nonzero SAME-SIGN, STRICTLY UNEQUAL generated speeds,
   and no pending work. Create a speed0, parentless, networkless JOIN with
   needs-attach=true. Do not assign its eventual source here.
2. SetGenerated first changes that generator's OWN value, then invokes the
   generator update below. Admit only an isolated root or a parented generator
   whose requested magnitude<=its current driven magnitude. Promotion of a
   parented generator, changing a root with dependent children, zero settings
   and other unqualified transitions refuse before mutation.
3. DestroyGenerator removes a present generator, records destroy-with-drops,
   withdraws its network membership, and runs the dependent-parent walk below.
   JOIN destruction and removal of the last generator are outside this packet.
   Control-induced destruction uses the SAME removal/effect path; no branch
   supplies an expected fixture-specific final state.
4. Settle visits present nodes in declared be_order. For each: process pending
   attach; decrement positive flicker by1; reconcile dirty networks; then process
   that generator's reActivateSource via generated-rotation update and clear the
   flag. This follows the relevant source super.tick/derived-tick ordering,
   while the global node order is an explicit native assumption. Repeat until
   no attach/reactivation/network-dirty work remains AND all flicker is0.
   Require a valid parent forest and reconciled groups at that barrier. A fixed
  64-round limit returns NonConvergent rather than fabricating settlement.
   Validation countdowns, lazy load callbacks and frozen-world behavior are not
   modeled; the closed fully resident fixture must never depend on them.

### Source-derived propagation, not endpoint assignment

Connected neighbors are the present path neighbors, in the explicit order for
JOIN and the sole JOIN neighbor for an endpoint. A pending attach executes the
following unity-ratio specialization of RotationPropagator:219-299:

- For currentC/neighborN recompute BOTH current and neighbor speeds. Both0:skip.
  Opposite nonzero signs, >256, flicker>128 or an inconsistent same-network
  speed condition return an explicit unsupported-domain boundary in this trial;
  do not silently emulate the other destruction fixtures.
- If abs(N)>abs(C), set C.parent=N, move C to N's current network, set C.speed=N,
  notify speed change, recurse fromC with neighbor enumeration restarted, and
  return from this invocation. The subsequent recursion must actually reparent
  slower descendants; merely selecting a largest source is insufficient.
- Otherwise current conveys at least the neighbor's magnitude. Same network:
  equal conveyed speed is a no-op; retain the source epsilon check and refuse
  an inconsistent faster conveyance. A current without a network cannot
  overwrite the neighbor's tree; a nonzero networkless current is an invalid
  state boundary. Different networks: set N.parent=C, copy C's
  network, set N.speed=C.speed, notify and recurse fromN, then continue C's
  enumeration. A cross-network current.parent=N would require the source's
  extra removeSource path; declare that inconsistent/unqualified state a
  boundary rather than improvise a cycle policy.
- Setting a generator parent can cancel reActivateSource when the new source
  drives at least that generator's own magnitude, as in the pinned source.
  Moving network membership marks surviving members of affected groups dirty;
  delete an empty group immediately. Reconciliation clears the group's dirty
  member flags. Marking all surviving members is declared native bookkeeping,
  not a claim about Java HashMap.findFirst dirty-member selection. Never leave
  the old generator's contribution duplicated in its former group.

The physical topology is acyclic, but parent/network history still matters.
An independent equal-source merge is refused at admission, so no undocumented
root tie-break is introduced. Same-network equal-speed encounters remain
necessary no-ops during ordinary recursion. No support for arbitrary cycles or
arbitration ordering follows.

### Generator control, removal and deferred activation

Specialize GeneratingKineticBlockEntity:30-52,89-176 without changing the solver:

- If previous driven speed equals the requested generated speed, skip applyNewSpeed
  but retain source capacity/network update behavior.
- Previous speed0: set speed=own, create own network, attach immediately.
- Parented and requested magnitude<=driven magnitude: same sign stays driven;
  opposite sign destroys CURRENT generator with drops. It does not become an
  inactive reverse contribution beneath the faster source. Capacity uses the
  newly requested own value even when driven speed remains unchanged.
- An admitted isolated root change detaches, sets speed=own and reattaches.
  Root speed reapplication and notification are source operations, not a max scan.
- Speed notification adds5 flicker on zero/nonzero or sign transitions only.
  For these zero-load, unfrozen fixtures effective=theoretical. Removed-node
  notifications cannot manufacture kinetic fields on its absent record.

Removal follows RotationPropagator:309-379, not a recomputation from all generators.
For each surviving neighbor whose parent IS the removed node, capture that
original missing parent and start a FIFO frontier at the neighbor. On each pop,
removeSource clears speed,parent,network; if the cleared node was a still-
generating parented motor, set reActivateSource. Enumerate its present neighbors
in declared order, skipping the original missing source and any neighbor without
a parent. If a neighbor's parent differs from the current popped node, append
that neighbor to the candidate list and do not queue it. Otherwise, append it as
a candidate if it still has nonzero own generation, and queue it as a dependent
child regardless. After the frontier empties, try the FIRST candidate that still
has a parent or own generation through normal propagation, then stop that search.
A cleared motor with speed0 cannot instantly convey its OWN generation merely
because it is a candidate: its pending generator tick supplies that activation.
For the accepted dominant-removal tree this produces the delayed A-root/JOIN
recovery; for leaf A destruction there are no dependent surviving children.

Network reconciliation uses present membership and each member's OWN generation
(KineticNetwork:133-175), including parented motors. At zero load, capacity is
sum(abs(own generated)*16384), stress0; do not use driven speed for capacity.
No unloaded contribution, HashMap accumulation-order claim or world-network ID
is modeled. During recursion tokens/memberships are retained state; do not
reconstruct them from the desired final parent tree. The reconcile barrier only
computes facts for the groups produced by those operations.

Destruction effects record node/kind/with-drops exactly once when a present record
is removed. Re-destroying an absent record refuses. The comparator maps the typed
motor effect to the authored creative-motor registry/count projection. This is
a DOMAIN EFFECT LEDGER, not item spawning, item movement, pickup, world loot,
recovery durability or conservation. Native drop physics is not qualified.

## Independent authored streams and measured projections

Original accepted captures: `bf96ed53-99ee-43e1-8365-8b90c8103044` and
`0e8d3404-3f21-44f3-9345-c453697a71f7`, complete800samples each. Load each in full
using the unchanged strict loader, pinned source/JDK/dependency/config/template
receipts. Three Java fixture names below are comparator selection ONLY.

For all three independent trial instances, author IDs A=0,JOIN=1,B=2, path
0--1--2, join_neighbor_order=[0,2], be_order=[0,1,2], capacity16384. Geometry/poses
are comparator context: A UP(2,2,2), JOIN Y(2,3,2), B DOWN(2,4,2). Native code
consumes normalized generation, not those faces or positions.

Common program: Place A generated+16; Place B generated-16 (DOWN's default
setting+16); Settle(initial, unmeasured); SetGenerated A+16 THEN B+32;
Settle(disconnected); PlaceJoin; Settle(merged). Then choose independent DATA:

| Comparator fixture | Remaining events |
| --- | --- |
| arbitration_unequal_merge | Settle(after) |
| arbitration_dominant_removal | DestroyGenerator B; Settle(after) |
| arbitration_subordinate_reversal | SetGenerated A-16; Settle(after) |

Public Java order is sample first; requests at10 after sampling; insert JOIN
at30 after sampling; destroyB or reverseA at70 after sampling. The comparison
maps native labels disconnected/merged/after to Java20/40/80. It does NOT assert
that one native Settle equals any particular number of Java ticks.

Independent source-predicted settled projections:

- disconnected: A+16 own+16 root, group[A], capacity262144; B+32 own+32 root,
  group[B], capacity524288; JOIN absent; no effects.
- merged: all driven+32, A own+16, JOIN own0, B own+32; A<-JOIN<-B(root),
  group[A,JOIN,B], capacity786432; no effects.
- after unequal: unchanged merged state, demonstrating quiescent idempotence.
- after B removal: A+16 own+16 root, JOIN+16 own0 parentA, B absent,
  group[A,JOIN], capacity262144; one motor destruction effect forB.
- after A reversal: A absent, JOIN+32 own0 parentB, B+32 own+32 root,
  group[JOIN,B], capacity524288; one motor destruction effect forA.

All present kinetic nodes have stress0 and flicker0 at these SETTLED projections.
Validate exact registry/properties/air shapes, generated/theoretical/effective
RPM, motor setting conversion (B setting=-generated), parents, normalized observed
member groups, capacity/stress and complete live drop maps in Java context.
Compare native presence, own/driven speeds, parents/groups/capacity/stress/settled
flicker and the declared destruction-effect projection. Never construct native
inputs/parents/groups from measured rows or preload the merged state.

Counts per original run:3programs x3checkpoints=9selected rows and22PRESENT kinetic
machine comparisons (6disconnected+9merged+7after), plus5explicit absent slots.
A terminal checkpoint has to result from rules/events, not its label. Later
Java120/160 observations may remain contextual stability evidence, not additional
native clock reproduction. Transient Java flicker9/8/0 in dominant removal remains
recorded; no sample-by-sample native tick equivalence is claimed.

## Small implementation/validation surface if accepted

Six PRIVATE files suffice; no new crate, dependency, engine API or permanent host
object. Proposed paths: NEW core/src/declared_arbitration.rs and its private unit-
test child core/src/declared_arbitration/tests.rs under crates/create-core;
lib.rs export ONLY;
tools/kinetic_probe_bridge.rs; NEW tools/compare_declared_arbitration.py;
tools/run_wasm_probe.py. Keep geometry, solve, existing tests/comparators,
Java/reference loader, guest and host unchanged unless a demonstrated interface
need receives a reviewed packet correction first.

The bridge's new strict form contains one spec and COMPLETE event stream inside
one case. Each native/guest evaluation constructs a fresh trial and returns its
checkpoint trace; host one-case dispatch therefore does not discard inter-event
history. Existing capabilities, identity/side-effect guards and source/artifact/
engine drift refusal remain intact. No cross-command state or host field is
needed. Case names/labels are metadata, so repeated trial labels are permitted
like legacy batches in BOTH runtimes; do not introduce a batch-name rejection
that the unchanged host cannot preserve. The comparator independently checks
its expected three case slots. Existing geometry duplicate policy is unchanged.

Different fresh design review precedes implementation; source freeze/independent
review precedes candidate execution. Root owns all Cargo/fmt/tests/runtime,
source/engine serialization and integration. Required native tests:

- Three streams above must evolve from absent slots to the literal expected
  parent/group/state/effect traces; no input contains an expected end state.
- Permute IDs and metadata labels while preserving explicit orders/operations;
  outputs must change only under the corresponding identity mapping. Mixed
  same-prefix continuations must distinguish root removal from leaf reversal.
- Swap requested magnitudes A+32/B+16: the rules must construct A as root,
  and removing that root must reactivate B. These are independently authored
  source-derived software checks, not additional measured Java cases. They
  prevent hardcoding B as the winner or A as the reactivated endpoint.
- Parent descendants, candidate selection, deferred activation, membership
  withdrawal and own-generation capacity must each have focused invariants;
  Settle's pending flags/flicker barrier must be tested, not just final RPM.
  The private test child can inspect those transitions without adding a public
  step/debug API or exposing an intermediate checkpoint as Java timing evidence.
- Unknown/duplicate IDs, wrong permutations, malformed events, present/absent
  misuse, >16events, opposed/equal independent merge, unsupported transition,
  >256, illegal parent/group/cycle state and nonconvergence specifically refuse.
  Preserve old solve multi-source/cycle/overspeed and numerical tests unchanged.

Actual native AND freshly packaged Wasm programs compare both original captures;
canonical traces must match across runtimes. Collect exact commands/exits and
source/test/compiler/lock/ref/template/engine/artifact hashes with before/after
checks outside Git. Source closure includes the new tests/comparator. Reuse the
existing isolated sandbox receipt runner with one explicit mode, not a general
experiment registry. Preserve all accepted geometry/legacy modes and controls.

Meaningful actual-probe controls: change reversal request-16 to+16 (A must remain
present); change external removal target B toA (survivors stay driven byB);
omit PlaceJoin (merged checkpoint remains disconnected). These are admissible
alternate programs and must produce useful exit1 differences, not fabricated
fixture-specific outcomes. A structurally valid alternate presence/parent/speed
trace is a differing value, not malformed solely because it differs from Java;
all declared slot identities still appear, with absent slots omitting kinetics.
A duplicate node-ID spec must receive ACTUAL probe
exit2/diagnostic/no partial stdout in native and sandbox; a comparator-only shape
failure, timeout or setup error cannot satisfy that control. Repeated metadata
labels must still execute as independent fresh trial cases in both runtimes.
Derived malformed trace/reference controls cover missing/duplicate checkpoints,
unknown parents, broken reciprocal groups, air kinetic fields, Boolean/nonfinite
numbers, extra fields and changed template; malformed evidence exits2, a valid
changed state exits1. Mark these as software controls, not new Java captures.

Run pinned offline/locked focused tests, core/host lints/fmt/Wasm compile checks
and retained horizontal12/48, vertical6/17 and legacy16/62 actual native/sandbox
comparisons plus strict reference regressions. No new Java or live Game smoke.
Final different independent evidence review precedes any result acceptance.

## Uncertainty, boundaries and stopping

Native ordering is explicitly authored, not verified Java scheduling. Exact
Java iterator/BE/callback sequencing, transient capacity propagation, validation
countdown and reactivation/flicker timing require additional read-only source
resolution or reviewed instrumentation BEFORE stronger timing claims. Existing
settled source/trace evidence supports testing this declared protocol; it does
not guarantee it will match. Stop on a settled mismatch; explain it from source,
revise the model through independent review, or identify the precise missing
instrumentation. Do not fit its final states or silently change round/order rules
from captured outputs.

Opposite/equal-source merging, ratios/cycles, source promotion, overspeed/flicker
destruction, unloaded state, all configs and arbitrary graphs remain unsupported
here. Actual world drops, persistence, rendering, peers, host ownership and
performance remain separate. No complete arbitration/source-order independence,
G1/G2 or live S2 claim is accepted by this PROPOSED packet.
