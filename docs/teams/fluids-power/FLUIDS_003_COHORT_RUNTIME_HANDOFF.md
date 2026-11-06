# Frozen cohort 003 runtime handoff — Saw, repaired Tank, millstone audio

This is the November 5, 2026 bounded `fluids-cohort-003-runtime` lease only.
The project lead supplied branch `codex/create-team-integration-003` at clean
`55339ce3347b2ce667281e8a3e6318ec856144e4`. The 676 native source inputs
matched `G:\Create Rusty\.local\native-team-integration-003\source-inputs.json`
(SHA-256 `b6b913501fdfc3fe4326dcafb53fc8c47a8eb7dccc697615ba0a5d984e18aa4b`)
before launch and after all processes exited: **zero mismatches**. The private
team worktree also remained clean. The immutable Linux game binary was
`edba263e154f198b4a62033d3c468ffa6567c371c5201eb5ccb8577cc0ba807b`;
control was `170e866057d97e73b9231c8084547d4a4a02346238db35901ef64bf3d0d6d449`.
The supplied Create 6.0.10 JAR matched
`ef87fe5709f1ba1f5b8bb20a2925b5afb4669e178fd6d8bf10c167759eefe37a`.
This is the `.11`, asset-cache-schema-14, audio-feature-enabled build on pinned
host `9c7f9b22`, Mixins `e8cd0aa3`, API 359 and Rust 1.99.1. Build and focused
test receipts belong to the project lead; this packet ran the game only.

## Sessions and raw evidence

- Integrated mirror: `G:\Create Rusty\.local\native-team-runtime-003\integrated-v6lpjqhh`.
  Sessions 00 and 01 used one world and a normal cold reopen; actor exit 0 both
  times.
- Dedicated mirror: `G:\Create Rusty\.local\native-team-runtime-003\dedicated-pjp59tx1`.
  Sessions 00 and 01 used a separate fresh world and a normal cold reopen;
  server, actor and rendered peer exited 0 each time.
- Saw case receipts and peer PNGs:
  `G:\Create Rusty\.local\native-team-runtime-003\receipts`. The packet-local
  copy of Machinery's `saw002_milestone.py` adapted dedicated teleport delivery,
  data-get chat replies, settled motor stop and physical drop position. The
  original Machinery helper stayed unchanged. No Saw Speed/input/progress NBT
  was written; only the creative motor's `ScrollValue` was a power fixture.
- Audio output receipts and PCM/WAV samples:
  `G:\Create Rusty\.local\native-team-runtime-003\audio-captures`. The
  `audio-fixture.json` describes the project lead's private null-sink output.
  All capture receipts match an owned game PID and one sink-input index; no
  microphone or other application stream was captured. After all eight game
  process exits, the project lead stopped only that private Pulse server;
  `audio-fixture-stop.json` records exit code 0.

Each session mirror has its process identities, calls, case receipts, logs,
screenshots and exit receipt. Failed fixture attempts are retained. A case
receipt's `passed` flag means the bounded control sequence and its stated
assertions passed; the conclusions below use the raw values and images.

## Finite results

**Repaired one-block Tank — PASS in both worlds for this profile.** Ordinary
Creative placement produced the standalone window tank. Ordinary water and
empty bucket clicks changed `TankContent` from empty to 1,000 mB and back;
Creative held stacks remained unchanged. Normal wrench clicks toggled the
window/plain block state and NBT in both directions. Lava-bucket use left the
tank empty. Eight whole water-bucket clicks reached 8,000 mB, and a ninth did
not exceed that cap. In the dedicated world, eight empty-bucket clicks then
drained it fully and an ordinary water click restored 1,000 mB. In the
integrated world, seven empty-bucket clicks brought it from 8,000 to 1,000 mB;
an earlier ordinary click had independently drained 1,000 mB to empty. Each
world retained ordinary 1,000 mB after its normal cold reopen.
An explicit offhand inventory setup, followed by ordinary clicks, showed that
the main-hand wrench takes priority over the offhand water bucket and that an
offhand water bucket fills when the main hand holds a stick. The inventory
setup is not counted as a fluid-transfer pass. The dedicated peer images
`tank-00-01-full-8000-peer.png` and
`tank-00-01-empty-after-full-peer.png` visibly show blue water then an empty
window after ordinary clicks. At 1,000 mB the level is below the front window;
that cold check is NBT retention, not a visible-level claim. No seeded
`TankContent` was used to establish these Tank passes.

**Basin — PASS for one regression exchange in both worlds.** Ordinary
Creative water-bucket use put 1,000 mB into the Basin; ordinary empty-bucket
use removed it. See `basin-00-02.json` integrated and `basin-00-01.json`
dedicated. This is one selected regression, not broader Basin parity.

**Saw002 — PASS in both worlds for the bounded path.** Ordinary placement with
the reviewed shaft/motor fixture showed both X and Z shaft poses. One dropped
andesite alloy entered the powered blade and produced six shafts; the actor
collected exactly six. An unsupported cobblestone entered and returned as one
cobblestone. A further ordinary alloy was held mid-process when its motor was
removed; after power propagation settled, remaining progress and input stayed
stable, survived normal cold reopen, and resumed to six shafts after restoring
the motor. Dedicated peer images show the Z pose and the X pose in later
input/resume frames; the `shaft-x` pose frame itself is leaf-occluded. Peer
images also show input, product, held cold input, resumed product and empty
blade. Final post-resume census in each world
showed empty Saw inventory, no nearby item and six actor shafts; the dedicated
peer had no shafts. The dedicated unsupported endpoint likewise showed empty
Saw, no nearby item, one actor cobblestone and no peer cobblestone. Those later
censuses do not retroactively replace the earlier case snapshots.

**Millstone audio — output PASS on the private test output, not a physical
speaker claim.** An ordinary dropped stack of 32 wheat entered the powered
millstone. It processed wheat into `create:wheat_flour`, while the registered
`create:crushing_1` and `create:crushing_2` event counters advanced. Input and
output survived each cold reopen. The integrated session-00 client log reported
`Horizon: no sound: no audio output device opened`. After the project lead
started a private Pulse null sink, both dedicated session-00 clients already
logged `Horizon: audio output default (44100 Hz, 2 channels, f32)` through the
host's default audio lookup; no output capture was taken in that session. At
the planned session-01 reopens, private per-process Pulse/ALSA settings gave
all three clients that same output initialization. Five-second output-only
samples on each owned stream measured:

| Client | Powered RMS | Settled quiet RMS | Notes |
| --- | ---: | ---: | --- |
| Integrated actor | 203.32 | 1.29 | Saw motor stopped before comparison. |
| Dedicated actor | 1,385.12 | 0.00 | The active sample may include forest music, so it is not an isolated machine level. The first quiet sample was 1,148.48 with that music playing; the client's music option was then set to zero before the final quiet sample. |
| Dedicated peer | 1,157.09 | 0.00 | Separate peer sink-input, zero after power-off. |

The RMS values are 16-bit PCM units from the private null sink. They establish
game-process audio output and active-to-quiet behavior in these sessions; they
do not establish human listening through physical speakers.

## Preserved fixture misses and limits

- Integrated `00-alloy-01` missed the blade, landing at Z 21.125. The corrected
  ordinary drop passed in `00-alloy-02`; the failed trajectory remains recorded.
- Integrated `00-pause-01/02` missed or returned the item. `00-pause-03`
  entered the blade, then stopped after two propagation ticks (148 to 146
  remaining); `00-hold-01` proved 146 stable and saved that state. The helper's
  dedicated pause uses the settled value and passed directly.
- Dedicated `00-alloy-01` aimed before the server teleport reached the client;
  its item landed outside the blade. The packet-local helper then polled player
  position before aiming; `00-alloy-02` passed. Earlier missed item entities
  were removed from the isolated pad before that retry. The later unsupported
  no-nearby-item census is an endpoint observation, not an untouched history
  of every world item. No source change followed.
- Integrated hand attempts 01–06 exposed the host's key-F and inventory-sync
  behavior and one stray test block, which was removed from the isolated pad.
  Attempt 07 passed. Dedicated hand attempt 01 queried offhand before its
  network update; attempt 02 passed after an explicit wait.
- Millstone attempts 01–03 preserve an event-name assertion error and two
  physical wheat-drop misses/pickups. Ordinary drop attempt 04 integrated and
  attempt 02 dedicated passed product and event checks. The dedicated audio
  attempt 01 stopped when the peer landed about 0.9 block from its requested
  viewing point; its accepted nearby point was used in attempt 02.
- Track was already qualified and unchanged. Funnel remained HELD in the
  separate integration packet. Neither was replayed or claimed here.

Project lead owns acceptance, the shared heavy build/test queue, final Pulse
fixture cleanup and any next runtime lease. No public release is authorized.
