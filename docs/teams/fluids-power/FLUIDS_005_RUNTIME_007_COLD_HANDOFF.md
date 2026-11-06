# FLUIDS-005 007 dedicated cold continuation handoff

**Status:** scripted server/cold checks PASS; canonical peer visual acceptance
pending Operations. No extra game run, source change, fixture replacement or
new initial PASS receipt. The exclusive game slot is released and no process
for this owned run remains.

## Exact run and evidence boundary

- Frozen source `f732bd481cfc3d9c4720bc51e4a31e7385e18020`, native tree
  `471136a9f27049a58bac2707c75b18466e94e17c`, 742 tracked native files.
  Reviewed case driver `69a4f80397271f1b545f18a986b49670f9f86ba1`,
  SHA-256 `59b7bf8602e9af0d14c96b541b418037966f54cc4ac1534ff15d71555c2672fb`.
- Same saved world as retry002 before-cold:
  `/home/arbiter/.cache/create-horizon/team-fluids-005/dedicated-d1m1sey4`;
  raw mirror `G:\Create Rusty\.local\native-fluids-005-runtime\dedicated-d1m1sey4`.
  The preserved missing `before-cold-result-00.json` remains absent. Its
  208-call before-cold record, 728/272 mB saved partial and three zero exits
  were separately verified in `FLUIDS_005_RETRY002_REUSE_HANDOFF.md`.
- Cold `session-01.json` SHA-256
  `96191cdc9e28db44ba0c80b4428e1193cca7fdeecb1b133d6c437a541b4d054a`;
  `calls-01.jsonl` SHA-256
  `da0b10166d4f5f5a9a5172f132fdb17f65aab8edaacf2a89798c8e50e26fe141`;
  `after-cold-result-01.json` SHA-256
  `23f88d3a834ac828ab3ef3e70c7cbbef3491f25625f696eb87396dfc03a78f29`.
  The result reports `PASS`; `exit-01.json` SHA-256
  `88bf610833fed722fed2a4849b1a6ed861029bb38b0d735ba7cc508c252e8946`
  reports server, actor and peer **all code 0**. The earlier `exit-00.json`
  also retains all three code 0.

The after-cold script checked the saved **Tank 728 / Basin 272 mB** pair and
`spawn_mobs=false` before refill, with a 1,000 mB endpoint census. It then
observed **Tank 0 / Basin 1,000**, explicitly refilled the Tank, and held
**Tank 1,000 / Basin 1,000** through 20 powered ticks as a separate 2,000 mB
full-input refusal. After stopping power, one ordinary Creative empty-bucket
click left **Tank 1,000 / Basin 0**. The actor inventory record shows one
`minecraft:bucket` both before and after that Creative click, with no recorded
water bucket. Treat the click as the previously accepted Creative Basin view
transition, not a player-held-water custody transfer. The packet's earlier
sentence suggesting water went into the player's bucket is unsupported by
this observed Creative inventory; Operations should keep that distinction
when deciding the final claim. No Pump water-custody conclusion is drawn from
the separate Creative drain.

## Actual dedicated peer frames

The peer rendered client produced three 960×600 PNGs in the same owned run:

| State | Raw file | SHA-256 |
| --- | --- | --- |
| Initial empty | `before-cold-basin-empty-peer-00.png` | `8f4a2222ff1532ce84351d4ead638c95a1f603aee26e8b08d17fdd21f5ed28da` |
| Pump-filled | `after-cold-basin-filled-peer-01.png` | `4d7d791349ef10f2b946744771cdf6e92dd3356ed00902b4b8d44ba9adfd9430` |
| Empty after bucket | `after-cold-basin-empty-after-bucket-peer-01.png` | `ba2282fb4364b715521859e06ce0b0cedafab45fd4227c6f8982361eeb08a83e` |

The screenshot controls succeeded and files are nonempty. Preliminary visual
inspection found the dark Basin top similar in all three frames; the chat
overlay also covers part of the foreground. These captures alone do **not**
establish a clearly visible occupied-to-empty transition. Operations owns
canonical pixel inspection and any decision on whether peer rendering is
qualified. Server NBT and peer images remain separate observations.

## Closure and inputs

The dedicated server log shows normal player/world/chunk saving; the case
issued normal peer/actor quits and the session stopped the server. A post-run
process check found no remaining game process for the run. Physical 001
remained clean at the HEAD/tree above. Source-map SHA-256
`0c97fc70d5ce5a8dc81bed1b5ccc21887fbc69628c94e0b0d1a6aff015c70f8a`,
binary receipt SHA-256
`2ec5726527df08817663fa847df0b2dfdbb783deecc31d961c92a4ebae5074f5`,
and prerequisite receipt SHA-256
`43de03a356d6deb4d76465730f1c75098c4083e6120d865a518b349c95715396`
were unchanged. Linux rehashes matched both held binaries, the Create JAR,
four prepared base manifests and three composed resource tables to the
frozen values in the 007 launch runbook. Earlier HOLDs and raw runs remain
preserved. The root owns final acceptance; do not turn script PASS into a
peer-visual or general-fluid qualification without that review.
