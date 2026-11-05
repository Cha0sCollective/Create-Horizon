# FLUIDS-001 candidate handoff — isolated water tank

Status: private local candidate, **source-reviewed CLEAR; runtime unvalidated**.
Team lane `codex/team-fluids-power`, implementation base
`7c03ca04f1011e26e5638718ee614c54a65b0be0`, implementation commit
`01bec38c3d607253f56797ca9e53ed5f1b437c9b`. The accepted central water
baseline remains `02d029f3bea09c62480e07e315e23d4868f9180a`/native `.8`.
This lane has no host, public engine, workspace pin, or Basin-codec edit.

## Exact changed source and contract

The source diff is `7c03ca04..01bec38c`; it contains 27 files. New
`native/create/mixins/create/fluid_tank_codec.rs` encodes one isolated controller
with `Window`, `Size=1`, `Height=1`, and explicit `TankContent` (empty compound or
plain-water `Fluid` up to 8,000 mB). New `fluid_tank.rs` exposes one native fluid
view and clipped Simulate/Execute fill/drain, server-authoritative Creative
water/empty-bucket clicks, window wrench, and bounded water rendering. Unsupported
components, lava, unknown/connected/boiler state and malformed NBT are refused
without interpreting them as empty. Survival regular-tank bucket clicks pass
through. Filled normal/sneak removal follows the Java source-inferred plain-block
drop and fluid-loss behavior; it still needs a focused runtime observation before
acceptance.

The four existing `fluid-custom_block_fluids.rs`,
`fluid-custom_fluid_tanks.rs`, `fluid-custom_fluid_fill.rs`, and
`fluid-custom_fluid_drain.rs` dispatch to Basin first and then the new tank.
`horizon-mixin.toml` adds the two tank modules. `runtime.rs`,
`placement_runtime.rs`, `burner_prediction.rs`, and `destroy.rs` add only tank
ownership/initial NBT, non-kinetic placement exclusion, per-hand Creative click
routing and malformed-state removal guard. `tools/catalog.py`,
`tools/content.py`, bundled `data/content.json`, the authored fallback tank
resources, and `assets/src/catalog.rs` add only the tank. The local resource
import cache schema changes from 11 to 12 because the catalog adds a block.

The pinned host `9c7f9b22` generic `ModBlockEntity` save/restore/sync path was
read before edits: `save.rs` 617–633/871–907 and `native_hooks.rs`
8615–8653/9407–9479 carry this per-block NBT without a host/global schema
change. Client update merge at `horizon-client-render/src/chunk/block_entity.rs`
198–205 requires the candidate's explicit empty `TankContent`. No claim of
actual cold-reopen or peer-render success follows from this static proof.

Pinned Java Create 6.0.11/`7ff2760e` supplies the one-controller capacity,
bucket permission, window wrench, and save-field reference. The imported port
and local art remain pinned separately to Create 6.0.10; new tank logic is
source-informed authored code, not an upstream native tank import. The official
NeoForge 1.21.1 `FluidTank` branch supports clipped fill/drain, but that source
file was not verified against exact NeoForge 21.1.219. See
`FLUIDS_001_PROPOSAL.md` for precise Java links and scope limits.

## Executed checks and review

- Windows Git confirmed exact clean base/head and the 27-file diff; staged
  whitespace check passed. The local deterministic `tools/content.py` generator
  ran only in this team worktree, without Cargo, game, Java or shared cache.
- Seven new tank resource JSON files parse and match their exact bundled
  `data/content.json` entries. Pinned `rustfmt 1.10.0` formatted only the two
  new tank Rust files. It parsed them but did not type-check the candidate.
- Independent read-only Sol High review of exact
  `7c03ca04..01bec38c` returned **CLEAR**, with no actionable source finding.
  It verified host NBT premise, Java transfer/click semantics, dispatch and
  catalog wiring. Wall interval 21:58:57–22:01:56 UTC (179 s), no reviewer writes.

No Cargo/staging/linking, focused Rust test, Java reference run, game/server run,
cold reopen, actual client image, or Basin runtime regression has been executed
for this candidate. Static CLEAR is not gameplay acceptance.

## Requested shared validation and integration limits

After the project lead grants the host-wide heavy reservation, compose this
source with other ready packets and run one shared stage/lint/build. Focused test
filters: `create_port_fluid_tank_codec::tests` and the existing
`create_port_basin::water_tests::canonical_water_roundtrip_and_strict_capability_boundary`.
Add an actual server capability check for partial-room fill, partial-content
drain, simulation nonmutation, component/lava/malformed refusal and failed-commit
results; those adapter paths do not have a new standalone host fixture yet.

The later frozen combined-binary milestone should check one isolated tank in
integrated and dedicated/two-rendered-client modes: ordinary placement,
Creative main/offhand bucket fill and drain with unchanged held stacks,
full/insufficient bucket refusal, Survival regular-tank click pass-through,
window toggle, normal cold water reopen, actual peer water-to-empty view and
source-backed filled removal, plus one accepted Basin water regression. Decode
saved owners and distinguish authoritative queries from rendered observations.
No multiblock, pump, pipe, adjacent transfer, other fluid/component, boiler,
Java/world import or broad network claim is proposed.

Integration may conflict with another team's edits to `horizon-mixin.toml`,
`runtime.rs`, `tools/content.py`, `data/content.json`, catalog/cache schema and
package version stamps. The candidate intentionally retains the `.8` package
stamp; the project lead owns serial version and shared-wiring composition on
its separate `.9` integration branch. Do not rebase this lane or treat the
review CLEAR as an integration/build pass. Root owns remote sync.
