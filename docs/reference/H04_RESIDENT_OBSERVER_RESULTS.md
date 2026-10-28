# H04a passive resident observer

Local engine commit `2f13f03afcaa519fa1765cf8f00b8454c103c43d`, tree
`ef83f2fed324e5dc2ce6047e0bbb604c4a0fea8e`, follows qualified H03b at 0264c18.
It adds an opt-in, feature-gated host observer at successful resident World::set.
Default worlds keep it disabled. This is a prerequisite seam, not full H04,
machine lifecycle, persistence or guest callback delivery.

The World-owned observer filters a bounded watched set and preserves every old/new
replacement in order, with position, actual game_time, checked sequence and an
immutable transient authority-instance/dimension scope. Cause is explicitly
unclassified. Invalid configuration rejects before enabling. No-op, unloaded and
out-of-height writes emit nothing. Existing LAN and sculk queues stay independent.

Overflow and sequence exhaustion are sticky observation faults. Drain returns an
error rather than a truncated successful batch; the passive observer never adds
a failure to the already authorized World::set operation. Explicit teardown is
required before replacement and returns the previous observer and pending/fault
state. This is not a transaction journal or persistent placement generation.

The isolated worker's pinned Rust 1.99.1 checks all passed, with no compile fixes:

- Eight new tests: bounded configuration, checked exhaustion, ordered filtered
  resident transitions, no-op/failed writes, independent existing queues, sticky
  overflow with committed World writes, scope/history across World swaps and
  default-disabled/explicit teardown behavior.
- One existing World regression test.
- Library Clippy with warnings denied and cargo check, both with
  `--no-default-features --features mods,paired-packages` and `mods` alone.
- Formatting and Git whitespace checks.

Raw logs remain `.local/h04a-{focused-tests,world-tests,feature-on-clippy,
feature-on-check,feature-off-clippy,feature-off-check}.log`. Cargo operations were
serialized after the H03b regression run completed.

Fresh independent read-only Sol High review found no concrete correctness issue;
the coordinator verified these unchanged physical SHA256 values after integration:

| File | SHA256 |
| --- | --- |
| src/lib.rs | `d391d886310435919de74fbf5e986a65a1ba4632f639c7ccc0f9654cca9d7391` |
| src/world.rs | `0d2a344bf7ab27fa44327ef0827f747b5f124d26655828dbc6cc25c4dddd8b57` |
| src/mutation_observer.rs | `746fc0617c94118e46a56d6ed3337d3d0e7b05bfe3dad3a5614f98f8e2cf258a` |

Review used behavioral read-only restrictions under workspace-write permissions,
and performed no builds or launches. Sequence exhaustion is tested directly on
the observer; preservation of World writes at exhaustion is supported by source
inspection, while overflow is exercised through World::set itself. Dimension
swap tests exchange Worlds; actual server travel remains separate qualification.
The enabled public-tree hook passed at the local commit. No engine push occurred.

Next: `docs/design/H04_GAME_OBSERVER_PACKET.md` explicitly routes a diagnostic
observer to the authoritative Game and checks actual command/player changes in
both startup modes. No automatic enablement or callback API is implied.
