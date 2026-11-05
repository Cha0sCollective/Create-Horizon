# Transport-002 proposal — two-belt handoff

Status: read-only discovery proposal, November 5, 2026. No transport-002 code,
build or gameplay work is authorized by this document. Proposed base is the
accepted private integration `1ac294026cf7f660a4d2901602081f77686f1a46`.
Operations must allocate the packet and host-wide runtime slot before execution.

## Smallest useful outcome

An ordinary player builds two separate short, powered, collinear horizontal
belts with the existing connector. Gold1 dropped on the first belt crosses the
seam to the second, then reaches the accepted Depot and stock hopper/chest.
Stopping the receiver with the existing Clutch/lever holds the item on the
source belt; releasing it transfers exactly once. This qualifies a distinct
controller-to-controller owner handoff, beyond the accepted belt-to-Depot path.
Use supplied materials and power as explicit fixtures. No survival crafting,
general routing graph, side-entry or opposing-direction claim follows.

## Reuse and reference

- The clean retained native upstream main `dd0f3141a33851ac20fa98b807a6532471dba727`
  and staging belt branch `7db3733953c49669b305b66edbb7d656872442cb`
  have no funnel, chute or filter implementation/content to select. The
  staging direct belt path is already in the accepted private source.
- `native/create/mixins/create/belt.rs:1195-1243` selects a neighboring belt,
  reads its controller/index and uses `insert_direct`; `:774-808` guards and
  syncs the destination controller write. The core
  `engine/src/core/transport.rs:292-294,574-595,636-679,790-809` refuses
  stopped/opposed/occupied entry, retains the source on refusal and preserves
  the transported record on acceptance. Its direct-chain conservation test is
  at `:1063-1125`; prior 17-test execution may be attributed, not repeated.
- Pinned Java 6.0.11 `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`
  `BeltBlockEntity.java:449-540` rejects stopped/incompatible/opposed or
  occupied targets, preserves whole input on refusal, copies the full record
  on acceptance and offsets a collinear seam by 0.26. `BeltInventory.java:198-219,349-373`
  retains the source until full acceptance and checks active/pending target
  stock. These two Java files have no diff from the pinned 6.0.10
  `ac0c444d9828da3453ae8cc65338e8de063286fb` snapshot. This is selected
  behavior parity, not full-version equivalence.

## Minimum exit checks and dependencies

1. Project lead first confirms the accepted source/pins remain the input and
   grants the one heavy slot. Use a new lane map, drivers, stage/world paths and
   ordinary connector setup; preserve all transport-001 and prior raw evidence.
2. One disposable dedicated server with actor and actual rendered peer: verify
   two different canonical belt controllers, gold1 on source, and no receiver
   stock/drops while the receiver is stopped for a bounded observation. After
   ordinary lever release, verify source0/receiver1 at handoff, then
   source0/receiver0/Depot0/hopper0/chestgold1, actorgold0 and no drops. Capture
   a peer view of the receiver occupied and its final empty state. Decode the
   normal final save and record all process exits.
3. Reuse unchanged single-chain cold-reopen, Depot peer-clear, stop/reverse and
   ownership evidence with attribution. Do not rerun the historical battery.
   This packet adds no cold-reopen or transfer-latency claim. If actual gameplay
   exposes a defect, keep a separate HOLD and repair only the demonstrated
   belt adapter/core path with fresh independent read-only Sol High source
   review and focused leased build/game checks.

No host, shared owner, schema, asset format or acceptance-rule change is
proposed. Funnel/chute/filter blocks require new content and adapters; Java's
Andesite Funnel also has placement, extraction, redstone and cooldown behavior,
and funnel-on-belt becomes a separate Belt Funnel. Defer those to later packets.
Transported Press/fan processing remains with Machinery coordination and is
outside this handoff. Primary operations owns shared integration and release
decisions; no new resource reservation is held by Transport.

Read-only specialists: `/root/trace` for retained native upstream/staging and
`/root/java_logistics` for pinned Java. Both recorded separate phase metadata
under `G:\Create Rusty\.local\team-operations\transport\phases`; operations
collects actual token counters. No production source changed during discovery.
