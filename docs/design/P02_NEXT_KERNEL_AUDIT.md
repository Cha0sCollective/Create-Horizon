# P02 next kernel audit

Accepted bounded reference-experiment design after fresh independent read-only Sol High review with no actionable findings. Reviewed draft SHA256: 49748f0c22c0533051e2671986f58e8441728d9ee8440a783d4548328923a6cb. No reference execution or outcome is yet accepted. Observe this four-case horizontal single-source geometry packet before adding a geometry adapter; retain all current solver boundaries. This proceeds from G0 independently of H08 participant preflight. Live S2/P03 integration still requires G1 (PRODUCT_SPECIFICATION.md:348-362, 455-458).

## Verified baseline and limits

Private repository was clean on `codex/h02-smoke-h03-notes` at `56269680c60595c28c5788465193ddfca34248aa` when inspected. Create was clean at `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`, matching workspace.toml. This remains private, source-informed personal port work; preserve the snapshot and attribution. No engine/source/adapter edits, execution, builds, network activity or tests occurred in this audit.

P02_RESULTS.md accepts declared-topology, settled single-source algebra: 16 checkpoints / 62 observations from each E/F reference, with meaningful wrong-ratio control. P02_WASM_RESULTS.md accepts the same selected checkpoints in the actual sandbox. Neither establishes discovery of physical connections, transition timing or live-world ownership.

P02_ARBITRATION_RESULTS.md accepts two fresh matching six-case / 800-sample captures (`bf96ed53-99ee-43e1-8365-8b90c8103044`, `0e8d3404-3f21-44f3-9345-c453697a71f7`). They establish their precise Y layouts, order, settings and scheduled callbacks: unequal driven/generated speed, removal/reactivation, conflict destruction and one overspeed outcome. They do not establish source-order independence, alternate axes, arbitrary cycles, unloaded contributions or a generic arbitration algorithm. The original ARBITRATION_PACKET proposal is superseded as status by that result note.

Actual core seams:

- `crates/create-core/src/lib.rs:48-64`: four explicit transmission ratios; geometry is caller-owned. Node/Source (68-79) hold no position, axis, parent, event or network identity.
- `solve` (123-256) recomputes from complete inputs. Multiple sources (188-190), conflicting cycles (200-203) and speeds above 256 (139-140, 225-226) remain explicit unsupported errors. It cannot reproduce destroyed-block selection or reactivation timing without a new, separately justified state model.
- `tests/kinetics.rs:61-85,192-220,257-271` test algebra/reciprocals and unsupported boundaries. Extending those with six observed survivors would encode layouts rather than establish a general rule.

No identified defect or missing measured single-source arithmetic rule justifies changing the pure solver now. Overspeed alone also needs world destruction/event semantics that this stateless API does not own. More arbitration ordering cases would be useful later, but still would not define the general policy. Geometry observations supply a smaller independent missing input contract without creating that state machine.

## Smallest candidate: horizontal large/small connection observations

Proposed fixture set `p02-horizontal-1`: exactly four fresh-world cases, 60 scheduled samples each (240/run). These are authored source-informed predictions to verify, not accepted outcomes. Use the existing Java GameTest observer/runner/strict loader; do not add a framework or touch create-core. Explicitly fix GameTest rotation to NONE and record/check actual world block properties so helper-coordinate rotation cannot masquerade as axis evidence.

All coordinates below are relative to the empty 16x8x16 template. Place in order: stone floor at y=0, motor M, shaft S, large cog L, small cog C. All passive blocks have the listed main axis except the one stated control. No other kinetic blocks or items.

| Cases | M/facing | S | L | C | Main axis / C axis |
| --- | --- | --- | --- | --- | --- |
| X connected / X mismatch | (2,3,3) EAST | (3,3,3) | (4,3,3) | (4,4,4) | X / X connected; Z mismatch |
| Z connected / Z mismatch | (3,3,4) NORTH | (3,3,3) | (3,3,2) | (4,4,2) | Z / Z connected; X mismatch |

One callback per tick: sample first, then action/assertions. At tick 10 request generated +16 RPM through real `generatedSpeed.setValue`: setting +16 for EAST, -16 for NORTH. At tick 20 assert settings, generated RPM and the first settled expectations below. At tick 30, after sampling, request generated -16 (settings -16 EAST / +16 NORTH). Assert reversed expectations at ticks 50 and 60; succeed after the final sample. Capture every intervening row without claiming an exact native transition tick. Default startup observations remain observations; do not normalize them away.

Independent settled expectations, derived from the source seams below:

| Window | M/S/L theoretical and effective RPM | C connected | C mismatch |
| --- | --- | --- | --- |
| Tick 20 | +16 each | -32 | 0 |
| Ticks 50,60 | -16 each | +32 | 0 |

M generates the requested signed RPM; passive blocks generate zero. Connected M/S/L/C share a network, zero stress and 262,144 SU capacity (captured motor fact 16,384 SU/RPM). In the mismatch cases M/S/L share that network; C remains unpowered with no source parent/network and zero stress/capacity. Assert all four exact block identities/axes/facing and an empty full live-drop map throughout the settled checks. Retain unexpected air/destruction explicitly and fail, rather than silently omitting a node. Review expected parent chains against the source before freezing assertions; collect and compare actual parents and normalized network equivalence in every row.

### Source seams for those predictions

Paths below are under unchanged `references/create/src/main/java/com/simibubi/create/content/kinetics/`:

- `motor/CreativeMotorBlock.java:43-51`: only the facing side has a shaft; axis follows facing. `motor/CreativeMotorBlockEntity.java:74-77` and `base/KineticBlockEntity.java:506-508`: signed setting is negated on negative-axis directions, including NORTH.
- `simpleRelays/AbstractShaftBlock.java:41-48`: shaft/cog shaft faces follow AXIS; `CogWheelBlock.java:37` inherits that behavior. `CogWheelBlock.java:130-143` provides the placement-validity check to verify for both mismatch layouts rather than assuming a forced state represents a legal connection.
- `RotationPropagator.java:54-81`: aligned mutual shaft faces convey +1. Lines 100-105 and 172-186: large/small requires equal rotation axes, zero displacement on that axis and absolute displacement one on each perpendicular axis; it conveys -2 from large to small. Mismatch therefore supplies no edge.
- `RotationPropagator.java:382-444`, `base/KineticBlockEntity.java:556-572`, `simpleRelays/SimpleKineticBlockEntity.java:25` onward: actual neighbor enumeration includes cog diagonals. Test real placement/propagation, not a direct private modifier call.

This packet intentionally does not establish all signed directions, every diagonal quadrant, equal cogs, small-to-large drive or perpendicular large-to-large connections. Those remain separate feature-matrix rows; four positive/negative-axis and connection/mismatch cases are a bounded next step, not arbitrary-axis closure.

## Implementation/evidence boundary for a later packet

Fresh read-only independent Sol High design review accepts this bounded code scope. Private edit surface: a new geometry Java fixture class, narrowly reused schema-3 extraction in PortReferenceTests.java, and explicit new-set dispatch/identity/layout/axis rules in tools/reference.py, tools/reference/init.gradle and tools/compare_reference.py. Current code hardcodes arbitration prefix/schema selection (PortReferenceTests.java:71-94), namespace/class selection (reference.py:42,72-85; init.gradle:15-18) and Y/up/down validation (compare_reference.py:145-215); adding Java cases alone would produce invalid evidence. Keep exact per-set adapter file lists, template hashes and legacy schema contracts. Do not broaden old validators to allow arbitrary axes or unknown fixtures.

Verify predictions and extraction first; a contradiction requires an explained, reviewed correction, not replacing expected values with captured speeds. Freeze assertions/adapter, then obtain two complete matching successful fresh-world captures, pinned source/JDK/dependency/configuration/template receipts and independent evidence review. Require strict rejection of wrong axis/facing, missing/duplicate rows, unknown parents, malformed/nonfinite numeric fields and changed template bytes. A legal-shaped changed RPM must produce a useful difference. Continue to load both existing accepted corpora and the arbitration pair unchanged. Raw worlds/binaries/configurations remain outside Git.

Stop on unexplained nondeterminism, invalid placement, incomplete receipt or unreviewed source changes. No new native conformance claim follows from Java captures alone. Only after acceptance should a separate packet propose a tiny explicit geometry-to-edge function and compare it, plus the unchanged core, against independently authored fixture layouts. No ECS, host API, protocol, persistence, performance or G1/G2 claim belongs here.

Core files inspected, SHA-256 (no execution):

- src/lib.rs: `459238df6021a378aa1e2a617f6343017cf56d6a7eb3e77c3b012688ba4979b3`
- tests/kinetics.rs: `e654c717d2a7d69f1b3baac6a72669eb99a725ef8d75a9c2d856a4413f803b3f`
- examples/kinetic_probe.rs: `198b77732b3248bcb90efc42710d1624b80c160d8b84806202c3177b5c9faf5a`
