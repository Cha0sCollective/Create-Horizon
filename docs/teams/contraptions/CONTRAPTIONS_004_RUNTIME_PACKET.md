# Contraptions 004 — disposable owner foundation runtime packet

Status: prepared, unexecuted. This packet checks the inert entity-owner foundation already compiled in root's integration 005 source `9651dba12d163896d45a8cc4ba1b0b3b0da1feff`. It makes no native source, owner schema, host, Mixins or world-capture change. Transport holds the shared game lease; Operations runs this packet after that lease is released.

## Frozen inputs and controller metadata

Root's prepared `.local/native-owner-foundation-005/session.py` launches a **dedicated server and one real peer only**. The peer is `TeamPeer`, whitelisted and level-4 op; there is no actor control socket. It pins the `binaries.json` game/control hashes, `source-inputs-r3.json` Git head/native tree, prepared resources and Create JAR before launch. It gives both children `CREATE_HORIZON_OWNER_PROBE=1`, the exact gated current-directory names, and server world name `contraptions-004-probe-world`. The peer's `--control-socket` is recorded at `session["processes"]["peer"]["socket"]`. Its server and peer log paths, run/mirror directories, session index, port, binary hashes, source head and source-map hash are all in the mirror `session-NN.json`.

The packet driver is `tools/packets/contraptions004_owner_foundation.py` in this isolated team lane. It attaches to an **already launched** mirror session JSON; it does not launch `session.py`, build, stage or alter a source tree. It checks source head `9651dba`, both frozen binary hashes and the dedicated server/peer roles, then creates a fresh `owner-check-NN` evidence directory under that session's mirror. It copies itself and the exact fixture handoff there with hashes. Use a new disposable run so the first index is `00` and cold reopen is `01`.

Root's sequence after acquiring the heavy lease:

1. Start `session.py dedicated` under the prepared WSL runtime. Keep it running and read the emitted `mirror/session-00.json` and `run` path. Run `python3 tools/packets/contraptions004_owner_foundation.py <mirror/session-00.json> --phase initial` from this team lane. The driver waits for the peer socket, teleports the op peer near loaded chunk `(0,0)`, sends the exact **single-line raw typed `/summon`** from `CONTRAPTIONS_004_PROBE_HANDOFF.md`, checks one server scan/native-guard line and one actual peer VIEW receipt line, queries entity raw NBT and position, requests server `/save-all`, quits the peer, and sends the matching session stop request. Wait for session 00 to record both exit codes zero.
2. Start `session.py dedicated --run <run>` against that same saved run. Read `mirror/session-01.json`. Run the same driver with `--phase cold`. It does **not** summon again: it waits for loaded cold discovery and peer tracking, checks the same one-shot logs and saved NBT, tries `/kill` and `/tp`, verifies the carrier still exists at `[8.5,64.0,8.5]`, then saves and stops normally. Require both exit codes zero.

The exact entry points, with `<mirror>` and `<run>` taken from session 00's JSON, are:

```text
python3 '/mnt/g/Create Rusty/.local/native-owner-foundation-005/session.py' dedicated
python3 '/mnt/g/Create Rusty/.local/workers/contraptions-004-probe/tools/packets/contraptions004_owner_foundation.py' '<mirror>/session-00.json' --phase initial
python3 '/mnt/g/Create Rusty/.local/native-owner-foundation-005/session.py' dedicated --run '<run>'
python3 '/mnt/g/Create Rusty/.local/workers/contraptions-004-probe/tools/packets/contraptions004_owner_foundation.py' '<mirror>/session-01.json' --phase cold
```

Run each `session.py` invocation as the supervised session process while its matching driver attaches; do not launch the cold session until the first exit receipt is complete.

The driver uses `horizon-ctl --socket <peer socket>` actions `wait-ready`, `command`, `status`, `block`, `chat-log`, `wait` and `quit`. Remote `command` sends the op peer's slash command to the dedicated server and acknowledges the send before dispatch. A unique `/say` marker bounds each query and guard command in the peer's chat log. The cold `/kill` and `/tp` checks require **fresh single-target success feedback from the command dispatcher**, followed by an unchanged entity Pos and retained raw entity; the control send receipt or unchanged Pos alone does not pass. The raw query must retain the private marker, UUID, canonical record, synced view and fixture tag. Three loaded fixture cells at the controller/source positions are read before and after each phase and compared across cold reopen; no block write command is sent. The command path is `horizon-ctl command "kill @e[tag=ch004_probe,limit=1]"` and similarly `tp @e[tag=ch004_probe,limit=1] 9 64 8`.

On a failed assertion or control error, including during setup, the driver records the original error, requests peer quit through the pinned control binary, signals only a peer whose live PID, executable, socket argument and cwd match the disposable session if quit does not finish, and writes the matching stop request so `session.py` can stop the server. Cleanup has bounded waits and records its own outcome in `failure.json` or `owner-check-NN-failure.json`. An absent exit receipt remains a HOLD for Operations to inspect.

The server JSON line must report assembly `[1,2,3,4]`, carrier UUID `[5,6,7,8]`, exactly one active scan and no refusal, `move_return=false`, `remove_return=false`, retained entity and unchanged position. The peer JSON line must report that same UUID/assembly, `Preparing`, the slime/log view cells with log axis `z`, and `canonical_record_present=false`. The driver writes raw control replies, both parsed lines, frozen-input metadata, checks and the session exit receipt. Keep the raw server/peer logs and the `session.py` mirror alongside the driver output. A late-tracking peer and normal cold reopen prove the raw saved VIEW envelope transports without a new hydration path.

This minimal trial does not exercise the separate game-test area-discard guard. Its existing route is `/test create` or `/test clearall` with a test instance whose bounds include the display, but those commands clear world blocks before entity discard. Keep them out of this finite no-world-block-mutation packet. The trial proves data transport and refusal of four selected removal/relocation paths, not a renderer, live bearing, controller ownership, source-block capture, collision or material conservation. A failed condition is a HOLD with its raw receipts; do not repeat or broaden the run without a concrete repair decision.
