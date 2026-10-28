# H03b static state materialization

Reviewed local engine commit `0264c18fd268ff369af64dcaa3dd8d94ddfdecd6`, tree
`4c563e1baa9e800740d276fadaf5da38a508bd84`, admits bounded static state plans from
captured paired packages. It follows the immutable planner checkpoint b1ced4f.
Full H03 and G1 remain open.

The declaration must use the exact `minecraft:iron_block` template and static
behavior; waterlogged is rejected. Every canonical tuple receives a consistent
registry/model/render/command/structure/NBT identity and the declared default is
used even when it is not first. States share the existing iron shape geometry;
rotation/mirror transforms preserve the tuple. This provides state data, not
Create orientation semantics or custom machine shapes.

Admission bounds expanded states to 65,536 per captured package, before retaining
each plan, and retains the existing final global state limit. Independent review
found that individually bounded plans could collectively overflow the u16
property-name vocabulary. The corrected materializer caps the union of vanilla
and every selected static plan at 4,096 distinct names. Review of the correction
and final candidate found no remaining actionable issue.

## Real game qualification

Rust 1.99.1, fast profile, no default features, `mods,paired-packages` enabled.
Frozen executable SHA256:
`50b495f01e32d2b1920cd01171aa773eba6269cbc8f0b40ae894d6c9c572641f`.
Its separate build receipt binds it to the commit/tree above; the original
baseline and earlier H02 executable remain preserved.

| Mode | Accepted run | Result | Archived driver SHA256 |
| --- | --- | --- | --- |
| Integrated server | `8155e33c-5ea5-455d-9246-c8eb2c017baa` | 50 assertions, extended/static pass | `3ab56a935e7daebe9633e32d80edefc86fac899f2c569408246006fd15693fb4` |
| In-frame | `10e1e6cc-a2c2-482c-a7c4-c7c969a4657f` | 52 assertions, extended/static pass | `9cb17515a22cdcb74a4e550370b2ecc51c94810b4aeb32094d31b34c837f00fd` |

Each uses a fresh isolated game/launcher/XDG home and world. Raw evidence remains
under `.local/h02d/game-runs/<UUID>`, outside Git. Both use identical eight-file
package contents and the native copy of owned resources with manifest SHA256
`f35140182f95862e5330fabe4c6a449805f2d83be94dce2f01e770a2ede68ce2`.
Vulkan llvmpipe was used; this is functional evidence, not performance evidence.

The diagnostic marker declares axis x/y/z and enabled false/true, with explicit
default `axis=y,enabled=true` (canonical ordinal 3). Actual game queries verify
the implicit default, all six exact tuples, and a separately right-clicked block
item's default before and after save/restart. Placement receipts independently
show air before, a changed result, and the exact resulting tuple. Both initial
and restart processes exit 0.

The earlier H02 client/server callbacks, side settings, single installed package
row and captured-content reload checks also pass. Changing package version and
disabling the required package each refuse startup with exit 1 and unchanged
saved-world file bytes, as asserted by the inspected driver's before/after
hash comparisons. Raw before/after hash manifests are not separately retained.
Settings storage is still nontransactional and is not machine persistence.

The final driver adds a live inventory-menu operation needed only in-frame,
whose initial hotbar is full. It uses the current tab target geometry, verifies
the actual stack, swaps slot 9 into hotbar 0 and checks unchanged counts and an
empty carried stack. Integrated already has the item in hotbar 0; its unchanged
coverage remains valid across this unexecuted-branch driver correction.

The coordinator inspected both screenshots: the diagnostic blue cube is visible.
All six tuples deliberately use the same cube model. No pixel, six-model,
orientation, custom-shape or animation conformance is claimed. An already-air
cleanup command can produce the harmless red no-op chat line in these images.

Fresh independent read-only Sol High follow-up checked actual receipts, archived
drivers, source identities and the inventory delta; no new findings. Review used
behavioral read-only restrictions under workspace-write permissions and performed
no launches/builds. Runtime binary/resource hashes were checked in the archived
receipts, not independently rehashed by the reviewer.

## Focused checks and excluded attempts

The final static real-resource and global-vocabulary tests pass (2 tests). The
paired library's 37 tests and feature-off library's 19 tests pass, with applicable
Clippy and formatting checks. Resource tests exercise all relevant state tables,
model/shape loaders and palette save/read; existing geometry indices stay stable.
Raw logs are `.local/h03b-*.log`.

The final isolated native Linux checkout at the exact engine commit/tree passed
`tools/pr_ready.py --quick --base 58e49d4d0717b7d2382a6e6e02b58ee3bd531cfd`
with exit 0: 369 Python, 1,378 Rust, five resource-root and 197 launcher tests;
formatting, Clippy and the public-tree check pass. Doc-test checking passed with
zero tests. Source status stayed clean. Raw summary:
`.local/h03b-pr-ready-isolated.log`. This run uses default feature coverage;
the separate focused paired tests and actual games supply opt-in evidence.
Vanilla parity oracles were explicitly skipped by `--quick`; no parity pass is
claimed. Its initial 23-file scope included an ignored local instruction overlay;
the committed comparison has 22 files. The overlay was excluded locally and
never staged or published. The combined comparison spans separately reviewed
H02d/H03a/H03b commits rather than one proposed public change.

Two earlier CI invocations were setup failures, not evidence. A Windows worktree
gitdir could not resolve in Linux. A subsequent process-level GIT_DIR/GIT_WORK_TREE
override leaked into Python tests that create temporary repositories, causing 23
errors and modifying the shared local Git configuration. The coordinator stopped
that run, removed the fixture's core.worktree and local user overrides, verified
the original effective identity, branch/tree, clean source and disabled upstream
push boundary, then used a separate native Linux clone without those overrides.
Recovery copies and failed logs remain outside Git under `.local/`. No commits,
source edits, remote changes or public pushes resulted from that failure.

An earlier concurrent Cargo run replaced a running test executable and caused a
resource-stamp `current_exe` failure. Its 17/18 result is excluded; affected final
tests were rerun serially and pass. Serialize all Cargo operations sharing a
target directory. A feature-off binary-fixture Clippy failure was corrected by
a mechanical cfg field initialization, with no runtime behavior change.

Calibration runs `61950e17-11aa-496c-b503-52ff574417cf`,
`e1a6ffd6-5abc-4d0d-9fcf-523a5e156d0c` and
`6510a566-4b98-4aca-ae4a-066cb7da6e79` are retained and excluded: camera visibility
assumption, main-inventory versus hotbar placement, and missing tabs in the
labelled-widget tree were test-procedure issues. Earlier successful run
`640c2a52-f884-4ec5-9660-b1ac6e015021` is superseded by the accepted integrated run.

The next packet is H04a's passive committed resident mutation observer. No public
engine push, PR, merge, release or gate promotion occurred.
