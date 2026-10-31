# Retained writer adoption on the registered foundation

Status: DESIGN PROPOSAL ONLY. No source assignment or composition authority.
S1/G1 remain open. Engine changes remain local and unpublished.

The retained native writer was deliberately implemented independently on accepted
retirement parent ab239890cd62595889f7f0302cd1396a5e2c547c. Its local tests cannot
claim the dropped-item or registered Game checkpoints absent from that parent.
This packet closes that integration gap before any C0a producer-close work.

## Inputs and start gate

Require BOTH clean, independently FINAL component commits: registered binding R
with sole parent accepted combined C1fa55c619a2f4666a505461822a97efa5732db01, and
writer W with sole parent ab239890cd62595889f7f0302cd1396a5e2c547c. Record their
full commit/tree/source-manifest/final-input/final-review pins in this document
before execution. No candidate hash is implied by the current dirty worktrees.
Current reviewed writer source is e4c2ecec1305252449435be62cfeeef8c44d9b2a6fbc49c35d47eaf1c2b2d21b;
current registered source manifest is 1f6e020ec52feee3448db229a8c9d5c7dc8b5ebce5131eb324ce15e79ce70caa.
These identify design inputs only and must be refreshed if either source changes.

Before composition require a fresh independent Sol High check of actual accepted
pins, disjoint source deltas, and API/effect compatibility; then a durable private
checkpoint. Preserve all resulting tracked-source inventory and packet-prefix
artifacts before each correction. A failing assumption means HOLD, preserve, amend
and review rather than adding an unreviewed fix.

## Exact change

Create an ignored clean engine worktree on R. Apply ONLY W's exact delta from ab
to crates/horizon-client/src/save.rs, staged without committing. This is a
one-file child delta, with no semantic edits and no new callers. Verify that R's
save.rs Git blob is exactly ab's, that W changes only that file, and that R's
seven binding paths remain byte-identical. C17 and R7 overlap only at the two
session_checkpoint implementation/test paths; R adds five distinct paths:
world/counter_restore.rs, world/slot_restore.rs, world/slot_restore_tests.rs,
game/counter_control.rs and game/levels/session_checkpoint_registered_tests.rs.
Their union is22; writer save.rs adds one, giving23 distinct source paths.
Only the one-file writer delta is open for composition. Produce an exact set
union and raw native/canonical Git union proof;
any CRLF conversion must be exact, disclosed and stable before/after execution.

Independent SOURCE CLEAR covers the exact staged source and effects before any
compile or runtime. Do not rerun old dirty-parent drivers after their component
commits; use a newly reviewed adoption driver with exact parent/scope/source guards,
fresh artifact names, isolated data and a target unique to this physical worktree.

## Required validation

Build actual lib, game-bin and protocol tests with the pinned toolchain and
mods,paired-packages,native-writer-test-control where applicable. Run all 225
registered/combined cases freshly on those artifacts, including every actual D
checkpoint/drop-custody case and all nine actual registered cases. Also run the
nineteen retained-writer normal/resource cases and complete legacy save suite
(currently58, including all eight mandatory resource cases). Run all21 fatal
writer fixtures as separately supervised prebuilt native processes: exact distinct
marker, SIGABRT, reaping, and process-local core limits. No Rust test launches a
compiler, Java or another test process. Old component receipts do not substitute
for these adoption executions.

Require formatting, client lib/bin/test-target Clippy with runtime on, runtime
absent and features off, protocol test-target Clippy, and public-tree check. All
runtime/data directories are disposable, inherited controls sanitized, and all
execution/Git writes remain Root-only and serial. Preserve failures before fixes.

After focused/check success make a normal attributed clean local commit with sole
parent R. Retire the staged driver. Run the applicable broader upstream self-check
on the exact clean commit in the separate isolated CI checkout/target; retain exact
source comparisons. Disclose the existing unchanged Halloween fixture and any
process-local non-Halloween condition; no parity or optimized runtime claim follows.
A different independent FINAL reviewer accepts the complete frozen evidence.

## Stop condition and scope limits

Accept only the exact union, fresh current-source results, clean local commit and
different-reviewer FINAL. This establishes writer adoption on registered B/D APIs.
It does not switch ordinary callers to the retained constructor, finish Game close,
activate guest callbacks, configure clients, pass G1, or authorize publication.
Only after adoption FINAL, fresh actual API review and another private checkpoint
may the separately accepted C0a bounded source assignment begin.
