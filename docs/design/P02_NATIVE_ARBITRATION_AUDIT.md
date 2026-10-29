# P02 native arbitration audit

DRAFT audit and next-experiment recommendation; NOT independent design acceptance
or implementation authority. No change to existing solve or geometry is proposed
here. Private source-informed personal port work preserves the Create reference;
no upstream Create contribution, public release, live S2, G1/G2 or performance
claim follows.

Coordinator disposition: retain the source analysis, but do not adopt the proposed
standalone driven-request classifier. A separate five-file API for one conditional
would add maintenance without reproducing the causal merge/removal behavior. The
next design instead investigates a bounded declared-topology event trial covering
unequal merge, dominant removal and subordinate reversal. That proposal requires
its own exact semantics and independent review before implementation. The original
recommendation below is retained as audit history, not active work authorization.

## Inspected state

Private branch `codex/h02-smoke-h03-notes` was clean at
`c1224d23a2e3b01e4e5ec8c17fa58e336d0f6abb`. Both static geometry packets are
qualified. The pure adapter supports XYZ passive axes, EAST/NORTH/UP motors,
mutual shafts, small/small and size-cog edges. DOWN/WEST/SOUTH motors and
perpendicular large-cog coupling still refuse. `Node`/`Source`/`Edge` and solve
retain no parent, source ownership, flicker, previous state, pending activation
or destruction history. Multiple Source records, conflicting cycles and speeds
beyond256 remain explicit unsupported errors; leave that contract untouched.

Read AGENTS/CURRENT_WORK, workspace pins, P02_ARBITRATION_RESULTS.md, the exact
six Java fixture schedules, current core/geometry and pinned propagation,
generator, block-entity and network code. Create is clean detached at
`7ff2760e4924d35d4bc2e22c4b19923f0178cd16` (6.0.11/MC1.21.1/NeoForge21.1.219).
Read-only native inspection also checked selected original rows in BOTH accepted
arbitration captures `bf96ed53-99ee-43e1-8365-8b90c8103044` and
`0e8d3404-3f21-44f3-9345-c453697a71f7`, each a complete800-row capture. No engine
access, candidate execution, import, WSL, Java/Python/Cargo, scripts, build/test/
formatting, network, commit or production edit occurred. Only this audit is new.

## What the six cases actually constrain

The shared merge fixtures place floor, A motor UP at(2,2,2), then B motor DOWN
at(2,4,2), with JOIN(2,3,2) initially air. Every scheduled callback SAMPLES FIRST.
At10 request A generated+16 THEN B's requested generated value. DOWN B's setting
is the NEGATIVE of generated RPM. At20 verify disconnected generated inputs.
At30, AFTER its sample, insert the Y shaft. At70, AFTER sampling, only dominant
removal explicitly destroysB with drops, or subordinate reversal requestsA-16.
Assertions are at40,80 and the final120/160. No private propagation/tick is called.

| Case | B generated request | Selected settled observation | Missing closure |
| --- | --- | --- | --- |
| equal merge | +16 | A root; JOIN<-A,B<-JOIN; all+16, capacity524288, no drops | Why this root under other insertion/neighbor order |
| unequal merge | +32 | B root; JOIN<-B,A<-JOIN; driven+32 while A STILL generates+16; capacity786432 | General priority, order independence, reparent/cycle history |
| opposite merge | -16 | JOIN air, one shaft drop; separate motors+16/-16 | General choice of which CURRENT entity is destroyed |
| dominant removal | +32 | Explicit B destruction; A/JOIN recover+16, capacity262144; motor drop1 | Parent-subtree clearing and generator reactivation scheduling |
| subordinate reversal | +32 then A request-16 | A air; JOIN/B remain+32, capacity524288; motor drop1 | Native merge-prefix construction, world destruction and full transition timing |
| large/small overspeed | No B | A+256, large cog air, small stopped; large-cog drop1 | Current propagation actor and reciprocal conveyed-speed destruction |

The last case instead places floor,A UP,large Y at(2,3,2),small Y at(3,3,3).
It begins at+16, verifies small-32 at30, THEN requests A+256. It checks the
captured default maximum256 and nominal conveyed magnitude512. This is not a
source-capacity arbitration case, and the current solver's OverspeedUnsupported
must not be changed to guess the observed destroyed cog.

The source callback input order is known. General enumeration is NOT an input
order proven by these six captures: RotationPropagator traverses actual connected
neighbors via Iterate.directions and additional cog locations. This audit did
not establish every dependency-owned iterator order, initialization/update flag,
world callback order or BE tick scheduling rule. Do not silently substitute ID
sort or maximum speed as Create's global propagation order.

## Why a maximum-speed snapshot rule is wrong

Under pinned `content/kinetics/`:

- RotationPropagator.java:219-299 compares conveyed speed in BOTH directions,
  destroys the CURRENT propagation entity on opposite sign/overspeed or flicker
  above128, recursively reparents faster trees, and has a same-network cycle
  guard/epsilon. Equal speed and same-network checks affect which path executes.
- base/KineticBlockEntity.java:98-143,191-195,307-352 distinguishes stored parent
  from own generation, adds5 flicker on zero/direction transitions, decrements
  flicker each BE tick, validates parent/load state, and changes network membership.
- base/GeneratingKineticBlockEntity.java:30-52 queues reActivateSource after a
  sourced generator loses its parent; its tick performs that activation AFTER
  super.tick. Its updateGeneratedRotation/applyNewSpeed at89-176 also has a
  DISTINCT control branch while a generator is driven by somebody else.
- RotationPropagator.java:309-379 follows only descendants whose source parent
  points to the removed entity, clears the dependent tree, collects possible
  replacement generators and tries a new source. It is not a fresh all-node max.
- KineticNetwork.java:133-175 sums capacity using each source's own GENERATED
  speed, whereas stress uses each member's theoretical speed. A driven A+32
  that generates+16 contributes16*16384, not32*16384. Capacity before reversal
  is(16+32)*16384=786432, not two driven+32 contributions.

These are persistent in-memory histories, NOT proof of disk persistence. They
must eventually be distinct state in a native event model; the current stateless
solve cannot supply them by adding another Source and selecting the largest.

The raw observations make the distinction concrete. In BOTH dominant-removal
runs at71, A and JOIN are already+16 but flicker9; at72 it is8 and by80 it is0.
In subordinate reversal at70 A is driven+32, own+16, parentJOIN and flicker0;
at71 it is air while JOIN/B retain+32 and flicker0. Thus two cases with the same
merged speed/capacity before the action follow different parent/control paths.
Do not infer the internal callback sequence merely from those sample boundaries.

## Smallest useful NEXT experiment: driven-generator control classification

Recommend a ONE-CASE native decision experiment before a stateful graph replay:
classify the subordinate reversal request under an explicitly authored DRIVEN
precondition. This is smaller and more defensible than implementing all six
fixture endings as a purported arbitration solver. It exercises a real missing
domain decision that a largest-source snapshot would get wrong, without claiming
to reproduce the unimplemented merge prefix or removal callbacks.

The precise source branch is GeneratingKineticBlockEntity.applyNewSpeed:
when hasSource(), abs(previous DRIVEN speed)>=abs(requested GENERATED speed),
and signs differ, call level.destroyBlock(CURRENT worldPosition,true) and return.
With the same sign it stays driven; a request greater than driven magnitude takes
a different detach/new-root/attach branch that this experiment must REFUSE.
updateGeneratedRotation uses the stored theoretical speed as prevSpeed here,
not the old own generated+16. Its prev==requested early skip also means an equal
same-sign request stays driven; do not manufacture a destruction in that case.

Proposed tiny dependency-free API, separate from solve:

- `DrivenInput { node:u32, source_parent:Option<u32>, own_generated:MotorSetting,
  driven_rpm:f32 }`, with node/parent identity explicit and normalized signed RPM.
- `classify_driven_request(&DrivenInput, MotorSetting) ->
  Result<DrivenDecision, DrivenBoundary>`.
- Decisions ONLY `StayDriven` and `DestroySelfWithDrops { node }`. This is a
  destruction REQUEST with drops enabled, not a world mutation or item ledger.
- Require a non-self parent, finite nonzero driven speed within+/-256, old own
  generation with compatible sign and magnitude<=driven magnitude, and a request
  whose magnitude<=driven magnitude. Missing parent, larger request/promotion,
  malformed/unstable precondition or other cases return a specific boundary
  without changing borrowed input. Existing nonzero MotorSetting validation
  remains. Zero-source removal, network merge and reactivation are not added.

The bounded parent and current speed are DECLARED history, not history computed
by the experiment and not a native initialization from captured fields. It does
not claim a general driven-generator API or arbitrary source-order closure.
Flicker need not become a dummy input: this chosen below-driven control branch
neither calls attach nor consults its propagation threshold. Do not add a clock,
counter decay, fake tick function or invented flicker qualification just to make
this classification look like a whole simulation.

Exact authored case:

- nodeA=0,parentJOIN=1; old own generated+16; driven theoretical+32; request-16.
- The public fixture prefix is A placement, B placement, ordered requests at10,
  JOIN insertion after30. Its settled precondition predicts B root, JOIN<-B,
  A<-JOIN. That PRECONDITION is assumed for this experiment, not reproduced.
- The native classifier must predict DestroySelfWithDrops(A). It MUST NOT simply
  retain A as a reverse generator below B's faster+32.
- No DOWN motor geometry is necessary: this operation acts on A's already
  normalized control inputs. B's settings/facing belong to the reference context;
  extending the geometry adapter merely to seed the trial would be unrelated work.

The native function is intentionally a pure decision projection, rather than a
half-implemented stateful graph. A subsequent declared-topology event experiment
would need parent-tree mutation, deferred activation and explicitly defined
scheduler/settling semantics. It should be a NEW reviewed packet once this guard
is qualified, not extra branches silently appended to solve.

## Reference projection and what it cannot establish

Use the full accepted strict loader to validate all800rows and pinned identities,
configs, dependencies and authored template for EACH original run. Select
arbitration_subordinate_reversal/070 as the pre-action witness and /080 as the
post-action witness: ONE conditional decision comparison per capture, not800
native traces, not a count of native machine-state comparisons.

The authored precondition above must be checked against the reference A record:
UP motor, own setting/generated+16, driven theoretical/effective+32, parentJOIN,
full ABC group, zero stress, capacity786432, empty full drop map. JOIN<-B and B
root/generated+32/setting-32 are contextual consistency checks, not native input
reconstruction. The native input always comes from the independent authored
constants/command, NEVER from copying those measured fields into an initializer.
A mismatch refuses that experiment rather than rewriting its assumptions.

After80, require A's explicit air-only shape and the complete live drop map
{'create:creative_motor':1}; validate surviving JOIN/B identities, unchanged+32,
parents JOIN<-B/rootB, group[JOIN,B], zero stress and capacity524288 as reference
CONTEXT. These observations support the source-informed inference that the
public control operation causes A's destruction with drops for this exact case.
They are not a native prediction of the surviving network or a native spawn of
one motor item. Native output only matches the conditional destruction-request
projection. Do not label it full subordinate-reversal conformance, drop/item
conservation, native parent/network recovery or a whole-fixture reproduction.

Rows71/160 were also inspected: destruction is already observable at71 and the
same survivor/drop context remains at160 in both runs. They constrain reference
history, but this classifier has no tick argument and cannot qualify those
transition times or reproduce additional native events by replaying it at each
sample. There is no direct Java log of the private branch; its association with
these public observations is explicitly a SOURCE-INFORMED inference.

## Concrete minimal file/test surface for a later packet

Five PRIVATE files, no dependency/engine/Java/geometry expansion:

1. NEW `crates/create-core/src/driven_generator_trial.rs` for the bounded typed
   decision API and specific boundary errors.
2. `crates/create-core/src/lib.rs`: module export ONLY; solve, its errors,
   MotorSetting and transmission arithmetic remain byte-for-byte unchanged.
3. NEW `crates/create-core/tests/driven_generator_trial.rs`: literal decision,
   malformed/precondition and source-derived boundary checks.
4. NEW `crates/create-core/examples/driven_generator_probe.rs`: one strict
   development JSON record/result form for this decision, using the already
   locked serde_json dev dependency. No existing shared bridge/guest/host edits.
5. NEW `tools/compare_driven_generator.py`: dedicated fixed-reference projection
   using existing strict load_run/decode/finite helpers; explicit canonical output
   and source/reference/artifact receipts collected by the coordinator. No new
   generic comparator/runner framework. Existing probe runner remains untouched.

A fresh DIFFERENT independent design review of an implementation packet must
precede this work; this audit is not that acceptance. Freeze and independently
review the source before any candidate execution. Native tests should assert
A32/own16/request-16 =>destroy; same-sign16 and equal32=>stay; opposite-32 at
equal magnitude=>destroy; greater magnitude=>unsupported; absent/self parent,
zero/nonfinite/out-of-range driven speed and incompatible old own facts=>specific
refusal. Repeated calls must leave borrowed input unchanged. Counterfactual tests
are source-derived SOFTWARE checks, not new measured Java cases.

Run the dedicated actual native example against BOTH retained complete captures
with ONE authored decision each. Require a useful changed-request(-16->+16)
control to differ with exit1; malformed numeric/ID/parent/record/output controls
must refuse with exit2, with no partial result. A valid changed reported decision
must differ, not be normalized away. Collect exact commands/exits, source/test/
compiler/lock/ref/template/native-artifact hashes and before/after drift checks
outside Git. No claimed Wasm execution follows from native execution alone.
Retain native core/geometry tests, lints/fmt/Wasm compile checks and the already
qualified native/actual-sandbox horizontal, vertical and legacy comparisons as
regressions. Final independent evidence review precedes a separate result note.

## Precise dependencies before broader arbitration

A full fresh-world native replay still needs an explicitly reviewed causal
model for propagation actor/neighbor order, parent/network reassignment and
cycle identity; source reactivation on the BE tick; flicker increment/decay and
validation scheduling; current-actor destruction and removal callbacks; and an
honest pure-world effect/drop adapter. Existing source/captures do not supply a
ready-made public host contract for any of those. The available six cases also
do not prove arbitrary ordering, cycles, unloaded contributions or all configs.
These are model/coverage dependencies, not a reason to launch a Java capture or
engine implementation automatically. No precise callback trace is invented from
matching end states. Stop on an unexplained precondition/projection mismatch,
invalid receipt, source drift or extra scope need; return to a bounded design.

Inspected pinned source SHA256:
RotationPropagator.java e91bca88411201202b6a196d9f07cff6307c5c3370ac9d96d7c4de1e36363514;
GeneratingKineticBlockEntity.java c20321691ca54ad46eb8949579972145d73fa926b40fd3b1a0e1e87819bfbdc2;
KineticNetwork.java 716c565a0fc412c2cd8fee61fd1beaacd2aca160ab0932cbde0f52418b3bbde0.
