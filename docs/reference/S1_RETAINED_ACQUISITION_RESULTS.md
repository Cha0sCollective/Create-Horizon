# Retained acquisition prerequisite results

Status: FINAL ACCEPT by different independent s1_acquisition_final for
b9463a5835da3203c0521704548092825a57e67e,
tree b0f191b6d6a122f2caf8eb647569ad2a0bf8ee6b, sole parent accepted adoption
e369cac6da8f6e0b604a928fcb2e5233be3cf17c. Exactly three permitted Rust files,
1260 additions/173 deletions. This is a dormant engine component, not C0a or G1.

Native and authority acquisition now retain actual acquired lock files before
fallible header/path work. Same-file repair retains the original failure;
checked finish transfers only a ready owner. Explicit lock-only release remains
distinct from complete world shutdown. Unresolved custody aborts before field
destruction. Existing trusted-path boundaries and legacy acquisition stay intact.
All three GeneratorData loader helpers are lexically scoped, including partial
spawn failure, early error, optional-result omission and child panic.

Author s1_c0a_packet_design; SOURCE/tool s1_acquisition_recheck; different FINAL
s1_acquisition_final. Private dd54742 preceded source authoring. All scoped
sources and packet artifacts were preserved before every correction.

Completed qualification follows adopted VALIDATION_WORKFLOW8631c054:

- Nineteen fresh normal cases on source manifest0330662b, plus five passing
  unchanged lock cases from manifestc6263c47:24 total, with their real revisions.
  Includes actual registered native roundtrip in both server roles and all five
  loader cases, including four real-resource families and a deterministic bad
  nonwaiting-return control.
- Four fresh separate direct fatal processes, exact armed markers, SIGABRT and
  reaping, with core dumps disabled.
- Four fresh static rechecks:format, runtime-on, runtime-absent and public tree.
  Reused feature-off/world checks from the original verified six-check run:
  only authority test fixtures changed, excluded from those configurations.
- Three native test artifacts retain opt0, debug assertions and overflow checks.
  Actual resources were frozen as28176files and verified at test batch boundaries.

The original runtime run remains failed:five lock cases passed, then an authority
fixture tried to write a regular file through a trailing directory separator and
received EISDIR, triggering the custody backstop. The correction normalizes only
test mutation targets; production behavior is unchanged. Original logs and
snapshots remain preserved. No failing run was relabeled as passing.

Unchanged77save/writer normal and21writer fatal coverage remains inherited at its
original accepted revision. Existing11worldgen algorithm cases were deferred/
reused; full broad replay was superseded for this component, not passed. Later
composition and full connected G1 requirements remain unchanged.

Current source manifest0330662b3b469dbf65b7e5f4260b63a1b878efac963ac2d423121edc3e1c83d1;
all three raw files match committed blobs exactly, without normalization.
Final input716544020739900f352aaec1556dd30ea83a3eb5b51d14db2aa74ce29c952ab6;
normald8906c2b; fatal71e7334c; static125215df; build26b65c91;
commit/source comparisonf7d7c991. Detailed receipts/logs remain local.
All dirty-parent qualifier modes retired after the clean child commit.

The different FINAL reviewer also cleared the exact three-file generic public
source boundary. Under existing owner authorization, normal pre-push history
checks passed, fresh public metadata and exact origin matched, and remote branch
codex/s1-retained-acquisition was verified at b9463a5835da3203c0521704548092825a57e67e.
No PR, merge, release, ordinary activation, G1 or live S2 is implied.
