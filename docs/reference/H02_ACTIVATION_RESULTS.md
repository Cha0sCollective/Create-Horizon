# H02d activation checkpoint

Status: reviewed local prototype; paused at the owner's request before real-game
smoke qualification. The owner asked to take a break at the next good pause point.
No public engine push, upstream contact, PR, merge or release occurred.

Engine base: `58e49d4d0717b7d2382a6e6e02b58ee3bd531cfd`. Local branch
`codex/h02-activation-integration` now contains catalog commit `04a6d74`, runtime
commit `26aa212`, and startup/admission commit
`60050caa3b70b3496f40cf2080b1c3c8354a67a6`. Its final tree is
`e13629cc51c0e95d026a9e2db1b71a1ca7c4ce49`.

## Implemented behavior

The opt-in paired feature discovers legacy and paired packages deterministically,
retains disabled declaration errors, and rejects selected ambiguous formats,
duplicate IDs and namespace collisions. Disabled pairs read only their bounded
descriptor. Selected paired descriptor, both executables and content are captured
once. Content startup, resource reload and cloned integrated-server Options use
those same immutable snapshots. Paired zip packages are unsupported.

Both components and optional UI callback types are checked before guest execution.
One installed package row shows the union of owner/client/server capabilities.
Client and server hooks run in separate Stores; the in-frame fallback has both.
Side settings directories and existing per-side fuel, memory and I/O quotas are
preserved. Two Stores have two resource budgets; no aggregate H14 policy is claimed.
Initialization publishes buffered output only after all selected sides succeed.
A paired callback failure discards its buffered output and stops both Stores.
Existing settings writes remain nontransactional; this is not H06 world rollback.

Selection and payload changes require process restart under this experimental
loader. External client connections, LAN publication and dedicated network startup
with selected pairs refuse before admission because H09 negotiation is absent.
The integrated memory connection shares startup snapshots. Active pairs cannot
mix with mutable legacy content owners; legacy code-only mods can coexist.

New local worlds pin paired identities and the exact host executable hash in
`horizon-paired-packages.json`. Invalid/missing/changed package records, changed
host executable and unmarked existing-world adoption refuse before upgrade,
migration or chunk preparation. Feature-disabled builds also check the record.
This is admission metadata, not a machine inventory/save system or migration tool.

## Checks and review

Bounded workers used isolated native worktrees on the H02c base. The coordinator
integrated their owned files, startup wiring and admission guards. Workers did not
stage, commit or publish. Their implementation checkouts are retained at
`.local/worktrees/h02-package-catalog` and `h02-role-runtime`.

- Catalog worker: 9 new focused test groups; 29 unit tests and 15 paired integration
  tests passed with the paired feature; formatting and Clippy with warnings denied
  passed. Covers discovery, declaration-only disabled pairs, malformed/dev-selected
  entries, ambiguous folder/archive formats, missing halves, duplicate IDs/mixed
  namespace owners, broken descriptor symlinks and captured reads after source removal.
- Runtime worker: 6 actual SDK/Wasm activation tests, 5 ABI test groups, and 32
  existing sandbox regressions passed; 1 existing sandbox test remained ignored.
  The ABI controls mutate all 13 required callbacks and all 3 optional UI callbacks.
  Feature Clippy across all mod-crate targets passed after a mechanical test slice
  fix. Receipt: `.local/h02-activation-runtime/checks.txt`.
- Integrated coordinator candidate: feature-on client binary Clippy with warnings
  denied passed; all 3 new world/network admission test groups passed in the actual
  client binary test target. These cover exact package/build matching, missing
  packages, malformed records, implicit migration refusal and network refusal.
  Raw compiler/test logs: `.local/h02d-integration-clippy.log` and
  `.local/h02d-admission-tests.log`. Initial feature-off compile and final feature-off
  client Clippy with warnings denied passed (`.local/h02d-feature-off-clippy.log`).
  The paired-feature build without the execution runtime also passed client Clippy
  with warnings denied (`.local/h02d-no-runtime-clippy.log`); selecting a pair in
  that build refuses with an explicit runtime-required error.
- Fresh independent read-only Sol High review inspected the exact final tree
  above against the H02c base and found no actionable issue. It traced startup,
  resource reload, side dispatch, storage, failure cleanup and admission paths.
  A possible optional-UI patch-version mismatch was resolved by reading cached
  Wasmtime 48.0.7 sources: precheck and generated activation use the same semver-aware
  export lookup. The reviewer made no source/index writes or launches. The actual
  runtime profile was workspace-write; read-only behavior was an instruction, not
  OS-enforced isolation. This was static review, not runtime qualification.
- All three local commits passed the enabled public-tree pre-commit check. Their
  combined tree exactly matches the reviewed tree. The Create reference remains
  clean at its pinned detached revision.

## Resume here

Run isolated real-game smoke tests with a small authored paired package, first
on the default integrated server and then with `--in-frame`. Test one installed
row, custom content registration once, both side callbacks, separate settings,
source replacement/resource reload, same-build restart and saved-world mismatch
refusal. The runtime test components are retained at Linux cache path
`/home/arbiter/.cache/create-horizon/target/paired-guest/components/`.
The fixture descriptor is in engine `crates/horizon-mods/tests/paired_activation.rs`.
Use fresh per-run game/launcher/XDG homes and fresh worlds; never Prism/playable saves.

Control-probe detail: `horizon-ctl command` executes the integrated server console;
use `chat /hello ...` to exercise the client fixture's command hook and `command
online` for its server hook. `options mods` followed by `ui` exposes installed rows.
Do not report these proposed smoke probes as executed evidence.

Real-game startup, broad post-integration regression/parity checks, network H09,
machine lifecycle/transactions/persistence and retained rendering remain open.
No live Create machinery, full H02/P02 completion, G1/G2/G3 or performance claim
follows from this checkpoint.
