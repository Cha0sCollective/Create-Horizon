# Transport-005 Funnel rerun preparation

Status: **prepared, not run or accepted**. Operations owns the heavy build and
runtime queue. The source-only005 private composition
`0f88acab511ecf07edf0ecf3cd43d986a137906f` has independent composition
review CLEAR, but `native-team-integration-005/binaries.json` has not been
frozen for this packet and no new runtime lease has been granted.

## Inputs and preserved evidence

- Transport lane `codex/team-transport` base `e90aa221ff8fb3afd5e0a25e0b6f5a0ee270d23f`.
  This packet changes only this handoff in Git; the executable preparation is
  packet-local under `G:\Create Rusty\.local\transport-005`.
- The retained `transport-004/driver.py` SHA-256 is
  `e15a437b59d621534d694cdfe0ae96fb7dc5ab3db88efc667c3bb44d7bc6bc08`.
  Its saved worlds, source/binary maps, raw logs, HOLDs, screenshots and
  `post-run-verification.json` remain unchanged. See
  `transport-004-funnel-runtime-hold.md` for exact prior evidence.
- New `transport-005/driver.py` SHA-256 is
  `20630f7c0b6f75c6be16b87b954fed5dc5b5cb996c4d2b421e8262cd4c3b5ea8`;
  `PLAN.md` SHA-256 is
  `a7f49912c0fd769e725664e32b87a6b5c3e1fbb10b75f1a2cc732c2d5a4aa940`;
  offline `check_prepare.py` SHA-256 is
  `28a5fad9b7589de68c98d2b5623d2926d1f16cfc80c4fd4a3182d5b5157cb21c`.

## Focused rerun contract

The driver retains the dedicated phase order and integrated setup/interactions
scope. Its source and binary gate now points only to frozen build005. After
every inventory action it reads real client slots and token-delimited server
`Inventory` data in the same live session. Dedicated actor and actual peer
each make the server query. Each checkpoint has at most 11 observations and
40 explicitly requested wait ticks; live query time also advances the game.
All observations are recorded. The side-face, physical Shift and loaded
offhand refusals require funnel3 with no placement, without relying on an
immediate predicted `inventory_change`. The supported underside click requires
funnel2 and the exact powered extracting block.

Integrated setup no longer kills living mobs. It settles and clears only
pre-existing dropped items within the trial fixture volume before supplying
gold, then checks that volume is empty. Later sentinel cleanup targets sticks
only. The dedicated chain still follows one chest gold ingot through world
item, source and receiver belts, Depot and sink, with actor/peer inventory and
global gold-drop checks. Actual peer occupied/empty captures still require
visual review. No wider gameplay or both-mode claim follows from a command
receipt alone.

## Checks and handoff

Offline `python3 -B .local/transport-005/check_prepare.py` PASS against retained
real 004 census/placement receipts and both `--plan` modes; `game_commands: 0`.
Independent read-only Sol High final-delta review: **CLEAR** for preparation,
including census semantics, offhand/Shift checks, scoped cleanup, custody and
execution guards. This is not a runtime or gameplay acceptance review.
No stage, Cargo build, server, client or game command ran in this packet.

Operations should supply the exact frozen build005 source/binary maps and a
new runtime lease, then run these packet-local phases in disposable worlds.
Any failed phase remains HOLD with its raw receipt. Completion and visual
reviews precede any acceptance or public statement.
