# Transport-002 packet — distinct belt-controller handoff

Status: fixture and harness prepared November 5, 2026; dedicated run pending
the project lead's host-wide heavy lease. This packet was allocated by the
lead after the read-only proposal in `transport-002-proposal.md`. The frozen
source baseline is `4e5faacad66851ba4dfae5f69502dd049a626053`, a descendant
of accepted private integration `1ac294026cf7f660a4d2901602081f77686f1a46`.
No native source change is proposed.

## Frozen inputs

- New raw packet: `G:\Create Rusty\.local\native-s6-transport-team-002`.
  `frozen-inputs.json` SHA-256 is
  `eba088bfa59397ad7e6ce259ae57ccfd854aa930e88efb881be5ce53930aa76b`.
  Its 634 native file hashes match the prior accepted `.9` transport-001 map;
  native tree is `fc71cb5f0fb0c038853902a7d2f775085b34a0f1`.
- Reuse held `horizon-client` SHA-256
  `63047934df11951c3c42303fd7da2664a2cb3465ee7279f11593eb711ab5426d`
  and `horizon-ctl` SHA-256
  `7b2a25cb1b134addedae199c4091188e3b1198c284d2590da4d05cc4ba145816`.
  The new launcher verifies them in place and does not copy to shared cache.
  It hashes the new drivers and fixture batches into each session receipt.
- Reuse the accepted resource/JAR pins and prior 17 focused core tests,
  cold-reopen, Depot peer-clear, stop/reverse and ownership evidence by exact
  attribution. No unchanged build, stage or broad battery is scheduled.

## One disposable dedicated run

1. Start the isolated `NativeTransport002` server and two rendered clients
   after the lead grants the heavy lease. `01-layout.json` places source belt
   x4..8 and receiver x9..12 at y65,z7 with ordinary shaft/connector clicks.
   Query both controllers and far-end controller references. Place source
   motor (4,65,6), receiver clutch (9,65,6), receiver motor (9,65,5), lever
   (8,65,6), and locked Depot/hopper/chest at x13,z7. The source and receiver
   motors are set through the drawn GUI to -32 and -1 RPM, respectively.
2. `02-hold.json` turns the receiver clutch off, drops exactly gold1 onto the
   source, and checks source speed -32, receiver speed 0, source1/receiver0,
   empty sink, actor0 and no loose item after a bounded wait. A stopped
   source cannot satisfy this hold check.
3. `03-release.json` releases the clutch and checks source0/receiver1 while
   the slow receiver still holds the item. `peer-occupied.json` captures the
   actual rendered receiver with gold1. Set receiver motor to -32 through its
   GUI, then `04-final.json` checks Depot gold1, unlocks the hopper and checks
   source0/receiver0/Depot0/hopper0/chestgold1, actor0 and no drops.
   `peer-empty.json` captures the receiver after transfer.
4. Quit both clients normally, stop the server and require three zero exits.
   `inspect_save.py` decodes the saved chunk and both player files, checks the
   two controllers and sink counts, equipment/offhand inventory, no loose
   entities and total gold1. Preserve command, timing, screenshot and save
   receipts in the new raw packet.

The test addresses one collinear horizontal seam and its stopped receiver.
It does not qualify broad routing, funnels, chutes, filters, cold reopening at
this seam or transfer latency. If live evidence fails, retain a HOLD, diagnose
the narrow source/fixture cause and request a fresh independent read-only Sol
High review for any substantive repair before proposing integration.
