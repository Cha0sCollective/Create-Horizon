# Trusted session authority results

FINAL ACCEPTED by a different fresh independent native read-only Sol High reviewer.
Reviewed authored-result input SHA256
8e43de41083715bef9e870c8ddfedb96ca8136d98b919098db1babc5485a3fe3.
Engine worktree
.local/worktrees/s1-session-authority on codex/s1-session-authority, base
872dac2da765f0b290bfcfe13502f7d97724be1f; clean LOCAL commit
938859881792438b39e5f597eb331a829e266199, tree
3ae121f3df5320c7eb8e63dd900aa9fac851be60. Exactly five files/740 insertions,
normal hook PASS; no public engine push.
Fresh independent design accepted exact6f98cc8d; independent source/delta reviews
CLEAR. Different final review accepted exact938/tree3ae121f3 and all evidence,
independently verifying229 cases/31groups, source/executable identities, broad
before/after comparisons and the explicitly bounded claims.

## Behavior and limits

The dormant explicit managed entry holds one native OS authority lock per trusted
WorldUUID in ONE configured namespace. New identified creation acquires it before
package/identity publication and streaming. Guarded recovery derives its expected
UUID from the selected owner's bounded identity, rechecked under its actual native
lock, even if that owner's native level is missing or corrupt. It retains owner
and authority locks through unchanged strict archive recovery and installs the
lease in the recovered WorldSession before returning Game. Managed native-only
reopen refuses. Existing unmanaged research primitive and checkpoint format remain.

Complete route separation and namespace UUID-target preflight precede all lock
writes. Linked/malformed/overlapping inputs refuse without repair. Authority files
remain after release: actual held locks establish exclusion, not marker existence.
Exclusion covers cooperating opted-in entries using the SAME trusted namespace;
legacy/unmanaged entries, another namespace and hostile concurrent filesystem
replacement are outside the claim. Linux is the qualified target; no Windows
runtime/reparse-point qualification is claimed.

No ordinary startup/close caller, failed-stop retention, connected participant
staging, in-frame recovery or filled-item ingress is enabled. Actual native Game
participant objects are not connected/admitted clients. The component test drops
a Game to prove lease release; it does not establish save-on-close. S1/G1 stay OPEN.

## Source identity

Five paths under crates/horizon-client/src: game/world_loading.rs;
game/world_loading/host_identity.rs; new
 game/world_loading/host_identity/session_authority.rs;
game/levels/session_checkpoint.rs and its session_checkpoint_tests.rs.
Full raw manifest: workspace .local/s1sa1-source-sha256.json.

Current authority source20333e3cc938dac333cc8e57ad5122a7517b264bfdd4030f2fd3cea94bdde795;
checkpoint52f95ec855185cc595cf83981c20b756a59b615f41ce2994f304b3635ed9b678;
tests72c389e3d5fc80b221bea94ea167903cc94d69bc9f3bee069f2d56b0364b731f.
The four production hashes did not change during the test-fixture correction below.
Native world_loading.rs (607 CRLF) and host_identity.rs (836 CRLF) differ from
committed/Linux bytes ONLY by exact CRLF-to-LF conversion. The other three raw
sources equal committed blobs. Reviewed broad guards verify both identities; raw
test-source pins remain intact. Before/after receipts must retain this disclosure.

## Runtime qualification so far

Refreshed final-source formatting, public-tree and three feature-lint configurations
PASS in49127 after the fixture correction.
Debug test build PASS with requested mods/paired-packages/native-writer-test-control
features and assertions enabled. Rebuilt current binary:
/home/arbiter/.cache/create-horizon/bin/s1sa1-tests-r68l9qj4/bin-tests,
SHA256f2c2e21469d75082d2c281fd51c9a549ea99a69e9e59d2ea5e0f76592fcaccce.
Library test executable15ef8e0af8c8557a3ad962ebc6f040b51f0b761281351dc7b87077e0a2958629
is unchanged across the fixture correction.

199 ordinary and all18 checkpoint-resource cases now have passing coverage across
explicitly split runs. All12 legacy resource cases also PASS on the rebuilt binary.
Qualified receipt27cae77be48229b1db3e9053d1f50d4942a6448e54c49aa871f93d76af9f375e
covers229 unique cases/31 groups. This is NOT one successful full-suite invocation.
Optimized43244 PASS: opt_level3, debug assertions false, tests_executed:false;
TYPECHECK ONLY, logSHAe0e9bd3a269d50022a170cd7e2966e621134893f2862f52359bcfc2481fb629a.
Clean committed broad22486 PASS at exact938:369 Python,1375 client and0 doc
failures; formatting/clippy/public-tree PASS and Cargo.lock unchanged. Duration8m29s.
Broad log SHA256409a350b1f64d07cf4df8a998130e13ab860dcfd9d67040c7221db9bce220d5c.
Before/after canonical-source comparisons match and both worktrees are clean.
The pr_ready header prints base872 as a comparison field; checked HEAD/source are
exact938 as required by the wrapper and both receipts. General parity is omitted
by quick mode. Its740-line size advisory is not a public PR/promotion approval.
No GUI, new Java parity, performance or connected-client claim is made.

## Failures, correction and split provenance

Source review found one P2: linked authority child/lock preflight followed owner
lock acquisition. The correction moved it before that write, retained rechecks,
and added fresh-owner unchanged-byte/no-added-file assertions. Independent delta
review cleared the exact source before execution.

The first runner70650 passed199 ordinary and resource0..16, then new17 aborted
SIGABRT(-6) with a stack overflow before native source construction. Raw source,
logs, receipts and persistent failed fixture root remain preserved. Read-only
frozen-binary prolog inspection found1246136-byte outer test,485448-byte production
wrapper and211528-byte open closure frames; it did not prove the wrapper alone
independently overflowed.

Only the new test fixture changed: two non-inlined helpers return Box<Session> and
Box<Game> before the outer caller reserves by-value return slots. All assertions,
real native host receivers and source retirement point remain; no production API
or recovery check changed. Independent exact-source review CLEAR. Corrected17
passed83.66s using the rebuilt binary with inherited RUST_MIN_STACK explicitly
removed. Its receipt/log are retained as s1sa1-focused-resource17-pass.

Reused199/resource0..16 were run on earlier binary9c374df979513a851228bdc4d6779801c24f5e2d3744b78655b628b83ae53a0e
and test-source4199ea2054242d8c8d91b6baa6c025e0bc452096b1e78fc5991d9ad929a95182.
Those receipts did not record whether RUST_MIN_STACK was inherited; no default-stack
claim is made for reused runs. The four production sources and reused case bodies
are unchanged. Different final-review preparation independently approved this
bounded coverage reuse with disclosure, not a one-binary/full-invocation claim.

The retained-case completion wrapper initially used an && chain whose first failed
absence test would not exit under bash set-e. Before execution it was corrected to
two standalone checks, independently TOOLCLEAR at29cf645d. Focused receipts are
copied only to absent destinations and compared. All five sources and every packet
artifact were preserved before each correction, including failed-attempt evidence.
Initial hash capture also preceded formatter completion; it was refreshed before
source clearance or execution. No stale hash snapshot was used to execute tests.

## Inputs and next gate

Rust1.99.1, MC26.3/protocol777/DataVersion5023. Pinned resource manifest
f35140182f95862e5330fabe4c6a449805f2d83be94dce2f01e770a2ede68ce2;
prebuilt pair receipt32c83f6b1fbffe44696f42dd3617b7a6e6836d18a46303f0d21189dcacb940b4.
Fresh compact persistent scratch roots under /home/arbiter/.cache/ch-s1; child Git
variables cleared, launcher/game/home/data/config/cache/temp isolated. Python-I and
explicit optimization refusal guard proof parsing. No Rust fixture compilation or
nested compiler/Java/Python/test subprocess was introduced. Prism, playable data,
reference snapshots, global Java and the owner's R01 decision are unchanged.

Only authored summaries/design enter the private repository. Engine source remains
local, normal hooks; no public push/PR/merge/release. The next dormant managed-stop
worker design has acceptance at000d1599 plus startup-route delta1bcefe02;
no source release until this
packet's final acceptance, clean local commit and private checkpoint. Ordinary UI/
process-lifetime routing, positive-owner staging, in-frame adaptation, authorized
interactions, admission/replication and committed presentation remain before G1.
