# Retained core native metadata results

Status: ACCEPT FINAL by different independent s1_metadata_final; S1/G1 remain OPEN.

## Candidate and scope

Engine candidate b7cdb4220029c7f5aff6afb18f3813fd13fdcc76, tree
64223bde93195136434dc197e30015126b67b8c3, sole parent
33e947a97d7186f5ee4bfc1f94aa820c8c5a0d54, branch
codex/s1-retained-metadata-core. Exactly eleven assigned Rust paths; game.rs
changes only the paired_close module declaration. All eleven tested raw files
are byte-exact Git commit objects, verified by commit-comparison b98ad7d5.

The dormant service keeps an exclusive borrow of the original Game, private
batch/attempt association, immutable native retry input and at most one actual
IO handle. Actual handles are registered before dispatch and harvested only when
finished. Errors, panic and deadlines preserve original owners and failure facts;
late success requires explicit Retry. Every selected family preflights before
any original saved marker changes. No original Game reference crosses the worker.

Selected families are world/merchant random sequences, player stats and
advancements, CommandStorage, stopwatches, border, scoreboard, bossbars, schedule
and clocks. Native JSON/NBT shapes, replacement artifacts and sparse/default
rules remain native-owned. Stopwatch milliseconds are sampled once as exact i64;
retries reuse that projection. Advancement tree loading retains and reaps actual
prefetch helpers, including selected legitimate empty trees. No ordinary save,
close, startup, network caller, constructor or Game job field was activated.

## Qualification

Fresh independent s1_metadata_source_review accepted exact source manifest
`da354db417ebfcfd0d7c317c3dfae8b0577d5beb7281d91b57bcee65afb3af38`,
qualifier `f2c356ca6ab65aad965c35fb9ae0c0922fde7004d352ca93078470a52dad015c`
and expected inventory
`efce5d4b0b333d0be08d6a889422981f85912df1376ae809938ec84be9f65ff7`.
Source/tool receipt f2aac9f1; final input58ec85d2 indexes72 exact artifacts.

- 44 normal cases PASS on their first runtime execution:32 new isolated actual
  Game cases, one exact-i64 library case and11 relevant native-save regressions.
  Actual normal subprocess runtime331.71 seconds. Ten fixed family holds exercise
  native errors, expired Pending/Busy, actual late failure, repair and same-input
  replay. Partial namespaces, native clock merge, sparse/default reset, notification
  preservation, stale/foreign association and all-batch preflight are covered.
- One direct prebuilt expected-fatal child PASS: exact marker
  HORIZON_NATIVE_METADATA_OWNER_DROP_ARMED, exact diagnostic
  `fatal: unresolved native metadata owner`, SIGABRT(-6), explicitly reaped,
  core dumps disabled;11.23 seconds. Exit91 fallback prevents a different owner
  backstop from qualifying the case.
- Six static checks PASS: full formatting; Clippy with runtime+paired+controls,
  without controls, paired without runtime, and feature-off; public-tree scan.
  Static subprocess runtime210.18 seconds. Build uses pinned Rust1.99.1,
  offline dependencies, two jobs and the same physical checkout target-s1ra1.
  Actual lib/bin artifacts are opt0, debug assertions and overflow checks enabled.
- Resources are the actual pinned26.3-cf62fe-smoke inputs, manifest f3514018 and
  complete input map cc72b7c2. Source/tool/binary/resource identities are checked
  at batch boundaries. Each Game process has a fresh cwd/home/data/config/cache/
  temp/game/launcher root; playable saves, references and account data are untouched.

Exact receipts: normal84788d41, fatal741c6487, checkscef92c56,
buildf2bb4388, commit comparisonb98ad7d5. All dirty-parent qualifier modes
are RETIRED after the clean candidate commit; no repeat qualification is inferred.

SOURCE review first required a real nonempty advancement positive and the missing
retained family failure branches. Compiler then caught unsupported TrySendError
into_inner (source618) and unused inspection helpers/style warnings (source929).
All sources and packet artifacts were preserved before each correction in
s1md1-attempt0..7; failed compile receipts/logs remain preserved. No failed normal
runtime case or hidden broad replay is relabeled as success.

## Reuse and limits

The accepted C0a preparation, native writer, acquisition and persistent Guest
results remain evidence at their actual predecessors. This packet does not rerun
or relabel those batteries. Its larger focused count covers distinct new native
family failure branches, with cheap relevant regressions rather than broad replay.

No world/image completeness, CoherentImage, CloseCommit, closed Game, authority
lock release, admission/Running readiness, connected client behavior, S1/G1 or
performance gate is established. Native codecs/filesystem operations remain
indivisible; no hard polling/UI latency, atomic batch, fsync or power-loss promise.
Maps, fight, trader, portal arrivals and raids/omen remain a later bounded metadata
assignment; coherent final save and ordinary lifecycle remain later S1 work.

Different FINAL receipt876d8a80 accepted exact candidate and all72 indexed hashes. The reviewer could not independently rehash Linux executables through its native filesystem surface; binary correlation relies on the accepted runner pre/post hashes, matching build receipt and actual Cargo artifacts.

Public source/privacy CLEAR for the exact eleven paths. Fresh GitHub PUBLIC metadata and exact origin verified before normal push; history hook PASS. Remote branch codex/s1-retained-metadata-core verified at b7cdb4220029c7f5aff6afb18f3813fd13fdcc76. The authored private result is checkpointed separately.
No PR, merge, tag or release is authorized by these results.
