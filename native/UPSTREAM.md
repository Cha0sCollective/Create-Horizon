# Native Create source provenance

The owner adopted this native base on November 4, 2026. `create/` imports the 500
tracked files of [Delete228/create-horizon](https://github.com/Delete228/create-horizon)
at `16c5d454b1b9860da01b78474ea5206f0664d384`, Git tree
`7ce954394753d1a6c490157660ec6eb7a6c0cb8e`. The initial import was byte-for-byte
from Git archive; no untracked files, Git metadata, builds or local resource inputs
were copied. Local modifications are subsequent private Git commits.

Original MIT license, copyright headers and `create/THIRD_PARTY_NOTICES.md` remain
intact. Upstream attributes the original Create team and the Rust port contributors.
This is source-informed reuse; no clean-room or full Java compatibility is claimed.

The imported package is version 0.5.0, originally referencing Create 6.0.10
`ac0c444d9828da3453ae8cc65338e8de063286fb`. Its compatible host is official Horizon
`fc02103818c212920fd24aa34a2b2f91ba49e56f` and Mixins
`a1e13326306c6bc074b11e6a4926deb551c4083e`, native API359, Rust1.99.1, Minecraft26.3.
Our selected behavior reference remains Create6.0.11/NeoForge21.1.219; bounded local
differences are tracked explicitly, without claiming the whole package was upgraded.

The package includes more content than our supported five-block target. Keeping
those files intact does not qualify or assign development of additional machinery.
Original art is imported only at runtime from the privately held pinned6.0.10 JAR;
that JAR, imported art, Mojang resources, worlds, caches and binaries are not in Git.
Source-authored provisional resources already tracked upstream remain part of its
source package.

`create/Cargo.toml` is a separate workspace. The repository-root Cargo workspace
and old host tools are preserved historical work. The active scope, reference pins
and acceptance are in `../docs/design/NATIVE_PLAYABILITY_PLAN.md` (relative to the
repository root: `docs/design/NATIVE_PLAYABILITY_PLAN.md`). No public engine import,
upstream contribution, merge or release is part of this adoption.

## Selected S6 Basin/Mixer reuse

Private dry-mixing increment reuses MIT source from upstream commit
`236c87a889cd76f7e663d1d182c28dc1145f7b14` (0.7.0-alpha.2), without replacing
the imported base or upgrading its host. `engine/src/core/basin.rs` and
`fluid_codec.rs` are exact source imports; `session_cache.rs` extracts only the
small utility from `core/transport.rs`. The Mixer connection test is also reused
from that revision. Basin/Mixer/recipe and Basin-only inventory adapters are
adapted from its corresponding files, preserving their headers and MIT notices.

Local changes restrict this increment to the reviewed dry recipe/plain inputs,
remove unavailable fluid/Press/Basin integrations and use the existing tint.
Fresh canonical inventory snapshots repair the observed post-hopper save/update
gap. A private generated identity comparator and existing placement/removal/render
hooks adapt the code to fc021038/Mixinsa1e13326. The original reference6.0.11
has no selected Basin/Mixer/burner/heat-rule source differences from6.0.10.
Upstream reports remain attributed reference evidence, not private gameplay passes.
Details: `docs/design/NATIVE_S6_BASIN_REUSE_AUDIT.md` and
`NATIVE_S6_DRY_MIXING_PACKET.md`; raw source hashes are local outside Git.

## Plain-water Basin candidate

The `.8` candidate selects upstream236c87a8's four existing custom-fluid hooks,
fluid capability/commit, bucket and Basin renderer functions on the already
accepted host. Private strict water-only input, empty output-fluid and exact
plain-bucket/state predicates keep mixed-fluid/source-discard cases unreachable.
Commit results and exact executed amounts are checked before bucket exchange.
The unchanged core, canonical tank codec, live-slot snapshot, Motion and existing
dry/brass definitions remain. Existing resources suffice; only the exact upstream
water/flour dough recipe is added, with version stamps. Native code and original
art provenance stay separate. Current upstream manifest/dough recipe were read
again before selection. See `docs/design/NATIVE_S6_WATER_BASIN_PACKET.md` for
reference equivalence, limitations and pending qualification.

## Accepted coal-heating selections

On `codex/native-s6-coal-heating`, `.7` selects MIT upstream
236c87a889cd76f7e663d1d182c28dc1145f7b14 burner simulation, visual core,
presentation and server-derived fuel-definition sync/protocol. Core behavior and
sync remain attributed source reuse; coal-only hand/prediction adapters, strict
supported-state guards and minimal particle-math/client-clock selections bound
the private capability. The source state-aware factory, placement, light,
comparator and wrench-loot path use the refreshed host's existing services.
Original saved entity/field names are retained. This is not a new host or
general effects/persistence framework.

The exact upstream heated brass recipe is the second explicitly admitted mixing
recipe; the dry recipe remains unchanged. Import catalog/cache schema11 selects
ordinary burner parts and scroll sprites from the player's pinned6.0.10 JAR.
Committed fallback models are authored cuboids referencing vanilla texture IDs.
No original art bytes are bundled. Empty cage is an item/factory/loot boundary;
capture, ignition, special/creative fuels, hats/goggles and fluids are deferred.
The owner selected Java's full-cap coal consumption. Packet
`docs/design/NATIVE_S6_COAL_HEATING_PACKET.md` and accepted result
`docs/reference/NATIVE_S6_COAL_HEATING_RESULTS.md` record its finite integrated/
dedicated qualification, actual client observations and independent review.

## Accepted bounded host refresh

After accepting dry mixing, the owner selected a bounded isolated refresh to the
same upstream port's host `9c7f9b2215dfed497aba5522c44f38926628648d` and Mixins
`e8cd0aa38cffef49cb2e088ef716baa107148c02`. Reviewed version `.6` retains the
private supported machines and recipes; it is not a whole0.7 import. The old `.5`
branch/maps/binaries remain preserved baseline evidence. Focused migration checks
and independent completion review passed; see
`docs/reference/NATIVE_S6_HOST_REFRESH_RESULTS.md` for its bounded claims.
Upstream now provides `ModRevision::same_entity`,
so this candidate removes the private generated comparator rather than duplicating
it. Four private S5 guards also admit the host's exact empty block-component
metadata without admitting nonempty/unknown payloads. New factory/renderer
capabilities are prerequisites for later selected heating
reuse, not acceptance of heat, fluids or every upstream feature. See
`docs/design/NATIVE_S6_HOST_REFRESH_PACKET.md` and `docs/CURRENT_WORK.md`.
