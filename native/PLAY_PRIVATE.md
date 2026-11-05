# Private Linux prototype

This launch example uses the personal five-type Creative profile. Later bounded
Survival/machine profiles are recorded in the native S4-S6 results; they do not
qualify every included block. The current private package
development candidate is `0.5.0-ch4os.11`, based on upstream0.5.0 with selected later reuse. Its new saw/tank/track/audio packets still need their recorded game qualification. Use the same private build on
the server and clients. Original Create IDs remain `create:`.

## Build

Use Rust1.99.1 and a clean official Horizon checkout at
`9c7f9b2215dfed497aba5522c44f38926628648d`. The pinned Mixins revision/API359 is
selected by `compatibility.json`. This workspace already has the Linux tools and
prepared Minecraft26.3 resources from the owner's launcher installation. Fresh
setups follow the pinned host's resource-preparation instructions using their own
Minecraft files, and this package's `tools/bootstrap.py` for local Rust tools.

From `create-horizon/native/create/`:

```sh
HORIZON_SOURCE="/path/to/pinned/horizon" ../../tools/dev.sh python3 tools/stage.py build --work-dir "$HOME/.cache/create-horizon/native-stage"
```

This builds the game with Horizon's existing audio output and the matching local
control tool; optional controllers remain disabled. Audio device availability
and actual playback still need an observation on the resulting binary.
Use a new/empty generated
output directory on Linux storage. Builds refuse a different host pin or an
unowned nonempty external output. The current workspace's compatible host lives
at `/mnt/g/Create Rusty/.local/evaluations/horizon-heating-refresh/source`.

## Start a separate instance

Choose a new instance folder. The helper keeps its game, resources, options and
launcher data there; it refuses an unowned nonempty destination. It preserves the
original default development instance when `--instance-root` is supplied.

```sh
../../tools/dev.sh python3 tools/play.py --resources "/path/to/prepared/26.3" --create-jar "/path/to/create-1.21.1-6.0.10.jar" --instance-root "$HOME/.cache/create-horizon/my-private-prototype" --test-terrain
```

On first entry, open chat and run `/gamerule minecraft:spawn_mobs false` to use
the qualified test profile. In this new empty world, run
`/kill @e[type=!minecraft:player]` before building to remove any initially generated
animals. The world is Creative and uses fixed test
terrain. S4-S6 separately qualify their finite normal-world Survival profiles.

The supplied Create6.0.10 JAR is a local resource input. The mod checks the exact
JAR and imports art automatically; it never runs the Java mod. Keep that JAR,
imported art, prepared Minecraft resources and instance folders local. See
[asset provenance](create/ASSETS.md) and [source provenance](UPSTREAM.md).

The development helper also accepts `--offscreen --control-socket /path/game.sock`
for the isolated rendered smoke checks. Offscreen mode does not grab the pointer.
Normal play opens a window. The currently verified graphics route is Linux/WSL
with Vulkan; performance and Windows-native packaging are not qualified.

## Use the milling chain

Get a Creative Motor, Shaft, Cogwheel, Large Cogwheel and Millstone from Creative
inventory. Connect a motor and shaft along their rotation axis. Small-to-large
diagonal gears halve speed; large-to-small doubles it. Both reverse direction.
The millstone accepts a shaft from below or a neighboring small cog connection.

With empty hands, right-click the side of the motor to open its speed board.
Choose a speed on the positive or negative row, then Done. Drop cobblestone into
the millstone from above with the normal drop key. With empty hands, right-click
the millstone to collect gravel. Existing source/speed diagnostics and normal
saves are used; no special host checkpoint system is involved.

## Coal and brass subset

In a separate supplied-materials Survival profile, place an occupied Blaze
Burner, a Basin directly above it and a Mechanical Mixer two blocks above the
Basin. Supply copper ingot1 and zinc ingot1 through a vanilla hopper and power
the Mixer at32RPM or more within the accepted network limits. Right-click the
occupied burner with plain coal; the heated recipe gives exactly two brass.
Collect Basin output with an empty hand. Empty cages cannot be fueled; blaze
capture/acquisition and other fuels are deferred. Plain-water Basin support is
separate from the heating recipe.

Fuel at the full10,000-tick buffer is consumed without adding time, matching the
owner-selected Java behavior. Sneak-wrench pickup gives a fresh occupied item
without retained fuel. Ordinary save/reopen preserves the placed burner's fuel,
which resumes ticking normally. See the exact
[heating result](../docs/reference/NATIVE_S6_COAL_HEATING_RESULTS.md) for fixtures,
limits and the separately counted additional presentation runs.

## Plain-water dough subset

With the same supplied-materials Basin/Mixer arrangement, right-click the Basin
with an ordinary water bucket to insert1000mB and receive an empty bucket. Supply
one plain wheat flour through a hopper and power the Mixer at32RPM or more;
collect one dough with an empty hand. An empty bucket can recover a full1000mB
before processing. Main/offhand exchanges and stacked empty buckets are supported.
An already full matching source refuses another water bucket without consumption.

Only plain input water is supported; lava clicks are refused at the Basin.
Mixed/output fluids, pipes/pumps, modified buckets and general fluid automation
are outside this profile. Whole buckets do not top up or drain a partial500mB
fixture. Normal save/reopen retains supported stored water/flour. See the
[water result](../docs/reference/NATIVE_S6_WATER_BASIN_RESULTS.md) for the exact
fixture/refill and rendered-observation limits.

Dedicated play uses the same built game through Horizon's existing `--dedicated`
route, with server/clients in separate instance homes. Existing mod negotiation
names a missing or different declared package version before play. It does not
authenticate arbitrary changed native code that declares the same version.
This is a private source-build route, with no launcher registry or public release.
