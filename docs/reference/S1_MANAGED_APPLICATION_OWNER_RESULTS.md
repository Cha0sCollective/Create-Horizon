# Pollable managed application owner results

Status: IN PROGRESS. Exact source and tools independently cleared.
Formatting, feature compilation and linked debug build passed; runtime/final acceptance remain pending. S1/G1 remains OPEN.

Parent: FINAL bridge7a08dc25c0e3135e3a0e0a4eaae7c11af5e1d99e.
Private prerequisite checkpoint1088968 was verified private and pushed before source.
Worktree `.local/worktrees/s1-managed-application-owner`, branch
`codex/s1-managed-application-owner`. Scope is exactly three integrated worker paths:
managed_stop.rs, managed_stop_tests.rs and managed_connected_tests.rs.

Accepted umbrella design inputcb1f1496b9404120bd44f734cd015a91b5fa5fffa2bc1345ed8af12b0c156db9
in `.local/s1app1-proposal2.md` allows this primitive-first stage separately from
actual window activation. No window/signal/default-launch/Running/render edits here.

Exact source manifest `.local/s1ao1-source-sha256.json` SHA256
2b627334cb4b4c0f6f041e9015d4e89ebe03802194ce94ba37af85cc88773780.
Productionadd07a5c803a913b120ee87236cc3efafa756950fe9114e5765cffcbfda3761d;
closed tests15688760d0a813ac7636b518b3d80cc0e80baceb61a85aa8d3902e00ed563823;
connected testse12b7440d7b74c8df1e266636b86dafc26050b000cd3e359d0c7f1d9796c0845.
Expected suitef31694dc6aa1eb26f9aad5d205a654f0abda58386da21a0ca17a449502b65b16.
Independent Sol High s1ao_source_review SOURCE CLEAR; procedural native read-only.

The private concrete ManagedWorker retains the existing first-field fatal guard,
control sender, one-shot offer receiver and coalesced status notification. It starts
only the existing NewClosed/Connected variants. poll never waits for unfinished
workers; only actual is_finished permits explicit join and receipt/result checking.
take_completion cannot join or consume a live handle, and returns the classification
at most once after a completed poll. Shared Stopped status can precede that explicit
join and cannot authorize application exit. After finite startup refusal, a queued
closed endpoint bundle is discarded instead of offered as new admission.
Both synchronous supervisor signatures remain wrappers around this owner.

Two appended direct actual-worker tests cover early completion refusal, deliberately
observed Stopped-before-poll, one-shot completion and stable post-completion polls.
Connected case reuses the genuine source/two actual memory clients, stone3/5,
terrain/teleport/Loaded, frozen clocks, native keys/leases and native continuation
verification. Closed case retains a real Copied publication failure, observer
replacement and explicit retry, then checks saved native bytes and exclusions.
Existing wrapper cases and exception bodies remain unchanged. Planned full suite:
203 ordinary+103 protocol+six resource+two legacy=314 normal cases/10groups;
two connected exceptional children remain separate from passing libtests.

All three sources and ALLprefix artifacts preservedattempt0/1/2 before edits,
normal-case discrepancy correction and formatting. Source/tool correction snapshots
continue before EVERY correction. Root alone runs all Cargo/tests/games serially;
fresh isolated directories/pinned resources and prebuilt pair are required.
No runtime, compatibility, optimized-runtime, performance or native-calendar pass
is claimed yet. The known native Halloween baseline qualification remains unresolved;
any conditional broad result must preserve and disclose actual failures.

Tool reviewer s1ao_tool_review TOOL CLEAR at exact2b627/f31694 pins, twelve
reviewed scripts. Root validation94912 now running fmt/three feature Clippies/
public-tree checks before building any runtime executable. Source/runtime remains
unchanged during checks. No compile pass or game test claim yet.

Validation94912 FINISHED0: fmt/threefeatureClippies/publictree. ReceiptSHA
e3dfb8a0ad124f5fba2ae60cdeba8f3e558d61d3dc118ba268953aeaa3400930.
Freshdebugbuild28279 running alone. Differentfinalreviewer s1ao_final_review
is doing native read-only preflight; actual runtime/commit/broad/final stillpending.

Fresh build28279 FINISHED0 (client test compile2m03s). Frozen directory
/home/arbiter/.cache/create-horizon/bin/s1ao1-tests-4otrxqoc;
bina605c8127d576b8bf4f46ccabc41f16184ef528bbc60dbc6335cba1c15718c69,
lib36532664d057c5c8bc4d45c76dfd071170f73bd6c6d9f99e4f671fec118ffbd3,
protocol0f44bef85b087bd0a96ae4c3bd4e0fff07e8ff360539a6426fac324ad567b3fc.
Build receiptc37fe5b1122adfbfc4db40bbc551c72770ef80416daa3ee725750fd6b76b60dd.
Debug assertions true/defaultfeaturesfalse/exactmods+paired+writer features.
Full positive62617 is RUNNING alone; no full-suite pass claim yet. Different final
reviewer completed source/tools/feature preflight without findings, FINAL withheld.
