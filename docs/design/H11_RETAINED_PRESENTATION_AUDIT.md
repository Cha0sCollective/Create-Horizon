# H11/H12 retained diagnostic presentation source audit

READ-ONLY planning evidence at basec0ad9e0 during fence qualification. No renderer,
wire/schema/API adoption, source release or H11/H12 pass. There is no retained
arbitrary world-model instance service or committed counter presentation transport.
H11 ownership/resource work can overlap H09/H10; H12 needs accepted H10 tracking
projection. Reading the window's local counter bypasses default integrated ownership.

Reusable seams (relative horizon/crates/horizon-client/src unless noted):

- SDK horizon.wit278-433 and game/mod_render.rs1/87-96 retain GUI shapes, not world
  meshes/materials/transformed instances or animation parameters.
- world/counter_restore.rs490-537/864-938 has persistent placement identity and
  checked committed reads; counter_trial.rs196-214/267-286 has counter, simulation
  tick, residency/fault but no signed speed/phase anchor/effective presentation state.
- game/counter_control.rs160-226/game.rs5444 preserve server-only commit/tail rules.
- gfx.rs2900-3007/3054-3089 has terrain arena/upload/release conventions;
  section/LOD instances1227-1237/3804-3827 hold origins/LOD, not arbitrary rotation.
- block_entity_render.rs266/327/896-929 builds transformed CPU vertices every frame;
  gfx.rs4252-4273/gfx/entity_batch.rs36-112 uploads entity geometry every frame.
  These are not evidence of retained model geometry/compact parameter updates.
- mesher.rs141-145/753-778 separates block-entity registration from static emission:
  block_entity=true alone does NOT suppress the static model. Avoid duplicate draws.
- mod_content.rs664-721/1311-1325/1425-1440 offers captured overlays/model tables;
  missing authored blockstate can install template geometry. H03 is static only.
- game/packs.rs174-249 stages resources/renderer, preserves old surface on failure,
  swaps after success and reloads streaming. Natural resource-generation boundary.
- game/remote.rs1789-1798/1114-1141 and gfx.rs3073-3089/3121-3155 remove tracked world/
  renderer data. New handles need immediate untrack/replacement retirement, not
  delayed cleanup through stale meshes. Remote399/573/1070 and game.rs9003/9146/9374
  provide connection/dimension/world retirement seams.
- clock.rs378-423/remote.rs2526-2553/2923-2928/3549-3557 provide freeze/partial/server
  tick data. Client time advances between updates; camera interpolation remains
  smooth while frozen and cannot drive authoritative machine phase.
- execution_role.rs5-20/game.rs1493/13694/14865-14871/integrated.rs250 distinguish
  client presentation/headless roles. Prove zero dedicated renderer allocation.

Smallest next investigation: bounded H11a retained ORIGINAL diagnostic part;
H12 connected animation contingent on H10. Missing pieces: immutable bounded model
resource, one resource-generation-scoped instance, compact parameters, stale-handle
retirement, explicit static/dynamic ownership and committed timestamped input.
No general guest render framework/per-frame guest callback is necessary.

Impact map, NOT accepted scope: new small gfx module/matching shader if needed;
gfx.rs declaration/frame/draw; new client presentation owner/Game field/frame;
packs reload; remote tracking retirement when H10 supplies it. Content/mesher only
if replacing a static part. Do not change physical iron binding incidentally.
Do not edit the protected fence five. Exact files require source-level design review.

Later required proof: one instance/retained allocation after warmup; geometry upload
only on resource/content change, compact frame updates; replacement/untrack/chunk/
dimension/session/package-disable/reload cleanup (success and failure); no duplicate
component; tick-anchored forward/stop/reverse/freeze/resume continuity across frame
schedules; rendering never advances machine/slot/recipe state; fresh integrated/
in-frame visible evidence, dedicated zero allocation. H12/G1 awaits H09/H10.

Asset direction already accepted: original boxes/axes/simple authored textures
(G0_DECISION.md31-34, HOST_CONTRACT.md). Record coordinates/pivot/axis/sign, bytes,
author/source revision and hashes. No Create/Flywheel art. Minecraft iron is the
native fixture base from local resources, not original art; never copy its assets
into Git. An original disjoint pointer can avoid duplicate geometry in a narrow
test, not prove arbitrary static/dynamic exclusion. No new owner choice required
for continued original-diagnostic design work. No proposal is adopted by this audit.

Root verified unchanged SHA256 anchors:

- gfx.rs 8653c318737989abeee14966fa6b0a0e6e457c79f1ccf6a02108fc8334199a3d
- mesher.rs 733322bc15171e14a8531f18fd075f947858f22cd469a55f3a4a910631f043d7
- mod_content.rs 1f1403b659c07030efd4331f6e51098df4b58c03a84c1a517d56dd301dbc2355
- game/packs.rs 372e7a22256a4adba315c2f206b9e660983fc52e821ad9e4c09a736149771da6
