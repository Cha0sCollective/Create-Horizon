# H08 explicit host world identity creation and reopen

DESIGN ACCEPTED after fresh independent read-only Sol High review of clarified
proposalSHA40c2dbe4dd6646223f61e44b10ed9ef62bf7620a8246c17e230238d23caffcb2
(originale3ea1fd3efb35a778765d033668cb150fe3ff9d2fccede5b230a4263954012d1).
Concrete header policy also has fresh independent DESIGN ACCEPT at revised
proposalSHA2cfa88adb700954a8510640ee73ed170a00b64c921a330495ef49450302026f8.
Proposal642220da was withheld for native negative-zero incompatibility and is not
accepted. Used-streamer has different final acceptance. Accepted clean input:
LOCALb0f1bf67704a15ded6d5d64ae1ce1a9c52c5a178/tree
015636f48428a160aa7fb01cc7b4b70b30d413fb. Source audit:
H08_HOST_WORLD_IDENTITY_AUDIT.md. Coordinator may release ONLY five paths on fresh
codex/h08-host-world-identity after recording/pushing this private checkpoint.
Exact-source review must verify concrete metadata/lock/lifecycle/sync/launch proof
before execution; source/runtime/final acceptance remains open.
This is an anchor/lifetime prerequisite, not ordinary machine activation, capture
admission, general recovery, migration or G1 acceptance. No owner input is currently
needed to source-review this proposal.

## Bounded objective and proposed paths

Create a host-owned durable world UUID in an explicitly opted-in NEW local world,
strictly reopen it under independently carried intent, and retain that authority
with the actual native session lock through authoritative Game lifetime. An anchor
must exist before machine records ever acquire authority. Current machine domain
initialization remains unchanged and unused by this packet.

Five proposed paths relative horizon-client/src:

1. game/world_loading.rs: explicit request intent, read-only preflight, opted-in
   lock/write ordering, PreparedWorld ownership and focused test declaration.
2. NEW game/world_loading/host_identity.rs: small strict immutable record plus an
   owned session context containing the native SessionLock and optional host UUID;
   defining tests for parsing, I/O, intent and failure preservation.
3. game.rs: context transport instead of a separately cached UUID; explicit internal
   identified-world entry and deferred-entry transport. Preserve existing legacy
   entry signatures/behavior. A separate identified pending variant can avoid
   changing legacy Local literals in startup_menu.rs.
4. game/integrated.rs: explicit identified entry/Launch intent to actual server
   WorldRequest, and context release with existing native shutdown paths.
5. NEW game/world_loading/host_identity_startup_tests.rs: focused real preparation/
   deferred/in-frame/integrated launch and lifetime proofs using native Game APIs.

Dedicated startup stays legacy. No CLI/UI/default opt-in, world.rs/counter initializer,
package v1/SDK/capture/schema rewrite or new dependencies. Inspect exact visibility,
entry and close/error paths before implementation; report any concrete dependency
outside five paths for amendment. Do not silently omit actual launch transport or
widen old helpers merely to construct fixtures. Root owns all execution/integration;
source author gets bounded native source read/edit only after release.

## Explicit intent and source of authority

Carry Legacy, CreateNewIdentity or RequireExistingIdentity intent through actual
request, pending entry, background loader and integrated Launch. Existing constructors
remain Legacy. Candidate data, directory contents and player UUID cannot invent
intent. This packet does not claim detection of a deleted optional anchor in a
Legacy open: required reopen must independently carry its requirement.

Use a small closed native identity record with fixed format version and canonical
nonnil UUID. Bound reads/encoding, reject duplicate/unknown/mistyped/truncated fields,
links/nonregular entries and unsupported versions. No candidate-provided expected
value, repair, implicit allocation on reopen or regeneration. Keep package v1 exact
and unchanged. The record establishes local host provenance, not adversarial
protection against replacement of all host files. A directory clone retains logical
identity; no fork/migration operation is introduced.

Future recovery must borrow expected_world from the original retained host context
BEFORE candidate selection/copy. A copied candidate anchor cannot become its own
expected authority. THIS packet proves only public World::restore_counter_frame
UUID mismatch refusal using independently retained host authority. Archive selection,
copy-boundary proof and recovery integration are explicitly DEFERRED: their helpers
are private outside five paths. Add no visibility bridge or broad recovery path.

## Required order and honest failure contract

First perform read-only package/identity/native-level preflight. Create requires a
genuinely new root, allowing only a regular native session.lock; allocate/encode its
UUID before package writes. Before native lock acquisition reject symlinks and
nonregular entries in the opted-in root, its existing ancestors and any existing
session.lock, since native acquisition otherwise follows/truncates that path.
Recheck eligibility under the acquired lock and prove refusal preserves a linked/
nonregular lock target. No filesystem-adversary race immunity is claimed.
Required reopen requires strict existing identity and bounded typed native metadata
with EXACT current DataVersion, rejecting layouts requiring migration. Freeze the
metadata byte bound and exact required types at source review. It must not turn
Level::read(None) into fresh-world creation. No upgrade/migration acceptance is
added; refuse unsupported opted-in inputs before any such writes.

For opted-in paths acquire native SessionLock BEFORE upgrade, migration or identity/
package mutation. Recheck new-root eligibility or exact preflight identity/native
level facts and package admission under that lock; changes refuse. Existing Legacy
ordering remains unchanged. Never construct an identity-bearing context after
silently dropping or replacing its associated native lock.

For create: unchanged record_world FIRST, then exclusive create_new/write_all/
sync of identity SECOND, then native Level/streaming/world preparation. Anchor-first
would make unchanged package v1 admission reject the root as existing. For unpaired
worlds record_world is a no-op, so preserve this same sequence.

Creation is explicitly NOT transactional. session.lock, valid package pins, or a
partial identity file may remain on failure. Preserve them with a concrete stage/
error; do not delete evidence, silently retry/regenerate or promise zero writes/
rollback. Subsequent CreateNew against that nonempty root refuses; RequiredReopen
refuses missing/malformed identity. Success of identity creation alone does not
prove subsequent world preparation or Game publication succeeded. Checked file and
parent-directory sync guarantees must be stated for the Linux validation target;
unsupported platform guarantees stay unqualified rather than silently promised.

## Ownership and defining proof

### Accepted concrete native-header restriction

Use existing public APIs inside the new child, not save::read_dat's tolerant/raw
fallback. Proposed bounds: level.dat compressed1MiB, inflated1MiB,16384 NBT nodes,
depth64; regular bounded reads and exact inflated consumption. Require gzip success,
empty named Compound root/Data Compound; DataVersion Int and Version.Id Int both
exact horizon_world::DATA_VERSION; Time Long, LevelName String and GameType Int.
Refuse root legacy region/entities/poi/DIM-1/DIM1 and embedded legacy metadata before
any upgrade/migration writes. Preserve other bounded current-native Data fields.

Do NOT require inflated bytes to equal native re-encoding. Source review found a
native Level::write counterexample: spawn yaw/pitch may contain -0.0, preserved by
the writer but normalized to +0 by the native reader. Adopt the native NBT reader's
normalization and last-duplicate-wins semantics for level.dat; make NO canonical-
wire or duplicate-free native-header claim. Required header types/version still
must match after bounded decoding, and exact ORIGINAL preflight bytes must remain
unchanged under the lock. Strict duplicate/unknown-field rejection applies to the
separate closed identity JSON record. Add a positive native-written -0.0 rotation
control and malformed typed-header refusals. No new wire scanner or full Level
schema is introduced; do not silently normalize or rewrite admitted source files.
Bounding level.dat does not bound all saved-data/player inputs used by Level::read.
This revised subsection has fresh independent design ACCEPT at2cfa88ad; the earlier
40c2 acceptance did not choose these concrete limits/policy. Proposal642220da was
withheld for canonical equality; this revision resolves that finding explicitly.

### Original lifetime and qualification obligations

Own lock and identity in one session wrapper so existing None/take/drop/close paths
release both. Audit normal close, preparation failure, cancelled background loader,
integrated failure/shutdown and deferred reopen. No stale identity may survive its
lock. Reading a borrowed UUID must not clone an independent lifetime authority.

Tests must use fresh owned roots and explicit per-process identity intent, never
owner saves/accounts/Prism/reference state. Freeze exact names after source handoff.
Prove paired AND unpaired new creation, strict reopen/UUID stability, duplicate/
size/type/version/link/nonregular/truncation refusal, public counter-restore UUID
mismatch with retained host expectation (no archive-copy claim), failed exclusive
create/write/sync and nonempty-root
refusal. Preserve existing bytes on reopen refusals; record permitted creation
artifacts precisely. No test failure may be credited as rollback success.

Exercise shared WorldRequest preparation, actual in-frame entry and actual integrated
Launch/server transport (a direct load_world call alone is not launch proof), deferred
entry/cancellation and session release. Existing unopted worlds retain behavior and
create no anchor. No dedicated, arbitrary-dimension or ordinary mod-activation claim.

Retain accepted native/streamer/actual-Game capture suites and ordinary tests, three
feature lints/fmt/scope/public-tree, fresh BOTH-mode GUI and broad qualification.
Exact-source independent Sol High review precedes candidate execution; different
final review accepts exact clean source plus evidence. All raw runs stay outside
Git; private checkpoints need fresh PRIVATE verification before each push. Engine
commits remain LOCAL ONLY. No general admission, promotion or publication follows.
