# H03a immutable state-plan checkpoint

Reviewed local engine commit `b1ced4fcd26fe9ecae5403b6968500342162ca1c`, tree
`ada6ce48c66937f9b48ea58cf8a32b63a99f0256`, follows the H02d activation tree.
This is a validation/planning seam, not live custom-state admission or full H03.

Under the opt-in paired feature, `content::states::StatePlan::parse` accepts a
format-1 static declaration with explicit property names, values and defaults.
It exposes immutable canonical property tuples and the explicit default ordinal.
Properties and values sort lexicographically; the rightmost property varies
fastest. Construction validates at most 64 KiB JSON, 16 properties, 64 values per
property and 4,096 states, with checked expansion and plain bounded ASCII tokens.
Unknown fields/versions, duplicate names/values and invalid defaults reject.

Legacy BlockFile admission remains unchanged and refuses unsupported state
declarations. Materialization must separately enforce the global state budget and
safe template/property restrictions; a per-block plan is not aggregate admission.

An isolated worker authored the module and five-line export without commits or
staging. Its 35 paired-feature library tests, 18 feature-off library tests,
formatting and library/test Clippy with warnings denied passed. Raw logs are
`.local/h03-state-plan-library-tests.log`, `-feature-off.log`, `-clippy.log` and
`-test-clippy.log`. Use the full library log for final coverage; the earlier focused
log predates a test rename. Six new test groups include independently specified
tuple/default expectations and unchanged legacy rejection.

Fresh independent read-only Sol High review found no actionable issue. Reviewed
Git blobs: content.rs `38889ddb9d6b8ea787bf29dbeea4a9401a651de5` and states.rs
`260f2620fbe0d419c37f98604080b77d72a37586`. The coordinator verified those exact
blobs after copying and committed only the two owned files. The enabled public-tree
hook passed. Review was static under behavioral read-only restriction in the actual
workspace-write runtime; no independent builds or launches were claimed.

Next: `docs/design/H03_STATIC_MATERIALIZER_PACKET.md`. Custom shapes, orientation
semantics, machine bindings and G1 remain open. No public engine publication occurred.
