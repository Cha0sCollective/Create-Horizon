# FLUIDS-001 — isolated plain-water fluid tank (proposal)

Status: proposal only, November 5, 2026. No production source, host pin, save
format, or acceptance change is allocated by this document. Candidate base is
accepted water `02d029f3bea09c62480e07e315e23d4868f9180a` (native `.8`);
this team's proposal lane starts at `76dfaf78370f41b2a5c68620c55be6ecab96f16f`.
Keep Horizon `9c7f9b2215dfed497aba5522c44f38926628648d` and Mixins
`e8cd0aa38cffef49cb2e088ef716baa107148c02` pinned. The imported port is
`16c5d454b1b9860da01b78474ea5206f0664d384`; selected native Basin reuse
is from `236c87a889cd76f7e663d1d182c28dc1145f7b14`.

## Reference boundary

The pinned Java reference is Create 6.0.11 at
`7ff2760e4924d35d4bc2e22c4b19923f0178cd16` (Minecraft 1.21.1,
NeoForge 21.1.219). These are source observations, not a fresh Java run:

- [FluidTankBlockEntity.java, constructor and capability](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/fluids/tank/FluidTankBlockEntity.java#L69-L102) starts a tank as one controller block with one fluid inventory. Its [capacity multiplier](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/fluids/tank/FluidTankBlockEntity.java#L525-L536) is configured buckets times 1,000 mB; [CFluids.java](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/infrastructure/config/CFluids.java#L9) defaults to eight buckets. The first packet therefore fixes one isolated block at **8,000 mB** under the pinned default, without adopting configurable or multiblock capacity.
- The controller [reads](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/fluids/tank/FluidTankBlockEntity.java#L415-L437) and [writes](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/fluids/tank/FluidTankBlockEntity.java#L480-L500) `Window`, `TankContent`, `Size`, and `Height`. A non-controller has a `Controller` reference. The native candidate needs a tank-specific, source-backed mapping; Basin's two-half `SmartFluidTankBehaviour` codec is a different format.
- [FluidTankBlock.java](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/fluids/tank/FluidTankBlock.java#L155-L183) passes ordinary tank bucket clicks through in Survival. Direct item exchange runs only for a Creative player or a creative tank. No Survival regular-tank bucket-click claim belongs in this packet. Any Creative exchange check must retain the Java-observed item and fluid result rather than assume Survival bucket custody.
- The Java tank [forms multiblocks](https://github.com/Creators-of-Create/Create/blob/7ff2760e4924d35d4bc2e22c4b19923f0178cd16/src/main/java/com/simibubi/create/content/fluids/tank/FluidTankBlockEntity.java#L97-L110); adjacent tank joining is excluded. The local 6.0.10 source commit `ac0c444d9828da3453ae8cc65338e8de063286fb` and pinned 6.0.11 have no diff in `FluidTankBlock.java`, `FluidTankBlockEntity.java`, or `FluidTankItem.java`. This does not establish general 6.0.10/6.0.11 parity.

The accepted port and the available native upstream snapshot contain Basin fluid
hooks and a Basin codec, but no tank, pipe, or pump implementation. Existing hand
crank and water-wheel sources are already implemented; FLUIDS-001 adds no rotation
source. Source provenance must identify new local tank code as authored adaptation,
not an exact native upstream import.

## Proposed capability contract

One stationary `create:fluid_tank` exposes one tank on the existing native custom
fluid API, with 0–8,000 mB of plain `minecraft:water`. The view reports actual
content and capacity. `Simulate` predicts the same bounded amount as `Execute`
without mutation. `Execute` fills or drains only after the complete current block
entity state is preflighted and the changed NBT is committed; sync follows the
commit. An unsuccessful commit reports no transfer. Exact drain must match plain
water; an `Any` drain must still return only plain water. Reject lava, component
payloads, zero/negative or excess amounts, mixed/opaque content, invalid geometry,
and malformed or foreign state without changing fluid or item custody. Preserve
unrelated supported tank fields when updating `TankContent`; do not flatten
unknown state to an empty tank.

No autonomous or adjacency transfer is proposed. A regular tank can receive or
give fluid through its native capability when a later scoped component uses it;
FLUIDS-001 alone is a bounded tank foundation, not a usable Survival plumbing
network. Creative-player bucket interaction may be included only with source-backed
stack and fluid checks. Pipe, pump, hose, portable fluid interface, Basin transfer
automation, multiblock connectivity, boiler/steam behavior, lava, arbitrary fluid
components, and world-fluid handling are excluded. Existing Basin water buckets,
codec, save fields, and dough recipe stay unchanged.

## Proposed file ownership

Tank-specific production files, if allocated, are new
`native/create/mixins/create/fluid_tank.rs` (adapter/capability) and
`native/create/mixins/create/fluid_tank_codec.rs` (strict tank-only NBT mapping),
plus the minimum `create:fluid_tank` block, item, model, loot, language, and catalog
resources under `native/create/content/` and `native/create/mixins/create/assets/`.
The four existing shared dispatcher helpers are exactly:

1. `native/create/mixins/create/fluid-custom_block_fluids.rs`
2. `native/create/mixins/create/fluid-custom_fluid_tanks.rs`
3. `native/create/mixins/create/fluid-custom_fluid_fill.rs`
4. `native/create/mixins/create/fluid-custom_fluid_drain.rs`

They currently call Basin only; their injection declarations are in
`native/create/mixins/create/horizon-mixin.toml` at lines 865–917. The project
lead must assign one integration owner for these four helpers, the manifest, and
any shared placement/click/removal/asset wiring. A tank worker can own only the
new tank-specific adapter, codec, and resources in an isolated worker worktree.
Do not modify `native/create/mixins/create/fluid_codec.rs`, host code, active pins,
or another team's domain as part of this proposal.

## Allocation gate and minimal exit checks

Before source work, the project lead must decide whether the new tank-specific
`TankContent` representation fits existing generic `ModBlockEntity` NBT services
or crosses the owner-gated schema boundary. Establish the exact supported
`TankContent` encoding and treatment of the other Java controller fields first.
If host/global schema, Basin codec, broad network, or acceptance boundaries must
change, stop for the existing owner gate.

After a bounded production allocation: focused pure tests for empty/partial/full
fill and drain, simulate/execute agreement, exact and any-drain amounts, malformed
state/component/lava/overflow refusal, and failed-commit nonmutation; then one
isolated placement and cold save/reopen check, a Creative-player bucket exchange
check against Java source behavior, an actual rendered water/empty view, and one
affected Basin water regression. Check filled-block removal behavior against the
pinned reference before claiming it. Reuse unchanged prior evidence. A fresh
independent read-only Sol High review covers substantive source and effects.
Cargo, staging/linking, Java reference runs, and game/server checks require the
project lead's host-wide resource reservation. No such checks ran for this proposal.
