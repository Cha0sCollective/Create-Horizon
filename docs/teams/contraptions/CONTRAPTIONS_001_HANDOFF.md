# Contraptions 001 — read-only native preflight candidate

Status: source candidate; shared compile/core tests pending. This packet
implements the allocated read-only bridge from
[CONTRAPTIONS_001_PROPOSAL.md](CONTRAPTIONS_001_PROPOSAL.md). It does not run a
contraption or change the accepted finite piston.

## Exact source and ownership

Team lane base: `ba17b3ce26cb03700649460a9cfba92828929bfe`.
Accepted central water base: `02d029f3bea09c62480e07e315e23d4868f9180a`.
Pinned host: Horizon `9c7f9b2215dfed497aba5522c44f38926628648d`
and Mixins `e8cd0aa38cffef49cb2e088ef716baa107148c02`.

The five files under
`native/create/mixins/create/engine/src/core/contraption/` were copied as
exact Git blobs from Delete228/create-horizon
`affd76f1301b61b264c08072f9a7aa8d3be0343c`, MIT, before local edits.
Their source blob IDs are:

| File | Upstream blob | Local treatment |
| --- | --- | --- |
| `mod.rs` | `c3cebab8c4cb5bd42e5e6e59b9306839c16d26c7` | Unchanged |
| `assembly.rs` | `6b781c9a56f40d5a69f5ba80476b772c2fa51482` | Reject outside-world and unsupported cells rather than returning a partial or empty capture |
| `transform.rs` | `42ff4a9ceefaccdd9bcd5681ab59b3dde56b218a` | Unchanged |
| `bearing.rs` | `a8235ca430ec044de9fca0a6c3c4c626230a06f8` | Unchanged |
| `tests.rs` | `4450c03f2acf06b8a95134d67097d3913e211df9` | Add the focused capture refusal test |

`native/create/mixins/create/contraption_preflight.rs` is new private
adapter source. It reads the pinned host's already loaded world state using
the guarded bounds/load/registry sequence of
`crates/horizon-client/src/game/native_world_read.rs:121-139`. The ordinary
`native_block` helper can report air for an unloaded cell and is not used.
There is no public Horizon edit or new native API.

## Behavior in this packet

The bridge implements the portable core's `CaptureView` over read-only
native block queries. It admits only a two-block in-memory scene with a slime
block at the bearing's front and one adjacent oak log, within one loaded
chunk. It checks the immutable server execution role and requires an actual
`create:mechanical_bearing` controller block entity with matching facing;
current private content has no live Bearing, so this is a closed future seam.
It rejects a client caller, mismatched bearing axis, unknown states,
block entities, malformed log axes, extra attached blocks, unloaded cells and
out-of-bounds cells. Glue and chassis are closed.

It returns the original source states, snapped rotated landing states, and
centers from the continuous pose. A target must be loaded, empty or still an
identical source cell, and free of a block entity. The vectors are transient
preflight evidence, not an assembly owner or a world transaction. No blocks
are removed, no proxy is made, no saved owner is added, and no collision,
actor, passenger, inventory, client rendering or physical motion is enabled.
The existing S5/S6 piston paths are not referenced or migrated.

## Root-owned serial wiring

After review, root may integrate these two shared declarations serially:

```rust
// native/create/mixins/create/engine/src/lib.rs
#[path = "core/contraption/mod.rs"]
pub mod contraption;
```

```toml
# native/create/mixins/create/horizon-mixin.toml
[[module]]
id = "contraption-preflight"
file = "crates/horizon-client/src/game.rs"
name = "create_contraption_preflight"
source = "contraption_preflight.rs"
```

These declarations compile the candidate but introduce no invocation. A
future read-only diagnostic caller must be separately identified and reviewed
before describing live Game observations. Do not wire a capture-removal or
bearing tick hook through this packet.

## Checks and remaining gates

The source blobs were compared to the immutable staged ref before local edits.
Focused source tests cover the newly closed outside-world path and the
adapter's capture/rotation/refusal cases, including a deliberate chunk-edge
refusal. Independent read-only Sol High review first held three fixture,
authority and controller issues; the repaired staged source delta was CLEAR.
Pinned Rust 1.99.1 `rustfmt --check` on the six owned Rust files and
`git diff --cached --check` passed. Test execution and compilation await
root's shared integration/build scheduling; no result is claimed yet. The follow-on live
assembly needs an owner-approved persistent
custody/save contract, revalidated world writes, scalable contact, one
authoritative pose for rendering and collision, and a separate gameplay
milestone. Machinery, Transport, Fluids and Trains/Schematics retain their
respective actor, item, tank and train-interface decisions.
