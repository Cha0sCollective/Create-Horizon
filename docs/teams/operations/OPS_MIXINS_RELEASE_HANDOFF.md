# Mixins release CLI tooling candidate

Worker: `/root/ops_mixins_release`, branch `codex/ops-mixins-release`, isolated
native-Git worktree `G:/Create Rusty/.local/workers/ops-mixins-release`.
Frozen base: `9651dba12d163896d45a8cc4ba1b0b3b0da1feff`.

The source-workspace setup builds the unchanged pinned CLI with
`cargo build --release --locked --jobs 4`, and returns the checked
`target/release/horizon-mixins` path to staging. There is no debug fallback.
The separate `.tools/mixins-built-release.json` receipt identifies source
revision, release profile and binary SHA-256. The old debug marker is untouched.
The complete private-source verification still precedes build/cache reuse;
Mixins pin `ac5d9e766380d575838c8250f2a7982281c1f44e`, source overlay, official
archive rules and private-source-only packaging restrictions are unchanged.

A receipt mismatch preserves the existing single regular CLI binary as
`target/release/horizon-mixins.unverified-<uuid>` before Cargo runs. This forces
the expected output to be missing even when Cargo fingerprints would say Fresh.
The configured target-root symlink is allowed after canonical resolution;
redirected profile directories, binaries and receipt leaves are refused. No
cache tree is cleaned or removed. Operations must retain its single-writer lease
for setup/staging; this is not a concurrent shared-target build protocol.

Validation: bundled native Python 3.12.14, workspace TMP/TEMP, compiler/network
stubbed. `python -m unittest -v test_setup_mixins test_mixins_source` passed six
tests in 0.108 seconds on the final source. The new regression exercises exact-profile build args,
cache reuse, a same-revision receipt with overwritten binary, preservation of
those foreign bytes, forced rebuild, later reuse and the retained debug marker.
`git diff --check` and a `stage.py` AST parse passed. Fresh independent Sol High
review by `/root/ops_mixins_release/release_tool_review` returned CLEAR on the
frozen four-file delta. Its actual task restricted the workspace-write runtime
to native/Git reads and its own phase metadata; no tests were run by the reviewer.
Only this result/timing documentation changed after review. No Cargo, WSL staging,
Java, game/server command or active shared-cache mutation was performed.

## Later root-owned lease and adoption

1. Preserve accepted unoptimized005 generated host source and its complete
   relative-path/SHA-256 manifest before running candidate setup/staging. Freeze
   the same native inputs, host pin, private Mixins pin, toolchain and resources.
2. Under a new exclusive Operations lease, use this candidate's source tooling
   in the established Linux environment. Time the one-time release CLI build
   separately from plan/apply and total staging. Record CLI SHA-256 and receipt.
   Keep the prior debug CLI/marker and any quarantined release binary.
3. Force one candidate apply into a separate owned generated-output directory;
   an unchanged stage-key cache hit cannot establish output equality or speed.
   Compare the complete generated host-source file set and every file's bytes
   against the frozen unoptimized005 output. Inspect any extra/missing/changed
   file before adoption; matching injection counts alone do not pass.
4. Root decides adoption and records measured timings/equality. If all generated
   source bytes match, reuse the unchanged gameplay evidence; this build-profile
   change does not call for a broad game retest. A mismatch remains a hold for
   investigation, with no silent acceptance or host/pin expansion.

Phase accounting is local under `.local/team-operations/operations/phases` for
worker `/root/ops_mixins_release`; actual token telemetry is unknown here.
