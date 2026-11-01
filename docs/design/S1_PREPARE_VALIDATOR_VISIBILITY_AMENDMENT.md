# C0a native validator visibility amendment

Status: ACCEPTED by fresh s1_composition_api_review at proposed document hash
d4221f72b68211f80f492880a4ceb3a865f2dcfe88626f9e6e10256e5e54acf3.
Composed predecessor FINAL is accepted. Source release still requires the private
durable source assignment; this status update changes no accepted scope.

Actual clean predecessor1956eb4946bd75138bed57bdcdf9ccf1d51115f9 combines the
accepted Guest and acquisition components without source changes. Read-only
author preparation found that the existing native column validator is private
to game/levels/session_checkpoint.rs. Duplicating its roughly200-line validation
in WorldLoading would add unnecessary maintenance and divergence risk.

Add exactly ONE sixth path to the original five-file budget:
crates/horizon-client/src/game/levels/session_checkpoint.rs. Its only permitted
change is making existing fn validate_column visible as pub(in crate::game).
Preserve its signature, implementation, accepted native profile and callers.
Existing item validators are already accessible within crate::game; no seventh
path, validation context, codec or native profile expansion is granted.

The retained Game-side close adapter borrows the original complete late column
from Streaming, reuses strict existing validators, and confirms that native
projection has discarded no payload. It validates/stages original supported
item entities, coordinates, UUIDs and duplicate/ambiguity facts before allowing
storage disposition. A one-use checked ticket keeps the full column and original
work lease in Streaming until actual native storage acceptance. AlreadyStored
requires concrete original storage provenance; refusal retains custody.

This dormant Game-side service has an existing Game resource context. Healthy
Vanilla/flat preparation with no jobs can close; this does not claim coverage of
ordinary pre-Game construction/error paths, which remain C0e. Do not use that
deferred wiring to omit any eventual S1/G1 lifecycle requirement.

Keep the original World in Box<World> inside the guarded boxed PrepareOwner for
stable close identity. The actual retained Streaming binds its Arc<SaveDir> at
construction. Its producer witness records actual joined membership; final
discharge checks both that witness and the same-allocation NativeWriterClosed.
Failed acknowledgement returns the same producer witness with custody armed.
An initialization panic with zero submitted work may yield a reaped failed-start
outcome; a held-work panic or unexplained missing payload remains Blocked.

All other accepted ownership, nonwaiting polling, eight-origin accounting,
explicit Retry, qualification and ordinary-activation boundaries remain. This
changes one internal visibility, not validation semantics or user permissions.
