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
current registered source manifest is 7502658a208dce433386d056818796df0b0605cd9c4e658a1b0584998c4c75af.
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

## Accepted concrete input pins, November1

Registered R FINAL f6a3c6a06f500234ed1b3140c8d7cdd37bb89e6f,
tree666bb0297cf79d046bc29fc079b1279179ce6b9e, soleparentC1fa55c619a2f4666a505461822a97efa5732db01.
Source7502658a; FINALinputc829e36c/summarydaf1af6f/all96indexedhashes,
different s1_registered_final. Focused216old1f6+9fresh7502 explicitlymixed;
all6currentchecks and fresh369Python/1379client/doc0broadPASS.

Writer W FINAL3fe2a2e8a85bc23e60ea53ebf74953a843cecc0f,
treeceafda333c6c39f68cb69a6240acf419f91d73ff, soleparentab239890cd62595889f7f0302cd1396a5e2c547c.
Sourced8556049/currentrawe4c2ecec; FINALinput972cc7e8/summaryd1271b97/58hashes,
different s1_writer_final. Fresh77normal/21fatal/5checks and369Python/1375client/doc0broadPASS.
Bothlocalcomponents retain their explicitcalendar/noParity/performance/G1limits.

Rootfreshreadonly check RsaveGitblob59346116454a442afac14c7db62b15b7f26f841b
EXACTLYequalsab; WdeltaONLYsave.rs. C17 unionR7 plusW1 is23canonicalsourcepaths.
Thisrefreshes acceptedconditionalconcept183946eb inputpins only. Require fresh
independent actualpin/API/delta acceptance and privatecheckpoint BEFOREcomposition.
Childdelta remainsONEsave.rs; no ordinarycaller orsourceunionpermissionexpansion.

### Full immutable SHA256 input references

Rsource manifest7502658a208dce433386d056818796df0b0605cd9c4e658a1b0584998c4c75af.
RFINALinputc829e36cde6a6ddc282d0aeede0a381876d5c8c828c84322aa262ebe8d2f49ea.
Rsummarydaf1af6f9b0db428c90fa6ab79d27df5d1b3b14a380e475d20e960fec7751db1.
RFINALreceipt832131aab825dfb2bb5c401b1256dc205b7257a9176dcbf6bdb04015aed13bcd.
Wsource manifestd8556049598151c0273ac5c18cc4d5730e17d8f51659f5f4312e4da23ba6b359.
WFINALinput972cc7e8f4c4eaf9300e1ebc7cb016450af61ace1f5223edf024ec48bdcf49a2.
Wsummaryd1271b9772d47cea8e1de2ee18c8200fd0d1d545a0b16c4d9089b2af6098c03a.
WFINALreceiptcb293d5bf17c4f6ea2d279e39e268007f2783dc766857f78e3adc13d48fb02d0.
These fill the explicitfullpin requirement without changing the acceptedee09
input/API refinement, onefilechilddelta or validation/source-start gates.
