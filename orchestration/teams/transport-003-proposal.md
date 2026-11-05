# Transport-003 proposal — standing Andesite Funnel to belt

Status: read-only discovery, November 5, 2026. This document does not allocate
production work or the host-wide heavy slot. The transport lane is clean at
`16f36ed6f04f132e23299825bd615ded98eb4c1b`; accepted native Create tree
`fc71cb5f0fb0c038853902a7d2f775085b34a0f1` is unchanged. Current host
source `9c7f9b2215dfed497aba5522c44f38926628648d` is clean.

## Smallest useful player outcome

One ordinary **standing, down-facing Andesite Funnel** under a supplied chest
and above an existing powered horizontal belt extracts gold1. The output is a
physical item entity, captured by the accepted belt, then delivered through
the accepted Depot and hopper to a sink chest. Redstone power pauses the
funnel with gold1 still in the source chest; removing power releases it.
Supply, power, chest and sink are explicit fixtures. This is one vertical
extracting orientation and one item, not a general logistics network.

## Reference and reuse

- Retained native upstream main `dd0f3141a33851ac20fa98b807a6532471dba727`
  and staging belt branch `7db3733953c49669b305b66edbb7d656872442cb`
  have no funnel implementation or content. Their belt core `Ending::Funnel`
  is a margin/hold option, not a block or transfer path. The accepted
  `belt.rs:1195-1278` handles belt, Depot and generic end insertion but never
  selects that ending. `belt.rs:1302-1338` already captures falling world
  item entities transactionally into the canonical belt controller.
- Pinned Create Java 6.0.11 `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`
  has the same selected funnel files as pinned 6.0.10. `FunnelBlock.java:52-65`
  defaults ordinary placement to extraction; sneaking selects collection.
  `FunnelBlockEntity.java:68-185,235-259` pauses under redstone, reads the
  inventory opposite the outlet, previews extraction, checks for an item
  obstructing the outlet, extracts up to one unfiltered Andesite item and
  spawns a physical item. The default extraction cooldown is eight ticks
  (`CLogistics.java:8`). A downward outlet reads the chest above through its
  downward face. The selected scope needs no retained funnel item buffer.
- The pinned Java `FunnelItem.java:29-51` converts only a horizontally facing
  placement on a valid belt to a distinct Belt Funnel;
  `FunnelBlock.java:168-179` leaves vertical funnels standing. A Belt Funnel
  has separate block content, shape/modes and direct belt-owner interaction.
  It is **not** this chest-to-world-item-to-belt route.
- The current host `game/machines/native_items.rs:116-169` exposes
  `native_container_slots` and `native_extract_item` with side-aware access
  and `view_changed` on commit. `game/native_hooks.rs:8389,8470,8645`
  supplies neighbor power, item-entity queries and block-entity sync. Existing
  Create code uses `Game::spawn_item` for physical output and the belt uses
  `native_set_item_entity` only when capture can commit. The belt is deliberately
  not a generic `ModContainer`; `native_move_items` cannot move directly into
  it. No new host API or multi-owner protocol is proposed for this packet.

## Narrow implementation surface, if allocated

1. Add `create:andesite_funnel` to the private content catalog and generator
   (`native/create/tools/catalog.py`, `tools/content.py`), with its standing
   downward shape, `facing=down`, `extracting=true` and `powered` state,
   item, blockstate/model, loot, creative entry and language.
   Regenerate the committed `content/` outputs,
   `mixins/create/data/content.json` and `mixins/create/assets/src/catalog.rs`.
   Use provisional models with vanilla texture IDs; keep imported original
   resources local. Update `mixins/create/NOTICE.md` for the newly used MIT
   Create reference files. Horizontal placement should be refused or kept out
   of the selected item path until Belt Funnel content exists.
2. Add a small private funnel adapter, registered in `horizon-mixin.toml` and
   `runtime.rs:105-159,760-807`. Use `placement_runtime.rs:45-110` for the
   selected down-facing state and ordinary block-item placement. The adapter
   reads redstone at its own block, previews the chest's downward-accessible
   slot through `native_container_slots`, checks the small output area for a
   blocking item, commits one with `native_extract_item`, spawns the physical
   item at the Java down-outlet position and starts the eight-tick cooldown.
   Use existing server-side block tick/sync/replication. No funnel filter UI,
   internal inventory or direct belt insertion is needed.
3. Check the ordinary chest-click placement interaction: Java's FunnelItem
   suppresses opening the clicked chest so the funnel places normally. The
   private `use_on`/placement hook may need a bounded equivalent. If the
   existing host cannot preserve that interaction, use a clearly labelled
   temporary backing fixture for the first run and leave direct chest-click
   placement as an explicit gap; do not silently call it ordinary placement.

## Minimum validation and open gaps

- Freeze source/pins and use one isolated dedicated server with actor and
  rendered peer after a lead-granted heavy lease. Build the funnel above the
  accepted belt and source chest. While powered, check source chest gold1,
  funnel paused, belt/Depot/sink0 and no item entity. Remove power; check
  source0, one outlet item event, belt controller gold1 transient, then
  belt0/Depot0/hopper0/sink chestgold1. Capture the peer's visible funnel and
  occupied→empty belt. Decode final save/player inventories/entities, require
  exact gold1 and normal process exits. Keep failed setup attempts.
- Focused logic checks cover down-facing placement, redstone pause, side-aware
  one-item extraction, outlet obstruction and cooldown. Reuse accepted belt
  capture/Depot tests and transport-002 custody evidence; do not rerun their
  broad batteries. No precise throughput or cold-reopen claim follows.
- `native_extract_item` commits before `Game::spawn_item`, whose signature
  returns no acceptance result. The one-mod fixture can prove its observed
  custody, but arbitrary native spawn-veto compatibility is an actual open
  atomicity gap. Do not assert universal conservation or invent an escrow
  protocol; if that guarantee becomes required, bring a separate host-API
  decision to the lead before editing. Horizontal Belt Funnel conversion,
  direct belt input, collection/manual mode, brass filtering and other
  orientations remain later packets.

Read-only specialists: `/root/trace` checked native upstream/staging; `/root/java_logistics`
checked pinned Java. Both closed their phase metadata. No source, content,
cache or runtime mutation occurred in this discovery.
