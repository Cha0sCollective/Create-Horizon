# H05c checked host counter phase results

Local engine commit `83649b8e6e80e9b8db0507009b81721816a094bf`, tree
`6f89e1f6ded10dc5e8f9c4233b7d92d453995f69`, follows the separate evaluator commit.
It changes only client counter_trial.rs/world.rs, adding captured guest binding,
opaque invocation-local phase tokens and checked completion/failure paths.
No Game dispatch or Wasmtime runner is added to World or its snapshots.

Configuration keeps Native as its default. Guest binding derives primary owner
and package identity from captured metadata, checking the exact declaration and
single bound block. Tokens carry immutable full configuration, exact record and
game time. Preparation checks authority/storage/phase without advancing committed
fields. Completion consumes the token, checks current identity/record/time, then
fresh eligibility/storage and every proposal scalar before publishing counter
and last_advanced together. Matching failures latch a finite fault while preserving
those committed fields. Stale failures cannot poison a replacement trial.

Native Trial::advance is unchanged; its World wrapper preserves its return shape
and explicitly refuses guest dispatch without mutation or native fallback.
Removal/retirement still works after faults. Native status JSON and controls are
unchanged. Backend identity belongs to each World configuration and follows swaps.

Fresh independent read-only Sol High design and exact two-file source review find
no issue. Final source SHA256:

| File | SHA256 |
| --- | --- |
| counter_trial.rs | `0c48191e5c7d580b7abbcc3fbd484b913e2b03ac8aa4f6a37a1a67fbe01fb936` |
| world.rs | `56c834cb7a8cbdcfebc9625ab3a66a0cdb4779a8410de95530553a8d29fd186e` |

All 13 actual host/native counter tests pass with runtime enabled and without it,
including captured binding, duplicate/stale work, changed configuration/instance/
counter/time, eligibility/storage changes, invalid replies, retained faults,
replacement/removal and native error precedence. All 172 binary tests pass (three
ignored). Runtime-on, paired-without-runtime and feature-off Clippy, formatting,
public-tree and normal commit hook pass. Archived .local/h05c-validate.sh clears
HORIZON_SESSION_FILE and serializes every Cargo/test operation; native status is zero.

The two-file diff is +680/-18, including expanded native fixtures/test matrices,
beyond the soft review-size guideline. No PR or public push is proposed. Combined
frozen game SHA256
`4e6f2cf531249df33798184c7840fb82e6c3e49c7c69a91ef22f11b7cc5c419d`
passes the unchanged native counter driver:

| Mode | Run | Checks | Final counter |
| --- | --- | --- | --- |
| Integrated | `3b4b4103-f5e1-48f3-99b2-b9bf00c4f946` | 362 | 64 |
| In-frame | `49db714e-0edb-444d-b6f2-701c805daa69` | 347 | 28 |

Normal initial/restart exits are zero with null cleanup errors. Expected refusals
exit one and preserve saved bytes. Only integrated changed-package cleanup races
an already-exited game; all other refusal cleanup records are null. Driver SHA
remains d19c4805... (full CURRENT_WORK pin), with archived sanitizing invocation
.local/h05bc-qualify.sh. These are native game regressions, not guest dispatch proof.

Combined broad quick checks at 83649b8 pass 369 Python, 2,384 workspace, two doc,
five content-root and 197 launcher tests, plus main/launcher formatting, lint and
public-tree. The test-only locked wat dependency selects this expanded pipeline;
all registry versions remain pinned. The combined range is seven files +1286/-18
with a soft scope warning; implementation is split into two reviewed local commits.
Parity lists 348 affected harnesses but is explicitly skipped. CI update records
candidate 83649b8; the footer prints the merge base. Composite counter/quick statuses
are zero. Final independent read-only Sol High receipt review finds no issue;
it checked source/build/run attribution and archived statuses without execution.
The evaluator remains an independently reviewed preceding unit.
Host tests show no token is issued in skipped phases; only later Game tests can
prove that means zero real evaluator calls. No callback, inventory, durable
generation, checkpoint, external networking, animation, performance or G1 claim.
