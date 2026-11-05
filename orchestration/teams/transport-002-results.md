# Transport-002 result — distinct belt-controller handoff

Status: focused dedicated fixture passed on November 5, 2026; independent
read-only completion review CLEAR. No native source or binaries changed.
The run used frozen source baseline `4e5faacad66851ba4dfae5f69502dd049a626053`
and native tree `fc71cb5f0fb0c038853902a7d2f775085b34a0f1`, matching the
accepted `.9` integration. The session hashed all 634 native inputs, held
binaries, pinned resources and initial drivers. No rebuild or historical
battery ran. Raw run: `G:\Create Rusty\.local\native-s6-transport-team-002\runs\dedicated-sukwt9m_`.

## Observed result

- Ordinary connector placements produced independent source controller
  (4,65,7), length5, and receiver controller (9,65,7), length4; both far-end
  references pointed to their respective controllers. GUI-set motor speeds
  were -32 and -1 before the receiver clutch was stopped with its lever.
- At the bounded stopped-receiver check, source speed remained -32.0,
  receiver speed was 0.0, actor gold0, source gold1 at `Pos: 4.75`, receiver0,
  and Depot/hopper/chest0. This is refusal under a powered source.
- After ordinary lever release, source0 and receiver gold1 at about
  `Pos: -0.15`, with receiver speed -1.0 and sink0. The rendered peer captured
  the gold item visibly on the receiver; its controller query independently
  read gold1. The receiver motor was then set to -32 through its drawn GUI.
- After the receiver cleared, Depot held gold1 with the hopper locked.
  Unlocking transferred exactly gold1 to the chest. Final live checks were
  source0/receiver0/Depot0/hopper0/chest1, actor gold0. The rendered peer
  captured the empty receiver and read chest1.
- Both clients quit normally; dedicated server, actor and peer process exit
  codes were all 0. The final save decodes source0/receiver0/Depot0/hopper0/
  chestgold1, no saved entities, and gold0 across the actor, peer and an
  additional default player file, including inventory and equipment fields.
  Total saved gold is 1.

## Evidence and bounded setup corrections

- Frozen input map SHA-256:
  `eba088bfa59397ad7e6ce259ae57ccfd854aa930e88efb881be5ce53930aa76b`.
  Final `saved-conservation.json` SHA-256:
  `3c707ba1d79fc8de55fde1e9c9e619f1bb00c79ff7298f683586a8d1f55d04c0`.
  `exit-00.json` SHA-256:
  `512a48d1246a3b986105ae2956fc3240ff2bf775f8fe073919d35245f3759c99`.
- `peer-receiver-occupied.png` SHA-256:
  `c5d9d1795dc8904bc8b718e9eaf92049a84162e19aa136ce5824fe04f0a87b7a`;
  `peer-receiver-empty.png` SHA-256:
  `50ea10310a1f1fc00292c9c3b21677afcdbf7d90ba91112a72641426b0e88206`.
  Images were visually inspected. Command transcripts and timed batch receipts
  remain in the isolated raw packet.
- An initial setup call preceded client socket creation. Later frozen setup
  stopped at a lever-view angle after both belts and clutch formed. Adaptive
  batches moved the actor to a visible support angle and completed placement.
  The motor GUI required a ground-level side face; the lever required the
  control client's automatic face choice. Failed calls and their receipts
  remain preserved. No game source was changed or reset. A first save decoder
  expected only two player files; the corrected read-only decoder required
  the two whitelist UUIDs and counted all three saved files.

This packet qualifies one collinear horizontal seam and stopped receiver in
one dedicated run. It does not establish broad routing, cold reopening at
this seam, transfer latency, funnels, chutes or filters. Prior accepted
single-chain evidence remains separately attributed in
`docs/reference/NATIVE_S6_BELT_TRANSPORT_RESULTS.md`. The host-wide heavy
runtime lease was returned immediately after the three zero exits.

Independent reviewer `/root/validation_review` inspected the raw command
continuation, actual peer images, final save and exit receipt and returned
CLEAR. The read-only review ran 22:30:55–22:31:45 UTC on November 5; no source,
cache or game mutation occurred during review. The save check covers the
decoded fixture chunk and all saved player records, supplemented by live
queries; it is not a whole-world census. The command-built power and hopper
unlock are explicit fixture controls, not a survival crafting claim.
