# Retained native writer close and repair retry

Status: WIP. Formatted source independently CLEAR; compilation and runtime have
not started. Different final reviewer reports preliminary no new blocker only.
S1/G1 remain open; no ordinary caller, startup or publication.

Worktree .local/worktrees/s1-native-writer-close, branchcodex/s1-native-writer-close,
baseFINALab239890cd62595889f7f0302cd1396a5e2c547c/treecf2c3c55/parent173ad2a.
Only crates/horizon-client/src/save.rs changes. Raw72da7bc8a2a384423c403d377534d905121bf3d639d12d06731b6c003a76ff24,
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

## Qualification inventory; not yet executed

Reviewed tool7987340b and final expectedc2e1d55a select19 normal/resource retained
cases (16normalLinuxarbiter+3resource), the complete legacy save tests including
all eight pinned resource cases, and21 SEPARATE fatal native processes. Each fatal
case must complete genuine assertions, flush its distinct marker, abort with
SIGABRT and be reaped; a generic panic is insufficient. Process-local core limits
avoid dumps without changing host settings. No Rust test spawns another test,
compiler or Java process. Children get fresh isolated homes and data; target-s1wr1
is dedicated to this physical worktree. Actual compile/results remain pending.

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
