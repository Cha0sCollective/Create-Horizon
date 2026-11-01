# C0a partial-acquisition ownership amendment

Status: PROPOSED conditional design amendment. C0a source remains blocked until
the separate three-file acquisition prerequisite receives FINAL, a fresh exact
clean predecessor API review and a private durable source assignment. No new path
or ordinary startup/close activation is granted by this document.

This refines the accepted five-path proposal3ef2bed2 using read-only mapping by
s1_prepare_api_map at e369 plus the three-file draft SOURCE CLEARcbf2db50:
save4c27079c, nestedauthoritycaa53775, generator91e55d15. Those draft APIs are not
yet compiled or accepted as a clean FINAL predecessor. Refresh the actual pins
before any source release. The exact C0a budget remains game.rs, streaming.rs,
game/world_loading.rs, game/world_loading/host_identity.rs and NEW
game/world_loading/retained_prepare_tests.rs.

## Own partial acquisitions before calling acquire

The original raw Option<SessionLock> is insufficient during acquisition. The
armed PrepareOwner must contain Option<RetainedSessionLock> before its first
acquire(), and Option<RetainedAuthorityAcquisition> before its first acquire().
Completed raw SessionLock/WorldAuthority exist only after checked finish. Keep
each completed object in the same owner before the next fallible step. Preserve
the original preparation error separately from subsequent cleanup errors.

PrepareFailure and the retained ready wrapper own this SAME PrepareOwner. Do not
publish an unguarded legacy PreparedWorld, return only an error string, or move
an acquisition into a fallible local temporary. The owner also keeps preflight,
request/root/namespace, completed WorldSession, retained SaveDir, partial/full
Streaming, Level/World and original generation/native payloads as applicable.

Worker-side sequence preserves existing checks:

1. Create owner, then existing namespace/intent/package/preflight checks.
2. Store native acquisition, call acquire; on error preserve both acquired and
   ready distinctions. An acquired partial header is actual lock custody.
3. Recheck identity under that lock. For a managed new identity, construct/store
   authority acquisition before calling it; retain header/post-path errors there.
4. Checked authority finish remains worker-side filesystem work. Store its actual
   WorldAuthority immediately. Wrong namespace/world/refused finish keeps custody.
5. Preserve existing expected-checkpoint/package checks and metadata ordering.
6. Borrow preflight and native acquisition through identity create/write/sync.
   Only then extract completed native SessionLock with no fallible operation after
   take; store returned WorldSession immediately, then borrowed install_authority.
7. Continue native Level/settings/save/terrain/Streaming/World/binding/final metadata
   steps, retaining each original resource before the next fallible operation.

Close Retry never replays identity creation or restarts acquisition. Explicit
acquisition repair is distinct from cancel, repeated close intent and writer
Retry. Repair resets/truncates/rewrites the same File; first failure remains
observable. An acquired partial wrapper must stay guarded through closure. A
narrow wrapper lock-only release does not certify PreparationClosed; producer
and same-allocation writer closure remain prerequisites for outer lock release.

## Identity helper placement

IdentityPreflight.world/kind are private to host_identity.rs. Proposed internal
borrowed helpers belong there, within the existing five-path budget:

```rust
fn retained_authority_acquisition(
    &self, namespace: &AuthorityNamespace, root: &Path
) -> Result<RetainedAuthorityAcquisition>;

fn finish_retained_authority(
    &self, namespace: &AuthorityNamespace,
    acquisition: &mut RetainedAuthorityAcquisition
) -> Result<WorldAuthority>;

fn finish_retained(
    &self, root: &Path, native: &mut RetainedSessionLock
) -> Result<WorldSession>;
```

Names/signatures are sketches, not public APIs. Preserve the old new-identity-only,
namespace/root and preflight rechecks. Validate readiness/route before extracting
anything. Authority finish revalidates filesystem paths and must not run in
nonwaiting UI/status polling. No nested session_authority.rs edit is authorized
for C0a; the prerequisite provides that capability.

## Native helper and entry boundaries

The prerequisite's lexical GeneratorData scope covers all three actual loader
children, including late failure and omitted optional result; it also covers the
already scoped nested blocks-support loader. VanillaTerrain and flat/TestTerrain
live-feature setup both reach it. Retain the actual outer initialization handle;
harvest only when finished. Child joins remain on that worker, never a UI poll.

The optional detached Ring search still must be omitted only on the retained path.

Global legacy PRELOAD is another actual handle: world_data_used replaces Started
with Used and can discard it. The dormant retained path must bypass the legacy
open_world_with_identity route that calls world_data_used, and must not start
preload_world_data. It does not claim that a pre-existing global preload is joined
or that ordinary title/startup is already covered. Actual ordinary preload and
resource-tail error/renderer/Game-construction edges remain explicit C0e work.

Core resource facts/biomes/environment loading is already lexically scoped.
load_resources performs fallible preparation before spawning its WorldRestLoader
and returns the owned Loaded tail immediately. Use the separate retained resource
harvest primitive; do not route through ordinary Loading::finish/start_game.
Level read/write/settings/package IO remains worker-side work; partial metadata
artifacts stay with the original owner on error. No additional helper was proven
to require a sixth C0a path in this inspection.

## Qualification delta

Exercise partial native and authority header/path failures with both real locks
retained independently; same-handle repair and refused finish; identity create/
write/sync failure without raw lock loss or replay on close Retry; genuine held
generator children under Vanilla and flat live-feature paths; cancel/late Ready/
late Failed harvested once without disposal; resource-only delayed/error/panic
outcomes; guarded ready adoption refusal returning the same owner.

All eight Streaming origins, original late payload/receipt custody, nonwaiting
status, explicit close Retry, actual finished joins and same SaveDir typed close
remain mandatory. This amendment adds no general executor, codec, protocol,
schema, feature, source path, smaller G1 or save compatibility promise. Independent
exact source/tool review, serial qualification, clean commit and DIFFERENT FINAL
remain required after the predecessor gates are actually met.
