# Native S6 — bounded dry Basin/Mixer result

Accepted bounded increment after fresh independent read-only Sol High completion
review CLEAR of the exact source map, commands, saved inventories and client images.
S1–S5 and earlier S6 pressing/mounted-depot/portable-interface results are reused
for unchanged behavior; this increment does not complete S6 or the modpack.

## Inputs and implementation

Branch `codex/native-s6-basin-mixer`, reviewed source `e52dc3376f5dfcc241fd7d7ccd0b87d31265dd05`,
based on accepted `6defc66746509cebd2213a41f917bd5e1a051a93`.
Audit and packet: `docs/design/NATIVE_S6_BASIN_REUSE_AUDIT.md` and
`NATIVE_S6_DRY_MIXING_PACKET.md`. Selected upstream reuse is
Delete228/create-horizon `236c87a889cd76f7e663d1d182c28dc1145f7b14`.
Native/content version `0.5.0-ch4os.5`, local asset schema10.

Retained host `fc02103818c212920fd24aa34a2b2f91ba49e56f`, Mixins
`a1e13326306c6bc074b11e6a4926deb551c4083e`, API359, Rust1.99.1 and
Minecraft26.3. Java reference remains Create6.0.11/7ff2760e; the relevant
Basin/Mixer/burner/heat-condition Java directories have no source diff from the
6.0.10 baseac0c444d. This is bounded source-informed reuse, not full-version parity.

Exact core Basin and NBT tank codec are reused. Narrow old-host adapters add
18 Basin slots, fixed-Y small-cog Mixer scheduling/rendering and original saved
inventory fields. A private generated-only born comparison retains transient
state across same-entity mutations. A Basin-only read-only serialization snapshot
repairs the upstream hopper-after-Basin-tick stale-mirror save race. No host pin,
official/public engine checkout, saved identity, generic recovery service or fluid
API changes. Existing portable notifications and other machine serializers remain.

The content delta preserves387 existing files, changes only three language/tab
files and adds16 selected files, with four version edits recorded separately.
Authored fallbacks and local original-JAR imports retain their separate provenance;
original art/resources and generated outputs remain outside Git.

Frozen source/resource map:
`cd8228d648b4e0fde510b71085724e4f9e5fbcafcc7e7fb0e83a87a1e4a5f0a6`.
Game `36e13c3f716cd43a53eebad58aa21ae806ddc7ba2e94e498028aea212678e235`;
control `32f373323e73c02f553fb86eb6f8c3235fc50c86b1f8fb12a887d36ff94d3c05`.
Raw packet `.local/native-s6-dry-mixing-01`, native-read mirror `evidence-complete/`.
Drivers are finite adaptations of the prior reviewed portable drivers, with their
old/new hashes in `driver-reuse.json`; source/binary guards and normal-exit-before-
reopen checks remain. Do not rerun frozen older drivers against this version.

## Supported outcome and observations

Normal generated, mob-free Survival Overworld, one downward/unfiltered Basin,
one Mixer two blocks above and one vanilla input hopper. The only admitted recipe
is plain andesite1 + plain iron nugget1 -> andesite alloy1. A fixture motor powers
a Y-axis small cog connected sideways to the Mixer. At +32 Mixer RPM the observed
stress is128SU; +28 RPM leaves both ingredients unconsumed. Machine/material
supplies, terrain/motor fixtures and raw-data negative fixtures are explicitly
separate from ordinary placement, menu transfer and empty-hand collection.

| Gate | Observation |
| --- | --- |
| Integrated ordinary operation | Player places Basin/Mixer/hopper, feeds both ingredients through hopper-menu shift clicks, observes running/retracted head, and collects exactly one alloy empty-handed |
| Capacity refusal | All nine output slots contain supplied64-alloy stacks; +32RPM does not start/consume the separate input pair |
| Ordinary recovery | Collection clears the blocked Basin and returns both preserved ingredients plus its fixture outputs; refeeding the same pair through the hopper and restoring power produces exactly one alloy |
| Unsupported filter | Nonempty `Filter.S6Unsupported=7` is retained, processing refuses and both input items remain; explicit fixture removal restores the supported profile |
| Shared-path regression | One fixture iron ingot transfers from a vanilla hopper to an existing Depot and is collected through its ordinary top interaction; a side interaction correctly does nothing |
| Integrated cold reopen | Normal stopped save retains one andesite/nugget pair; after a cold reopen +32RPM produces one alloy, collected normally; Basin/hopper finish empty |
| Dedicated placement/input | Same ordinary Basin/Mixer/hopper placement and menu input on one loopback dedicated server with two matching rendered clients |
| Power stop | Actual peer captures show lowered/rotating head; stopping mid-cycle retracts it and leaves the input pair intact, with no output |
| Dedicated cold reopen | Stopped/unprocessed save decodes to the exact pair, empty output/hopper and empty saved player inventories; both clients actually reconnect before power is restored |
| Dedicated completion | Restoring +32RPM produces exactly one alloy; ordinary actor collection leaves Basin/hopper empty and actor1/peer0; actual peer product/empty images show the presentation change |
| Shutdown | Two integrated sessions and two dedicated sessions, eight actual game/server/client process exits, all code0 |

The singleplayer run is `s6-mixing-integrated-runs/integrated-ni0mfyt4`; the
dedicated run is `s6-mixing-runs/dedicated-bp4rqma2`, under the isolated WSL cache.
Dedicated sessions use loopback-only server binding, two whitelisted identities
and distinct homes/configs/mod directories. No playable save, launcher or Java
setting was changed. No game remains running.

Integrated accounting ends with578 actor alloys:575 remaining supplied output
fixture alloys plus three actual recipe products. The576-alloy initial capacity
fixture had one alloy explicitly removed by the later data command. The third
input pair was supplied as a cold-save fixture; the first two used ordinary
hopper-menu input. The regression's one iron ingot is returned to the Depot in
the reopened session to leave an empty collection hand. No dropped item remains
at the recorded stage markers. Dedicated accounting is separate: one supplied
pair becomes one actor alloy; peer, Basin and hopper remain empty.

Full machine-chunk NBT and saved player records were decoded through the end of
each payload. Canonical InputItems/OutputItems and generic Items describe the
same18 native slots and are not extra owners. Pinned Horizon's stock
`lan/playerdata.rs:save_owner` intentionally writes the singleplayer owner's state
to both the level.dat-named player file and its own profile file; both integrated
records agree at each save. They are the host's local-owner mirrors, not two
connected players. Dedicated final records contain just one actual alloy.
The first dedicated snapshot helper stopped on the different exit-receipt shape;
its corrected read preserved region/player files verified to predate restart
metadata, before further game commands. Raw saves, file times and source excerpt
remain local. No crash-atomicity or arbitrary-user ownership claim follows.

## Verification and preserved limits

Eighteen unchanged imported Basin core tests PASS. Some cover inactive fluid
quirks; they do not qualify fluid gameplay. The actual `horizon-client-lib`
serialization-boundary invariant PASS1 covers immediate insert/extract encoding,
read-only mirrors/identity, unknown sibling retention and opaque Size17. Imported
Mixer cog/no-shaft/stress connection test PASS1. Formatting, version consistency,
stage5, focused Clippy (34 nonfatal warnings) and final locked build PASS. The
build includes the final no-shaft correction and test-only connection helper.
Fresh independent source/effects review cleared this exact candidate for live use.
Unchanged historical batteries were not replayed.

Preserve earlier import/staging/helper-shape failures and the incorrectly selected
game-crate test invocation that ran0 tests; the latter supplies no pass evidence.
Also preserve the failed automated top-face aim at the hollow Basin, unsupported
`item replace ... container.9` fixture command, Depot side no-op, local evidence
quoting/output-filename/receipt-reader failures, and the150-tick raw-data output-
space probe. That command has no ordinary inventory notification; its short wait
observed no automatic recovery. Only the ordinary clear/reinsert recovery above
is accepted. No speed/timing offset or production change was made to fit it.

Saved restart qualification is stopped/unprocessed, not an active transient-timer
checkpoint. Active recipe and processingTicks remain source-transient. Screenshots
establish actual client presentation; item counts and RPM come from authoritative
queries and saved NBT, not from counting pixels. Exact unique products and no
remainders avoid upstream's separate multi-output overbooking/loss quirks. The
owner's selected millstone loss policy does not authorize those new cases.

Heat, brass, fluids, contextual falling-item intake, filters/spouts, other recipes,
crafting progression, mounted Basin/Mixer, broad terrain/world compatibility,
Open to LAN and crash/replay guarantees remain outside this dry packet. Next is
a separately scoped upstream heating-reuse packet. Broad lifecycle/API/schema or
acceptance changes still require input before expansion.
