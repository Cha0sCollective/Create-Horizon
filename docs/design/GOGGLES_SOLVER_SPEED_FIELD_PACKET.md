# Goggles selected solver-speed publication

Status: concrete schema proposal for owner decision; no production edit. Based
on accepted native55339ce and Experience source candidate d2bc736 (HELD). The
Machinery read-only audit found the existing mixed-component solver already
retains signed per-node `Network::theoretical` before overstress makes effective
`Speed` zero. The separate five-type propagation diagnostic supplies
`create_horizon:kinetic.TheoreticalSpeed` only for wholly eligible S2 components;
it cannot serve the selected small water wheel or wheel-driven millstone HUD.

## Bounded change

Add one private derived presentation field, `create_horizon:solver_theoretical_speed`,
to the existing block-entity NBT/update tag for `create:millstone` and the small
`create:water_wheel`. The producer in `native/create/mixins/create/runtime.rs`
already obtains the authoritative existing `Network` for those nodes. Use its
signed finite Float per-node theoretical value for `Running` and `Overstressed`,
and Float zero for `Idle`. If a selected map entry is absent/non-finite, or the
status is unsupported (conflict, inconsistent cycle, excessive speed or size),
publish an explicit `Tag::Byte(0)` unavailable tombstone under the same key.
Clients accept only finite Floats with an allowed status; Byte zero is never a
speed. Do not omit the key on invalidation: the host merges update tags by
inserting/replacing keys, so omission could retain an old valid client value.

Compare the previous optional field with the recomputed value when deciding
whether to sync (including Float-to-Byte invalidation), so theoretical-speed changes during overstress still reach
clients even when effective Speed remains zero. Recompute from the solver on
every authoritative machine tick. A saved field is derived presentation data,
never input to network simulation, processing, source choice or propagation.
Existing worlds without the field are valid; the first authoritative tick
supplies it. Clients require the field and an allowed `NetworkStatus`; missing,
malformed, non-finite or unsupported values leave the selected provider
unavailable. Do not fall back to effective Speed for overstressed calculations.

`ModBlockEntity` saves its NBT, so this is explicitly a new persisted private
field, despite being derived. Existing host update-tag and client merge paths
carry it; no new packet, native host service, serializer, general diagnostics
framework or owner is proposed. The existing propagation diagnostic and
destructive-propagation eligibility retain their current meaning. The accepted
S5 piston owner and the separate contraption foundation are outside this packet.

## Files, checks and acceptance

Operations allocates the selected producer delta in `runtime.rs` and coordinates
Experience's matching `presentation.rs` repair from held d2bc736. Keep the small
wheel/millstone selected formulas, spectator/shaft guards, existing Speedometer
trace and localized labels; repair the doubled label colon. No large-wheel or
other provider is added. Update source provenance and finite coverage only.

Use meaningful focused checks for signed speed, zero/Idle, overstress with
unchanged effective RPM, unavailable statuses and missing/malformed client
fields, and valid-to-tombstone-to-valid client invalidation and recovery. Verify the actual producer
and update-tag path, not only tests that inject a convenient field. Review the exact stable source with independent
read-only Sol High, then use the shared compile/runtime lease. Reuse unrelated
Funnel/Saw/Tank/audio evidence. On actual integrated and dedicated clients,
observe millstone impact and small-wheel capacity against authoritative values,
including zero/overstress, Goggles off, F1, spectator, shaft and retained gauge
behavior. Cold-reopen the selected producer/HUD state in both modes and observe
the recomputed server field and actual client overlay. The existing supplied-sheet craft/equip checklist remains a separate
observation; none of this proves full progression or full Create tooltip parity.

## Decision boundary and source receipts

The owner must approve this one persisted-field addition before implementation:
`AGENTS.md` says, "Ask before changes beyond authorized stage outcomes or the
adopted architecture, schema, framework and acceptance boundaries." The current
HUD allocation required a source-supported producer and held when it was absent.
The owner-approved contraption foundation concerns a different record/lifecycle.

Audit inputs: native55339ce, exact host9c7f9b2215dfed497aba5522c44f38926628648d,
Java7ff2760e4924d35d4bc2e22c4b19923f0178cd16; Machinery audit at lane
9fef22a8c44047903d8eb25efceb3f07b0e9e073. Read
`native/create/mixins/create/kinetic_runtime.rs` (`supported`, `diagnostics`,
`network`), `engine/src/core/kinetics.rs` (`Network::theoretical`, effective speed),
`runtime.rs` (network calculation, NBT publication and sync), and
`docs/teams/experience/EXPERIENCE_003_HELD_HANDOFF.md` in the Experience lane.
The root owns the final publication contract and private checkpoint.
