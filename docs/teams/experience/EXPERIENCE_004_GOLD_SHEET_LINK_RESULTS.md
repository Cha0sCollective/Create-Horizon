# Experience 004 — integrated gold sheet to Goggles result

Status: **ACCEPTED** for the finite integrated Survival path from one supplied gold ingot through ordinary Press/Depot collection to one Goggles table craft. Operations inspected all five frames and accepted the bounded result after fresh independent completion review `/root/gold_link_review` CLEAR on `4f760a85b01edd9f72a8faaaac918d6c737dd76c`. The prior source packet at `f9ebae21647739a6f3f88a54a532ed64e9591cb6` received fresh independent read-only Sol High review CLEAR before the root-allocated `experience-gold-sheet007-runtime` lease. No production or driver change was made for this run.

## Frozen session and evidence

- One integrated actor in disposable `TeamRuntime007`; session `G:\Create Rusty\.local\native-team-runtime-007\integrated-7jit27ml\session-00.json`. Frozen source HEAD `f732bd481cfc3d9c4720bc51e4a31e7385e18020`, native tree `471136a9f27049a58bac2707c75b18466e94e17c`.
- Raw controller responses, including initial state and each inventory/block/menu boundary: `G:\Create Rusty\.local\experience004-gold007\controller.jsonl` (84 events). The separately retained normal `quit` acknowledgment and integrity record are `exit-and-integrity.json`. Actor log and launcher exit receipt are beside the session; actor exit code 0.
- Actual captured frames in the same raw directory: `01-gold-on-depot.png`, `02-golden-sheet-on-depot.png`, `03-collected-golden-sheet.png`, `04-goggles-recipe-result.png`, and `05-crafted-goggles-empty-grid.png`. Operations directly inspected all five. Its `operations-visual-review.json` receipt has SHA-256 `f8d4dadb19722f549ea7b61db07317941a54349e4b1d1491c33dbf0a3cf11cad`.
- Post-exit checks: all 742 frozen source files match the map, with zero mismatches. Source map `0c97fc70d5ce5a8dc81bed1b5ccc21887fbc69628c94e0b0d1a6aff015c70f8a`, game `fb8ec9b36084b3a5d4d77e56ea4dcb7cd5f1b49eac4d52fc2998f50cdc2ce2db`, control `170e866057d97e73b9231c8084547d4a4a02346238db35901ef64bf3d0d6d449`, Create JAR `ef87fe5709f1ba1f5b8bb20a2925b5afb4669e178fd6d8bf10c167759eefe37a` matched the frozen receipts. The completion reviewer verified the 84 controller events and source/map/JAR/image hashes; the cached Linux binary identities remain backed by the launcher and pre/post receipts.

## Continuous item ledger

The launcher began Creative; `gamemode survival` feedback and subsequent state confirmed Survival before any fixture stock. The pinned 26.3 `gamerule spawn_mobs false` and `gamerule minecraft:spawn_mobs` both reported false. The actor head and inventory were empty, Depot held/output empty, Press speed 0/Idle. A leveled disposable floor and the reviewed Depot/Press/shaft/motor/table geometry were prepared. Motor started at `ScrollValue:0`.

| Boundary | Actor | Depot / press |
| --- | --- | --- |
| Gold grant | 1 `minecraft:gold_ingot`; no sheet/Goggles | Depot empty |
| Ordinary insertion | Gold 0; selected hand empty | Held gold 1; output buffer empty |
| Powered conversion | Gold 0 | Press Running, speed −64, `Applied:1`, `Finished:1`; Depot held exactly 1 `create:golden_sheet`, output buffer empty |
| Empty-hand collection | Sheet 1; gold/Goggles 0 | Depot held/output empty; motor returned to speed 0/Idle |
| Vanilla grants | Sheet 1, glass 2, string 1 | No replacement sheet or Goggles grant |
| Table result | Collected sheet at grid 104, glass at 103/105, string at 101; result slot 200 showed 1 `create:goggles`; cursor empty | Ordinary 3×3 crafting table |
| After taking result | Goggles 1; sheet/glass/string/gold 0; cursor empty | Reopened grid 100–108 and result 200 empty; Depot held/output empty |

The plot clear dropped incidental leaf litter. Actor inventory recorded 17 after Depot insertion and 23 near the craft; it was neither cleared mid-chain nor used in the recipe. A post-craft server item-entity query for a loose `create:golden_sheet` within 12 blocks returned `Test failed`, consistent with no duplicate; the authoritative Depot and actor states carry the primary custody proof.

This run proves only the finite path from one explicitly supplied vanilla gold ingot through the ordinary Depot/Mechanical Press and the existing Goggles table craft. It does not prove mining/world-obtainable gold, other tag members, other recipes, a dedicated-mode gold chain or full Create progression. The previous supplied-sheet Goggles craft was observed in dedicated mode; separate integrated head/HUD/cold observations used a labeled fixture. Those earlier results retain their own provenance and were not replayed.

## Handoff

Experience branch `codex/team-experience` supplied this result only. Operations accepted this finite result and owns private synchronization. The heavy runtime lease was released after normal exit and integrity verification. No further gold gameplay is scheduled for this packet; any next allocation waits for project-lead integration.
