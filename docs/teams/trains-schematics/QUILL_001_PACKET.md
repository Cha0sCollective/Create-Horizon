# QUILL-001 — client-only two-corner selection candidate

Status: isolated source candidate on `codex/team-trains-schematics-quill-001`,
based on accepted private `55339ce3347b2ce667281e8a3e6318ec856144e4`
(`.11/cache14`). This packet does not save a schematic, send a custom packet,
edit world blocks or change the accepted stationary track result. The project
lead owns shared manifest/catalog/content/cache/version composition and the
host-wide build/game reservation.

## Source and selected behavior

Pinned Create Java 6.0.11 `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`
registers `schematic_and_quill` with a one-item stack
(`AllItems.java:492-495`). Its client handler selects first and second corners,
reports no target, and cancels with sneak/right-click
(`content/schematics/client/SchematicAndQuillHandler.java:93-135`). A vertical
hit on a nonreplaceable block selects the adjacent cell (`:138-164`), and the
source outline includes both endpoints (`:180-195`). Java can target up to
75 blocks and select midair with a separate key; this bounded native client
uses the host's ordinary block hit/reach and omits midair targeting. The native
16-block-per-axis cap, persistent coordinate/size HUD, and reset on item,
screen or world exit are deliberate profile choices; the Java handler does
not impose that cap or reset its stored corners on those exits.

The new pure `engine/src/core/quill.rs` computes normalized inclusive bounds,
refuses a 17th block on any axis while preserving the first corner, handles
missing target/cancel/third click without world effects, and selects the cell
above/below a vertical nonreplaceable hit. `quill_client.rs` keeps transient
state in the client process. `quill_key.rs` queues one action for each right
mouse press edge, including quick separate clicks between frames; repeated
held-use callbacks remain claimed without advancing a corner. The interaction
helper claims a main-hand Quill right-click before ordinary item use and draws
a readable coordinate/size HUD. `quill_frame.rs`
and `quill_unload.rs` provide explicit reset seams. No world-space outline,
prompt or export is implemented. A third click leaves the complete pair in
place and says saving is unavailable.

## Serial shared integration requested from the project lead

- Export the new pure module as `create_engine::quill` from
  `native/create/mixins/create/engine/src/lib.rs`. Add `quill_client.rs` as a
  distinct `create_port_quill` Game module. Bind the five new thin helpers
  `quill_key.rs`, `quill_click.rs`, `quill_hud.rs`, `quill_frame.rs`,
  `quill_unload.rs` to the existing `key`, `interaction_key`, `hud`, `frame`,
  `client_level_unload` hooks. Keep
  this call seam separate from Experience's `presentation.rs`. The pinned host
  invokes cancellable `interaction_key` before ordinary item use
  (`horizon-client/src/game/remote.rs:7329-7345`), so the client-only handler
  prevents outgoing world-action packets for the Quill click.
- Add `schematic_and_quill` to the item catalog and generated item declaration
  with `max_stack_size=1`, a vanilla-texture provisional model, item definition,
  English fallback name and creative-tab entry. The accepted generator's
  `content.py:299-306` otherwise defaults to stack 64/iron-ingot art;
  `:573` rewrites the tab later, so include the item in the final generated
  list. The existing importer traverses catalog `ITEMS` models
  (`assets/src/lib.rs:549-551`); keep original art in the user's private
  local-JAR cache only. Root resolves the corresponding asset-cache key and
  package version alongside other shared changes.

Art provenance: staged Create 6.0.10 source
`ac0c444d9828da3453ae8cc65338e8de063286fb` and exact local JAR SHA-256
`ef87fe5709f1ba1f5b8bb20a2925b5afb4669e178fd6d8bf10c167759eefe37a`.
The JAR's `assets/create/textures/item/schematic_and_quill.png` SHA-256 is
`fd6dfa54ed4c6a610dd9a046db3380b30b6adebd2fb1681af39e2753986f6f8a`;
its `models/item/schematic_and_quill.json` SHA-256 is
`e8d2549df2819bb71583acac175c4468be27f1097311fb970e1a22a6e4b411c1`.
No original image, JAR or generated private cache enters Git.

## Focused checks and limits

Pure tests cover reversed coordinates, inclusive 16/17 boundary, no-target,
cancel, third-click retention, vertical-hit selection and held-use repetition.
On root's later
host-wide lease, compile the composed module and run its focused tests, then
one disposable integrated/dedicated two-client UI milestone: hold the first
click through several use intervals, release, then click the second corner;
check size/coordinates, over-cap refusal, item/screen/world reset and
noninterference with another held item. Check that no world cells or item
counts change and that the peer's own view does not acquire the actor's
selection. No existing track game battery is repeated.

Java's `SchematicExport.java:38-63` writes a separate compressed file and
`SchematicAndQuillHandler.java:207-230` can upload it. File export, server
validation, schematic item, vanilla structure-block Save/Load integration,
placement, transforms, materials, cannon, moving assemblies, train graph and
external Java/world import stay out of this packet. Static inspection or
source review alone cannot qualify the UI or gameplay.
