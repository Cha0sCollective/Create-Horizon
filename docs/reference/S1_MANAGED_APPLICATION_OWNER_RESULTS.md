# Pollable managed application owner results

Status: FINAL ACCEPTED. Independent final review accepted the bounded pollable owner.
The exact candidate passes314 normal cases, two separate exceptions, formatting,
three feature checks, public-tree audit and optimized typechecking. Native broad
fails the unchanged Halloween fixture; fresh parent comparison reproduces it.
S1/G1 remains OPEN. No public publication or ordinary window/Running activation.

## Candidate and review

Local commit: a56e0136a87ef723d4866c341ccbe11ec3678bd9.
Tree:3f60401097db1580c2f35736b8ccda7d27b1433c.
Sole parent: FINAL bridge7a08dc25c0e3135e3a0e0a4eaae7c11af5e1d99e.
Worktree `.local/worktrees/s1-managed-application-owner` on clean branch
`codex/s1-managed-application-owner`. Normal native Windows Git PublicTree hook
passed; three files548 insertions/64 deletions. No public push/PR/merge/release.
Private FINAL prerequisite1088968 was pushed before source; interim private
checkpointabf40d4 preserves this stage's progress and window audit.

The accepted application design input is
cb1f1496b9404120bd44f734cd015a91b5fa5fffa2bc1345ed8af12b0c156db9,
frozen `.local/s1app1-proposal2.md`. This is its separate primitive-first stage;
the real window/signal stage remains gated on this stage's FINAL/private checkpoint.

Source manifest `.local/s1ao1-source-sha256.json` SHA256
2b627334cb4b4c0f6f041e9015d4e89ebe03802194ce94ba37af85cc88773780.
Suite `.local/s1ao1-test-expected.json` SHA256
f31694dc6aa1eb26f9aad5d205a654f0abda58386da21a0ca17a449502b65b16.

| Path under crates/horizon-client/src/game/integrated | Raw SHA256 |
| --- | --- |
| managed_stop.rs | add07a5c803a913b120ee87236cc3efafa756950fe9114e5765cffcbfda3761d |
| managed_stop_tests.rs | 15688760d0a813ac7636b518b3d80cc0e80baceb61a85aa8d3902e00ed563823 |
| managed_connected_tests.rs | e12b7440d7b74c8df1e266636b86dafc26050b000cd3e359d0c7f1d9796c0845 |

Independent Sol High s1ao_source_review SOURCE CLEAR and s1ao_tool_review TOOL CLEAR
bind these exact files/suite and twelve reviewed scripts. Different reviewer
s1ao_final_review has checked preflight, focused runtime, exceptions, compile,
commit and calendar evidence; FINAL is withheld until complete authored results.
Reviewers used procedural native read-only operations in write-capable runtime;
root alone executes formatting/Cargo/tests/games/Git writes serially.

## Behavior and meaningful proof

The private concrete ManagedWorker retains the existing first-field fatal guard,
control sender, one-shot endpoint receiver and coalesced status notification. Its
only starts are the existing NewClosed and Connected variants. No Game, raw task,
host, native transport owner or JoinHandle escapes. There is no Clone/cancel/
discard/detach API, and Drop never joins. A live owner unexpectedly unwound is fatal.

poll does not wait for an unfinished worker. Only actual is_finished permits taking
the handle, explicitly joining and checking the terminal result/receipt. A shared
Stopped view may appear before that explicit join. take_completion cannot join,
consume a live handle or request a stop; after joined classification it yields the
result once. Repeated completed polls cannot rejoin, repeat the result or offer
closed endpoints. A startup-refusal bundle queued before the first poll is dropped.
Both synchronous supervisor entry signatures remain wrappers around this owner.

Two new direct tests preserve the actual source and client machinery. Connected
case restores3/5 stone in two actual memory clients, checks terrain/teleport/Loaded,
rows/core/keys/leases and frozen clocks, then deliberately observes shared Stopped
without polling. Taking completion there returns None; explicit completed polling
joins and yields exactly one receipt. Three later polls remain stable. The native
continuation is genuinely verified after join. Closed case retains an actual Copied
publication failure, replaces an observer, explicitly retries, verifies saved bytes
and native/UUID exclusion release, and checks the same early/one-shot completion.
Existing wrapper cases and exception bodies remain unchanged.

## Focused qualification

Validation94912 passed fmt, three feature Clippy checks and public-tree audit;
receipt e3dfb8a0ad124f5fba2ae60cdeba8f3e558d61d3dc118ba268953aeaa3400930.
Features: runtime+paired, paired without runtime, neither. These checks ran before
runtime builds; no source correction was needed after exact-source clearance.

Build28279 froze debug-assertion executables at
/home/arbiter/.cache/create-horizon/bin/s1ao1-tests-4otrxqoc.
Exact features mods,paired-packages,native-writer-test-control; default features false.
Build receipt c37fe5b1122adfbfc4db40bbc551c72770ef80416daa3ee725750fd6b76b60dd.

| Executable | SHA256 |
| --- | --- |
| bin-tests | a605c8127d576b8bf4f46ccabc41f16184ef528bbc60dbc6335cba1c15718c69 |
| lib-tests | 36532664d057c5c8bc4d45c76dfd071170f73bd6c6d9f99e4f671fec118ffbd3 |
| protocol-tests | 0f44bef85b087bd0a96ae4c3bd4e0fff07e8ff360539a6426fac324ad567b3fc |

Full62617 passed314 normal cases/10groups:203ordinary,103protocol,six resource,
two legacy. Ignored cases are not passes. Receipt
17c4984c82d8311ba13c9afa3d1b180bd4c66c75977ab73747b9919bbe1064ff.
Group seconds .85/1.57/107.63/107.18/107.82/66.23/103.81/28.51/28.56/29.58.
Direct connected observed3408 pending polls/3Ready/oneoffer/epoch1; direct closed
5974 pending polls/3retained/epoch3. These counts are observations, not latency or
throughput qualification. Both had early completion unavailable after Stopped and
one consumed result followed by three stable completed polls.

Two separate root-orchestrated exceptions, not ordinary libtest passes:

| Case | Actual result | Receipt SHA256 |
| --- | --- | --- |
| Ready supervisor unwind73756 | Actual SIGABRT(-6), required fatal marker, reaped;93.56s | 68ac30cd81f848b73a0be854270fbd774c8a985d0f7fbc3d83627aca3a2c3495 |
| Late departure41947 | Retained2Guest/2GuestPlay/rows/core/keys/clock17/locks, refused close; explicit owned-fixture SIGTERM(-15), reaped;93.38s | 2498cf9c0e7f89f61989efa10cec9cc2c475f1cc61d4f6e5658d0108f6d73096 |

Both source epoch1 images had full member inventories and hashes checked. Ten late
custody observations are supervisor checks of coalesced status, not distinct worker
publications. Late departure is retained failure, not successful shutdown or a
new disconnect/reconnect policy.

Optimized TYPECHECK65219 passed opt3/debug assertions false/tests_executed false;
receipt8dcdf505cd0ad72f7d87d1470c2be4b5f55405abedec41f29f9a9b074794a4ac,
loge1a4ba825c08a7f0fda12670de96d55e31ea4e7588ca18b1ea7fbf7623caeffd.
This is compilation only, not optimized runtime or performance qualification.

## Broad and calendar qualification

Native broad48063 FAILED:369Python/fmt/workspaceClippy/doc0/publictree pass;
client lib1172pass/1fail/51ignored, solely
vanillagen::structure_mobs::tests::ocean_ruin_drowned_match_vanilla with Halloween
jack_o_lantern/RNG mismatch. Original failure remains preserved:

| Artifact | SHA256 |
| --- | --- |
| s1ao1-pr-ready.log | 891940077cce2c675f718c753a344b016ca01c7ed9236c30a266712a100faf5d |
| s1ao1-broad-status.json | b0d9844c9e61a72c6864f3a5e49e82d8adc4f4afd3ee4875143a8da5bb2042ef |
| s1ao1-broad-source-comparison.json | 32709e72bd7152df93148667cccccd76e2c5a6c329906794027e8aa0b444c242 |

Before/after source maps match: all three raw/Git/Linux files are byte-identical,
with zero CRLF normalization exceptions. Candidate remained clean and unchanged.

Fresh four-cell calendar30710 verified candidatea56e/parent7a08 using the same
frozen library executable d58387c6564ce3e3ce47f653d9b60f319912220cb5129bc63ad699b04676c1c2.
Each revision failed the exact one test101 with identical assertion vectors under
observed UTC Oct31 and passed the exact one test0 under observed Honolulu Oct30.
Local/UTC/TZ/date class were recorded before and after each fresh process. Expected
failures are not normal passing tests. Clean current checkout was restored.
Receipt13e1bd41557b2c8d3c37c680eca95f8bf37bacfba8a4a8aba762896232dfd509.
No source, system clock or global timezone change occurred.

Separate NON-HALLOWEEN-CALENDAR-CONDITIONED broad99538 PASSED:369 Python tests,
1375 client tests, formatting, workspace Clippy, doc tests(0), public tree and
unchanged Cargo.lock. Reverse-consumer checks are compilation only; `--quick`
parity was unexecuted. No optimized runtime, native-Halloween broad pass or general
default-stack claim is made.

The wrapper pinned the fresh current-versus-parent receipt, changed only child TZ
to Pacific/Honolulu, and checked non-Halloween before and after. Observed local
Oct30 21:29:00-1000 through21:35:07-1000; UTC Oct31 07:29:00Z through07:35:07Z.
Native Halloween baseline fixture qualification remains explicitly UNRESOLVED.

| Artifact | SHA256 |
| --- | --- |
| s1ao1-conditioned-pr-ready.log | e211578e216e4dcd801b0af673e5bb50a649182c61ec157ffcc0ee79a91d3f8f |
| s1ao1-conditioned-broad-status.json | 5c4b266f62e24c1cae2d62edc8801851c1a9269ca6c2b3eddf953cc2dbc82be7 |
| Both source comparison maps | 32709e72bd7152df93148667cccccd76e2c5a6c329906794027e8aa0b444c242 |

Before/after maps are identical; every scoped native/Git/Linux source byte matches
exactly, with no normalization exception. Clean a56e/source2b627 remained unchanged.

## Preservation and remaining scope

All three sources and ALLprefix artifacts were preservedattempt0/1/2 before edits,
normal-case discrepancy correction and formatting; attempt3 before commit and
attempt4 before calendar comparison preserve the complete earlier evidence.
All dirty-parent s1ao1 guard/build/validate/test/exception/fast scripts MUST NOT run
after commita56e. Broad uses a separate exact-clean-commit guard.

This is the pollable primitive only. No native window, signal-policy, default
launch, Running, machine-slot ingress, live admission, TCP/dedicated/in-frame,
rendering/animation or G1 completion follows. Offline profiles are not cryptographic
authentication. Native pipe buffers remain unbounded. Typed publication faults do
not establish OS I/O/power-loss durability. Prior source-fixture ordinary-LAN reader
limitations remain. Runtime tests use fresh isolated directories/pinned resources/
prebuilt pair, with no playable saves/accounts/reference/global Java changes.

The next real frozen-window stage remains gated on FINAL, clean successor pin and
private evidence checkpoint. Implementation notes in S1_FROZEN_WINDOW_API_AUDIT.md
are read-only planning, not new source authority. Full S1 continuation remains the
owner's instruction; public publication and later stages remain unauthorized.

Independent FINAL ACCEPT by s1ao_final_review at exact authoredinput
9e9c7930dd3098070183b00659461bc1bd52b4aa573438425bd83c76ca2882e1, preserved as
.local/s1ao1-final-input.md. Exactclean-a56e/source2b627; no actionable findings.
Native Halloween qualification remains unresolved; window/Running/G1 remain open.
