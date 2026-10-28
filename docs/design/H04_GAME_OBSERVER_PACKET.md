# H04b authoritative game observer fixture

Follow H04a's reviewed passive resident observer with a bounded, explicit control
fixture. This validates the seam in actual games before machine lifecycle uses it.
It adds no automatic observer or guest capability and is not full H04.

With paired-packages enabled, a local control command may enable observation of
all registry states of one exact known block name, with a checked capacity in the
existing observer bounds. Status, drain and explicit teardown operate on the same
World-owned observer. Existing integrated-server calls route the operation to the
server in the owner's level; external remote worlds refuse. A separate read-only
local status deliberately reports the window's own World to prove its replica is
disabled. Default and feature-off behavior remain unchanged.

The host allocates a checked transient instance ID, captures the storage level's
dimension at enable, and preserves it inside events. Drain serializes canonical
state names/properties from the frozen registry, positions, actual world time,
sequence, captured scope and explicit unclassified cause. Invalid IDs/faults must
produce errors, not fallback air or partial successful batches. No persistence,
placement generations, machine inventories, callback delivery or networking API
is established by this diagnostic fixture.

Extend the existing isolated game smoke driver to assert default disabled,
authoritative enable versus replica disabled, exact command/player transitions,
multiple same-position replacements without coalescing, no-op exclusion, ordered
sequence and sticky overflow without rejecting the corresponding world writes.
Verify observer history does not persist across restart. Exercise both integrated
and in-frame startup with frozen source/executable receipts. Piston, deferred
guest writes and actual dimension travel require separate concrete probes before
claiming those paths qualified; pure World swap tests alone do not prove them.

Keep the implementation small and generic in Horizon. The coordinator owns
integration and serial builds, with a fresh read-only independent Sol High review
of source and driver followed by actual receipt inspection. No public publication
or broad control framework is needed.
