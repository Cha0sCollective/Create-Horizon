# Connected recovery results

Status: FINAL ACCEPTED (bounded connected-recovery packet).
S1/G1 remains OPEN. Owner-authorized work continues through subsequent packets.

## Exact candidate and scope

Local engine commit c95ba7ec7ccdaba06e4a23cea5fdbb4e4ddeb076, tree
8cd9d431f8a07c06e42810a339f809371acece62, sole parent
23e09dd1f0ec3c2a0a69e6e002ddab4a92736b11; codex/s1-connected-recovery.
Eight files,3836 insertions/32 deletions; clean native worktree, normal .githooks
Public tree check PASS. Native Windows Git committed; Linux identity/hook attempts
failed before commit. No hook bypass or source change was used. Size is a local
bounded packet, not public PR/release approval. Engine has NOT been pushed/PR'd/
merged/released. Private documentation checkpoints are separate.

Raw-source manifest1dffa36fc51b3ccccdf3775eb2aa51ffc37ef2bf414850e1883de12dc78797bd
is independently SOURCE CLEAR. Accepted design inputb2adefe5 and native-close delta
inputd2e9f38d. All source/prefix evidence before corrections remains in attempts0-16.

| Path under engine | Tested raw SHA256 |
|---|---|
| crates/horizon-client/src/game/lan.rs | d3d289824647e29e0d6b71e4aa36716eb7f945bc0a9626787ed70f08585989a7 |
| crates/horizon-client/src/game/lan/connected_recovery.rs | 7fc425da416f32857cb92232e11a6b04f33f67af338c6e9dd78b6e6fb9ddcf94 |
| crates/horizon-client/src/game/lan/connected_recovery_tests.rs | 546a6067633f04ab00e38d7a0f7dafa30c1bafe1e96b6494fb82fd4d25897a72 |
| crates/horizon-client/src/game/levels/session_checkpoint.rs | 66acb0dc2ba7e26a26e13d8aad4da0e3b38293a74f618703e1365b8b63aa360d |
| crates/horizon-client/src/lan/mod.rs | 2647b5d1bbaab1edae4723779d116102daef6361dd6d7cb70397277607439a88 |
| crates/horizon-client/src/lan/conn.rs | dc4b062cbf810272f68ca0d970779a240fd31fdeae87f5018e3ff27ed4104b05 |
| crates/horizon-client/src/lan/hub.rs | 9fb536ca3591185f207009876f00cb82f131341d94796d7daae4857ea9788135 |
| crates/horizon-protocol/src/connection.rs | b6e06cdd064e20ee2af31896ee0b107fe31ccaf51e058fefc71fefc017293345 |

Seven native files are byte-identical to committed/Linux files. Protocol connection.rs
has558 CRLFs; its exact CRLF-to-LF replacement equals the Git/Linux blob. Both byte
identities are compared before and after broad validation; no other difference allowed.

## Behavior qualified by current runtime evidence

A synchronous owner acquires managed native/UUID authority before image selection,
restores verified native world/counter/empty slot, and offers real memory connections.
Genuine fixed offline MemoryOwner/MemoryGuest joins precede sealed row installation,
ordinary placement/terrain encoding and actual client startup acknowledgements.
No fabricated Guest or loaded/placed bit, ordinary recovery tick or Game handoff.

Actual source profiles are Survival0, owner commands enabled, spawn_mobs disabled
BEFORE admission/ticks/positive owners. Real commands give stone3 and stone5;
server and respective client inventories are observed before capture. Source native
close precedes recovery. Recovery checks actual positions/dimensions/lit home/loaded
acknowledgement, player/body rows, clocks, counter/slot bytes, and actual live receipt
keys. ReadyFrozen repeats observations without simulation advancement. Publication
failures retain those same owners and locks; stale close refuses, explicit retry
uses a fresh attempt/epoch. Confirmed continuation plus live close gate precedes
transport close/all tracked reader-writer joins, retirement and authority release.

Before-Ready failures finish native cleanup before dropping the unpublished candidate;
source archive stays unchanged. After-Ready departure retains Guest AND GuestPlay,
actual state/keys/leases and refuses close. Normal departure/Running remains separate.

## Passing normal checks (not combined with exception children)

Build16498 froze debug bin7d83e91face868210fa05418145a9a92cc62c50d5f9bf279fba5b949aa3734fa,
lib2ee2057a92a1f4d08f17dfb4ea85e48147227331a50bfff7ed7886e3a936519a,
protocol964fa6025277f11e0145281d4d6823566aa5f025e122f6f33d165a1c1cd905f8,
under /home/arbiter/.cache/create-horizon/bin/s1cr1-tests-_yvujj1u.
Actual builderf74ca6e7, debug_assertions true; features mods/paired-packages/
native-writer-test-control, default features false. Pinned resource manifestf3514018,
prebuilt five-file pair receipt32c83f6b (retained older83649 guest build, no nested builds).

Full run24577 FINISHED0: **328 normal tests in24 groups** =203 ordinary bin +103 full
protocol +9 paired library accounting/native-write cases +13 actual-resource cases.
Ordinary156 ignored tests are NOT counted as passes. One complete current-suite
invocation after profile correction; focused successful0/7 runs remain distinct.
Receipt4bd817308da682110fef4e8137f0dab6b83cd71247d5976f421c95fa781c6829.
Resource matrix: success, duplicate/wrong/missing profiles, actual Joined-before-install
and pre-Join disconnects, withheld Loaded then departure, wrong teleport, prohibited
packet, inbound overflow, Copied/Promoted retention/retry, three prejoined attempts
including a partial native frame. Names/counts/exits/log hashes/source/binary/tool
identities and isolated child contexts are checked; zero counts/timeouts do not pass.

Three ADDITIONAL existing wrapper component regressions pass with the same bin/source:
- Legacy0 managed recovery/positive rows/filled slot14 items/copy exclusion/failure
  lock release,85.19s; receipt583b5f3465bedc519d6fc7733d005cdee26af9daf28d0b8774eeb03802d7a9ae.
- Legacy1 direct zero-roster dedicated/inert cache/orphan refusal,40.66s;
  receiptf3873563c2bda31ccb6f549946dafe0aae36f7aa5bf12364a6d2701b243a7748.
- Legacy2 four invalid roster/storage mappings/no partial Game/native lock release,
  76.45s; receipt246a9dbaa97d63de19f399664aebcf89137a01e79ef64152fee67118f5b6eb4a.
These old component fixtures add no genuine connected-client claim; their own scratch
cleanup behavior is unchanged. Regression tools03d98cbd/64ea2682 independently clear.

Refreshed27795 FINISHED0: fmt, three feature Clippies and public tree PASS. Optimized
fast profile TYPECHECK ONLY PASS: opt3/debug_assertions false/tests_executed false,
log758cd2d3f03c32e6c27f992920077083c7ef3a618281ed69be003cd79db18ebb,
receipt59deae7bc293f7a4116a5cf88ea96423392288d706b84779b8bf7acabbc5a584.
No optimized runtime or broad default-stack claim.

## Separate isolated exception proofs

All use the same current source/bin, explicit child flags, isolated cwd/home/data/
config/cache/temp/game/launcher paths, core limit0, exact signals and reaped processes.
They are NOT libtest/normal passing cases or ordinary shutdown/crash-durability proof.

| Case | Actual result | Seconds | Receipt SHA256 |
|---|---|---:|---|
| LAN unresolved owner | SIGABRT(-6), exact guard | 4.83 | 4a92fca4cbdab72ee15a45af0d3fdfd3b67c9bd5f093b9e5e082abf1a0569334 |
| Actual native writer unresolved owner | SIGABRT(-6), exact guard | 27.39 | be81dea674b1a7038e5c95ce2df708ff29aa40586936c18e308a816ba40dea38 |
| Actual recovered ReadyFrozen unwind | SIGABRT(-6), exact guard/source epoch1 full member inventory | 92.89 | 28fde246cdd88db47b0ebb13854df1a6a3bfe9a5fe9516069070d3749e374eb6 |
| Late departure retained |10 actual observations/players2/GuestPlay2/rows/core/keys/clocks/leases/close refusal, then explicit owned-fixture SIGTERM(-15), source epoch1 full members |89.54| af81ad5b7a1c06e459ac425fdf7cf033010e047483a07851a28b0bac2f79582f |

First three use orchestrator873332d8. Failed late19640 parsed a partial stderr JSON
line; except path killed/reaped its fixture and recorded verified:false. All8/prefix
preservedattempt15. Independently cleared9f7d3094 requires a complete newline record;
late46250 alone reran and passed. Earlier completed proofs retain their own tool
identities. Parent never treats timeout, ordinary failure or missing marker as proof.

## Preserved failures and material limits

Earlier compiler failures24188/26887/94170: deprecated atomic/MSRV conflict then native
image partial move/private fixture helper. Reviewed CAS preserves Rust1.90 floor;
NativeRecoveryImage retains every final correlation. No lint waiver or skipped check.
Source failures36437/20435 occurred before recovery: real Creative guest acquired
native reach modifiers rejected by strict capture. Diagnostic-only change exposed
the error; Survival selected before ownership. No attributes/state were cleared or
checks weakened. Those are failed resource attempts, not expected-fatal qualification.
Creative-derived attribute preservation/client synchronization remains required S1 work.

Staged limits cover64 mailbox events/256KiB payload and16MiB queued-plus-active
outbound bodies, at most3 tracked attempts. Underlying memory pipe remains unbounded;
no total-memory or uniform close-latency promise. Typed post-copy/post-rename faults
are not OS IO/power-loss durability. Exclusion concerns cooperating managed entries
sharing one trusted namespace. Source-fixture ordinary LAN outer readers remain
untracked: explicit client session disconnect and native online0 are observed;
only staged recovery claims all tracked reader/writer joins.

Current scope does NOT qualify ordinary UI/lifecycle, gameplay/filled ingress,
Creative/sprint/equipment/effect state, offline departure/replacement, TCP/auth,
live package agreement, machine replication, committed presentation or in-frame mode.
Remaining S1 outcomes and combined integrated/dedicated(two exact clients)/in-frame G1
are still required. Live S2 requires G1; no reduced S1/save-compatibility promise.

## Clean-commit broad result and native-calendar baseline defect

Original native-calendar93264 FINISHED1 at exact clean c95:369 Python/fmt/workspace
Clippy/docs0 PASS; client lib1172 passed/1 failed/51 ignored. Existing
vanillagen::structure_mobs::tests::ocean_ruin_drowned_match_vanilla observed
jack_o_lantern/shifted RNG on October31. Native StructureLevel reads LocalDate;
Halloween equipment consumes additional random draws. Both implicated
vanillagen/structure_mobs.rs and mobs/finalize.rs are UNCHANGED versus parent23.
Failure logb3f39037205ec56968422f43547a7a3ee15736e71b45fe69a85ee8f37d8c1d7a,
statusdbd842510007cb8a6a036b2c7910072076ea856779107f2739de07fe57a2a09c;
all8/prefix evidence preservedattempt17. This failure is not erased or called a pass.

Independently cleared calendar tool08ca21d2, run42650 FINISHED0, performed four
fresh-process exact-selector comparisons with identical resources/compiler/default+
mods(no-paired)debug profiles. Observed UTC/local date/TZ/offset before/after each;
class remained stable. Current/base Halloween assertion left/right vectors are
IDENTICAL. Both non-Halloween cases passed. These expected failures are not normal
passing-test counts. No source, machine clock or global timezone changed; clean c95
checkout restored. Receipt24fb4c3133f3c9169f8ff582195b678bca4d2b125da481e33c7687f26e20b951.

| Revision | UTC October31 | Honolulu October30 |
|---|---|---|
| Parent23e09dd | exact expected assertion FAIL101,7.71s | exact1 PASS0,7.58s |
| Candidatec95ba7e | exact expected assertion FAIL101,8.14s | exact1 PASS0,7.19s |

**NON-HALLOWEEN-CALENDAR-CONDITIONED broad66352 FINISHED0** at clean c95:
369 Python +1478 client/protocol tests PASS; docs0 failures, fmt/workspace Clippy/
public tree PASS, Cargo.lock unchanged. Process-only TZ=Pacific/Honolulu,
local2026-10-30T19:21:05-1000 through19:28:21-1000;
UTC2026-10-31T05:21:05Z through05:28:21Z. Native Halloween fixture qualification
remains UNRESOLVED known baseline; no native-calendar all-green claim.

Independently cleared conditioned wrapper2d7953ff requires and pins the four-cell
receipt, exact clean single-parent commit/8-path scope, non-Halloween dates before/
after, and separate output files. Tested raw/Git/Linux source comparisons match
before=after; native and clone remain clean at exact c95. One558-CRLF protocol file
normalizes exactly to Git/Linux LF; seven raw files identical.
Broad log39cbecf0df30b5c78fc1b9b672981b955e16a6a92280c4f8cfb0e5bead80c719;
comparison0243bd27452aa68bb1e203dd57888568b9f38550972d173653d8c8587cb27158.
Scope is workspace reverse-consumer COMPILE through Clippy, touched client/protocol
runtime/docs. --quick omits general parity. No consumer-runtime/default-stack/public
PR/release claim. The3836-line insertion size advisory is not publication approval.

Different final reviewer s1_connected_final_review found no material preflight
findings in source, normal/resource/component/exception and calendar proofs. FINAL ACCEPT was returned for exact authored input
7c8af21f980c21c83ee71bf0a8a7eb55982db3a2d86925fb4f594516ad600f01 and
committed source c95ba7ec/tree8cd9d431. No blocking findings. Do not mark S1/G1 complete from this bounded result.

After actual FINAL/private checkpoint, conditional six-path managed connected-worker
bridge design026d3415 starts at this clean successor. No Game escape or normal
Running/UI authority follows. Owner-authorized S1 work continues through the plan.
Dirty-parent23 s1cr1 guard/build/validate/test/exception/regression/fast scripts MUST
NOT EXECUTE after commit. Exact clean-commit broad tools are separate routes.

Independent final reviewer: s1_connected_final_review, native proceduralread-only.
FINAL ACCEPT input7c8af21f; exactenginec95/tree8cd9/raw1d,328normal+3component+
4exceptionproofs; optimizedTYPECHECKONLY;369Python/1478clientprotocol broadPASS
NON-HALLOWEEN-CALENDAR-CONDITIONED. NativeHalloweenbaselinefixturequalification
UNRESOLVED. S1/G1/publicationremainOPEN. Thisacceptance releases onlynextreviewed
workerbridge afterprivatecheckpoint/successorpin; ownerrequestcontinues.
