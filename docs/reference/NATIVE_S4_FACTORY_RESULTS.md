# Native S4 — PASS within the declared scope

Native S4 is COMPLETE. Fresh independent read-only Sol High completion review
found no actionable evidence gap or acceptance overclaim. The approved packet is
`docs/design/NATIVE_S4_FACTORY_PACKET.md` (80c2daf5). S1–S3 remain accepted within
their original profiles. This stage adds one normal water wheel and one useful
Survival factory in ordinary host-generated terrain.

## Reuse and inputs

No production source, protocol, engine service or save format changed. No build
was needed. Existing native recipes, interactions, power/kinetic math, millstone
inventory/processing, vanilla hoppers/chests, replication and normal saves supply
the outcome. The large-cog placement failure was a controller visibility issue;
an ordinary temporary vanilla support block resolved it without a game patch.

Unchanged pins: private0.5.0-ch4os.1, imported port16c5d454, Horizonfc021038,
Mixinsa1e13326/API359, Rust1.99.1, Minecraft26.3. The selected Java reference is
Create6.0.11/7ff2760e/NeoForge21.1.219; original art/base stays6.0.10. Port main
10cb93e6 and experimental3607f097 were checked before this packet; recent changes
did not require replacing the compatible tested host. Provenance remains intact.

S3 source/binary input map is
`c880261c7e8603464a2c963c8122f3c194618832c58fba43fa25560af35a5e10`.
Held S3 game is `f12a7cdcf1fb1d434a75e1079357a0b159cf82541dafe9c42f94569e16e0e4b8`;
matching ctl is `a1d6054a746623085a88134d01f261fff86e2eed2cef25f413e93047f8f71970`.
Launchers verify that input map, unchanged source/resources and held binaries.
Each client/server has separate isolated homes and Create resources; inherited
account-session-file and Wayland overrides are removed. Dedicated binds loopback,
offline mode with a two-name whitelist. Real Survival/mob-free state is queried.

## Integrated recipe and factory path

`integrated-9fhneh9o` uses normal generated seed5 terrain, Survival, and
`spawn_mobs=false`. The leveled plot, stone/glass scaffold, materials/tools and
vanilla chests/hoppers are fixtures. No Create item was supplied for this claim.
Natural leaf litter is recorded separately from the relevant inventory counts.

Eight selected generated recipes agree with7ff2760e after ingredient-codec-only
normalization. Actual GUI crafting/application/mining consumes the real inputs:

- 8andesite +8iron nuggets →4alloy; 2alloy →8shafts.
- 2shafts +2planks →2small cogs; 2shafts +4planks →2large cogs.
- 1shaft +8planks →1water wheel.
- 1alloy applied to an ordinarily placed stripped log →andesite casing; normal
  pickaxe mining/collection obtains1casing and damages the tool.
- 1small cog +1casing +1stone →1millstone.

The actor ordinarily places six nodes from that crafted stock: wheel[0,66,0]
axisX →shaft[1,66,0]axisX →large[2,66,0]axisX →large[3,67,0]axisY →small[4,67,1]
axisY →mill[5,67,1]. A chest/hopper above feeds3plain cobblestone; a hopper/chest
below extracts outputs. Empty output stores while input3 sits in the stopped
mill demonstrate no raw-input theft. No machine-NBT input/speed write is used.

Ordinary bucket placement creates a contained falling current. FlowScore1 powers
signed8/8/8/8/−16/+16RPM, capacity256SU against64SU load. Removing the source with
the bucket stops the line. After drainage, Timer177/input3/Speed0 remain unchanged
over another40ticks. Normal quit and cold reopen preserve that state and player
inventory. Restoring water finishes exactly3gravel in the output chest; all other
factory inventories are empty. The real chest screen shows3; ordinary shift-click
collection gives the actor3gravel and leaves the chest empty.

During controlled departure, the host twice reports the factory unavailable via
`execute unless loaded`; after return its current, speeds, power and3stored gravel
remain intact. This is the host's loaded/lit condition, not a proof that every
timer stopped or every chunk object was absent. Temporary forceload tickets are
removed. All four integrated sessions exit0.

## Dedicated and actual client observations

`dedicated-msonj7j7` uses another normal seed5 Survival world, explicit mob-free
rule, server and two matching rendered clients. Create/vanilla stock is provided
as the second-mode fixture; the recipe claim reuses the integrated observation.
All six Create nodes and four vanilla containers are ordinarily placed. Three
cobblestone move from the real chest UI into the mill while unpowered, with no
input theft. Powered speeds/capacity/load agree with integrated play.

Removing water pauses Timer153/input3/Speed0. The peer sees full wheel/shaft/cog/
mill states, disconnects/rejoins, and the correlated authoritative query remains
153/input3. Normal shutdown of both clients and server returns0. Cold server and
fresh clients retain153/input3/empty output; restoring water produces3gravel and
empties all other stores. The peer's actual chest screen shows3gravel, shift-click
collects3 into its inventory, and the authoritative chest becomes empty. The actor
has no cobblestone/gravel. Both modes retain Survival; all six successful-run
dedicated processes exit0. Dedicated departure is not claimed anew; integrated
S4 departure and unchanged general dedicated S2 loaded-condition evidence are
identified separately.

Root inspected running wheel/factory and output-menu images. Actual submitted
model matrices supply rotation evidence: integrated wheel8.000/mill16.014RPM,
dedicated actor8.000/16.000 and peer8.000/16.000. These are median nonzero absolute
rates from consecutive normalized poses (trace time is ticks at20Hz); signed
speeds above come from server queries. They do not claim a Java phase match.
Machine NBT queries remain authoritative feedback, distinct from replicated block
states, submitted render poses and real container UI/inventory observations.

## Validation, failures and limits

Fresh independent Sol High reviews cleared the packet, integrated adaptive helpers
and dedicated adaptive helpers, including a bounded seed-setting correction.
Python syntax passes. Unchanged Rust/core/compatibility evidence is reused from
S1–S3; no historical battery or full crafting replay was needed for an aiming error.

Preserved incomplete actions include the initial wheel orientation/replacement,
invisible shaft-face aim, rejected console `save-all flush`, first dedicated
startup rejecting `--seed`, initial pre-login operator-name resolution and a
title-screen `wait` after peer disconnect. None is a pass. Normal quit/cold reopen,
not that rejected console command, supplies save evidence. The seed moved to
existing `level-seed=5`; connected identities were granted op through the owned
server console. Existing controller operations then continued from valid state.
The rejected dedicated startup exits1; successful-run counts above exclude it.

Raw runs remain local under Linux cache/s4-runs; copied commands/images/receipts,
recipe comparison and stable driver inputs remain `.local/native-s4-01`.
Integrated commands SHA256`ea65b9679cf73978344da7f431aa193a9552525b1f747d8e7dfc0b54a77c389d`;
dedicated commands`90d21f464be8db0f7244b3678c6cdccbdc0b484beb3b63779478c457c89473fe`.
Collected evidence summary SHA256
`d194a4ab8206881c51fd4f03699727f195386515ca02b9dcdf447d18f477dd10`.
Resources, JARs, worlds, generated runs, binaries and original art stay outside Git.

This qualifies recipe obtainability from vanilla fixture materials and one finite
water-powered milling factory. It does not qualify a complete mining playthrough,
other machines, competing mixed generators, arbitrary-world/modpack compatibility,
Java client/world import, Open to LAN, interrupted-crash coherence, or a public
release. Mixed-wheel source-parent arbitration remains outside the S2 five-type
source-transition guarantee. These limits do not revive historical G4 acceptance.
