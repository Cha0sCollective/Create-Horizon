# Selected goggles solver-speed producer — source candidate

Owner approved the one persisted private field in the concrete primary packet
`docs/design/GOGGLES_SOLVER_SPEED_FIELD_PACKET.md` at
`781ceb56b193eb56657fe96f78f3b6ac33e2538b`. This producer implements
only that packet's small `create:water_wheel` and `create:millstone` server
field. It does not qualify the Experience HUD or its gameplay.

## Exact candidate

Isolated author `/root/goggles_producer_author` worked in
`.local/workers/machinery-goggles-producer`, branch
`codex/worker-machinery-goggles-producer`, directly from accepted source
`55339ce3347b2ce667281e8a3e6318ec856144e4`. Its clean commit is
`8447f6d4bc085963e7d0b9bbdd50d23b3a23bc37`. Machinery cherry-picked
the identical patch into its lane as
`b74e7697f38542321627bdde0f48b34a36773af9`; stable patch ID
`a4c22377b6adbb050f0e04b7a8b6e068c26494a7` matches both commits.
The only changed file is `native/create/mixins/create/runtime.rs`.

On an authoritative selected machine tick, the new helper publishes
`create_horizon:solver_theoretical_speed` from the existing solver's per-node
map. A finite signed value is a Float for Running/Overstressed; Idle is Float
zero; missing/nonfinite or unsupported status is explicit Byte zero. It
compares the previous raw optional Tag with the new value, so an invalidation
or theoretical change syncs even while effective `Speed` stays zero. The
value remains derived presentation state, not a solver, source, propagation
or processing input. The five-type `create_horizon:kinetic` diagnostic,
destructive eligibility, core solver, large wheel, other providers and host
API are untouched.

Two focused tests in the same file use actual solver results for signed speed,
overstress with zero effective RPM, changed theoretical speed at unchanged
effective RPM, Idle, conflict and missing/nonfinite map entries. They inspect
valid-to-Byte-to-valid payloads through the existing `update_tag` producer.
They do not run the pinned host client merge. `git diff --check` passed;
Rustfmt, Cargo tests, compilation and game/control processes were not run.

Independent read-only Sol High `/root/goggles_producer_review` returned
**CLEAR for source** on exact `55339ce` → `8447f6d` at
02:08:40–02:10:38 UTC (118 s). It checked the pinned host's NBT persistence
and key-replacement merge source separately: [save.rs:617–633](https://github.com/Cha0sCollective/Horizon/blob/9c7f9b2215dfed497aba5522c44f38926628648d/crates/horizon-client/src/save.rs#L617-L633)
and [block_entity.rs:203–213](https://github.com/Cha0sCollective/Horizon/blob/9c7f9b2215dfed497aba5522c44f38926628648d/crates/horizon-client-render/src/chunk/block_entity.rs#L203-L213).
The author worked
02:02:08–02:07:53 UTC (345 s). Neither used the shared heavy queue;
verified token counts remain unknown.

## Root/Experience boundary

The held Experience candidate `d2bc7365b2eef4f657b0063cb0d2d5b38591f30d`
still reads `create_horizon:kinetic.TheoreticalSpeed`. Experience owns the
matching `presentation.rs` repair to read the distinct solver field and to
reject Byte/missing/nonfinite/unsupported values. Root owns composition,
shared build/runtime lease, both-mode server/client observations, cold reopen
and acceptance. This candidate must not be called a compiled or gameplay pass.
Drill003 stays prepared and unaffected.
