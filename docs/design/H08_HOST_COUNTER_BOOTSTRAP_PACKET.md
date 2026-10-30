# H08 initial-world host counter bootstrap

DESIGN ACCEPTED, NOT RELEASED. Fresh independent read-only Sol High review accepts
proposalSHA c0413fb40217d5e077dda5bb39e209def153205c8d5cbd27a41437aa3e93c858,
with no blocking finding. Exact source must verify one-time binding, pointer identity
with a live strong marker, explicit actual dimension and final infallible commit.
No implementation or execution authorization follows until final identity acceptance, a pushed
private checkpoint and a clean successor branch pinned to79b5951/treee0d7d894.
Source audit: H08_HOST_COUNTER_BOOTSTRAP_AUDIT.md. G1 remains open.

## Bounded objective

Provide a dormant native adapter that initializes ONE counter domain with the
retained host world UUID, only on the actual initial Overworld of an explicitly
CreateNewIdentity session. Legacy and RequireExistingIdentity sessions refuse.
No ordinary command, automatic activation, archive capture or recovery is added.
The existing random-world initializer and transient diagnostic commands keep their
behavior. A saved UUID or a missing domain is insufficient initialization authority.

The independently established premise is: this new logical identity has never
successfully bootstrapped a host counter; this is its initially bound World; and
no alternate trusted host path has installed/restored records under that host UUID
outside this grant. Current ordinary durable activation is absent. This is a
trusted native Rust caller boundary, not protection against arbitrary host code.
Required reopen remains refused until durable allocation/history recovery exists.

## Six proposed source paths

Relative to crates/horizon-client/src:

1. world.rs: optional weak incarnation marker, default None.
2. world/counter_restore.rs: opaque non-Clone/nonserialized bootstrap grant,
   checked initializer and focused defining unit controls.
3. game/world_loading/host_identity.rs: create provenance and session-owned grant.
4. game/world_loading.rs: bind grant after constructing the actual initial World.
5. game/counter_control.rs: separate private Game adapter and test-child declaration.
6. NEW game/host_counter_bootstrap_tests.rs: focused resource/lifecycle controls.

No SDK, codec, package pin, save schema, dependency or broad helper visibility
changes. Report concrete outside-scope dependencies for review before editing.
Root alone owns formatting, Cargo/tests and integration; serialize all execution.

## Authority and atomicity

WorldSession owns an opaque one-use grant with host UUID, a strong Arc marker and
spent latch. Only CreateNewIdentity creates it. World retains only the corresponding
weak marker, bound once during preparation to the initial actual Overworld, never
at first initializer use or from thread-local state. Moving or parking that SAME
World preserves binding; a replacement World and dimension return cannot recreate
it. Preparation supplies actual dimension explicitly; the core must not infer it
from chunks. No grant cloning, serialization, reopen reconstruction or reset.

The new initializer borrows the grant mutably and checks unused status, matching
initial incarnation/dimension, absent trial/domain, supported binding/target,
resident native target and valid nonnegative clock before publication. Existing
matching, dormant, faulted or detached ownership also refuses. Construct all checked
keys/trial/state first; owner publication and consuming the grant are the final
infallible commit. Failed validation/allocation preserves grant availability,
existing owners, heads, clock, RNG and chunk bytes. No repair, clearing or fallback.
Reuse the checked trial allocator; never reset/exhaust the global instance counter
in tests. Local atomic exhaustion controls may prove allocator failure separately.

The Game adapter uses actual retained prepared package/catalog and native registry,
with an explicit physical vanilla iron target for this prototype. Registered alias
resolution is a different binding contract; do not silently substitute one for the
other. Do not place blocks, refill state, take current owners or invoke guests.

## Required evidence and stop condition

Defining tests cover one successful retained-UUID bootstrap, fresh runtime keys,
exactly one successful consumption, refusal after replacement/dimension mismatch,
Legacy/reopen refusal, occupied/faulted/dormant state, invalid clock/binding/target,
missing residency and preservation on each refusal. Prove same-World move/parking
does not accidentally revoke the unused grant. A source review must check the
all-fallible-work-before-publication boundary and unused-history premise explicitly.

Resource tests must use actual identified in-frame and real integrated Launch/server
owners through normal preparation and close, without mutating running Game roles.
Use fresh isolated worlds and the existing prebuilt pair fixture; no nested Cargo,
Java, Python or fixture scripts in tests. Preserve old random/transient behavior and
all prior identity, capture, slot, counter, owner and streaming regression coverage.
Exact-source independent clearance precedes execution. Root qualifies feature
lints/fmt/tree, focused and retained tests, fresh BOTH-mode game smoke and broad
checks, then obtains a different final independent review before accepting.

Stop on a concrete unsupported dependency or genuine owner decision; otherwise
address technical review findings within this bounded design. No ordinary runtime
capture/recovery, global readiness, multidimension history, item-bearing machine,
wire/scalar completeness or S1/G1/S2 acceptance is implied.
