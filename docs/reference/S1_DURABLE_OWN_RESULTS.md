# S1 durable OWN and unused history: frozen component result

November 4, 2026. Component s1own1 only. S1/G1, deliverable1/C0/fullC1 and ordinary
activation remain OPEN. Final/privacy and public-sync decisions are recorded
separately; this report does not authorize a PR, merge or release.

Candidate codex/s1-durable-owned-history:
fbd4046b932eef52de3977fd87691745d1afa231,
treebd3a439ad6c0fc1810e5dc5ce20222fd5f769fe3,
parent0db9427b37d34fb6277e794136cf11a4265ab1c4.
Exactly 6 existing source paths: 2,879 additions and 301 removals, within the accepted 8-path budget.
game.rs and paired_close.rs were reused unchanged. Normal commit hook passed.
All 8 raw tested files equal committed blobs, comparison receipt
.local/s1own1-commit-comparison.json SHA256
3c6fa6c121123568a8fb68e6dec64c64032db6ed69de5955161759f526df65f6.

## Behavior established

An explicit trusted ordinary NEW request establishes immutable OWN only through
its actual native identity/session, namespace lock, original preparation and unused
World grant. The bounded record is outside the native image and binds original
anchor/logical UUID, fixed mob-free Creative TestTerrain Overworld, role/native
player mapping, exact existing executable/package PIN, captured Guest binding,
canonical archive and recovery destination parent. File, key-directory and namespace
parent synchronization precede editable exposure. Refusals retain original custody.

The constructor derives one fresh archive child from its actual logical UUID under
a configured canonical parent. Different new UUIDs cannot share an archive even
before first publication. OWN configuration roots conservatively exclude logical
UUID path components; only internal key/archive children add UUIDs. This prevents
nested archive/native-root configuration without a registry. Strict reads/rechecks
preserve canonical/no-link routes, actual namespace/native exclusion and that layout.
The original anchor remains selection/exclusion identity; recovery uses fresh working
destinations and the SAME stable archive. Old process-owned routes remain strict.

Unplaced is a separate closed complete-image kind with no target/counter/slot or
allocation heads. It carries the complete native world, actual players/progress/
stats/items, saved columns, clocks and other supported native owners. Genuine live
unused admission and no domain/trial/slot/history/physical alias are required; every
saved column is checked. Claimed keeps existing GuestV3 and slot formats, independent
heads and retirement semantics. A dormant claimed world cannot become Unplaced.

Fresh owned recovery acquires actual anchor/namespace custody and loads current
resource/assets/tree/package/startup owners; it reconstructs no old Arcs, completion,
NativeReopenClaim or source Game. Newest complete image is authoritative. Complete
native/player/item state is restored together; failed/interrupted/newer working
files remain separate. Copy/read/fence/install reuse original ordinary preparation
and unpublished constructor custody. Only after full restored-state comparison can
an Unplaced image issue one NEW admission bound to that actual World and session.
The old grant is neither cloned nor decoded; existing-UUID for_new_host_world is not
used. Original constructor publication remains last. Cancellation/Retry retains
actual workers, native locks and preparation.

## Focused observations

Tests2: both tests PASS, receipt .local/s1own1-tests-2-result.json SHA256
393f888b274d17e81e980d9cc2f0e69538ff4bccb471199289d8db0589fb0e06.

1. game::levels::session_checkpoint::tests::registered::local_tests::owned_durable_processes_restore_unplaced_and_claimed
   PASS, 179.91 s. Parent launches exactly TWO serial OS child processes using the SAME
   exact built test ELF. A (PID 389) ran for 66.01 s; B (PID 794) ran for 82.75 s, separately loaded actual
   resources/packages/tree/startup; both child receipts record the exact same hash.
   A constructs both actual LOCAL worlds before either first close under one shared
   parent. Distinct UUID archives and nested-parent/native-root refusal occur before
   effects. Actual namespace contention, zero-complete-epoch refusal/preservation,
   live no-domain physical-alias rejection and original failed-copy/Retry are covered.
   A publishes genuine Unplaced and Claimed epochs 2, including a real Wasm tick and
   rich inventory/ender/slot contents. A also closes a genuine newer claimed working
   destination with inventory count 2, and leaves an authored interrupted pending
   candidate. The pending candidate is a test control; real FailCopied/Retry covers
   the original publication path separately.

   B exercises actual owned recovery refusal/checked closure for unknown association
   fields, role, native UUID and PIN digest, retaining negative input bytes. B
   restores Unplaced, refuses wrong-World/duplicate admission, uses the actual first
   click exactly once, and refuses spent unused admission/second target. Claimed
   restores counter and Guest-slot bytes exactly from the complete image, including
   inventory count 4 rather than the newer working count 2, slot 3 and ender 7 with rich
   components. No unused grant exists for claimed recovery. B closes BOTH into their
   SAME bound archives, producing strictly newer complete epochs 3/4, keeps all
   pending predecessors byte-identical, selects/verifies and activates those complete
   successors, preserves both observed heads, and rejects prior runtime counter/slot
   keys. Independent-head validation beyond these observed heads reuses unchanged
   existing Guest/frame coverage; this is not a new unequal-head fixture.

   Log .local/s1own1-test-0-2.log SHA256
   ed45c06959d83e8ecddc6a99f1e603322c1acf575e5c9d28a79711a37817a52e.
2. game::levels::session_checkpoint::tests::registered::local_tests::ordinary_local_actual_body_coherent_retry_recovery_and_cancel
   PASS, 56.45 s. Existing original LOCAL body/record, failed-copy Retry, original
   unpublished cancellation, full recovered rich state and continuation-close
   regression. Repeated because the common native recovery/projection was factored.
   Log .local/s1own1-test-1-2.log SHA256
   fcceb32d8330a7b5bc40d28c9ec1d98c460f6e2681e330a27526872f9bbf67c8.

No historical full battery was repeated. Existing unchanged strict newest-complete/
corrupt-successor, claimed frame and network evidence remains attributed to its
original component, not claimed as fresh execution here. Current core acceptance is
LOCAL; default Memory/dedicated two-client fresh-process ordinary dispatch remains.

## Build, source and tool attribution

Final source6 map .local/s1own1-source-sha256.json SHA256
01ca778203d3beeb5abd03864b0bd90253eed6e95b5f74e71fa907b537a1d70b.
Raw source1..6 copies/maps remain .local/s1own1/. Substantive source3 and test-only
source5/6 deltas SOURCE CLEAR by independent read-only s1_own_source_review. Source6
receipt705ee44aacd005e5d2ac94bc42e010d14a6c51ca782fadf04bf8afe406361eb6.

Pinned Rust1.99.1 offline/jobs2, existing warm target-s1nc1. Formatting1 PASS0.91s
produced source4. Staticchecks1 source4 PASSfmt0.84/Clippy57.95/features-off16.06/
paired-noMods38.70. Source5/6 change tests only; unchanged production retains that
feature coverage. Optional warnings match frozen previous headings/counts.
Final static3 source6 PASSfmt0.85 and supported all-target Clippy(-Dwarnings)24.02s,
receipt9e35ce579b180504186488720e7e3c30bc4738648412e5d7f4e7a55e19973b6d.

Build2 PASS29.19s, receiptf051a0d6242d7aa93893056acb6cdaf6dc33c2c02babe70c2a3393ac6cf1142e.
Test bin269a8abcc7d9f64de0933134af44cd5a9596c588c82ab2ab219c89d7d6ddcde9;
lib92ffe646ec76a43e5753a2c41efbc27a0d73f358b8b73fac6ab76e483174ffa7.
Frozen28176 resource files manifestcc72b7c2ed9e343b987c759941bd66559a25dda1c47c976047737f9fa06e297d;
genuine pair receipt32c83f6b1fbffe44696f42dd3617b7a6e6836d18a46303f0d21189dcacb940b4.

Reviewed compiler .local/s1own1-compile.py SHA256
e0ea6321f36fe3d54f78f3b56da201dd9c3a4ba1e8c723c8d6aa0e60ce7c0b72;
qualifier .local/s1own1-qualify.py SHA256
93eb6b56aeee48f22dbde12641c1e0a52472f7ee2effb4a63394977063790a58;
expected2selectors16c8ff39030c966adb1ab65f5a5ab7573f886138848e6c60c461ed731edcb5cd.
Different read-only s1_own_tool_review TOOL CLEAR, receipt
e27f4c99d0521cd4a693903f3aaffb299b9a297b3a144bdd1657156d59b18165.
Parent/branch/source/artifact/environment/input guards, isolated homes, timeout
process-group containment and fail-closed native evidence retention remain active.
Compiler modesformat/checks/test-checks and qualifier build/tests only were authorized;
no game build/launch under this packet. These parent-pinned tools are now RETIRED
at candidate commit; do not run them on the new HEAD.

## Preserved refusals and failed run

Source1 HOLD123254bb: two new UUIDs could bind one still-empty archive. Accepted
archive amendmentb6f8c919/privatecheckpointde498e7/gate1b4fbadf corrected this.
Source2 HOLD8277fc5b: a second archive parent could nest inside the first UUID archive.
Accepted route amendment97ffccf0/privatecheckpoint8072e5e/gatec82b77e1 corrected this.
Original designfbf5ca4f, baseline6bba16ff/pathlist6ff93361 and privatecheckpoint455f9719/
sourcegate3e256677 preceded initial edits. No path/profile/framework expansion.

Tests1 source5 is permanently FALSE156.36s/exit101. A passed; B passed Unplaced
successor and actual claimed activation/counter byte comparison, then fixture called
native-only slot_checkpoint_bytes for Guest ownership. Correct BindingMismatch
triggered its panic/retainedStreaming abort. Existing LOCAL regression did not run.
Source6 ONLY corrects the fixture to the existing Guest-aware SessionCounter encoder;
production is unchanged from formatted source4. No failure was reclassified as pass.
False receiptdbd657768001ef1bbf3cb88b9ac08d99df27b931b2a623d1a788acce73929bca;
loge5c7f915f3a4f376950488054ef185a908ba1b46c60ea271ae55190990d0ebad.

Both failed native case trees/shared archives/negative inputs remain under
.local/s1own1/failed-worlds-1/s1own1-tests-0-f7g8e7tl/horizon-owned-restart-388.
Successful native evidence: 12 directories (two durable case trees plus ten existing
LOCAL native/archive/authority directories), A/B receipts and shared archives remain
.local/s1own1/failed-worlds-2/. The historical failed-worlds label does NOT mean tests2
failed. All owned compiler/test processes finished/reaped; no cleanup occurred.
Materialized resource links/homes were excluded, not traversed into the evidence.

## Limits and following work

This is the accepted fixed mob-free Creative TestTerrain Overworld and one captured
RegisteredCube Guest/type/schema1 only. Zero complete epochs refuse and preserve;
there is no reconstruction of unsaved pre-first-checkpoint play or live initial
autosave. Archive remains fewer than 256 entries, image 64 MiB; exhaustion refuses.
Working/failed copies remain; no retention daemon or cleanup. Exact executable PIN
means this establishes same-build recovery, not migration across changed binaries.

Ordinary NEW/menu/CLI dispatch, responsive in-frame recovery, network Unplaced close,
actual empty counter activation, one-slot/departure/fault custody, replication/H11
integration and combined G1 remain explicit following work. Open to LAN stays
refused/deferred. No ordinary positive ingress, arbitrary existing worlds/generators,
new entity/menu/schema framework, S2/S3 gameplay, asset redistribution or release is
established. The separate follow-up caller map is a proposal, not source authority.
