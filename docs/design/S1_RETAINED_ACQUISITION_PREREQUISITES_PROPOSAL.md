# Retained acquisition prerequisites before C0a

Status: PROPOSED, no source or execution authority. C0a remains blocked under its
existing five-path budget. This is a separate bounded prerequisite, not an implied
expansion of that assignment. Source starts only after independent actual design
acceptance, accepted exact predecessor FINAL and private durable assignment.

Fresh s1_prepare_parent_review checked clean writer-adoption e369cac6da8f6e0b604a928fcb2e5233be3cf17c,
tree459d436b07bc35d32264c4e1def9bca072868ccd, sole parent Rf6. It found three
internally losing edges before C0a's PrepareOwner can receive ownership:

- SessionLock::acquire takes the OS lock, then set_len/write_all/flush can fail
  while the File remains a local. Returning Err releases that acquired lock.
- AuthorityNamespace::acquire obtains an authority lock, then fallible lock_path
  validation can discard it. Retaining the caller's native world lock is insufficient.
- GeneratorData::load starts three Worldgen loader helpers. Later spawn/read/parse
  errors can detach earlier handles; absent optional features also skip the blocks
  join. Both VanillaTerrain and flat/TestTerrain live-feature setup reach this loader.

The retained writer API itself is adequate. No lower ownership guarantee, detached
cleanup worker, active lock release, UI-thread waiting or general executor is selected.

## Exact proposed three-path maximum

1. crates/horizon-client/src/save.rs: additive retained native lock acquisition,
   partial acquisition custody and focused inline actual OS-lock tests. Existing
   native writer and ordinary SessionLock::acquire behavior remain unchanged.
2. crates/horizon-client/src/game/world_loading/host_identity/session_authority.rs:
   additive retained authority acquisition/custody and inline tests. Existing
   managed/recovery entry paths remain unchanged; no new ordinary caller.
3. crates/horizon-world/src/worldgen/generator.rs: guarantee completion of all
   three actual loader children on every return/unwind, including skipped optional
   result and partial spawn failure; focused inline tests and existing data checks.

No Cargo/feature/other test path, Game field, C0a caller, World mutation, protocol,
schema, SDK/WIT, generic task system, public release or compatibility promise.
If three paths cannot implement the retained contract, stop for an exact amendment.

## Retained lock/authority contract

Acquire into a non-Clone custody object that exists before the first successful
OS lock. Once the OS lock succeeds, put its actual File/SessionLock into that
owner BEFORE any fallible metadata initialization or post-acquisition validation.
An expected failure returns or leaves the SAME custody object with the acquired
lock and original error. No Err path, retry, local temporary or destructor may
silently release it. Refusal before acquisition creates no authority capability.

Retained retry repairs/revalidates that same acquisition; it never substitutes a
new lock, identity or route. Header repair must reset the same File's cursor and
reinitialize the full header after a partial write; a second acquire or append at
the failed write offset is not repair. Readiness must distinguish an acquired partial lock
from a completely initialized, validated lock. A ready completion transfers only
after all checks, with no fallible operation after take. Failed completion returns
the same custody. Existing borrowed install_authority stays unchanged.

Retained authority custody records exact checked namespace/key/world, original
authority lock and readiness. It must retain post-acquisition path-check errors
and must not derive authority from zero counts, a path string, world UUID alone,
or successful caller-owned world lock. The already acquired native owner lock
remains separately owned by C0a. New retained acquisition stays dormant until the
later C0a consumer explicitly uses it. No rewrite of legacy recovery is required.

An explicit terminal release operation is allowed only for a custody owner that
has no producer, writer or world resources of its own; it discharges ONLY its
actual lock. It must not claim PreparationClosed or authorize release of another
owner. Later C0a must retain the acquisition until its complete producer/writer
close proof. Unresolved retained destruction uses the adopted fail-closed policy;
normal callers must explicitly finish or release. Do not use a Drop cleanup join.

## Loader helper lifetime

Prefer a lexical scoped-thread lifetime for the existing three loaders if it can
preserve results, error/panic semantics and actual thread names. Every started
helper must finish before GeneratorData::load returns or unwinds, including when
a later spawn or parse fails and when optional features omit the blocks result.
This is explicit worker-side initialization work. C0a retains the outer actual
initialization handle and harvests only when finished, so no Game poll performs
these joins. Legacy successful resource/generation results remain unchanged.

The legacy loader's error return may now wait for its already-started children;
this is the explicit bounded cleanup correction, not a responsiveness claim.
Do not detach helpers, spawn disposal threads, suppress a panic, add a global
executor, or replace owned handles with counts. If lexical scopes cannot prove
all actual helper lifetimes, propose the minimum concrete owner API before coding.

## Required qualification

Independent exact SOURCE/tool review before Root execution. Preserve ALLthree
sources and ALL prefix artifacts before every correction. Root alone formats,
builds, executes and writes Git, serially and in isolated scratch directories.

Test actual OS exclusion before/after held acquisition and explicit release;
post-lock initialization errors retain the same actual lock; repeated repair,
wrong route/correlation, post-authority-acquisition validation errors and failed
finish do not lose it. Include actual filesystem failures where feasible and
clearly distinguish injected branch controls from real I/O. Fatal ownership
guards use separate direct prebuilt processes, never nested Rust subprocesses.

Exercise genuine held loader workers with late parse error, absent optional
features, partial spawn failure and child panic. Observe that the enclosing
initialization handle cannot finish before all started children, release/reap
them before assertions/teardown, and require results/errors/panics to be truthful.
Do not rely on sleeps or elapsed time alone. Use narrow test-only inline controls,
not a new production executor or arbitrary callback interface. Client library
controls needed by binary authority tests must use the existing
native-writer-test-control plus debug_assertions gate; library cfg(test) alone
does not expose them to the binary. No new Cargo feature/path is assumed.

Run existing lock/authority and writer regressions plus representative actual
GeneratorData loading/generation controls using pinned resources with no silent
skip. Run relevant client/world feature checks and broader checks. Declare exact
counts/artifact profiles and any conditioned calendar limitation. Then a normal
clean local commit and DIFFERENT FINAL. Refresh C0a's actual predecessor APIs and
record a new private assignment before its original five-file implementation.
