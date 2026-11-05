# Native S6 — coal heating and exact brass

Completion review CLEAR for the bounded increment. S6 remains OPEN as a feature-
family stage. The owner explicitly selected Java's coal consumption at the full
10,000-tick buffer, adding no time; the reviewed implementation is unchanged.
Packet `docs/design/NATIVE_S6_COAL_HEATING_PACKET.md`; earlier source/build
checkpoint `NATIVE_S6_COAL_HEATING_CHECKPOINT.md` remains historical evidence.

## Exact source and verification

Private branch `codex/native-s6-coal-heating`, reviewed source
`5ac874ff0d7cfbf14bb8b26b2cabf891929ffbee`; base accepted host refresh792d7a7.
Native/content0.5.0-ch4os.7, asset schema11, host
9c7f9b2215dfed497aba5522c44f38926628648d/Mixins
e8cd0aa38cffef49cb2e088ef716baa107148c02. Selected MIT upstream236c87a8 code;
reference Create6.0.11/7ff2760e remains distinct from imported source/art6.0.10.

All607 native files still match source map
341f7a89baf048c272c287e0868344e1f8cb49f85996d526a8b629b8facf1212. Actual
runtime input map e7228f048d96e1a02a1bbc2456277e0a57da2cc14d82e33caae93dc722aef8ef.
Held game70b1bfd5a66a7c2970fa73c66897fb1c9eb61b61d85228372926915d8a1c9672,
control e1458e7a4b5d8cfe1f80198b2b5defd6abec5ed74bc4277a332859099fe4c0bb.
Accepted `.6` source/maps/binaries/saves are preserved, with held hashes checked.

Fresh independent read-only Sol High source/effects, repair-delta and completion
reviews CLEAR. Formatting, all60 injections/29 modules, four version manifests,
28 executed focused tests, Clippy35warnings and locked build PASS. These checks
were executed for the source checkpoint and reused unchanged for the owner's
docs-only fuel decision; no compiler/unit-test or historical full-stage replay.
The earlier checkpoint documents the actual importer/JavaString failures and
source repairs. No production change was needed during gameplay qualification.

## Actual game milestone

Disposable normal generated Survival seed5 Overworld, spawn_mobs=false, local
homes/launcher/config/mod paths and original resources isolated. Dedicated server
binds127.0.0.1 and whitelists the two matching rendered test identities. Supplied
materials/machines, platform/power and temporary camera/heat fixtures are explicit;
crafting and blaze acquisition are not qualified.

| Gate | Integrated | Dedicated with two actual rendered clients |
| --- | --- | --- |
| Ordinary empty cage | Empty item places blaze=none, no heater entity; coal refused without consumption | Same |
| No-heat recipe refusal | Copper1/zinc1 stay in Basin with Mixer32RPM and NONE, then SMOULDERING | Same |
| Ordinary cage pickup | Sneak-wrench returns empty alias; supplied occupied item then places SMOULDERING heater | Same |
| Ordinary fuel | Charcoal refused; plain coal1 becomes0 and stores normal fuel1600, KINDLED | Same, through actual client prediction/server authority |
| Normal cold save/reopen | Savefuel1600 and pair1+1; first live reopened1585 after15 elapsed ticks, pair retained | Savefuel1600; first live reopened1554 after46 elapsed ticks, pair retained |
| Heated recipe | Exact two brass from copper1+zinc1, ordinary empty-hand collection | Same |
| Full-cap decision | Labelled frozen10000 fixture; ordinary coal1 becomes0, buffer remains10000 | Same |
| Fueled pickup | Ordinary wrench returns fresh filled item; re-placement has fuel0/time0 | Same |
| Heat thresholds | Labelled126 fixture, stock step1 ->125KINDLED, next1 ->124FADING; later live/save0SMOULDERING | Same thresholds, then stock step124 plus wait130 ->0SMOULDERING observed live |
| Retained regressions | Exact dry alloy1; ordinary Depot placement/drop/catch/collection of iron1 | Same |
| Main final custody | Actor brass2/alloy1/iron1; Basin/hopper/Depot empty, burner0 | Actor brass2/alloy1/iron1, peer/default0; same machine result |

Integrated run `s6-coal-integrated-runs/integrated-qp41l3u0`: two actual sessions,
both normal exits0. Dedicated `s6-coal-runs/dedicated-8rfg_ore`: main two sessions
have six normal process exits0. Stock tick freeze/step are explicit controls;
freeze does not persist as a claimed frozen restart. Reopened fuel advances
normally before observation rather than resetting or duplicating.

Actual peer images show occupied burner head, ordinary rods/flame and Mixer head
movement. The first Basin product view was below its rim and a raised view hit
the placement scaffold; neither proves product visibility. Two focused extra
dedicated sessions corrected only this observation, without a source/build/full-
battery replay. The second extra view was verified empty before producing a
third recipe, then actual screenshots show brass in the Basin and that same
unobstructed view empty after ordinary collection.

Preserved main `final-world-01` holds actorbrass2 and burner0SMOULDERING. First
camera supplement `final-world-02` holds actorbrass4; last live fuel1280, saved1240
KINDLED. Clear-view `final-world-03` reuses that saved fuel (first frozen1214),
produces another brass2 and ends actorbrass6/alloy1/iron1, savedfuel1001KINDLED.
These are three separate two-brass recipes, not six output from one recipe.
The last two sessions add six normal process exits0:14 total qualification exits0.
No game is running.

All final actor inventories also contain the supplied empty cage1, wrench1 and
unused refused charcoal1. They are fixture materials, not recipe outputs. Peer
and default dedicated saved inventories/EnderItems are empty. Integrated local-
owner save-file mirrors agree and are one owner, not duplicate stacks. Full
machine-chunk and every saved player record were decoded through their end;
native Items and canonical Basin InputItems/OutputItems are one slot set.
All final Basin/hopper/Depot slots are empty; no dropped item remains at final
markers. Saved burner block state is read from its actual packed palette index,
not inferred from palette membership or fuel NBT alone.

## Evidence limits and preserved failures

Raw packet `.local/native-s6-coal-heating-01` is outside Git. Native-readable
`evidence-integrated/`, `evidence-dedicated/`, exact commands, compact sessions,
exit receipts, full cold/final/player decodes, actual images and line-numbered
log excerpts are indexed by `completion-evidence.json` with artifact hashes.
Independent review matched all83 indexed mirror artifacts and all607 sources.

Initial terrain edits generated leaf-litter fixture pickups25 integrated/28
dedicated. Explicit item cleanup and leaf-only player removal occur before the
first fuel/product; raw logs preserve them and random-tick fixture0. Later
materials/products are not recreated or removed for accounting. The first
charcoal selection failed because it was in main inventory; ordinary slot9->4
clicks repaired setup. Early client-start races failed before gameplay actions;
socket readiness checks resolved them. Their logs remain.

Integrated `actor-burned-out.png` was captured early after only8 wait ticks and
actually shows FADING113. It is excluded from burn-out presentation proof; later
live fuel0 plus exact savedSMOULDERING and the dedicated correctly waited case
supply the distinct observations. Occluded main and `peer-raised-*` Basin images
are excluded from product-to-empty proof; only `peer-clear-brass-product.png`
and `peer-clear-basin-empty-after.png` establish that peer transition. Screenshot
filenames/command success are not acceptance evidence by themselves.

This accepts supplied occupied burners with plain coal, the selected cap
behavior, exact brass recipe, retained dry/Depot composition and ordinary cold
restart. It does not accept arbitrary fuels/recipes/components, fluids, capture,
ignition, hats/goggles, crafting progression, Java/world import, broad
contraptions, frozen/tick-anchored cross-client phase, crash/replay atomicity,
public publishing, release or cleanup. Next family requires its own bounded
reuse assessment/packet; the accepted `.7` result does not widen those limits.
