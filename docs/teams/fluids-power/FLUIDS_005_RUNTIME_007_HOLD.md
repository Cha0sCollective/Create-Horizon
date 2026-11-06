# FLUIDS-005 007 integrated runtime HOLD

**Status:** one fresh integrated run stopped during setup; no Pump active-tick,
rate, refusal, reverse, cold-reopen or peer claim. Dedicated mode was not
launched. The exclusive game slot is released. The reviewed packet remains
`bdb2e1d16b7ea3e8e543d6e69acad080940fadfa`; no driver or production source
was changed after the failure, and the failed world is retained.

## Raw run

- Owned world:
  `/home/arbiter/.cache/create-horizon/team-fluids-005/integrated-sxsxi_r_`
- Windows mirror:
  `G:\Create Rusty\.local\native-fluids-005-runtime\integrated-sxsxi_r_`
- `session-00.json` SHA-256 `2f6215a02d7a976689fe91ef5e582c906a72e36f97ba2a2fab1d5e0cd4e0fd03`;
  `calls-00.jsonl` SHA-256 `c611ff75c594096120114a97b736011ce4089534248c6d579ef307a7dec6f89d`;
  `before-cold-result-00.json` SHA-256 `2fcadf8602a0b70739733bc0db5b4549543a7548674fce0478b001a08c153bb3`;
  `actor-00.log` SHA-256 `befc315b6710d2f36cd4c53c1be008f2f02ac2497780f8f76044805afe7e3f7b`.
- `exit-00.json` SHA-256 `586002fb7429ee9820d0b86a09ef9e8d6a05e1142c72532d5192a670a1b3fb95`:
  actor exit code **0**. On HOLD the actor received `tick unfreeze`, `save`
  (`ok:true`) and `quit` (`ok:true`); the session completed normally. No
  remaining actor PID was observed. The world and all raw artifacts are kept.

## Concrete driver issues before any retry

1. `tools/fluids_005_runtime/case.py:52-66` binds a fresh, unique-marker
   `time query gametime` reply, then parses only a terminal number. The pinned
   host replied **`The game time is 189 tick(s)`**. The assertion failed at
   the first `tick(2)` *before* any scripted active Pump step. This is a
   driver reply-format mismatch, not a Pump rate result. Keep the unique
   marker and single fresh reply; parse this observed result explicitly and
   review the narrow correction before another fresh run.
2. The setup command at `case.py:164`, `gamerule doMobSpawning false`, returned
   chat `Incorrect argument for command` even though the control wrapper
   reported `ok:true`. The pinned 26.3 host registers `spawn_mobs` in
   `horizon-client-core/src/gamerules.rs:115`; its data fixer maps legacy
   `doMobSpawning` to `minecraft:spawn_mobs`. A corrected setup must set the
   current rule and verify the actual rule/query feedback, rather than rely
   on the wrapper's `ok` field. This run therefore did **not** establish the
   declared mob-free profile.

These are two driver/setup issues. No fixture replacement, retry, or
acceptance relaxation was performed under this game grant.

## Frozen input and closure checks

After the normal close, physical 001 was clean at
`f732bd481cfc3d9c4720bc51e4a31e7385e18020`, native tree
`471136a9f27049a58bac2707c75b18466e94e17c`, 742 tracked native files.
Source-map SHA-256 remained
`0c97fc70d5ce5a8dc81bed1b5ccc21887fbc69628c94e0b0d1a6aff015c70f8a`;
binary receipt SHA-256 remained
`2ec5726527df08817663fa847df0b2dfdbb783deecc31d961c92a4ebae5074f5`;
prerequisite receipt SHA-256 remained
`43de03a356d6deb4d76465730f1c75098c4083e6120d865a518b349c95715396`.
Linux held game/control binary SHA-256, the Create JAR SHA-256, all four base
manifests, and all three composed tables were rehashed and matched the exact
receipt values in
`G:\Create Rusty\.local\native-fluids-005-launch-007\RUNBOOK.md`.
No client/server is left running for this owned run. The user-facing game
claim remains HOLD until a separately reviewed driver delta and new root
game lease produce complete integrated and dedicated evidence.
