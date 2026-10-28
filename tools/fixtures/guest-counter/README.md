# Guest counter component fixtures

Private source fixtures for SDK binding and componentizer buildability research.
The server and `client/` are separate unpublished standalone cdylib workspaces;
neither changes the private root workspace or the engine workspace. No build,
component inspection, normal activation or Game callback qualification is implied
by these sources.

The pinned build and paired-evaluation results for this workspace are recorded in
[H05_RUST_COUNTER_FIXTURE_RESULTS](../../../docs/reference/H05_RUST_COUNTER_FIXTURE_RESULTS.md).
That prerequisite proof does not establish guest execution in a game.

Prerequisites: pinned Rust 1.99.1, its wasm32-unknown-unknown target, and the local
Horizon checkout. The server pins wit-bindgen 0.60.0 with the SDK's macros/realloc
features. Before building, the coordinator copies the canonical SDK WIT directory
from `../../../../horizon/crates/horizon-mod-sdk/wit/` into this fixture's
`wit/deps/horizon/`. Keep that copy byte-identical and ignored; do not duplicate or
edit SDK definitions in tracked fixture source. The quiet client's SDK path is
`../../../../../horizon/crates/horizon-mod-sdk` relative to `client/`.

The coordinator resolves each standalone Cargo.lock offline and builds each crate
serially for wasm32-unknown-unknown. Expected release core-module names are
`create_horizon_guest_counter_server_fixture.wasm` and
`create_horizon_guest_counter_client_fixture.wasm`. Use the existing
`horizon_mods::manifest::componentize` path with the corresponding
`horizon-mod.toml`; no custom component encoder, generated bindings or adapter is
provided here. Built components and prepared dependency copies stay out of Git.

The server exports canonical `horizon:mods/server-events@0.1.0`; init registers
no hooks or commands, and every ordinary callback is inert. It also exports the
canonical `horizon:mods/players@0.1.0` dependency because server-events uses its
player types. These extra **fixture-owned exports** return online=[] and tell=false
and make broadcast a no-op. They grant no host capability and perform no host I/O.
Exporting the dependency is intended to avoid a live type-only players import;
the actual component must still prove zero imports after the standard build.

The inline WIT export deliberately names the optional instance exactly
`counter-trial`, without a package-qualified interface name. Its synchronous step
echoes world-instance, instance, game-time and expected-counter, proposes checked
counter+1, and rejects u64 overflow. It retains no counter or other guest state.
The quiet client uses the ordinary SDK Mod/export_mod interface with an empty
Registration; SDK-generated optional exports must be inspected rather than assumed
absent. Neither fixture defines fault modes or a command/control protocol.

The native `examples/preflight.rs` consumes exactly one explicit prepared paired
folder containing captured client/server components and the coordinator's minimal
paired descriptor/content. After review, the coordinator can run
`cargo run --offline --example preflight -- <prepared-paired-folder>` with fresh
owned homes and sanitized session/TMPDIR settings. There is no default folder or
package assembly in the example. Its native-only dev dependency uses the existing
runtime APIs to preflight both halves, refuse any actual import and typecheck the
server evaluator before ordinary initialization. It then activates the same captured
snapshot under Runs::Both, requires one Running entry with a server half, compares
captured identity and the whole 9-to-10 proposal, and rejects unexpected ordinary
mod output. This is paired activation/evaluator evidence;
it creates no World and does not establish token commit or Game callback behavior.

Require fresh source/security review before execution. After building, inspect
actual imports/exports and run mandatory ordinary preflight plus evaluator type
checks against the captured server component. Stop if either component retains
imports; do not relax the evaluator boundary, mutate canonical SDK WIT or infer
purity from capabilities=[] alone. A later paired package needs its own content
declaration, capture/activation checks and Game tests. These fixtures establish no
host-token correctness, checkpoint, persistence, external networking or Create
behavior.
