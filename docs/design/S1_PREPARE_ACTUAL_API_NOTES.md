# C0a actual API notes at the qualified component inputs

Read-only preparation map by s1_prepare_api_map. No source authority or runtime
acceptance follows. C0a still waits for writer-adoption FINAL, its clean exact
predecessor, fresh API review and private checkpoint.

Inspected Rf6a3c6a plus exact staged W3fe save.rs in the adoption worktree; both
use writer Git blob ea0eff116ed534a92bb3b4e71aff67121576e7b1. Native raw hashes
can differ only through the separately recorded checkout line endings.

The accepted five-file proposal remains3ef2bed2. Its new test path is exactly
game/world_loading/retained_prepare_tests.rs, declared in world_loading.rs.

Concrete implementation boundaries:

- Allocate retained preparation custody before raw lock/acquisition. Identity
  recheck, package checks, identity write/sync, Level read, terrain/stream spawn,
  counter-world binding and final Level write can all fail afterward. Return the
  same partial owner; avoid consuming it through a fallible question-mark path.
- Existing loader cancellation spawns a disposal thread and must not be reused.
  Retain the actual initialization/resource handles and join only when finished.
  Game::finish_open_world consumes prepared resources and is not speculative
  transfer; refuse with the same prepared object before entering it.
- Streaming has EIGHT NativeWorkOrigin variants: Generate, Decorate, Finish,
  Light, Mesh, Save, SaveProtosCallback and SaveFinishedCallback. Optional Ring
  search is a distinct ninth helper outside this accounting. Omit only that
  optional invocation on the retained branch; preserve the ordinary branch.
- Preserve the shared queue, ownership book, returned receipts, queued-owner
  map, results, deferred leases and original Chunk/Proto/Region owners. Existing
  update, take_deferred, poll_save, native-write fence and Drop are not suitable
  nonwaiting all-origin close algorithms.
- Actual preparation submits no jobs before returning. This supports closing
  a partial no-World owner with idle producer handles, without inventing a World.
- Close the same SaveDir allocation only after producer payload reconciliation
  and actual joins. new_retained → begin_retained_writer_close → poll/status →
  explicit retry → finish retains non-Clone custody. None/busy, Blocked and failed
  finish return the same owner. Unresolved retained Drop aborts. Only the genuine
  producer witness plus NativeWriterClosed permits terminal PreparationClosed
  and release of authority/lock; zero counts or accepted callbacks do not.
- Streaming is a library and cannot call binary Game/D validators. The latter's
  raw-column helpers are private; there is no ready-made generic classifier.
  Keep complete late payloads owned in Streaming and Game-specific policy at the
  permitted preparation boundary. A real need to expose another path requires
  a precise amendment, not an assumed sixth-file allowance.
- Existing integrated Fixture is Game-visible and reusable, but its fields are
  private. It needs pinned resource env fixed per process; do not Options::read.
  Library cfg(test) controls are absent from binary tests; use the existing
  native-writer-test-control plus debug_assertions pattern. Release, poll and
  reap genuine stalled workers before assertions/teardown.

This map found no demonstrated mandatory sixth production path. It is a source-
start aid; exact predecessor APIs and ownership tests must still prove the design.

November1 correction: fresh independent actual-parent review found three internal
acquisition/helper edges omitted by this initial map. SessionLock::acquire can
drop its acquired File on header initialization failure; nested authority acquire
can drop its lock on post-acquisition path validation; GeneratorData::load can
detach three loader children on early return or optional-result omission. Both
VanillaTerrain and flat live-feature setup reach that loader. Therefore the
five-file C0a scope is blocked, despite the outer caller map above. The separate
three-path prerequisite proposal2df6d8fe was conditionally accepted, with no C0a
source expansion. C0a requires its FINAL and a new actual-parent API review.
