# Native Create subset for Horizon

Private development workspace for the owner's personal Rust mod. The first planned
reliable release supports a creative motor, shaft, small and large cogwheels and a
millstone that converts cobblestone to gravel. Gameplay is not implemented yet.

S0's reference corpus passes repeatably and G0 accepts a bounded experimental
direction. The pinned Horizon base runs; the first private Rust kernel is next.
Live machinery, host foundations and G1/G2/G3 are not implemented or passed.

- [Current work](docs/CURRENT_WORK.md) identifies the next bounded action.
- [Product specification](docs/PRODUCT_SPECIFICATION.md) preserves the supplied
  version 0.1 plan, including proposed decisions and acceptance requirements.
- [Development](docs/DEVELOPMENT.md) covers checkouts, toolchains and commands.
- [Roadmap](docs/ROADMAP.md) maps the staged scope and current gate status.
- [Host contract](docs/design/HOST_CONTRACT.md) is the H01 experimental direction.
- [G0 decision](docs/design/G0_DECISION.md) records scope, evidence and remaining risks.
- [Reference environment](docs/reference/ENVIRONMENT.md) records P01 tooling and pins.
- [Orchestration adoption](orchestration/ADOPTION.md) records the policy source and
  effective local configuration.
- `workspace.toml` pins the engine, Create reference and Rust compiler.

`create-horizon-core` starts as an empty domain library. Host and client/server
adapters will be added when an S0 contract gives them a concrete interface. The
Rust scaffold's checks establish build configuration only.
