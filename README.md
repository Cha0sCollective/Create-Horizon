# Native Create subset for Horizon

Private development project for a personal Rust Create port. The active code is
under `native/create/`, reusing the existing native Create implementation and
Horizon's server, inventory, save and rendering services.

The supported target is a creative motor, shaft, small and large cogwheels, and a
millstone converting cobblestone to gravel. The November 4 native plan replaces
the old Wasm/in-frame route. Open to LAN and the custom cross-owner crash-recovery
framework are deferred. Existing reference results and legacy work are preserved.

- [Current work](docs/CURRENT_WORK.md) gives the active branch and next action.
- [Adopted native plan](docs/design/NATIVE_PLAYABILITY_PLAN.md) defines S1 acceptance.
- [Native S1 results](docs/reference/NATIVE_S1_PLAYABILITY_RESULTS.md) records the
  source/build, integrated and two-client dedicated checks, and their limits.
- [Native S2 results](docs/reference/NATIVE_S2_KINETICS_RESULTS.md) records source
  arbitration, reference block/drop outcomes, topology and normal restart checks.
- [Authorized stages](docs/design/STAGE_PROGRESSION_PLAN.md) defines the S2–S6 route.
- [Native development](native/README.md) gives build and focused-check commands.
- [Source provenance](native/UPSTREAM.md) identifies the imported MIT base and pins.
- [Roadmap](docs/ROADMAP.md) separates the active native route from historical stages.
- `workspace.toml` pins the compatible host, port, references and tools.

The Create behavior reference remains 6.0.11; the imported base and local art
bootstrap are 6.0.10, with selected differences reconciled explicitly. Original
art and Minecraft resources stay local. This is a bounded source-build prototype;
full Create/modpack compatibility, existing-world import and release distribution
have not been established.

The root Cargo workspace, old [host contract](docs/design/HOST_CONTRACT.md),
[reference corpus](docs/reference/ENVIRONMENT.md), [G0 decision](docs/design/G0_DECISION.md)
and historical G1 work remain available for attribution and relevant reuse. They
are not the current native build or evidence that old G1 passed. The original
[product specification](docs/PRODUCT_SPECIFICATION.md) remains a planning baseline.
