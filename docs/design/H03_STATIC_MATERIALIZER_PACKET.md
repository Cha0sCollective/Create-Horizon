# H03b: static diagnostic state materialization

Owner continuation authorization covers this next bounded content prerequisite.
The reviewed H03a plan is local commit `b1ced4fcd26fe9ecae5403b6968500342162ca1c`.
Its parser alone does not admit custom block states. Materialize a small safe
static diagnostic subset now; do not claim full H03 or G1.

Use only paired immutable captured content, the exact `minecraft:iron_block`
single-state template, declared properties/defaults from StatePlan, and identical
inherited geometry/physics for every tuple. Reject `waterlogged` because the
engine derives fluid behavior from it even for unknown block names. Feature-off
and legacy declarations retain existing behavior. Preserve raw JSON validation;
do not silently overwrite duplicate descriptor fields through an intermediate map.

Give NewBlock a source-ID/output-ordinal distinction. Duplicate the approved
source row for every canonical tuple; assign explicit defaults and self references
per output ordinal. Update facts identities/descriptions, gameplay numeric IDs,
model-state schema/tuples, render state lists/default, command property schema,
per-state flags, structure state counts and identity rotations/mirrors. Every NBT
state identity list, including nested occlusion, needs matching tuples. Cached
shape arrays repeat the source index and reuse existing geometry. Block/item
registry ownership remains once per block. Vanilla IDs/defaults must not move.

Initial evidence: six tuples with nonfirst default index 3; all corresponding
JSON/NBT tables and actual loaders agree, model binding resolves every tuple,
save round trips preserve properties, and static transforms preserve each tuple.
Reject multistate templates, single-state block entities, unsafe properties,
legacy unpinned admission and unsupported versions. Run focused feature-on/off
checks and existing real-resource content regressions with an explicitly available
owned resource root; a missing-resource early return is not evidence.

Coordinator owns integration and actual game checks in both startup modes. One
bounded isolated worker owns parser/materializer/tests; no worker stages,
commits or publishes. Fresh independent Sol High review must inspect the exact
candidate and any consequential fixes. Custom shape overrides, orientation-aware
placement/rotation, machine bindings/lifecycle/transactions/persistence and retained
animation remain later work. The `static` preview deliberately preserves tuples
under rotations; it does not represent Create axis semantics.
