# FLUIDS-005 finite actual-game water-line packet

**State:** prepared, unrun. Operations owns the heavy game/build queue and must
provide the frozen 007 source map, successful check/restoration/build receipts,
held game/control binaries and prepared resources before either world starts.
This packet is a one-off use of the existing `horizon-client` and `horizon-ctl`
session/control pattern. It does not change production code, a host API, schema,
pin or general test framework. A static script review is not a gameplay pass.

## Frozen identity and launch gate

Run from the Linux host used for the native 007 game, not from Windows Python.
Before launch, fill the following variables from Operations' actual receipts;
the displayed paths and hashes are inputs, not values to infer from a prior
cohort. In particular, `SOURCE_HEAD` is the **full physical 001 commit** named
by the final frozen 007 map, and `RESOURCES` is the actual prepared resource
root from its prerequisite receipt. The runtime fails before starting a client
if any supplied digest, source head/map relationship, JAR, held binary, or
required resource manifest differs.

```bash
PACKET='/mnt/g/Create Rusty/.local/teams/fluids-power/tools/fluids_005_runtime'
BINARIES='<absolute 007 build run>/binaries.json'
BINARIES_SHA='<sha256 of that exact receipt>'
SOURCE_MAP='<absolute frozen 007 source map>'
SOURCE_MAP_SHA='<sha256 of that exact map>'
SOURCE_HEAD='<full 40-hex physical 001 HEAD from that map>'
JAR='/mnt/g/Create Rusty/.local/evaluations/create-horizon-05/inputs/create-1.21.1-6.0.10.jar'
JAR_SHA='ef87fe5709f1ba1f5b8bb20a2925b5afb4669e178fd6d8bf10c167759eefe37a'
RESOURCES='<absolute prepared base resource root from 007 prerequisite receipt>'
python3 "$PACKET/session.py" integrated \
  --binaries "$BINARIES" --binaries-sha256 "$BINARIES_SHA" \
  --source-map "$SOURCE_MAP" --source-map-sha256 "$SOURCE_MAP_SHA" \
  --source-head "$SOURCE_HEAD" --jar "$JAR" --jar-sha256 "$JAR_SHA" \
  --resources "$RESOURCES"
```

Use `dedicated` in place of `integrated` for the second, separate fresh world.
Start each `session.py` command in its own terminal; it prints the exact `run`
and `mirror` paths and waits for the case to close the world. The mirror is
under `/mnt/g/Create Rusty/.local/native-fluids-005-runtime/<run-name>`;
the world, player homes, configs and sockets are isolated under
`/home/arbiter/.cache/create-horizon/team-fluids-005/<run-name>`. No existing
playable save, Java launcher profile or prior cohort world is used.

For each mode, in another terminal, set `SESSION` to the **printed mirror**
`session-00.json` path and run:

```bash
python3 "$PACKET/case.py" "$SESSION" before-cold
```

Wait for the first `session.py` process to print its `exit-00.json` receipt
with all process codes zero. Then restart the *same owned run* using the
identical frozen options and `--run '<printed run path>'`:

```bash
python3 "$PACKET/session.py" integrated \
  --binaries "$BINARIES" --binaries-sha256 "$BINARIES_SHA" \
  --source-map "$SOURCE_MAP" --source-map-sha256 "$SOURCE_MAP_SHA" \
  --source-head "$SOURCE_HEAD" --jar "$JAR" --jar-sha256 "$JAR_SHA" \
  --resources "$RESOURCES" --run '<printed run path>'
```

Again substitute `dedicated` for that mode. Once the new `session-01.json`
appears in the *same mirror*, set `SESSION` to that path and run:

```bash
python3 "$PACKET/case.py" "$SESSION" after-cold
```

Wait for zero process codes in `exit-01.json`. Preserve `session-*.json`,
`exit-*.json`, `calls-*.jsonl`, both `*-result-*.json` files, full client/server
logs, `pre-cold-amounts.json`, the owned world and the dedicated peer PNGs.
Any assertion or nonzero exit is a HOLD, with the raw failure retained. Never
delete a failed run to make a later attempt look continuous.

## Fixed world and observations

The case clears only a disposable pad and creates this exact setup at Y=65:

| Position (X,Z) | State / role |
| --- | --- |
| (20,20) | `create:fluid_tank`, initial `TankContent` water 1,000 mB |
| (21,20) | `create:mechanical_pump[facing=east]` |
| (22,20) | `create:fluid_pipe[axis=x]` |
| (23,20) | `create:basin[facing=down]`, initially empty |
| (21,21) | `create:cogwheel[axis=x]`, side drive |
| (22,21) | `create:creative_motor[facing=west]`, `ScrollValue=0`, then `±16` |

There is no Mixer above the Basin, extra fluid endpoint, other Pipe, inserted
item, output fluid or second input fluid. The player inventories are cleared
at setup; a single bucket is given only at the end of the dedicated peer-view
sequence. The Tank water seed and later refill are explicit fixtures, not
ordinary bucket-fill claims. Actor commands are server-authoritative; peer
PNG files are real rendered client observations and need visual inspection.

Before cold close, the script uses `tick freeze`/`tick step` to measure:

1. With zero pump RPM, 16 steps leave Tank 1,000 / Basin 0.
2. With actual pump speed magnitude 16 RPM, ten active steps move exactly
   80 mB from Tank into Basin, or 8 mB per active tick.
3. An adjacent extra Pipe at `(22,65,19)` refuses ten steps without a water
   change. Removing it restores the same 8 mB/tick transfer.
4. Reversing the motor reverses actual pump speed sign, while ten active
   steps still move 80 mB **Tank → Basin** because Pump facing sets direction.
5. Stopping at a measured partial pair leaves both amounts stable after
   unfreezing. A normal save and client/server exit writes `exit-00.json`.

After a true cold relaunch of the same world, the script first checks the
exact prior pair and the 1,000 mB endpoint census. It then runs the powered
line to Tank 0 / Basin first input 1,000, with no item or other Basin content
introduced. At least 125 successful active ticks are needed from a full
1,000 mB source, plus startup; the bounded 150-step finish allows for the
already measured partial state and destination capacity. It explicitly refills
the Tank to 1,000 while the first Basin input remains full, then shows a
separate **2,000 mB** endpoint census unchanged over 20 powered steps. This
is capacity refusal, not water creation by the Pump.

Only dedicated mode captures actual peer images of the empty initial Basin,
the Basin filled by Pump, and the Basin emptied by one ordinary empty-bucket
click after the motor is stopped. Record the actor inventory and authoritative
endpoint NBT before/after that click. The bucket transfer is a distinct
previously accepted Basin behavior; its water leaves the two-block endpoint
census for the player's bucket. Visually inspect the three peer PNGs for the
same Basin, visible changed/empty state, unobstructed framing and no stale
frame. Filenames or successful screenshot calls alone do not establish a peer
pass. Compare the reopened server NBT (backed by the preserved normal save)
and peer images as separate evidence. No pipe-flow animation, exact level-phase parity, audio,
interrupted-tick crash atomicity or generic fluid-network claim follows.

## Prior evidence and qualification boundary

Reuse unchanged Tank creative bucket, wrench/capacity and Basin bucket,
render/cold evidence in `FLUIDS_003_COHORT_RUNTIME_HANDOFF.md` and
`docs/reference/NATIVE_S6_WATER_BASIN_RESULTS.md`; this packet does not rerun
their batteries. The changed Pump/Pipe path needs both modes, ordinary cold
reopen and actual peer views above. Operations' separate focused injected
destination-failure/restoration gate is required for callback rollback; this
normal-world script does not claim to exercise that path. An independent
read-only review of this exact packet is required before running it.
