# Native S5 bounded mechanical-piston results

Independent Sol High completion review CLEAR for the selected S5 profile. This is the selected
one-pole, east-facing normal `create:mechanical_piston` with Z shaft axis in a
same-chunk Creative TestTerrain world with mob spawning disabled. The initial
client state reported one mob; no claim of an empty mob population is needed
for these piston observations. It moves one inert stone and
one pole/head unit. The private trial motor carrier is separate and is not
counted as the actuator result.

## Exact source and checks

- Java behavior reference: Create 6.0.11 `7ff2760e`; local visual input is
  separately pinned Create 6.0.10 JAR SHA-256
  `ef87fe5709f1ba1f5b8bb20a2925b5afb4669e178fd6d8bf10c167759eefe37a`.
  Imported Rust port `16c5d454`, official Horizon `fc021038`, Mixins
  `a1e13326`, native API 359, Minecraft 26.3 and Rust 1.99.1 remain pinned.
- Private branch `codex/native-s5-mechanical-piston` starts at reviewed and
  private-synced carrier feasibility `a6668abf`. It adds normal piston/pole/head
  content and one native piston owner, two shaft ports with 4 stress per RPM,
  finite stone/head body planning, derived peer geometry and guarded cleanup.
  Source review first held four ownership/sync/sweep issues, then one body-push
  ordering issue. Both deltas were corrected; the final narrow Sol High source
  review was CLEAR. Original HOLD findings remain in review history.
- Pinned build 02 input map SHA-256
  `58adc2a9f85ce9870828dafa630e51d20f5a9ec14f50e00b969efe18fedef860`
  passed in 214.96 s. Game binary SHA-256
  `1d0cbf80a950e70eba34c7db7fd527576eded2f4ea1d76a1cc7ed099cc8833d6`;
  control binary
  `507b23e1a8f68a3f11da9c677fd2ec2bf52f93e73d7ebfe2affcc9f7a7863500`.
  Pinned Rust format check and exact generated-client Clippy both exit 0;
  Clippy reports 25 non-fatal warnings. The focused mechanical-piston kinetic
  test passed (one selected, 57 filtered) in 3.66 s; its existing receipt is
  `.local/native-s5-01/actuator-core-test2.log`. Explicit final format and
  `git diff --check` exit-0 receipts are in
  `.local/native-s5-actuator-01/check-receipts.json`.

## Focused dedicated game milestone

The disposable run is `s5-actuator-runs/dedicated-tbnin77l` under the owned
Linux cache, with raw packet commands/replies in `.local/native-s5-actuator-01`.
Its two dedicated sessions each used one server and two rendered clients;
all six processes exited 0. The first fixture placed blocks through commands
to isolate actuator behavior. Later, after safely mining the base, the actor
placed a fresh piston, pole and stone through normal `interact` actions; their
resulting states and powered stroke were checked separately. During the first
setup, a newly placed motor briefly had its default speed before its value
was set to zero; the checked starting state was fully retracted with one pole
and one stone.

| Check | Observed result | Raw packet |
| --- | --- | --- |
| Shaft and stress | Piston east/Z receives -64 RPM on its north/south ports, capacity 1,048,576, stress 256; +64 on reversal. At 1 RPM magnitude, stress 4. | `step02`, `step03`, `step04` |
| Extension | Phase 2, offset 1; pole cell x5 air, head x7, sole stone x8, no item entity. | `step02-output.jsonl` |
| Normal retraction | Phase 0, offset 0; pole x5, head cell x7 air, stone stays x8, no item entity. | `step03-output.jsonl` |
| Fractional stop and reversal | At -1 RPM, Phase 1 has both capture flags 1 and offset 0.048828125; zero RPM places at grid 0 with stone x7/pole x5. A separate sign reversal from offset 0.05078125 places at grid 0 with the same two originals and no drops. | `step04-output.jsonl`, `step15-output.jsonl` |
| Neighboring collision | An oak fence at x8,y64 intrudes into the swept stone cube. Capture is refused before either original disappears. Adding the fence after capture stalls at offset 0.0625 with both pieces still owned. | `step07-output.jsonl`, `step10-output.jsonl` |
| Peer push and replica | Server moved peer x8.50 to x9.31 during extension; peer's own position is x9.31 and its block replica has head x7/stone x8. Side-view extended and fractional-motion peer images were visually inspected. | `step06-output.jsonl`, `step06-peer-output.jsonl`, `step05c-output.jsonl`, `step16-peer-motion-output.jsonl` |
| Blocked owner removal | Ordinary mining while fence blocks restoration leaves base, both capture flags and both proxy cells; owner reports `RemovalRequested=1`, `removal-blocked`. Clearing fence yields `removal-ready`, stone x7 and pole x5. Mining retry removes only base; stone/pole remain once, no dropped item in Creative. | `step10-break-output.jsonl`, `step10-after-break-output.jsonl`, `step11-output.jsonl`, `step11-after-output.jsonl` |
| Active-motion cold reopen | Before normal close, Phase 1 offset 0.05078125 has both captures 1. Server and clients exit 0. Reopening the same world lets the -1 RPM piston finish at Phase 2, head x7/stone x8, pole x5 air, no item entity. Both clients rejoin; peer block replies agree. | `step08-output.jsonl`, `start-console.log`, `start-reopen-console.log`, `step09-output.jsonl`, `step09-peer-output.jsonl` |
| Ordinary placement | Actor interactions create east-facing/Z-axis piston x6, east-facing pole x5 and stone x7. A real shaft/motor then drives it to head x7/stone x8 at -64 RPM/stress 256 without drops. | `step12-output.jsonl`, `step13-output.jsonl` |
| Vanilla regression | Vanilla piston separately extends head x4 and stone x5, then retracts head while leaving stone x5. | `step14-output.jsonl` |

The first peer-camera attempt (`step05b-output.jsonl`) could not see the
head's east face behind the stone and ended with a `look-at` error. The next
side-view attempt (`step05c-output.jsonl`) succeeded and supplied the extended
peer image. This was a camera/driver correction, not a source fix.

The generated head has no separate item declaration; its authored loot entry
returns `create:piston_extension_pole`. The selected milestone checked its
world form and one-pole ownership through both endpoints and removal, but did
not separately qualify Survival mining of the settled head.

## Limits

This result supports only the selected one-cell normal-piston profile on the
pinned host. Other facings/shaft alignments, more poles, connected structures,
sticky pulling, block-entity payloads, fluids, riders, mounted interactions,
cross-chunk travel and general contraption assembly are not qualified. Static
world and body clearance is conservative for the finite full-cube example.
Normal close/reopen is demonstrated; interrupted multi-owner crash atomicity
is not. The run does not qualify full Create, the modpack, an existing Java
world, integrated S5 play, or a public release.
