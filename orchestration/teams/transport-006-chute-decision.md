# Transport-006 decision packet: one downward chute

Status: **source-only decision proposal, no implementation or runtime**. This
packet asks Operations to obtain the owner's decision on a new block-local
item owner and saved field before assigning source work. It does not change
the accepted finite Funnel route, the old Funnel HOLDs, the host, a pin, a
saved-world schema, or any acceptance rule. Transport's clean lane base is
`7073f67c5de6e731f52d707c28fcfdc22cfc2ce9` on `codex/team-transport`;
this document is the only planned Git change.

## Source and reuse boundary

The clean physical Candidate007 checkout is detached at
`f732bd481cfc3d9c4720bc51e4a31e7385e18020` (`.15`, native asset cache
18). Its `horizon-native.json` pins Horizon
`9c7f9b2215dfed497aba5522c44f38926628648d` and Mixins
`ac5d9e766380d575838c8250f2a7982281c1f44e`. Its native Create catalog
has 37 blocks, 18 standalone items and no
`chute`, `smart_chute` or filter ID/handler. The current public
`Delete228/create-horizon` main was checked at
[`dd0f3141a33851ac20fa98b807a6532471dba727`](https://github.com/Delete228/create-horizon/tree/dd0f3141a33851ac20fa98b807a6532471dba727);
its clean local snapshot under `.local/evaluations/create-horizon-08-review/source`
also has no chute/filter handler. There is no existing Rust chute to wire in.
The private import pin remains `16c5d454b1b9860da01b78474ea5206f0664d384`
from `workspace.toml`.

The local Create 6.0.10 source snapshot is clean
`ac0c444d9828da3453ae8cc65338e8de063286fb`; the authoritative 6.0.11
behavior reference is clean `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`.
Their `ChuteBlock.java`, `ChuteBlockEntity.java` and `ChuteItemHandler.java`
are byte-identical across those two local revisions (SHA-256 respectively
`cae174e6bdc24a16bc1434d4080b1b3cd265f44ef09d0cc9591b5d51a081a700`,
`2a624edae6971aff9967745b425d6a83a5c159b9ddb24623025abeb687191600`,
`6c2e232d03e3f6f4770be886def30a6558b4a701ff02593c0305ca01d88c5e5b`).
Java's chute extracts from above, inserts below while retaining an unaccepted
remainder, saves held `Item`/`ItemPosition` with fan fields, and spills a held
item on destruction (`ChuteBlockEntity.java:337-420,552-568,599-605`). It
also supports more inputs and up to 16 extracted items per cycle. The first
native profile below is deliberately narrower, not Java/world import parity.

## Proposed first behavior and item owner

Add **one filterless `create:chute`**, fixed `facing=down`, `shape=normal`,
`waterlogged=false`, placed directly between an upper chest and lower chest.
This first Creative-only route has no recipe or advancement and extracts at
most **one item per cycle** from the upper chest, irrespective of item ID or
components. Only server-authoritative ticks move it. A one-slot registered
`ModContainer` exposes no unqualified side automation or menu. The exact
owner sequence for a gold1 example is:

1. Upper chest1 / chute0 / lower chest0. The chest owns gold until
   `native_extract_item` confirms removal from its underside; the chute
   cannot pull while its slot is occupied.
2. Store that returned item in canonical `ModBlockEntity.items[0]`, set
   block-local `ItemPosition` to finite `1.0f`, rebuild the derived peer
   inventory view, and sync. The chute now owns gold1; the chest is0.
3. Gravity moves `ItemPosition` toward `0.5f`. At the lower chest, use
   `native_insert_item` from above and keep its returned remainder in the
   same canonical slot. Successful acceptance clears the slot and resets
   position; the lower chest owns gold1. A full, missing or incompatible
   lower output leaves gold1 in the chute at `0.5f`; it does not disappear,
   retry extraction, or create a world item.

The selected transfer commits at `ItemPosition=0.5f`. Java instead simulates
downward output after the item crosses0.5 and commits after it crosses0
(`ChuteBlockEntity.java:173-178`). The pinned `native_insert_item` commits and
returns the remainder but has no simulation flag. This **earlier transfer is
an explicit first-profile deviation** requiring the owner's approval alongside
the one-item extraction limit and no open-air drop.

The **only authoritative item state** is canonical slot0, saved by the
existing host Mod container path. Add block-local NBT `ItemPosition` as one
finite float in `[0,1]` (empty default0; occupied at entry1 and blocked0.5).
Any `Inventory` NBT or client display copy is derived from slot0 and must be
rebuilt from it after a cold load or change; it is never a second source of
truth or a fallback owner. Invalid or missing position with an occupied slot
must reset to0.5 and attempt only a verified lower-container insertion on a
later server tick; it must not eject or re-extract. No Java `Item`, `Pull`,
`Push` or `BottomAirFlowDistance` field is adopted for this no-fan profile.
This is a new block-local owner/field
contract needing the owner's approval. It does **not** propose a new global
save schema, engine owner framework, host API or pin.

Normal break and block replacement must use the registered-container host
pre-removal spill **exactly once** for slot0; do not also spawn an item from
the NBT view. A cold save/reopen while blocked must restore one canonical
item and its position, show one occupied peer view, and later release exactly
one item to the lower chest. Empty and occupied peer sync must be checked
with a **client-local replicated slot read** against server counts. A server
`/data` reply displayed in peer chat is not client-local evidence. The fixed
normal, no-fan chute follows Java's hidden-item presentation
(`ChuteRenderer.java:29-31`), so geometry screenshots cannot prove an item
sprite or item occupancy. If existing native client facilities cannot expose
that read without a host change, stop and return the precise gate to
Operations; do not substitute an overlay or silently add an inspection API.
Ordinary save/reopen and replacement are selected; crash atomicity, arbitrary
old saves and Java world import are not claimed.

Candidate007 already calls `native_container_slots` and
`native_extract_item` in `mixins/create/funnel.rs:54-75` and consumes the
`native_insert_item` remainder in `mixins/create/belt.rs:1278`. Its existing
registered one-slot container pattern is in `runtime.rs:1182-1207`; pinned
Horizon saves Mod NBT plus canonical items and spills registered containers
before ordinary removal (see `MACHINERY_004_CRUSHING_WHEELS_DECISION.md:77-93`).
These sources support a no-new-host-API design, **subject to focused
validation**. If an existing callback cannot safely observe or commit the
selected ownership transition, stop for a concrete host/owner decision; do
not add a general transfer framework under this packet.

Unsupported here: side, belt or world-item input; adjacent-chute, diagonal,
upward or fan routes; smart chute, filtering and filter UI; open-air item
drop; waterlogging; Survival obtainability. The open-air retention is a
bounded **difference** from Java's chute, which can drop into open air. A
future extension must receive its own reference cases and acceptance gate.

## Exact content-file budget and art provenance

Frozen Candidate007 `native/create/mixins/create/data/content.json` SHA-256
`5f564b5f5f36254dfdceb0814b3a3cea0fbf579788f405beae78b405f1d7b754`
has exactly **499** bundled text-file keys. The native asset loader rejects
more than **512** (`assets/src/lib.rs:45-49`): **13 slots free**. This is a
content-file cap, not a saved block-state palette. The proposed native
generator adds exactly these **five new bundle paths**, all currently absent:

| New path | Selected purpose |
| --- | --- |
| `data/create/horizon/block/chute.json` | One fixed downward/normal/dry block and registered item declaration. |
| `assets/create/blockstates/chute.json` | One fixed state pointing at `create:block/chute/block`. |
| `assets/create/models/item/chute.json` | Item parent `create:block/chute/block`. |
| `assets/create/items/chute.json` | Item model selector. |
| `data/create/loot_table/blocks/chute.json` | Drop the chute block; canonical slot spills separately. |

Result: **504/512**, eight slots remaining. Update existing English/Russian
language, Creative tab, generator and catalog files without new bundle keys.
No recipe, advancement, filter, diagonal, fan or smart-chute content enters
this profile. `check_distribution.py` requires the new block ID in
`catalog.BLOCKS`. The importer must skip that block's full Java blockstate
and item-model auto-import, add only `chute/block` to `PARTS`, and use the
selected native blockstate/item JSON above. Its existing model traversal
then imports **four original-art files** from the separately pinned 6.0.10
NeoForge JAR: `models/block/chute/block.json` and textures
`block/chute.png`, `block/chute_hole.png`, `block/industrial_iron_block.png`.
All four entries were confirmed present in JAR SHA-256
`ef87fe5709f1ba1f5b8bb20a2925b5afb4669e178fd6d8bf10c167759eefe37a`.
Original model/PNG bytes stay in the private supplied-JAR cache, not the
distributed text bundle. The **native asset-import cache version** must
advance from18 to19 because its catalog changes; that is not a saved-world
schema or JAR/engine pin change. Model resolution and actual peer appearance
still require focused proof. Do not raise the 512 cap to make this packet fit.

## Decision and later validation gates

Operations should seek the owner's approval of the one-slot chute owner,
`ItemPosition` and derived-view cold/removal rules, the Creative-only
downward scope, transfer-at0.5 timing and explicit Java differences above.
Only after that decision may a bounded source packet be assigned against a
stable accepted/frozen
Candidate007 base. This decision document does not authorize implementation.

If approved, validate the exact five-file budget and private art import,
format/compile and focused owner tests, then one ordinary integrated and one
dedicated/two-client route: chest1→chute1→lower chest1 with all other owners0;
full/missing output retention; cold reopen while chute-owned; ordinary break
and replacement spill once; actual peer client-local occupied/empty state
paired with geometry frames. The hidden normal item sprite is not a visual
acceptance target. Preserve failing receipts. No historical battery, broad
chute feature set, public
release, or general host/schema work follows from a pass.

Read-only discovery checked actual branches/revisions and changed no source;
no Cargo, Java or game lease was used. Independent read-only Sol High
decision review: **CLEAR for this decision packet only** after a focused
presentation/timing correction and read-only delta review. Transport timing
is in the team phase ledger;
token counters remain Operations-owned and are unavailable in this packet.
