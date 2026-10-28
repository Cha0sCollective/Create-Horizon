# H06b World-owned slot lifetime results

Local engine commit `1e1b2243934289d92a2193de88ec58311d0af069`, tree
`f6b8f12758f7201f28aba5487afc0d65f707e51c`, adds the
paired-packages-gated native library companion giving one World a private
move-only slot projection, captured Configuration and opaque scoped allocation
key. It is independent of the counter's Copy Record/retirement diagnostics.
No Game callsite enables it. The acceptance boundary is
docs/design/H06_WORLD_SLOT_OWNERSHIP_PACKET.md.

Actual World::set and authoritative stream insert/remove hooks update ownership
synchronously. Same-state writes retain identity. Unload suspends the allocation;
matching arrival resumes its identity and contents. Changed blocks, resident chunk
replacement and mismatched reload move it to one owned pending retirement. Further
writes stay effective but cannot replace that retirement or admit more items.

World-wrapped transfer plans retain the opaque trial identity/incarnation and
delegate to H06a only after fresh key, fault, residency and actual-storage checks.
Taking retirement transfers the complete owner once and does not automatically
admit another. Explicit reconciliation can admit empty under a fresh key; explicit
teardown returns active/pending owned values. None of this is automatic Drop or
crash retention. Receivers remain responsible for the values they take.

Reviewed source after pinned formatting:

| File | SHA256 |
| --- | --- |
| client/src/world_slot_trial.rs | `5d621e713fc964c3944967be5e8e2628fab70c0c8991cef01ba86a503acc9951` |
| client/src/world_slot_trial/tests.rs | `16c79065f87e1aa5d472ddf54671e582c147420f7179acfe606466a4256cf00f` |
| client/src/world.rs | `78b2b714df6f3deaba6a500e7bc504135d067e259373451ea89fe46a66fd1fa7` |
| client/src/lib.rs | `4428319d12db6decb860c8fc29a629c4184f35e62151591078eb9999f05f55d0` |

Fresh independent read-only Sol High design and implementation review find no
issues in the bounded unit. Eight focused native matrices exercise real World and
Chunk values: both-direction transfers, raw nested BundleSelected/opaque component
retention, native-identity accounting, changed limits/player revisions, duplicate/
stale plans, repeated pending writes/replacements, unload/resume, exchanged scopes,
re-enablement, teardown, incarnation exhaustion and counter-fault coexistence.

Temporary map lending uses actual mem::take/restore of World.chunks; it does not
execute a Wasm SDK callback. An intentionally altered loaned map also demonstrates
fresh state refusal, followed by real physical removal retaining the owner despite
a counter fault. This does not establish lifecycle coverage for direct map writes.
The package fixture supplies captured machine metadata only; stub component bytes
are never instantiated by these tests.

All eight focused matrices pass with runtime+paired and paired alone, zero ignored.
The existing 13 counter, two mutation-observer and two residency-observer unit tests
pass. Runtime-on, paired-without-runtime and feature-off client library/binary/test
Clippy pass with warnings denied; pinned format and public-tree checks pass.
The four-file +947 change includes 371 module and 544 child-test lines; its soft size
warning is disclosed. No dependencies or lockfiles change.

All ten actual World mutation/residency/storage regressions also pass, zero ignored.
The normal local commit hook passes. Broad quick passes 369 Python and 1357 client
tests, zero doc tests, format/lint/public-tree and unchanged lock checks at the
exact local commit. Content-root and launcher checks are skipped as untouched;
parity is explicitly skipped in quick mode. The +947-line scope warning remains.
Final independent read-only receipt review finds no issues, verifying exact source/
checkout pins and actual scripts/logs/status. It does not rerun validation; current
hook configuration and standalone public-tree success are verified, while the
coordinator's observed commit-hook output was not separately archived in h06b-*.
Raw records stay under workspace-root .local/h06b-* paths. No source fixes were
needed after review or execution.
Existing H05d real-game results remain
unchanged coverage, not new Game execution of this unconnected service.

Live player authority, checked item-entity receivers, all World-loss paths,
off-resident entity-section ownership, save/retry retention, guest slot callbacks,
replication, persistence, performance and broader H06/H07/G1 remain open. No stable
SDK, public push or Create conformance is claimed.
