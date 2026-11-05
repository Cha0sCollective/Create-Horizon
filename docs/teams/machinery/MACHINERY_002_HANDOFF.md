# Machinery 002 — selected stationary saw source handoff

**State: source candidate, independently reviewed CLEAR; build and gameplay pending.**
The candidate is limited to an upward-facing Mechanical Saw accepting one ordinary
dropped `create:andesite_alloy` while powered and returning exactly six
`create:shaft`. The source packet is `MACHINERY_002_PACKET.md`.
The project lead reported the source composed at root candidate `9823168`
with a fresh composition review CLEAR; that is not a runtime acceptance.

## Exact source and provenance

- Lane: `codex/team-machinery`, base `1dd69f988b6c1c3edd9cc98289d4f724dd238183`,
  reviewed implementation head `5f557a7d7d73e87a01c7f5f8bf4a727a4134a78d`.
  Later commits are documentation-only handoff and motor-fixture guidance.
- Pinned private reference: Create 6.0.11 `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`.
  The generated source recipe has one andesite alloy, six shafts and
  `processing_time: 200`; the saved processing layout, speed scaling and
  shaft orientation follow the corresponding Java saw classes.
- Current public engine source check: Delete228/create-horizon
  `dd0f3141a33851ac20fa98b807a6532471dba727`, complete 992-path tree
  (`truncated=false`) with no saw/cutting path. Tree receipt:
  <https://api.github.com/repos/Delete228/create-horizon/git/trees/dd0f3141a33851ac20fa98b807a6532471dba727?recursive=1>.
- Original Create art is referenced for an optional offline import only; the
  committed fallback models use vanilla texture IDs and include no original
  art bytes. Provenance is recorded in `native/UPSTREAM.md` and both notices.

## Implementation and shared dependencies

The new pure `engine/src/core/saw.rs` owns the 200-unit cut and 20-unit
post-application clock, zero-RPM pause, unsupported-item return and one-item
intake arithmetic. `mixins/create/saw.rs` owns dropped-item contact, selected
recipe decode, 32-slot saved inventory, result ejection, collection and
blade/input/shaft rendering. The upward saw has two horizontal shaft ports,
4.0 SU/RPM impact and item travel perpendicular to the chosen shaft axis.

The project lead should serially reconcile proposed shared wiring in
`engine/src/core/kinetics.rs`, `engine/src/lib.rs`, `runtime.rs`,
`placement_runtime.rs`, `horizon-mixin.toml`, `horizon-native.json`,
`assets/src/{catalog,lib,tests}.rs`, `tools/{catalog,content,check_distribution}.py`,
`data/content.json`, generated `content/`, and the two notice files.
The 36-file implementation diff before this handoff is `1dd69f9..5f557a7`.
This lane did not change the host, another team's domain API or the public
engine repository.

## Checks and review

- Generated selected content from `tools/content.py`: 31 blocks, 17 items and
  454 bundled files, below the 512-entry limit.
- `tools/check_distribution.py` passed for the candidate after aligning the
  development `.10` version, supplemental recipe list and notice copies.
- Focused Rust formatting and `git diff --check` passed. The pure saw tests
  and native assets tests are written, but Cargo has not run under this lane.
- Independent read-only Sol High source review of `1dd69f9..3f46ea6` found
  a shaft-pose and packet wording defect (HOLD), with no other actionable
  custody, save or clock finding. The focused repair review
  `3f46ea6..5f557a7` was CLEAR, including both shaft axes and unchanged blade
  pose. Review intervals were 22:46:17–22:49:38 UTC (201 s) and
  22:50:32–22:50:56 UTC (24 s).

## Pending bounded validation

No heavy Cargo build, native staging, integrated test or game run was started;
the project lead owns the shared host-wide lease. Once leased, run the focused
`create-engine` `saw::tests` (require a nonzero listed and executed test count)
and the selected kinetic saw test, the
`create-assets` bundled processor/content-import checks, native preflight and
the composed locked build. Pose-check both `axis_along_first` settings. The
packet's real-game milestone remains pending: ordinary one-item input, exact
six-shaft collection, unsupported return, stop/resume, normal cold reopen,
peer-visible blade/input/product/empty states and normal exits. Native intake
uses grounded-item top-contact polling, so this packet claims an item landing
while already powered; it does not claim Java fall-callback equivalence for a
previously resting item.

The accepted motor GUI and power controls may be reused as a labelled fixture.
Verify actual motor source, network and saw RPM; a new camera/GUI battery for
that unchanged control is unnecessary. Use ordinary interactions for new saw
behavior, and do not write saw Speed, input or progress fields to manufacture
processing evidence.

The packet-local `.local/machinery-002/RUN_ORDER.md` and driver take the next
root-frozen dedicated session and control binary as inputs. They were prepared
from the existing team-runtime session/call semantics; no new binary or
gameplay receipt is claimed here.

Phase intervals are tracked in the local team-operations ledger. Token usage
is not recorded in a verified local counter and remains unknown.
