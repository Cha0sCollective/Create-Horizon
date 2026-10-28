# H08a13 synchronous dimension-transition ownership results

Local engine commit `dc12bd60aeaee33f09b18d9e8ffee4d763a52355`, tree
`2844edb54226baee3e17c3d92a004852311fed22`, retains source native entity owners
and metadata while synchronous
saving and target preparation run. save_dimension_entities now captures without
removal and returns the exact resident coordinate set only on save success.
switch_level applies the shared existing cleanup and EntityMemory reset only after
target preparation succeeds, immediately before World replacement. enter_dimension
restores the same taken player on error. The nearby synchronous chunk handoff
clears only submitted dirty coordinates after successful Region/POI saving.

Fresh independent read-only Sol High design and source/test review find no
blocking issues. The shared cleanup preserves prior selection, unknown records,
anonymous metadata, UUID/raw mappings, root/rider rules and async-unload behavior.
No SaveDir writer, parser/defaults, Game-slot enablement or shutdown policy changes.
The accepted boundary is docs/design/H08_DIMENSION_TRANSITION_OWNERSHIP_PACKET.md.

Reviewed source SHA256:

| File | SHA256 |
| --- | --- |
| client/src/game.rs | `be4909cd0683e8665184ad973694298d949b9654670c06e03fe5bf312a4860ea` |
| client/src/game/persist.rs | `5d0676944dafcc2e2895a0e0702486b311d7e22e1890a90f92e3d526975ab403` |
| client/src/game/integrated.rs | `195c7e90916c186a8a12a0482d1a5943ab8429302eb8a6d5a04d79f845a2926a` |
| client/src/game/integrated/transition_ownership_tests.rs | `f03b4893ed12a985dc34b4238e51c56c2410d9ca2ca6bf080991541a0642cdad` |

Four explicit ignored tests run from frozen native test binary SHA256
`698d40d4f3fedfce29933c80e088aac8b78c6406acffb51b392a01a521247505`.
The build receipt pins all four candidate source hashes; it precedes the final
source commit and does not claim a clean-commit binary build. The isolated runner
checks binary/source hashes, the pinned resource manifest, and all four exact test
names before execution. It uses fresh cwd/HOME/data/config/cache/temp, clears the
session file and preserves /run/user/1000. All four pass, zero ignored, in 16.62s:

- Entity-folder obstruction makes actual enter_dimension fail after accepted
  entity enqueue. The native player/entities, UUID/raw mappings, rich item/boat
  contents and unknown/anonymous records remain. Repair and direct producer retry
  retain the sticky error while fresh disk readback remains nonempty and exact.
- Actual target worldgen preparation fails after successful departure saving.
  Source owners/player/metadata remain; restoring only that Game's resource option
  permits normal dimension commit, coordinate scaling, clock carry and cleanup.
- Actual region-write obstruction retains all dirty markers and the taken player.
  The target-failure case separately proves successful saving clears only submitted
  resident keys and preserves missing/unrelated markers.
- The shared cleanup preserves outside-key records and retained ridden boats/carts,
  removes departing root/riders and selected metadata, and leaves successful entity
  capture borrowed until cleanup is explicitly committed.

These tests use real loaded resources, headless Game construction and production
methods. No mocked acceptance closure or alternate ownership algorithm is used.
A complete switch retry can fail at its earlier sticky chunk-flush barrier; the
entity test deliberately retries the actual producer rather than claiming that
unreached path. Disk failure retains queued NBT while SaveDir lives; this fix also
retains native ownership and does not assert immediate snapshot loss.

Ordinary binary tests pass 173, with eight ignored; four of those are the explicit
resource executions above. All three feature lint configurations, formatting,
public-tree and the normal local commit hook pass. Actual hook/commit output is
archived. Four-file +621/-112 scope includes 492 child-test lines and existing
cleanup factoring; the soft size warning is disclosed. No dependencies or locks
change. Broad quick passes 369 Python/1357 client/zero doc tests and format/lint/
public-tree at the exact commit. Content-root/launcher checks are skipped as
untouched, parity explicitly skipped; the 733-line soft warning remains.

Frozen optimized game SHA256
`7a70263c620f6013e4c5eb020aa0f167fd988e088daec4584ad4d22256f5b46c`
passes unchanged in-frame full-counter run
`b6127120-093c-4360-a5f5-d51981c4fd72`, 347 checks/final27. Normal initial/restart
exits0 and refusal exits1 all have null cleanup errors. The original integrated
run `23ce9294-d959-4e3d-ae3d-01e7daea8059` fails at90 checks BEFORE the counter
phase: sampled lower SDK-write target was grass, but actual authoritative previous
state was dirt. Both SDK events/scope/sequence are recorded; exact transition1
refuses the stale expectation. It exits cleanly0. This does not establish a
transition-fix regression or a successful integrated counter run.

Preserve original composite counter1/quick0. The qualified parent helpers remain
unchanged. A separate stable-target subclass prepares only that lower cell as
inert stone, runs all parent assertions and restores the captured original named
state. It records its source/hash and setup/restoration separately, retaining
normal ticking and observations. Independent source review found dispatch-only
and stale-chat verification weaknesses; both were fixed BEFORE execution with
current coordinate-specific success feedback and exact resident-state no-op
verification. Three parent hashes are checked before import and session creation.
See H05_COUNTER_FIXTURE_STABILITY_PACKET. Wrapper SHA256
`b09390209a9f03f3597ddcd25f9a412d40cc93344ad1858b78994a76ce31c35e`
passes against the SAME game in both modes:

| Mode | Run | Checks | Final counter |
| --- | --- | --- | --- |
| Integrated | `5e9ecb59-3816-4bda-9257-fa6a3551777e` | 368 | 64 |
| In-frame | `326d5ed4-494a-4a77-808b-f241183b5615` | 353 | 19 |

The additional six checks concern only fixture setup/restoration. Both recorded
SDK cases prepare grass as stone, observe exact stone-to-glowstone transitions,
and verify current command feedback restoring grass. Setup/restore errors are null.
Normal initial/restart exits0 have null cleanup errors. Refusals exit1 preserve
saved bytes; only integrated changed-package cleanup races the already-exited
process with a missing control socket. Other refusal cleanup errors are null.
Separate stable statuses are integrated0/in-frame0; the original failed composite
remains unchanged. Final independent functional and wrapper receipt reviews find
no issues. They reconcile actual archived sources, control/results, isolated
execution guards, compiler/build provenance and the exact CI checkout. They do
not rerun candidates or independently rehash Linux-only executables. The no-op
restoration and simultaneous case/restore failure branches are source-reviewed,
not exercised by the two successful cases. No native failure is relabelled as
passing and no engine fix was made
to satisfy the stale fixture.

Raw records stay in workspace-root .local/h08a13-* and named game-run folders.
No engine candidate fixes have been needed after review or execution.

The repaired synchronous method is primarily the in-frame path. This is not
all-transition rollback, normal-error Game/SaveDir lifetime retention, asynchronous
unload acceptance, off-resident entity ownership, coherent recovery or G1. It does
not enable the new World slot service or establish live player transfers.
