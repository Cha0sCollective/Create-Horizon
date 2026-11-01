# Retained native writer close and repair retry

Status: FINAL ACCEPT, bounded local opt-in writer component on ab.
Current e4c/d855 build,77normal,21separatefatal,all5checks and postcommit broad PASS.
Different s1_writer_final accepts clean local3fe2a2e. No ordinary Game caller/adoption/G1.
S1/G1 remain open; no ordinary caller, startup or publication.

Worktree .local/worktrees/s1-native-writer-close, branchcodex/s1-native-writer-close,
baseFINALab239890cd62595889f7f0302cd1396a5e2c547c/treecf2c3c55/parent173ad2a.
Only crates/horizon-client/src/save.rs changes. Initial source72da7bc8a2a384423c403d377534d905121bf3d639d12d06731b6c003a76ff24,
manifestc3d4095afd5634e363dc8aba350c4c7ee107f1560fc26d9d42da14ac82c0532c
have exact independent source clearance after Root formatting. Design8ecc6f48 and
independent-sourceamendment017ca420 had concept and fresh ab/API acceptance;
verified-privatee99c3aa preceded one-file source authoring. Bounded author
s1_c0a_packet_design, source/tool reviewer s1wr_source_review, different final
reviewer s1wr_final_review. Root alone formats/runs/commits serially.

## Intended bounded service

The opt-in retained constructor installs shared write admission before the first
native batch. Close seals every Arc alias, retains actual writer handles through
EOF/drain, polls without waiting on locks or filesystem work, and joins only
finished handles. A non-Clone completion owner stays bound to the same SaveDir.
Retained Open SaveDir or unresolved close-owner destruction aborts BEFORE native
handle fields can detach; this is an unexpected-loss backstop, not normal retry.
Legacy constructor, queue/flush and Drop remain their separately preserved path.

Original native Records for Region, Entities and Poi correlate with actual accepted
batches/latest sequences, pending ownership and native durable receipts. Successful
writes release payload owners. Worker-side bidirectional verification detects
missing/uncorrelated records; unexplained counts, poison and error mismatches stay
owned Blocked. Explicit retry uses the same original latest records and real native
IO/strict POI processing, refreshes selected cache entries while retaining poisoned
inner file owners, and checks a typed current error witness before advance or
retirement. A later genuine failure cannot mask an unknown previous cell/witness.
No new codec, journal, general executor or World/Game close capability is claimed.

## Qualification inventory and first-source results

Reviewed tool7987340b and final expectedc2e1d55a select19 normal/resource retained
cases (16normalLinuxarbiter+3resource), the complete legacy save tests including
all eight pinned resource cases, and21 SEPARATE fatal native processes. Each fatal
case must complete genuine assertions, flush its distinct marker, abort with
SIGABRT and be reaped; a generic panic is insufficient. Process-local core limits
avoid dumps without changing host settings. No Rust test spawns another test,
compiler or Java process. Children get fresh isolated homes and data; target-s1wr1
is dedicated to this physical worktree. Initial actual opt0/debug/overflow-checking
build PASS: libcb7041ef7d5793544e5ae2983db91a10aa419a8d9aa84e057250068fe3707449,
bin95a8e9dba7355579f199aa9c2d72f58fb07c45a1369485a416f13b102e84ffbb.
InitialnormalruntimePASS:19newnormal/resource+58legacy save tests, all8legacy
resourcesexecuted, zeroignored. All21fatalprocessproofsPASS on72da; checks blocked on five style issues. Broad/FINALpending.

Source review blocked88ff for blanket-cache poison waiver, one-way missing-record
reconciliation and missing repair controls;04f for uncorrelated sticky-error
retirement. Exact source/prefix snapshots precede every correction. Corrected9966
passed source review; final72da differs only mechanically by Root formatting.
Added cases prove surviving claims/empty POI sections, genuine negative cache,
retained-cache rereads/unrelated entry preservation, partial outer Region acceptance
then Poi refusal, payload release and actual unknown-error races. Static review
and test definitions are not runtime evidence.

Local qualification is limited to APIs available at ab. Genuine D Game item
save/restore/recapture cases are ABSENT there and remain MANDATORY in later separately
reviewed adoption into the accepted B/D foundation. A local writer FINAL will not
claim that composition or ordinary retained close. Postcommit broad must disclose
the unresolved ancestor Halloween fixture and any process-local calendar condition;
no parity, optimized runtime, performance or G1 acceptance follows.

## Current correction and evidence boundary

Source72da/c3d passed77normal and21distinct marked SIGABRT/reaped processes.
Checks then found five Clippy style issues, without a runtime failure. Preserved
all source/prefix before correcting four let chains and one test slice conversion.
Prefmt513b received SOURCE CLEAR; Root formatting changed only one test call.
Current save.rs e4c2ecec1305252449435be62cfeeef8c44d9b2a6fbc49c35d47eaf1c2b2d21b,
manifestd8556049598151c0273ac5c18cc4d5730e17d8f51659f5f4312e4da23ba6b359,
expectedfcd6a067/tool7987 independently SOURCE/TOOL CLEAR. Write/join evaluation
order and all selectors unchanged. Fresh build/77normal/21fatal/checks remain
required; previous passes are not current-source passes. Original runtime artifacts
are preserved together under .local/s1wr1-source72da-runtime and attempt snapshots.

## Owner-requested pause

Paused after the separate registered225test battery finished. No fresh writer
build/test/check battery started. Current e4c/d855 remains SOURCE/TOOL CLEAR;
different s1wr_final_review also found no blocker in exact five mechanical fixes
and formatting, with FINAL HOLD. Allcurrent source+prefix frozen in
.local/s1wr1-attempt18; old72da runtime archive remains separately intact.
Resume ALLfour current stages before commit/broad/FINAL. No current-source
runtime pass, adopted Game close or G1 is claimed.

## November1 resumed fresh current qualification

Current e4c/d855 passes fresh opt0/debug/overflow build (lib3874570c556f428e),
77normal (19retained+58legacy, all8legacyresource/zeroignored) and21separate
fixed-marker/SIGABRT/reaped fatal cases. Receipts6a78c645/52fcb670. All5checks
PASS (fmt, lib/bin/test Clippy runtimeon/runtimeabsent/featureoff, publictree),
receipt6cba380f. All actual log hashes verified. No old72da result substituted.
Normal attributed/public-tree-hook local commit3fe2a2e8a85bc23e60ea53ebf74953a843cecc0f,
treeceafda333c6c39f68cb69a6240acf419f91d73ff, soleparentab239890. Clean worktree.
Snapshotattempt19 preserves complete precommit source/runtime. ALL7987dirty-parent
modes RETIRED after3fe; only reviewed broad376c may execute next on that clean pin.
Different FINAL pending. No adopted retained Game close/D regression/G1 follows.

## Independent FINAL acceptance, November1

Different s1_writer_final FINAL ACCEPT on exact clean3fe2a2e/treeceafda33/parentab.
Frozeninput972cc7e8f4c4eaf9300e1ebc7cb016450af61ace1f5223edf024ec48bdcf49a2,
summaryd1271b9772d47cea8e1de2ee18c8200fd0d1d545a0b16c4d9089b2af6098c03a;
all58indexedhashes verified. Native/Git/Linux sourceIDENTICAL before/after,0CRLF.
Actual postcommit broad exit0:369Python/1375client/doc0/fmt/clippy/publictree PASS.
Main logd670a9e5/status01d82637/comparisona1fa7b0b. ExplicitNON-HALLOWEEN
TZPacific/Kiritimati2026-11-02T06:08:42+1400 to06:28:35+1400; unchangedancestor
Halloween defect UNRESOLVED. Scopewarning2842linesdisclosed. NoParity/optimized
runtime/performance/G1. Review nativeREADONLY, no reviewerexecution.
This FINAL covers onlyoptinwriter onab; laterregisteredB/Dadoption MUSTfreshly
qualify actualGameitemcases. Legacyconstructor unchanged; nofullGameclose/lock
release/generaljournal/portablecodec/publication. EarlierWIP/pause sections arehistory.
