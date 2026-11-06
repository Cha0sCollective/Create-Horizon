# Experience 003 — existing-code Goggles craft and equip check

Status: runtime checklist only. No game, Cargo, Java or staging run occurred in
this team. Use it under the project lead's shared runtime reservation, independent
of the held Goggles HUD source candidate `d2bc736`. This packet qualifies no HUD
text or kinetic provider behavior.

## Source and fixture

The accepted native input at `55339ce3347b2ce667281e8a3e6318ec856144e4`
already bundles `recipe/crafting/kinetics/goggles.json`, the `c:plates/gold` tag,
the head-equippable Goggles item and its head-context model. The shaped grid is
`" S " / "GPG"`: one `minecraft:string` at menu slot 101, two
`minecraft:glass` at 103 and 105, and one `create:golden_sheet` at 104. The
output slot 200 should contain exactly one `create:goggles`. These concrete tag
members are declared by the bundled `c:strings`, `c:glass_blocks` and
`c:plates/gold` tags. The supplied golden sheet is a **fixture**; this check
does not qualify gold-ingot pressing or world-obtainable progression.

Run in an isolated disposable Survival world with a normal crafting table,
matching Create-enabled resources and two rendered clients on the dedicated
server. Use separate server/client homes, launcher/config paths and a new
control socket per process. The project lead records exact source, binary,
resource/cache and world identifiers before play. A fresh actor must have an
empty head slot and no Goggles or ingredient stock before the fixture grant.
Supply only one golden sheet, two plain glass blocks and one string to the
actor; give the peer no Goggles. Do not insert a crafted output, write the
head slot by command, or change a playable save. The existing `play.py` launcher
defaults to Creative; use the approved Survival launch path instead.

## Actions and expected observations

1. Query the actor's inventory and mode before interaction. Record exact
   `counts` for the three fixture items, zero Goggles, and `armor[0]` with
   `part=head` empty. The controller's `inventory.counts` includes armor and
   the cursor, so record individual hotbar/main/armor slots as well as totals.
2. Interact with the crafting table and confirm the ordinary inventory menu.
   Move each source stack from its observed inventory slot into the four grid
   positions above, using a right click for each glass cell. Confirm the
   visible result slot 200 is one Goggles **before** taking it. Shift-click
   the result, close the menu and query inventory again. Require string 1→0,
   glass 2→0, golden sheet 1→0 and Goggles 0→1, with an empty crafting grid
   and no carried stack. Save the menu and inventory observations; a recipe
   JSON or successful command alone is insufficient.
3. Select the crafted Goggles in the hotbar, face empty space and right-click
   in world play. Require `armor[0]` to be `create:goggles` count 1, the
   selected hand slot empty and total Goggles still exactly 1. Confirm the
   authoritative server equipment state separately from the actor's client
   inventory view. Do not use `item replace ... armor.head` for this action.
4. Capture an actual rendered actor third-person view and a separate peer
   client view of the actor's head. The peer must still have no Goggles in its
   own inventory. Distinguish the two rendered views from server item queries
   and model declarations. If the head model is absent, record that as a
   render failure even when the equipment slot is correct.
5. Open the actor's ordinary inventory and shift-click head slot 500 to move
   Goggles back into an empty inventory slot. Verify head empty, one Goggles
   retained and peer head model gone. If the shift-click puts the item in main
   inventory, move it to an empty hotbar slot through the menu. Re-equip from
   the hotbar by ordinary right-click; verify the head slot and peer model
   return with count still 1.
6. Quit normally, cold reopen the dedicated server and both clients, and
   confirm the actor remains in Survival with one worn Goggles, the peer still
   sees the head model, and the three ingredients remain consumed. Record
   process exits, reopened world identity and fresh client views. A server
   query after reopen cannot stand in for peer rendering.

The existing `foundation_checks.py:crafting` path demonstrates the same normal
crafting-table menu slots and input/output clicks; `engineering_checks.py`
demonstrates an ordinary in-world Goggles right-click and screenshot. This
checklist uses those controls without modifying either retained test or
replaying its broader battery. A one-off driver is unnecessary for four grid
inputs; preserve the short ordered controller transcript and screenshots in
the isolated runtime evidence folder outside Git.

## Pass boundary

Report each craft, equipment, actor-view, peer-view and reopen observation
separately. Any missing ingredient decrement, duplicated item, wrong head slot,
missing peer model or failed reopen is a failure for its step. A passing packet
supports only fixture-stock Goggles craft/equip/render in the observed mode and
world. It does not clear the held HUD review, gold pressing, a full Survival
progression chain, all cosmetic perspectives or broad Create UI parity.
