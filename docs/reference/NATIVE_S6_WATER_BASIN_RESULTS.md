# Native S6 — plain-water Basin and exact dough

Completion review CLEAR for the bounded increment. S6 remains OPEN
as a rolling feature-family stage. Packet:
`docs/design/NATIVE_S6_WATER_BASIN_PACKET.md`. Accepted coal/brass `.7`, including
the owner's selected full-buffer coal consumption, remains preserved.

## Exact source and focused checks

Private branch `codex/native-s6-water-basin`, reviewed source
`06c460f9c80f725d3d698510c937649b064ca1df`, based on accepted074122b7.
Native/content0.5.0-ch4os.8 retains host
9c7f9b2215dfed497aba5522c44f38926628648d/Mixins
e8cd0aa38cffef49cb2e088ef716baa107148c02, API359/Rust1.99.1 and asset schema11.
Selected MIT upstream236c87a8 fluid adapters/hooks reuse existing host services;
the core, canonical tank codec and save schema are unchanged. Reference
Create6.0.11/7ff2760e stays distinct from imported source/art6.0.10/ac0c444d.

All612 native files match runtime input map
34aa86142346922d5e4e8aa3a86a29aed6eee695fb71415b39ce66e1f45dd83f.
Held game69a8537a1623bc3e309d1a8fb58151d9d0de1220bd803071272ec31960e80515,
control446f2f1fe8c15ee082a25c2ea8dacd79b66df8bcf4768150373f2f3caf319cda.
No production source changed during actual gameplay qualification.

Fresh read-only independent Sol High source/repair-delta review CLEAR. The first
review caught offhand UseItem fall-through: the host's older reaction callback
only checks main hand. The repaired existing per-hand prefix claims Basin bucket
interactions before vanilla UseItem, retaining server authority without predicting
Basin contents. The pre-routing HOLD binary was never used for gameplay.

Formatting, four version manifests, exact content delta, stage64 injections/
29 modules, Clippy34warnings and locked build PASS. Three actual water adapter
tests were rerun after routing repair. Two exact recipe tests passed before that
repair and are reused unchanged: five unique relevant tests, not eight. The old
coal/dry/host batteries remain attributed evidence rather than repeated checks.
Embedded content adds only the exact upstream dough recipe; among prior entries
only the package-version stamp changes. No new imported art is required.

## Actual game milestone

Disposable normal generated Survival seed5 Overworld, spawn_mobs=false,
random_tick_speed0, isolated homes/launcher/config/resource/mod paths. Dedicated
server is loopback with two matching actual rendered clients. Supplied materials,
platform, power and explicit500mB fixture are test setup, not acquisition or
crafting progression. Ordinary bucket clicks, hopper-menu input and empty-hand
collection use real game input.

| Gate | Integrated | Dedicated with two rendered clients |
| --- | --- | --- |
| Lava refusal | Main lava and main-stick/offhand-lava clicks retain lava1, water0 and adjacent air | Same; no vanilla spill/fall-through |
| Ordinary main water | Water bucket1 -> empty bucket1; Basin1000mB | Same |
| Stacked offhand empty | Empty stack2 ->1; recovers exactly water bucket1, Basin0 | Same |
| Ordinary offhand water | Water bucket1 -> empty bucket1; Basin1000mB exactly once | Same |
| Full matching source | Another water bucket is refused and retained, Basin1000mB unchanged | Same |
| Normal cold save/reopen | Canonical input water1000mB plus flour1 retained | Same |
| Powered recipe | Exact flour1 + water1000mB -> dough1, ordinary collection | Same |
| Insufficient-water refusal | Explicit500mB fixture/flour1 remains without output; whole-bucket drain and refill refuse without consumption | Same |
| Recovery | Remove only supplied500mB fixture; ordinary whole-bucket refill processes the retained flour into second dough1 | Same |
| Retained regression | Exact dry andesite/nugget -> alloy1, ordinary collection | Same |
| Final custody | Actor dough2/alloy1/empty buckets4/lava1/stick1; all machine slots/tanks empty | Same actor; peer retains incidental leaf_litter4, default owner empty |

Integrated run `s6-water-integrated-runs/integrated-bxoo3yet`: cold/final worlds
00/01 and two normal process exits0. Dedicated run
`s6-water-runs/dedicated-v56mwton`: cold/final worlds00/01, six normal process
exits0. Eight total exits0; no game remains running. Tick freeze is an explicit
live control, not a claim that freeze persists through restart.

Actual integrated images show stored water/flour and dough. Its final empty
state is supported by authoritative queries and saved NBT; no integrated
post-collection empty screenshot was captured. Actual dedicated peer images
`peer-water-cold.png`, `peer-dough-product.png` and `peer-empty-after.png` show an
unobstructed water-to-product-to-empty transition from the same verified view.
`peer-final-empty.png` also shows the final empty Basin after the dry regression.
Server queries and rendered observations are separate evidence.

Cold actor inventory in each mode totals empty buckets3, water bucket1, lava1,
stick1. Final totals empty buckets4, lava1, stick1, dough2 and alloy1. One empty
bucket resides in the distinct saved `equipment.offhand` field in both snapshots;
it must be included alongside Inventory. The live control's Held field mirrors a
hotbar slot and is not another owner. Integrated local-owner save files mirror
one owner; they are not duplicate supplies. Basin generic Items and canonical
InputItems/OutputItems likewise represent one slot set.

The dedicated peer acquired leaf_litter4 during setup before the first water
supply or bucket interaction. Its cold/final player file hashes agree and it
holds no Basin product. The raw pickup timing supports incidental setup pickup;
the individual dropped source is not logged. The default dedicated owner is
empty. Every saved player/Ender inventory and whole machine chunk was decoded
through its end. Final Basin/hopper slots, both tank halves and overflows are
empty, chunk entities are empty and final global no-drop markers pass.

## Preserved attempts and limits

Raw `.local/native-s6-water-basin-01` stays outside Git. Native-readable
`evidence-integrated/` and `evidence-dedicated/`, commands, compact sessions,
eight process-exit records in four receipt files, all saved-owner/chunk decodes,
actual images and log excerpts
are indexed by `completion-evidence.json` with40 artifact hashes and612 source
hashes checked. Original full session/input maps and held binaries remain local.

The initial dedicated bucket batch failed a camera face lookup while the actor
stood west, before its first bucket interaction. Four material give commands had
already succeeded. The focused continuation moves east and skips exactly those
four commands, avoiding duplicate supplies; all original traces remain. Terrain
fixture drop cleanup precedes water interaction. No source repair or full-battery
replay was needed for this setup correction.

The first evidence helper counted only Inventory and therefore omitted the
separate26.3 offhand bucket; another assertion expected an empty generic Items
key instead of its native omission. Both helper assertions failed visibly and
were corrected against actual full saved NBT. Live bucket assertions already
included offhand; no game state or production source was changed to fit results.

This accepts plain water only in canonical Basin input tanks, empty fluid output,
ordinary exact buckets, the strict flour1000mB/dough recipe and normal cold
restart. Focused actual adapter tests cover component-bearing fluid/buckets,
zero fluid amount, invalid bucket counts and canonical roundtrip. Malformed or
opaque canonical-tank refusal is source-reviewed inherited codec behavior, not
a newly executed malformed-NBT test or live-game case. The explicit500mB fixture is
removed before whole-bucket refill; no partial-bucket top-up or automatic recovery
is claimed. Existing tank capacity/variety rules remain unchanged.

Lava contents, mixed/output fluids, sponges, pumps/pipes, arbitrary recipes,
filters/side outputs, broad automation, crafting progression, Java/world import,
one-shot Force-field equivalence, crash/replay atomicity and public release remain
outside this result. Known mixed-fluid discard/multi-result reservation cases
remain unreachable in this supported subset. Prior item-loss and coal-cap choices
are not generalized to other losses. Next family needs a separate reuse audit and
bounded packet; a broad API/schema/framework or acceptance change needs owner input.
