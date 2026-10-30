# H08 host counter bootstrap source audit

Read-only audit at LOCAL engine79b5951e2e86c17b40d6c5f0a8c26e191ef9219b,
tree e0d7d8941a204d723f23afa1605f5a541d4f47de. This is planning evidence,
not implementation or runtime acceptance. Identity final qualification is pending.

`world/counter_restore.rs::initialize_counter_domain` currently allocates a random
WorldId after checking vacancy, clock and residency. It starts generation/high-water
at1 and counter0, builds a checked trial, then publishes owner fields. Reopen also
constructs a new World with no domain: absence does not prove generation1 is unused.
The retained host UUID must never by itself authorize restarting that history.

WorldSession owns lock plus UUID, but no new-versus-reopened provenance. Preparation
finishes that session before constructing World. Binding a future initialization
grant to the actual initial World therefore needs world_loading.rs as a transport
path. A session-only flag would survive World replacement and dimension return.

Existing observation scopes are optional, caller-supplied and replaceable; chunk
membership revisions track edits and can wrap. Runtime counter keys exist only after
trial allocation. SaveDir identifies storage, and a World moves during preparation
and dimension swaps. None is a stable, virgin World incarnation token. A small
optional weak marker in World, backed by a session-owned strong marker, is justified.

The existing counter-control path resolves a registered captured alias. In contrast,
NativeBinding::from_prepared binds the native prototype to physical vanilla iron.
A host bootstrap adapter must use the retained prepared bundle/catalog and registry
and an explicit physical target. It cannot equate the registered alias with iron,
place blocks, or silently replace current owners.

Audit pins SHA256:

- world.rs 21e16c0452d208af120e3cc1435db505229265b387e8058c5b1e480cf6dac8f7
- world/counter_restore.rs 289ac07b80603d46a0729a9e480a1240dde7bee12db34447989a2c422111ffa5
- counter_trial.rs 94fcace8e6f9d373febe393e08f56c2c98cdc71a1bca1fc1e0074d7556e755c2
- game/counter_control.rs e3002b536cca1b63244f9ea1fad6dfc9a9e930da0f7362c4920d8e50929b67ea
- game/world_loading/host_identity.rs 8ebb3f42440e68ba046d5340631af96cbaf3f6714437fc8db57b99610feb93e3

One finite native read-only source audit informed this proposal. Root must verify
these pins, obtain fresh independent design review and close identity qualification
before any implementation release.
