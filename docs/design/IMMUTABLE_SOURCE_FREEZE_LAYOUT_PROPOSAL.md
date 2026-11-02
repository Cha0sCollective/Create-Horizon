# Immutable review inputs and continued source work

Status: PROPOSED operating workflow refinement; fresh independent Sol High review
required before adoption. No source authority or new product/stage permission.

Current FINAL inputs directly index live files in the physical
.local/worktrees/s1-retained-acquisition checkout. In particular frozen C0d147
includes save.rs/streaming-related integration sources, and later freezes include
LAN/client code. Future ownership work must edit overlapping sources. Preserve
those frozen files and indices unchanged; do not reinterpret hashes or silently
replace an old input. Keep that physical checkout as the accepted reference.

Create one new stable active worktree at acceptedc73abbef on a codex/ branch, e.g.
.local/worktrees/s1-native-close-composition. Future review inputs archive mutable
workspace files instead of making that next live checkout permanently immutable.
Use a NEW target per that physical checkout, never share or copy a target between
physical checkouts. No cleanup/deletion or old-target retirement is authorized by
this refinement. Live storage check: Cfree256767164416bytes, Gfree421604806656bytes,
current target-s1ra1 approximately18GiB. One new active target has adequate room.
Inspect cold-build profile/cache cost and keep updates; no repeated old batteries.

At qualification freeze, copy every candidate source/config/build-recipe input
that may be edited later into an immutable packet-owned source-input archive.
Record exact relative live path, archive path, SHA256 and canonical Git blob id in
a mapping. Existing packet-local receipts/logs/tools are frozen directly once no
further correction is allowed. Existing private accepted documents may be pinned
directly only when they will remain immutable; archive other mutable inputs too.

Qualification still executes ACTUAL live candidate code: the guarded runner checks
branch/parent/scope/live raw hashes and actual Cargo artifact/profile/features,
binary/input identities at batch boundaries. An archive is never executable proof.
No running compiler, test, source mutation or Git write is parallelized. Preserve
all packet sources/prefix before corrections, as already required.

Before clean commit and different FINAL, independently compare archive bytes,
qualified live bytes and index/commit blobs, complete source inventory and sole
parent. FINAL reviewers read the actual commit/diff and execution evidence, hash
the immutable archive initially/finally, and verify live-to-archive equivalence at
the review boundary. Frozen FINAL input indices refer to these immutable copies,
not mutable worktree files. Every qualification claim keeps its actual source pin.
Any CRLF/LF equivalence remains separately explicit and independently reviewed;
this creates no general normalization, stale-artifact or test-reuse waiver.

After FINAL and checkpoint, later accepted packets may edit the same active
worktree, retaining its dedicated target through normal Cargo tracking. Its old
source archive, map, receipts and frozen index stay unchanged. Existing earlier
frozen live paths stay untouched in the original physical checkout; future
archives cannot retroactively replace those inputs. Publication/visibility/hooks,
separate SOURCE/tool and different FINAL, stage gates and owner boundaries remain.
