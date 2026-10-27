# Native Create subset for Horizon

Private development workspace for the owner's personal Rust mod. The first planned
reliable release supports a creative motor, shaft, small and large cogwheels and a
millstone that converts cobblestone to gravel. Gameplay is not implemented yet.

S0's reference corpus passes repeatably and G0 accepts a bounded experimental
direction. The pinned Horizon base runs; a first private Rust kernel calculates
settled single-source speeds/stress and matches selected reference checkpoints
both natively and inside Horizon's development Wasm sandbox.
Live machinery and the complete host foundations remain pending; G1/G2/G3 are open.

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

`create-horizon-core` is independent of Horizon. Its narrow evidence and unsupported
cases are in [P02 results](docs/reference/P02_RESULTS.md). Host and client/server
adapters still require the S1 foundations; pure calculations do not establish
live-game conformance. The [Wasm evidence](docs/reference/P02_WASM_RESULTS.md)
records the exact sandbox scope. The [D0 baseline](docs/reference/BASELINE_D0.md)
is software-rendered B0 context, with performance qualification still open.
