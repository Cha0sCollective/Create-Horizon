# Experience 004 — one gold sheet link into Goggles

Status: source-only packet prepared for the project lead's queue. No game,
Cargo, Java, resource staging or production edit was run for this packet.
This selects **one** continuous item path from a declared vanilla gold-ingot
fixture through the existing Mechanical Press/Depot to the already accepted
Goggles table recipe. It does not claim that gold is obtained by mining or
that full Create progression is complete.

## Existing source and evidence to reuse

- `native/create/content/data/create/recipe/pressing/gold_ingot.json` is the
  retained `create:pressing` recipe: one `#c:ingots/gold` input produces one
  `create:golden_sheet`. The packaged `c/tags/item/ingots/gold.json` includes
  plain `minecraft:gold_ingot`. `tools/content.py:foundation()` rewrites the
  bootstrap recipe from the retained `tools/data/recipes.json`, preserving the
  convention tag; no recipe repair is indicated.
- `native/create/mixins/create/runtime.rs:678` looks up native pressing recipes,
  resolves the one input through the host's ingredient service, tests the
  stack, and resolves one output. The existing press tick uses this result for
  the Depot input at `press.y - 2`; the existing Depot/inventory service applies
  and stores the output. There is no new adapter or save field to add.
- `native/create/content/data/c/tags/item/plates/gold.json` includes
  `create:golden_sheet`. The existing shaped Goggles recipe consumes that tag
  with two glass blocks and one string (`" S " / "GPG"`). The completed
  Experience 003 milestone proved the ordinary supplied-sheet craft and exact
  consumption in dedicated play. Its separate integrated and dedicated
  equip/head, selected HUD and cold observations retain their fixture
  provenance; see
  `.local/experience003-prep/HANDOFF.md` and its root completion review CLEAR.
- [Accepted S6 pressing](../../reference/NATIVE_S6_PRESSING_RESULTS.md)
  qualifies ordinary one-ingot iron pressing/Depot collection, client
  presentation and cold retention in its stated scope. It does **not** qualify
  gold. This packet tests only the missing gold recipe and its item handoff,
  using that established machine, interaction and controller geometry.

## Proposed single integrated run, if root allocates the runtime lease

Root owns the frozen source/binary/resource receipt, isolated disposable world,
host-wide lease and final acceptance. Machinery owns the gold press/Depot
conversion observation; Experience owns the downstream table census. Root
coordinates their handoff within one actor/session. Use a Survival actor with
operator commands only for fixture preparation and marked server queries.
The root launcher may start an integrated world in Creative; explicitly change
to Survival and observe the mode before the item baseline. Keep full controller
transcript, authoritative block queries, player inventory/menu reads, actual
output frame if captured, logs and normal exit receipt. A controller command's
`ok` acknowledgment alone does not establish the resulting world or item state.

Reuse the accepted S6 press geometry in a fresh, leveled test plot:

| Fixture | Position/state | Purpose |
| --- | --- | --- |
| Depot | `[5,64,5]` | one held input/output; ordinary actor interaction |
| Mechanical Press | `[5,66,5]`, `facing=east` | two blocks above Depot |
| Shaft | `[6,66,5]`, `axis=x` | existing power connection |
| Creative Motor | `[7,66,5]`, `facing=west` | fixture power, initially `ScrollValue:0`, then 64 |
| Crafting table | `[8,64,5]` | ordinary 3×3 Goggles recipe UI |

Use the existing controller verbs in this order, recording actual responses.
Coordinates assume the plot floor is at Y=63 and an unobstructed actor stand
within ordinary 4.5-block interaction reach. Fixture `setblock` and motor
`ScrollValue` commands prepare equipment only; they do not write press,
Depot, network or solver item state.

After root has created the plot, these are the controller argument tails (each
is sent to the frozen actor socket; `command` takes the quoted game command):

```text
command 'setblock 5 64 5 create:depot'
command 'setblock 5 66 5 create:mechanical_press[facing=east]'
command 'setblock 6 66 5 create:shaft[axis=x]'
command 'setblock 7 66 5 create:creative_motor[facing=west]'
command 'setblock 8 64 5 minecraft:crafting_table'
command 'data merge block 7 66 5 {ScrollValue:0}'
command 'gamemode survival'
state                                  # observe Survival
command 'clear @s'                     # before the item baseline only
inventory
command 'data get block 5 64 5'
command 'data get block 5 66 5'
command 'give @s minecraft:gold_ingot 1'
inventory
command 'tp @s 5.5 64 8.5'
state                                  # observe the Depot stand before aiming
select minecraft:gold_ingot
look-at block 5 64 5 up
interact 5 64 5 up
wait 5
inventory
command 'data get block 5 64 5'
command 'data merge block 7 66 5 {ScrollValue:64}'
wait 5                                # repeat boundedly with block reads
command 'data get block 5 66 5'
command 'data get block 5 64 5'
command 'data merge block 7 66 5 {ScrollValue:0}'
look-at block 5 64 5 up
interact 5 64 5 up
wait 5
inventory
command 'data get block 5 64 5'
command 'give @s minecraft:glass 2'
command 'give @s minecraft:string 1'
inventory
command 'tp @s 8.5 64 7.5'
state                                  # observe the table stand before aiming
look-at block 8 64 5 up
interact 8 64 5 up
screen                                 # observe the 3×3 menu
```

Use marked chat queries, as in the accepted S6/Experience controls, to pair
each `/data get` response with its command. Before each `look-at`/`interact`,
observe the actor at a nearby stable ground stand; neither teleport nor game
command acknowledgments establish completion. The remaining `click-slot`
source indices depend on the observed inventory; grid indices and expected
counts are fixed below.

1. Prepare the disposable floor/air, set the five fixture blocks above, set
   motor `ScrollValue:0`, switch/verify Survival and empty head, and clear only
   the actor's fixture inventory **before** the census. Verify `block` at the
   Depot, Press and table; query both block entities and observe the Depot
   empty and Press `Speed=0`. Record any unrelated natural pickup separately.
2. Supply exactly `minecraft:gold_ingot 1` to the actor. Before the press,
   require actor ingot 1, golden sheet 0, Goggles 0; Depot held/output empty.
   `select minecraft:gold_ingot`, `look-at block 5 64 5 up`, then
   `interact 5 64 5 up`. After an observed update, require actor ingot 0,
   Depot held `minecraft:gold_ingot` count 1 and output buffer empty. The
   selected hand must be empty for later collection.
3. Set the existing motor fixture to `ScrollValue:64`. Advance in bounded
   waits while querying `data get block 5 66 5` and
   `data get block 5 64 5`. Require the powered Press network Running with
   nonzero speed and a completed strike, then exactly one `create:golden_sheet` held
   on the Depot, no gold ingot and no second sheet in its output buffer.
   Do not infer success merely from elapsed ticks, a recipe JSON or a visual
   animation. Stop the motor after this observation; preserve a failure if
   conversion never occurs within the bounded run.
4. With the actor's hand empty, `look-at block 5 64 5 up` and
   `interact 5 64 5 up` to collect through the ordinary Depot path. Require
   actor `create:golden_sheet` count 1, ingot 0 and Goggles 0; authoritative
   Depot held and output slots empty. Check for any loose extra sheet in the
   test plot. This is the Machinery→Experience handoff; retain the same actor,
   item and world without granting a replacement sheet.
5. Only **after** collection, supply two `minecraft:glass` blocks and one
   `minecraft:string` as explicit vanilla fixture stock. Verify actor sheet
   1, glass 2, string 1, ingot 0 and Goggles 0. Open the ordinary crafting
   table, require a 3×3 inventory menu, and move from the *observed* source
   slots: string to grid slot 101, glass to 103 and 105, and the **collected**
   sheet to 104. Require result slot 200 to show exactly one Goggles before
   taking it. Shift-click the result and close; require sheet/glass/string 0,
   ingot 0, Goggles 1, empty relevant grid slots and empty carried cursor.

The finite item ledger is:

| Checkpoint | Gold ingot actor/Depot | Golden sheet actor/Depot | Glass | String | Goggles |
| --- | --- | --- | --- | --- | --- |
| After first grant | 1/0 | 0/0 | 0 | 0 | 0 |
| After ordinary insertion | 0/1 | 0/0 | 0 | 0 | 0 |
| After one completed press | 0/0 | 0/1 | 0 | 0 | 0 |
| After ordinary collection and later vanilla grants | 0/0 | 1/0 | 2 | 1 | 0 |
| After table craft | 0/0 | 0/0 | 0 | 0 | 1 |

Record exact actor inventory slots and Depot held/output NBT at each boundary;
`inventory.counts` alone cannot show the Depot side. Extra natural items, if
any, stay in the raw inventory record and do not become fixture ingredients.
The actor must receive **no** `/give create:golden_sheet` or Goggles in this
run. A failed press/collection cannot be masked by the already accepted
supplied-sheet craft.

## Stop condition and limits

One passing integrated chain from a declared `minecraft:gold_ingot` fixture
to one ordinarily crafted Goggles is the stop condition. Reuse the previously
accepted both-mode Press and Goggles evidence for unchanged placement, head
rendering, peer HUD, suppression and cold reopen; do not replay those batteries.
No new recipe, item, block, owner, host service, schema, driver or production
code is proposed. Vanilla glass, string and mining remain Horizon services;
they are supplied here and are not reimplemented or retested. This packet
does not establish world-obtainable gold, general tag-member parity, all
pressing recipes, all Create progression, or public release readiness.
If the observed plain gold ingot fails the existing tag/adapter or item
handoff, preserve the raw failure and return the concrete source gap to root
before any repair or scope expansion.
