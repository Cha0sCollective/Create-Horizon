# S6 stationary straight track — team candidate

Status: source candidate in `codex/team-trains-schematics` at
`79c95f7c0ce87a5502761a8b4c9642250f3163a0`. Independent read-only Sol High
review held two findings at `aa30f68e93ca7ead94301141fffeb9cc280c9099`;
the narrow repair delta received CLEAR at `79c95f7`. Shared build/game
qualification is pending. This packet is one finite
track foundation, not a rail graph, train, schematic or S6 completion claim.

## Inputs and provenance

- Accepted private base `02d029f3bea09c62480e07e315e23d4868f9180a`;
  team lane starts at operations adoption `76dfaf78370f41b2a5c68620c55be6ecab96f16f`.
- Pinned Java behavior reference: Create 6.0.11 `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`,
  `TrackShape`, `TrackBlock`, `TrackBlockItem`, `TrackModel` and `AllShapes`.
  The original local art input and staged source are Create 6.0.10
  `ac0c444d9828da3453ae8cc65338e8de063286fb`, local JAR SHA-256
  `ef87fe5709f1ba1f5b8bb20a2925b5afb4669e178fd6d8bf10c167759eefe37a`.
  These are distinct versions; no general equivalence is claimed.
- Public Rust port main checked at `dd0f3141a33851ac20fa98b807a6532471dba727`:
  it has no track behavior/content. The accepted host stays Horizon
  `9c7f9b2215dfed497aba5522c44f38926628648d`, Mixins
  `e8cd0aa38cffef49cb2e088ef716baa107148c02`, native API 359.

## Selected behavior

1. Register original `create:track` with source state names `shape=xo|zo`,
   `turn=false`, `waterlogged=false` on ordinary placement. Source `ZO` is
   default. This first profile accepts dry air targets and headings within
   22.5 degrees of an orthogonal axis; diagonal and ambiguous headings refuse.
2. Conservatively refuse a raised neighbor that could select an ascending
   source shape. A nonreplaceable neighbor with replaceable space above is
   treated as a refusal, even when the source might accept a straight shape.
   `canSurvive` implements the source rule: a track directly below is refused;
   air or other blocks below are allowed. Neighbor changes rely on the existing
   host survival hook. Track-on-track item use is claimed without a stack
   mutation because source curve selection has no owner in this packet.
3. Use the source flat 16x2x16 collision plate. The provisional authored model
   uses vanilla iron-block texture and stays within the cell. With the user's
   exact local JAR, import only original X/Z straight models and track item art
   into the private instance cache. The original wider selection outline is
   not yet implemented or qualified. One authored loot table returns one
   `create:track`; a narrow `minecraft:mineable/pickaxe` tag supplies the
   source tool requirement. Survival crafting remains outside this profile.
4. Do not create a track block entity, global rail graph, owner tick, station,
   curve, junction, portal, schedule, train, water placement, Wrench behavior,
   or new save format. `turn=true` and `waterlogged=true` remain registry
   representable for source state names but are not ordinary supported actions.

## Candidate file ownership and shared integration

- Team-owned behavior: `mixins/create/engine/src/core/track.rs`,
  `mixins/create/track.rs`, `track_placement.rs`, `track_survival.rs`, and
  `tools/content.py`/`tools/catalog.py` under `native/create/`.
- Isolated candidate shared wiring: `mixins/create/horizon-mixin.toml`,
  `mixins/create/engine/src/lib.rs`, `mixins/create/use_on.rs`, asset importer
  `mixins/create/assets/src/{lib.rs,tests.rs,catalog.rs}`, generated public
  content and `mixins/create/data/content.json`. The project lead must resolve
  shared manifest/catalog/version integration serially. Asset cache schema 12
  is needed because the selected resource catalog changes; this is not a
  world-save schema change. Package version remains on the accepted `.8` pin
  until project-lead composition.

## Exit checks and gates

- Pure angle tests: Z/X headings, diagonal/boundary refusal, wraparound.
  Asset tests: only eight selected straight original blockstate variants and
  their two models imported; a missing/mismatched selected variant refuses.
- Static checks: generated JSON, bundled byte equality, manifest/Python parse,
  focused Rust formatting and whitespace. These do not prove compilation.
- After root grants the host-wide reservation: focused Rust tests and locked
  build, then one isolated integrated and dedicated/two-client milestone for
  ordinary X/Z placement, refusal, one-item break/collection, actual model
  visibility and normal cold save/reopen. Reuse accepted piston/portable evidence;
  run only affected interaction checks because shared `use_on` is touched.
- Fresh independent read-only Sol High source review is complete on the
  source and repair delta. Root owns resource scheduling,
  integration, acceptance and any private sync. Java execution, public release,
  external schematic/world import and broader train contracts remain out of scope.
