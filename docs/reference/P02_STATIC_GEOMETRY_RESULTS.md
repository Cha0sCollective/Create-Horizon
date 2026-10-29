# P02 bounded native static geometry results

Accepted private source candidate c821d21d42639d699a1fe5248b09426b7bd35760 after
different independent read-only Sol High design/source/receipt reviews. Seven
files change +1249/-15; no new dependency, lock, engine change or live activation.
Boundary: P02_NATIVE_GEOMETRY_PACKET.md. Personal source-informed reference use
preserves clean detached Create7ff2760e; no Create contributions are planned.

The pure adapter derives canonical nodes/edges from complete dry static block
records: X/Z shafts and cogs, EAST/NORTH motors, mutual adjacent shaft faces and
same-axis large/small diagonal meshing. It checks legal cog placement, duplicate
IDs/positions, unsupported domain/physical couplings and widened coordinate
differences. Existing solver arithmetic/errors remain unchanged. Small/small and
perpendicular large/large coupling, Y/other facings and unknown kinds refuse.
All-quadrant/translation/ID-permutation checks are source-derived software support;
measured Java equality covers only the four accepted horizontal layouts below.

The development JSON bridge adds exact geometry records alongside the unchanged
legacy nodes/edges form. Two source-review findings were corrected before any
candidate execution: the sandbox host now checks geometry-name uniqueness across
the full batch, and runner source hashes include the new geometry integration
test. The narrow host-file addition passed different independent design-delta
review; source and later formatting deltas pass independent review. Repeated
legacy names retain their prior policy. Manifest/capabilities, one-case dispatch,
identity/reply/side-effect checks and engine/source drift refusals stay intact.

## Executed qualification

Pinned Rust1.99.1 offline/locked checks pass:12new geometry test groups,10unchanged
kinetic tests,3host tests, core/host Clippy with warnings denied, both workspace
format checks, core Wasm compile check and Python compilation. Tests use literal
expected edges/results and malformed-input controls. No failed runtime attempt
occurred; initial source-review findings were resolved before execution.

Both accepted Java captures are independently loaded in full:4fixtures/240rows
each, with selected ticks20/50/60 giving12checkpoints/48machine observations per
capture. Inputs come from authored coordinates/orientations/settings, without
captured speeds/parents/networks or declared edges as solver inputs. Native and
actual server-component sandbox outputs match Java theoretical/effective speed,
stress/capacity, derived operating state and authored exact connectivity.
All four canonical output files are byte-identical, SHA256:
ffd3bb047489430cff620804ff1659a0db02b6d8584a53b334af35b8d008db28.

| Evidence | Original run |
| --- | --- |
| Native/sandbox geometry against both horizontal captures | 5ef1c474-7d92-406c-ac43-845184baeee9 |
| Existing E/F declared-topology sandbox regressions | 105aa62a-6c3f-4baa-bb53-927655287089 |

Both runs bind clean engine63a7d62fae27dcd9ce8bfbe1209616f4e4988607 and clean
private candidatec821d21, with completed source/reference/engine/artifact drift
checks. Their shared core Wasm SHA256 is
7b09bf64bf7de291c6d81339acfec2cbec7bda4b8fb86f97e0bbb38f875b93d1;
component package SHA256 is
9c9d7256f72c3d33baa5c76616146b6d02222255584fde7928cbf44a6446f817.
Geometry native executable SHA256 is
6d60d046564ae6da714fcc9c00ccb3d1c4e49f426fa4899b1c74e1df7fc12487.
The sandbox remains an isolated TestGame with one virtual fixture actor and only
the existing SDK players capability; no live world or player server is involved.

Eight actual-probe controls pass: axis/facing/reported-ratio changes differ with
exit1 in both runtimes; duplicate valid geometry cases refuse with actual exit2,
the duplicate-name diagnostic and no partial stdout in both. Twenty-one derived
software replay controls separately pass: one baseline0,19malformed outputs2 and
one valid RPM difference1. Those copies test comparator rejection, not additional
captures or alternate native implementations. Original outputs remain unchanged.

Unchanged native and sandbox E/F comparisons still match16selected checkpoints/
62machines per capture; wrong-ratio controls differ with exit1. Retained six-case
730-row, fifteen-case1730-row and arbitration800-row pairs still compare equal.
No old Java GameTest execution or new Java capture is claimed by those regressions.

The final independent reviewer directly rehashes every recorded source/reference
file, retained packages, canonical outputs and derived replay/payload, and reads
both original240-row captures to compare all selected machine values. All command/
exit/completion evidence and unchanged source identities pass. Linux-cache raw
Wasm/native binaries are unavailable through native reviewer filesystem access;
their final checks are supported by the completed coordinator runner receipts.

Raw originals stay outside Git under workspace-root .local/p02/wasm-runs/<run>.
Coordinator receipts/logs are .local/p02g1-*; derived controls are named in
p02g1-controls-result.json. Prefmt sources and initial manifest remain archived.
Verbose Git CRLF warnings remain in raw logs; all checks complete successfully.

This qualifies static bounded geometry and selected settled sandbox equality.
It does not qualify propagation timing, source-parent selection, arbitrary-axis
conformance, multi-source arbitration, live graph ownership, persistent machines,
item processing, peers, retained rendering, performance or full P02/G1/G2. Live
S2 integration still requires G1; S3 remains the original five-component subset.
