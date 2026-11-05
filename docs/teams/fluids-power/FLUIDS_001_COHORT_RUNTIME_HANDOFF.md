# Frozen cohort runtime handoff — tank, straight track, millstone audio

This records the **frozen** combined source `abf48a5ec909b9b392ece15c56ed23f248cf3ed3`
only. It does not include the later tank codec repair or the later audio-enabled
build. The integration worktree stayed clean. All 661 native files matched
`G:\Create Rusty\.local\native-team-integration-001\source-inputs-r4.json`
(SHA-256 `3f204b535652c05435f3c2b817359da1caef36fdf759a5a63b837470c765fa14`)
both before launch and after all processes exited. Immutable game SHA-256 was
`d1d25c272aae2c49c19c7f564dcd0e3c3553cf698daea1a3a72976c329906055`;
control SHA-256 was `a459358e59882f926051d91485a666e23de5f18b953465ac069479a8072bc2ce`.
The supplied local Create 6.0.10 JAR matched
`ef87fe5709f1ba1f5b8bb20a2925b5afb4669e178fd6d8bf10c167759eefe37a`.
Package was `.10`, resource cache schema 13, pinned host `9c7f9b22`, Mixins
`e8cd0aa3`, native API 359, Rust 1.99.1. Project lead's unit/build receipts are
separate from these game receipts.

## Sessions and raw evidence

- Integrated: `G:\Create Rusty\.local\native-team-runtime-001\integrated-mn_pl_nk`,
  sessions 00 and 01 (normal cold reopen), actor exit 0 in both.
- Dedicated: `G:\Create Rusty\.local\native-team-runtime-001\dedicated-deu20fmt`,
  sessions 00 and 01 (normal server/client cold reopen), server, actor and
  rendered peer exit 0 in both. Each mode used a new owned home/world; its cold
  reopen reused that world with new sockets. The prepared resources were linked,
  not copied. The G evidence mirrors total
  about 12.7 MB; no shared cache was cleaned or duplicated.
- Each mirror retains `session-*.json`, `exit-*.json`, `calls-*.jsonl`, case
  receipts, failed attempts, game logs and PNGs. A case receipt's `passed` flag
  means its control sequence completed; behavioral conclusions are below.

## Per-feature result on this frozen binary

**Tank — HOLD.** Ordinary player placement produced
`bottom=true,top=true,shape=window`, `Window:1b`, `Size:1`, `Height:1`,
`TankContent:{}`, and retained `components:{}`. In both worlds a Creative
water-bucket click left `TankContent` empty, the held stack unchanged, and no
peer water. Normal wrench left `Window` and shape unchanged. The dedicated
Creative empty-bucket click also left explicitly seeded 1,000 mB unchanged.
The integrated Survival water-bucket click passed through to vanilla: tank NBT
was unchanged and the held water bucket became an empty bucket; its placed
world water was subsequently removed from the isolated pad.

`data modify block ... TankContent set value ...` supplied **explicit** 1,000 mB
and empty setup fixtures, not capability or ordinary-fill passes. Server NBT
retained the 1,000 mB fixture across both normal cold reopens, but rendered
actor/peer tanks showed no internal water surface. Dedicated peer images:
`dedicated_tank-initial-empty-peer-00.png`,
`dedicated_tank-seeded-water-1000-peer-00.png`,
`dedicated_tank-seeded-empty-after-water-peer-00.png`, and
`dedicated_cold-seeded-water-after-cold-peer-01.png`. The tank's actual
water-to-empty peer-render gate therefore **failed** on the frozen source.
The focused transfer seam unit checks had passed in the project lead's earlier
shared test run; they do not override this live failure.

**Tank repair, outside this frozen binary.** Pinned host
`block_entity_item::place` inserts empty `components`; `apply_placed_item`
decodes the tag; `decode_mod_block_entity` retains `components` in internal
`ModBlockEntity.nbt`; and `encode_block_entity` copies that NBT before adding
identity/coordinates for `/data get`. The tank codec's strict key guard rejected
the retained empty field before fill, wrench, or draw. A worker committed the
one-file narrow repair `3ccf56d88236d7b188da65f497ff0f7a1f19bd7a` on
`codex/team-fluids-power-repair`, based on `936c80b2`; it accepts only an empty
`components` compound and retains nonempty/malformed/foreign refusal. Its
native-envelope regression covers fill, empty, window toggle, preservation and
refusal. Independent read-only **Sol High** delta review was **CLEAR** for
`936c80b2..3ccf56d8`, 23:25:58–23:26:45 UTC (47 seconds). No repair build or
game run was performed in this lease.

**Basin — PASS for one affected exchange.** In the integrated world, an
ordinary Creative water-bucket click wrote 1,000 mB into input tank 0; an
empty-bucket click restored explicit empty `TankContent`. Held items were
accounted for in `basin-00.json`. This is the selected Basin interaction
regression, separate from the project lead's focused Rust Basin test.

**Straight track — PASS for this finite profile.** In the dedicated world,
ordinary Z/X placements with the actor's bearing delivered before item use
produced `shape=zo` at `(30,65,20)` and `shape=xo` at `(37,65,26)`.
A 135-degree diagonal attempt returned no placement; the target remained air
and the track count stayed five. Track-on-track item use changed neither block
nor count. A separate X track broke with a Survival diamond pickaxe, dropped
exactly one `create:track`, and was collected while the actor was still in
Survival (inventory count one, `creative=false`). Both retained tracks and
their shapes survived normal integrated and dedicated cold reopens. Rendered
peer before/after images include `dedicated_track-z-straight-peer-00.png`,
`dedicated_track-x-straight-peer-00.png`,
`dedicated_cold-z-track-after-cold-peer-01.png`, and
`dedicated_cold-x-track-after-cold-peer-01.png`.

**Millstone audio — event path PASS, audible output UNOBSERVED.** Qualified
motor/cog power gave the millstone `Speed:-16.0f`; an ordinary dropped wheat
stack entered `InputInventory` with 32 items. Integrated client `heard`
counters advanced to 124 `create:crushing_1` / 62 `create:crushing_2`, then
133/66 at power removal and stayed 133/66 during the quiet interval.
Dedicated actor and peer counters likewise increased with powered input, and
both stayed at 49/25 after the quiet interval following power removal.
The exact frozen game was built `--no-default-features`; pinned
`horizon-client` defaults its audio output feature, so this binary cannot
open a mixer stream. `status.sounds` showed zero channels/decoded audio.
No claim of audible output follows from the event counts. No microphone,
unrelated-app capture, install, or new audio instrumentation was used.

## Preserved fixture misses

- `track-00.json`: immediate ordinary placement saw a stale server bearing;
  an attempted X became Z, a diagonal attempt became X, then the pickaxe was
  in main inventory rather than the hotbar. Bearing synchronization and a
  clear hotbar resolved these setup errors in later cases.
- `track_recover-00.json`: expected diagonal refusal made the generic place
  helper return an error. The subsequent target query confirmed air.
- `pre_cold-00.json`: forced east-face interaction could not target the thin
  track plate. An ordinary unforced-face click succeeded with no state or
  inventory change; raw control calls retain that receipt.
- `dedicated_track-00.json`: all track behavior through Survival break ran;
  the one-off recorder then failed parsing a list-form drop position. Raw
  subsequent calls show the actor still in Survival, teleported onto that
  single drop, and collected one track. No source behavior was retried.

Root owns selective repair integration, the audio-enabled future build,
acceptance and any further runtime lease. No public release follows.
