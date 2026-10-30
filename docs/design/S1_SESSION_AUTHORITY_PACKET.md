# S1 trusted world-authority precursor

DESIGN ACCEPTED by fresh independent native read-only Sol High review at exact
proposal SHA2566f98cc8d39ea8c42ebbc3026b4c4485bbe1c94e0e541e130c930abc24d026c76.
Snapshot: workspace .local/s1sa1-design-revision2.md. Initial87b0 draft was withheld
for lock-write-before-overlap-check ordering; corrected996f was accepted, and this
exact final revision adds the explicit archived-world regression. No source/runtime
acceptance is implied. Private checkpoint precedes source release.
Base engine872dac2da765f0b290bfcfe13502f7d97724be1f
on codex/s1-session-checkpoint. The checkpoint foundation is final accepted;
this packet closes its cross-copy authority gap for an explicit internal managed
entry. It does not claim ordinary activation, failed-worker-stop retention,
real connected roster staging, in-frame recovery, filled ingress or G1.

## Concrete boundary

A native session.lock protects one destination folder. Two recovered copies can
therefore hold two native locks for one WorldUUID. Add a separate non-clone OS-held
authority lease keyed by the expected WorldUUID in ONE trusted coordinator namespace.
The namespace is a private explicit configuration value, never read from the archive,
manifest, destination, working directory or package. All managed entries must use
that same configured namespace. Different namespaces, legacy opens and uncooperative
processes are outside this precursor's exclusion claim; no implicit global guarantee.
There is no new environment option, command, user interface or ordinary caller.

The namespace root is an existing absolute real directory outside source world,
archive and destination trees. Reject parent traversal, symlink/reparse targets and
non-directories in checked paths, and linked/nonregular session.lock files before
native acquisition. Canonicalize/recheck paths so spelling aliases use one namespace.
No automatic deletion or cleanup of authority files. Its per-world child name is
exact canonical WorldId text; acquire via the existing SessionLock. Same-process
independent handles must contend, not rely on a process-local map or a mere marker.
The same UUID copied to another anchor/destination still selects the same lock.
Threat boundary is cooperative local processes and trusted stable parents, not
hostile concurrent filesystem replacement or another user choosing another namespace.

## Trusted identity and ownership

New identified creation: keep existing native lock and identity preflight ordering.
After its under-lock recheck, acquire the world-authority lease for the newly allocated
UUID BEFORE writing package/identity records or constructing streaming/world state.
Retain it inside the resulting non-clone WorldSession alongside its native lock.
An opted-in preparation failure returns without playable state; retain created
artifacts and allow ordinary RAII to release locks. Legacy callers remain unchanged.
Do not offer managed existing-native open: archive recovery is the authoritative
reopen path, not native-only fallforward. Reject this intent before mutation.

Recovery: FIRST preflight the complete route (trusted owner anchor, namespace,
archive and destination), rejecting all pairwise overlap involving the namespace or
owner anchor, and archive/destination overlap, BEFORE acquiring any lock or writing
any target. This is mandatory because SessionLock::acquire writes session.lock.
The wrapper receives this full route; there is no externally callable anchor-lock
constructor without its archive/destination context. Then create a non-clone trusted-owner lease from the explicitly selected owner
anchor folder. Read ONLY its bounded current identity record (existing strict schema),
preflight and acquire the anchor's native session.lock, reread EXACT identity bytes
under lock, then acquire namespace/UUID authority. Retain both locks in that lease.
Do not require the anchor's level.dat to be readable: interrupted native data is why
archive recovery exists. Never infer expected UUID from a candidate manifest or mint
a bootstrap grant for recovery. A copied anchor with the same identity still contends
on the namespace/UUID lock. An already open anchor refuses before archive selection.

Add an internal authoritative-recovery wrapper taking the checked namespace, a
managed request containing the trusted owner/archive/destination and target/type/
deadline (NO expected UUID field), and the real native LanHost. After complete route
preflight and locked identity acquisition, construct the existing RecoveryRequest
with expected_world derived ONLY from the lease. Keep the lease alive throughout
newest-complete selection, strict validation, copy and candidate preparation. Move it
into the returned Game's matching WorldSession before returning success. Native
session and authority are never Clone, exported as a bool or reconstructed from
UUID bytes. All failure paths drop unpublished candidate state before releasing
its lease; a successful returned Game owns the lease until its session is retired.
No alteration to the existing checkpoint format, newest-image refusal, owner profiles,
provider witness or un-managed research primitive. Old tests remain valid for their
explicitly un-managed scope, not evidence of managed exclusion.

## Scoped implementation

Work in a new local codex/s1-session-authority worktree based exactly on872dac2.
Released paths, contingent on design acceptance:
- game/world_loading/host_identity.rs: optional private authority owner in WorldSession;
  new-identity acquisition and bounded identity-anchor access seams only.
- new game/world_loading/host_identity/session_authority.rs: namespace/lease path and
  acquisition logic with focused native lock tests (split tests file if needed).
- game/world_loading.rs: explicit internal new-identity authority configuration,
  reject managed native-existing route, move lease into PreparedWorld session.
- game/levels/session_checkpoint.rs: separate internal guarded recovery wrapper;
  existing un-managed primitive/signature unchanged.
- game/levels/session_checkpoint_tests.rs: focused actual native Game coverage using
  existing resource fixtures. No assertion weakening or synthetic replacement world.

No saving.rs, integrated.rs, commands, options, networking, release paths or item
ingress changes. The next lifetime packet must retain actual worker/Game/host and
these leases when checkpoint publication fails or is uncertain; merely adding this
lease does not solve that problem. Returning an error from ordinary serve remains
unsafe for managed activation and is not enabled here.

## Checks and stop conditions

Before source execution: fresh independent read-only Sol High exact-source review.
Root alone formats, executes serialized checks and commits. Preserve every scoped
source and packet artifact before corrections. Isolated scratch and no playable data.

Focused native tests must independently establish:
1. Same namespace+UUID contends across distinct anchor paths and separate handles;
   distinct UUIDs coexist; dropping last actual owner permits reacquisition.
2. Legacy and un-managed research behavior remains unchanged. Managed new creation
   owns authority before streaming/publication; failed acquisition leaves original
   identity/package/native bytes unchanged and no playable Game.
3. Anchor trust: malformed/missing/oversized/unknown identity refuses; identity-only
   anchor succeeds despite missing/corrupt level.dat; byte recheck detects changed
   identity; no fallback UUID/bootstrap; live anchor lock refuses.
4. Path aliases contend; parent traversal, linked namespace/child/lock/anchor/identity
   and namespace/world/archive/destination overlap refuse without target writes.
   Explicit regression: select archive/epoch-N/world as trusted owner anchor and
   verify every archived byte/path remains unchanged and no session.lock appears.
5. Actual resource Game new+guarded restore owns lease after helper return; another
   copy with same UUID cannot recover concurrently; after the Game/session actually
   drops, a new valid restore succeeds. Real positive native roster objects retained;
   no actual connected-admission claim.
6. Guarded path refusal happens before any lock writes, archive access or copy; recovery
   failure leaves no playable Game and releases locks; incomplete artifacts preserved.
7. No hidden subprocess build/fixture compilation in Rust tests; no ordinary caller.

Appropriate checks: formatting, all three relevant feature-lint configurations,
focused new tests, retained identity/checkpoint Game/resource regressions using pinned
resources/prebuilt pair, optimized TYPECHECK, and clean committed broad pr_ready quick.
No GUI or Java parity claim is needed for a dormant ownership-only change; any changed
vanilla behavior expands review/check scope. Different independent final review
must inspect exact commit/results before bounded final acceptance. Engine local only;
private plan/results checkpoints require fresh privacy+origin guard. No public release.

A source correction remains within this boundary only when it preserves the stated
trust and lifetime contracts. Missing exclusion/lifetime proof or unavoidable broader
behavior returns to design review, not silent optional checks or reduced S1.
