# Testing strategy review after the storage checkpoint

Status: PROPOSED operating/validation refinement, not adopted. Owner reports that
testing consumes too much development time and requests examination. Finish the
active battery, checkpoint/sync and clean storage first; start no further large
suite before this review. Preserve S1/G1's full behavior and connected-game gates.
Require fresh independent Sol High review before adopting rule changes.

## Observed costs to examine

- The current Guest queue repeats all302parent regressions plus23new cases,
  nine evaluator cases and21fatal processes, after six static checks. A further
  broad run would repeat formatting/Clippy and1379largelyexisting client tests.
- Fourteen new actual Guest cases use separate processes, many with repeated
  resource/package setup. Ten use the normal primary pair; four fault variants and
  wrong-live capture have distinct fixture requirements. Inspect exact counts
  before batching; never change fixture identity within a process using globals.
- Cold build/static costs include a7m30client test build and repeated feature
  configurations. CI used multiple ~12GiB targets for one physical checkout.
- Root source guards rehash and perform many Git subprocesses before/after each
  individual group despite frozen source, exclusive execution and immutable
  prebuilt test binaries. Measure actual overhead from existing receipts/logs.
- Obsolete targets total roughly250GiB; copied test binaries alone occupy60GiB.
  Retaining every old executable is not automatically required for an ordinary
  software result whose source, build recipe and logs are preserved.

## Proposed decision rules

1. Run cheap format/type/lint checks before expensive execution. Recheck affected
   targets after a mechanical correction; reuse valid unchanged source/behavior
   coverage explicitly rather than relabeling old execution as fresh.
2. For substantive code, select defining new tests and regressions through actual
   changed behavior/dependencies. A touched filename alone does not justify every
   test in that crate. Expand for failures, unresolved risk and integration needs.
3. Use full integration/broad suites at meaningful composed checkpoints and stage
   gates, rather than automatically repeating them after every bounded packet.
   Retain the complete end-to-end, conservation, fault, authorization, recovery,
   replication and both-game-mode G1 requirements. No smaller S1 or weaker R01.
4. Reuse normal incremental Cargo caches within one physical checkout and known
   profile. Keep separate targets for different physical worktrees; never repeat
   the previously observed cross-worktree target collision. One maintained CI
   target for its single physical checkout may replace per-packet CI targets.
5. Keep worlds/state isolated. Where actual global fixture initialization permits,
   group compatible cases by immutable variant within a process while creating
   fresh worlds/owners. Keep fault variants separate and fatal processes separate.
6. Prefer one frozen-source/artifact/input validation at each meaningful batch
   boundary, with end checks and exclusive mutation rules, instead of full repeated
   hashing/Git inspection before every case. Preserve failure provenance and
   truthful counts/profile/fixture identity. Avoid a new general test framework.
7. Preserve authored source and small useful result summaries in Git, appropriate
   logs locally, and only current/needed regenerable artifacts. Retire obsolete
   target/binary copies after verified checkpoint and inactivity. Any permanent
   retention rule requires this review; current cleanup remains separately gated.

## Required concrete review output

Assess what each existing suite adds and its risk/cost, using actual code and
completed receipts. Identify the smallest defensible remaining qualification for
Guest2035b79e after its325/9/21/6battery, without inventing coverage or accepting
FINAL prematurely. Explain whether any default-profile/ordinary-call behavior
still requires a separate run and why. Guest's ordinary startup/admission remains
outside its component and mandatory later G1 work.

For precursorcbf2db50, inspect proposed110normal/25fatal/six checks. Identify
unchanged writer/protocol/etc coverage that can be reused, and the focused native
lock/authority/loader failure tests and representative real-data/Game controls
that genuinely need execution. Preserve exact SOURCE-before-runtime and a
different independent FINAL for substantive changes, while removing duplicate
ceremony rather than changing component behavior promises.

Return a concrete future workflow, coverage matrix and recommended test/build
budgets or triggers. Distinguish correctness requirements from optional assurance.
No engine edits, new tests or test execution are part of this review.
