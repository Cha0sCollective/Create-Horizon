# G0 feasibility decision

Status: G0 accepted for bounded engineering experiments on October 27, 2026 after
the strengthened E/F reference pair and independent review. This authorizes experiments, not
a supported host API, playable mod or release.

## Evidence and decision scope

The pinned Java environment runs the real Create block entities. Fifteen cases
cover direct transmission, three cog ratios, source disconnection, finite stress
equality/overload/recovery, milling speed/interruption/reversal, storage/input
edges, retrieval/overflow and the motor's value-setting handler. The owner exercised
the actual motor edit and observed its rotation change; the isolated save records
−256. See `../reference/S0_RESULTS.md` and `EDGE_CASES.md` for exact scope.

The owner selected reference-compatible full-output result loss (R01). That choice
does not relax transfer/recovery accounting. S0 does not prove every lifecycle,
orientation, source-arbitration, network or save case needed for the MVP.

Horizon runs the default integrated server from the exact pinned base and passes
its existing two-client chest/inventory checks. These are vanilla baselines, not
new machinery. H01 identifies the missing generic services and their test backlog;
the existing draft PRs do not supply a completed machine lifecycle or save path.

## Accepted experimental direction

| Decision | Direction and owner/fallback |
| --- | --- |
| Provenance | Personal source-informed port; accurate source pins/attribution. Owner authorized Create reference engagement; no Create contributions. No clean-room claim. |
| Engine dependence | Local required Horizon integration build based on the pin. No upstream merge is required for experiments. Coordinator owns generic services; private core remains independently testable if host work blocks. No public push or upstream submission is authorized by this decision. |
| Contract | H01 version 0.1 is the experimental design direction, including captured dimensions/generations, per-level phase, atomic staged effects, coherent checkpoints, exact peer identity and retained rendering. It is not an implemented or stable API. A conflicting live result requires a recorded revision. |
| Content/package | Namespace `create_horizon`; one content owner and separately sandboxed client/server halves. Initial compatibility is exact engine/package/content match. |
| Assets | Original diagnostic boxes/axes and authored simple textures for host proofs. Reference assets remain local; no redistribution decision is implied. Functional five-component models remain later P06 work. |
| Hardware | Development profile D0: AMD Ryzen 9 7900X, 24 logical CPUs, Ubuntu 24.04 WSL2 with about 15 GiB RAM/4 GiB swap, Mesa 25.2.8 llvmpipe software Vulkan, 1920×1080 offscreen. This is a named development profile, not hardware-GPU release qualification. |
| Performance | Retain B2 (1,024 nodes/16 networks/16 active mills/two clients), 20 TPS and provisional added-work budgets from the plan as future targets. S0 does not ratify an unmeasured B2 or GPU-cost promise. D0's B0 baseline only establishes empty-machinery feasibility; confirm suitable release hardware and budgets with H14/Q04 before a performance claim or delivery forecast. |

The measured D0 one-minute warm observation ran 2,559 frames over 60.04 seconds,
42.62 average FPS, p95 frame 56.41 ms, and mean/max sampled **smoothed** server
tick time 0.568/1.844 ms. The latter is not a raw tick percentile. The parent
five-minute collection was deliberately stopped before its first window completed
so compilation could not contaminate it. Raw run:
`.local/s0/baseline-runs/d80a6edd-e3db-42a2-87ad-62442574406c/` in the parent workspace.
A fresh five-window collection follows the corrected E/F captures. No performance
pass is inferred from an aborted window or from software rendering.

The strengthened E/F pair is `b531ce69-28ce-4a5a-a8a2-e5ea3b10ebe2` /
`d0528c63-86c7-416b-9dbf-5ee33c1e6a50`; all 15 tests pass in each. The 1,730
samples and source/JDK/adapter/dependency/configuration records compare equal.
The fresh B0 collection is `a5c799f5-f1d2-42b5-a937-10643766fd71`; five windows
completed with a clean exit. See `../reference/BASELINE_D0.md` for individual
results and the first window's background-compilation caveat. These data do not
count as B1/B2 or hardware-GPU performance qualification.

Performance budgets remain provisional rather than pretending that this software
renderer validates hardware-GPU targets. This explicitly defers that portion of
the plan's G0 budget ratification; it does not waive N03/N07 or close Q04. A switch
to a hardware renderer requires a new baseline before comparing costs.

## Bounded next work and stop conditions

The private P02 first packet is settled single-source algebra as defined in
`P02_FIRST_PACKET.md`: explicit topology, measured signed ratios, disconnection,
stress/equality/overload/recovery and motor value limits. No live-block or full
trace-timing claim. Stop with native tests, a useful differential probe and an
independent review; unsupported source arbitration and orientations stay explicit.

The first host experiment remains one original generic counter machine, one owned
slot, one dimension-scoped transaction and one retained diagnostic instance. Split
it into small reviewable steps. H02 first validates paired package identity and
side execution; H03/H04 then establish content/lifecycle before persistent records.
Do not advertise persistence merely because the counter increments.

H08 is a material risk: normal machine/chunk/player/entity saves need a coherent
checkpoint and propagated errors. Do not add an installation-level sidecar and
call it durability. If the bounded save experiment requires a broad vanilla save
redesign, stop that host packet and record concrete options/cost before expanding it.

G1, G2 and G3 remain open. No original product acceptance test is waived by a
scaffold, algebra check or successful reference run. The current owner instruction
ends this work session at 7 p.m. Central on October 27, 2026.
