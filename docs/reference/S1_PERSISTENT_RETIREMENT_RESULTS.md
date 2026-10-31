# Persistent retirement and explicit readmission

Status: FINAL ACCEPT for the bounded persistent-retirement/readmission component.
Different independent s1pr_final_review accepted exactab with no blocking findings.
Frozen inputfbdc8978c71bfbded79d38aad75d797d11c85ab55c3ac45876241445c27aa198,
summaryad4fa5b09d66b32a5afa54eff9bc1591fd5d0a3561f3c02f2cfef68698b3e58f and
all25 indexed receipt/log hashes were independently verified. S1/G1 stay open;
no publication is enabled.

## Candidate and boundary

Worktree .local/worktrees/s1-persistent-retirement, branch
codex/s1-persistent-retirement, base173ad2a7b496b11b15e1492ad57c4ce4da9cf21d.
Clean LOCAL candidateab239890cd62595889f7f0302cd1396a5e2c547c,
treecf2c3c5578941319ca068fb5959052e034c4c9dd, soleparent173. Normal native
PublicTree commit hook passes; all six tested raw source hashes stay unchanged.
The accepted proposal input is09f7bca60bdea943dd871eb8bdd6735ac261743f378408cb5dafbb8ba8405c1e.
Window FINAL and verified-private checkpoint4573856 preceded authoring; the
independent design reviewer freshly checked that exact clean base and its APIs.

Exactly six source paths: world_slot_trial.rs, world/slot_restore.rs and its
tests, game/slot_retirement.rs, game/levels/session_checkpoint.rs and its tests.
Current raw manifest5139a3dff4bed20ea54185824f3f4943836c6e951e724798833e18a7f33e907d
has independent exact-source clearance before compilation. Root wrote the Game
half, bounded author s1pr_world_author the World half; Root formatted and ran
all validation serially. Different s1pr_final_review accepts the exact candidate
and completed evidence within the limits below.

Checked persistent take moves the original healthy BlockChanged pending owner
after native identity/configuration/physical residency/clock/head checks. It
preserves both generation heads and trial identity, sets healthy Dormant, and
does not admit implicitly. Explicit reconcile stages a fresh empty slot before
publication and is idempotent for a valid active slot. Later changed writes do
not automatically admit a persistent Dormant slot. Transient admission and raw
detach-take behavior remain unchanged. Keys select trusted native allocations;
they do not authenticate a player.

The Game receiver shares the existing authority/context/item/count/ID/reservation
preflight. Every fallible preparation precedes checked take; then it moves the
original stack into one native ItemEntity. Empty acknowledgment creates no item,
ID or RNG change. Checkpoint correlation admits only existing-format
BlockChanged counter history, retaining complete CounterFrameV2 validation and
envelope/package/world/clock/generation correlation. There is no new codec,
journal, ordinary startup, close or Running caller.

## Completed focused checks

Current build receipt e388017674dfc8c1466848734a98dedd338b413221ed0cb43bf4531ed28263e3
freezes actual opt0/debug-asserting/overflow-checking test executables. Bin SHA:
12edf504f5b73bb64f82574c198b59db777350beaf02caa5a8314cbcfd7f4a88.
The runtime driver is byte-identical reviewed1a83c4ea, retained under
s1pr1-qualify-runtime.py; CHECKS-only driverb073e040 adds test targets to lint.

All54 focused cases across8 groups PASS, receipt
8c02f05b90c21b714eaef25d9ba4ad317cb01a3b5af6d3bc30e5c2aa7d0a174e:

| Group | Cases | Actual boundary |
| --- | ---: | --- |
| Native slot restore/lifecycle |13| Includes five new tests, original nested payload address, independent heads, replacement order, refusals/exhaustion and matching streamer reload without catch-up |
| Slot trial |8| Existing transient allocation/ownership regression |
| Counter restore |16| Existing durable native counter regression |
| Slot transfer |8| Existing transaction regression |
| Actual Game legacy receivers |6| Includes malformed root/all modeled nested counts, context/ID/residency and frontend/remote refusals |
| Actual Game rich retirement |1| Both IntegratedServer and DedicatedServer constructor roles; genuine retained ItemEntity and sink failures |
| Actual Game empty retirement |1| Both roles, genuine checkpoint/restore/recapture and six resealed corrupt-history refusals per role |
| Existing actual Game two epochs |1| Both roster roles, variable residency shapes and body/item conservation |

The rich fixture keeps14 plain root items and transfers an additional whole rich
seven-item stack from an actual participant cell into the machine. Total root
count21 is conserved before removal, after delivery and after refused capture;
nested metadata/counts/runtime selections are compared explicitly. No rich
remainder is left in the player inventory and no delivered entity is cleared.
After explicit reconciliation, capture refuses its genuine ENTITY owner, as the
next dropped-custody component is not yet implemented. Pointer preservation is
proved in borrowed World tests; public Game diagnostic snapshots clone metadata.

The empty case retains slot1 while repeated physical writes reach machine4,
delivers empty without ID/RNG change, explicitly admits slot2/machine4, refuses
old slot/counter keys, completes a genuine save/recovery/recapture and preserves
independent heads/history. Resealed complete-image controls reject Explicit,
ChunkReplaced, nonpreceding/zero retired generation, future clock and malformed
retired counter payload. Matching World streamer reload keeps keys/content with
no catch-up; resident replacement and mismatched reload keep existing faults.

These are real headless Game constructors in both server roles, not standalone
external TCP/default-integrated-window/dedicated-two-client qualification.

## Preserved attempts and outstanding checks

Initial810 candidate passed51 native/legacy cases; its rich test reached delivery
but left unsupported rich player metadata, masking the intended ENTITY refusal.
That fixture failure is preserved. The reviewed f1 correction passed all54 cases.
Expanded test-target lint then found only four needless references; their reviewed
mechanical cleanup produces current5139, rebuilt and rerun to all54 PASS above.
All scoped sources/prefix artifacts were retained before corrections, through
attempt8. Earlier source810/sourcef1 receipts/logs remain separately labeled.

Formatting, all three client feature configurations including test-target lint,
and public-tree check PASS, receipt4c1f86b13909a28de9e6a48f7c7275a312726d8f0bed0eb26f303a29d93efada.
The reviewed
postcommit broad driver19d98159 requires a clean sole-child commit, exact six
paths, accepted raw hashes and explicit raw/Git/Linux comparison. It runs only
under an observed NON-HALLOWEEN process-local date and PASSES at exactab:
369Python/1375Rust, formatting/Clippy/publictree and zero doc cases. The recorded
local date remains November1 Pacific/Kiritimati before01:27:32 and after01:36:33.
Ancestor173's preserved
Halloween failure/four-cell comparison remains an unresolved qualification limit;
the tool requires unchanged calendar source bytes. No new native-Halloween,
parity, optimized runtime, general stack, rendering or G1 pass is claimed.

Before/after comparisons bind all six native raw hashes to exact committed/Linux
source bytes. Only game/slot_retirement.rs differs, solely by182CRLF-to-LF
conversions: raw78eeffdd6896cd67e6b60519369fa57fef593a1ecbe04bfcd707ac84ae84de52,
canonical01190432427ad59bb8e9db5459b3597e52888855854747916375497d13a3312b.
All other scoped sources are byte-identical. No blanket all-three raw equality.

Ordinary machine placement/registered binding, dropped-owner coherent saving,
authorized player transactions, persistent startup/close/Running, external
admission/replication and retained animation remain required S1 outcomes.
