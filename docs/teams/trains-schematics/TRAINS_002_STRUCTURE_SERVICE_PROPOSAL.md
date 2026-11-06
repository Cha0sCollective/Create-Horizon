# TRAINS-002 — bounded native structure selection and save/load proposal

Status: read-only packet proposal, 5 November 2026. No source, schema, host pin
or gameplay outcome changes here. Accepted straight X/Z track evidence on the
first cohort remains accepted and is not repeated. The next proposed result
qualifies one existing **vanilla structure-block** service with accepted static
Create blocks; it is not a Create schematic, quill, Schematicannon, Java/world
import or train system.

## Existing service and exact limit

Pinned Horizon `9c7f9b2215dfed497aba5522c44f38926628648d` was inspected in
the read-only local source at `.local/evaluations/horizon-heating-refresh/source`.
`crates/horizon-client/src/game/structure_edit.rs:73-110,179-207,347-360`
provides in-game structure-block name, offset, size, Save, Detect and Load UI.
`crates/horizon-client/src/game/structure_blocks.rs:145-181` writes GZIP NBT to
`<world>/generated/<namespace>/structures/<path>.nbt`;
`:186-245` captures blocks, their block-entity NBT if any, and optionally
entities; `:289-347` saves the selected area; `:363-390` loads through the
native template placement path. The standard UI Save call passes
`ignore_air=false` (`:299-307`), so air inside the selected box is part of this
fixture; `ignore_entities=true` excludes entities. The UI size and offset limit
is 48 blocks per axis (`crates/horizon-client-render/src/structure_block.rs:10-14`).
On a dedicated LAN guest, structure editing requires Creative and permission
at least 2 (`structure_blocks.rs:568-582`). The peer can remain an observer.
The template writer is private and the area save method is `pub(super)`; this
does not already expose a Create-specific schematic/quill workflow.

## One finite qualification packet

1. Use the project lead's frozen accepted native source and host-wide runtime
   reservation. Set up new disposable integrated and dedicated/two-client
   worlds, with isolated launcher/config/resource paths. Do not use a playable
   save. Establish a single loaded one-layer selection containing one dry
   `create:track[shape=zo,turn=false,waterlogged=false]` and one
   `create:track[shape=xo,turn=false,waterlogged=false]`, separated by air.
   Explicit fixture state is sufficient: ordinary track placement was already
   qualified. Keep the structure block itself, all other block entities and
   incidental entities outside the selection.
2. Through the in-game Save-mode structure-block editor, set a private test
   name, exact offset and size, `ignore_entities=true`, and save to the world.
   Record UI success, selected world cells and the resulting world-local file.
   Decode its GZIP NBT read-only: exact size and positions; palette/blocks
   contain only those two track states and selected air; no per-block NBT or
   entity entries. Record the format's `DataVersion`, without asserting Java
   interchange.
3. Through the in-game Load-mode editor, place that same saved template in a
   separate clear area of the **same world**, at no rotation or mirror and
   full integrity. Query the authoritative destination states and obtain
   actual actor and dedicated peer images of the two restored track models.
   A server query alone does not establish client presentation.
4. Close normally and reopen each disposable world. Verify the saved template
   remains in the world-local generated path and both placed track states
   remain exact. Keep process exits, NBT decode, world queries and peer images
   together. Qualify only this static selection/save/load path; do not add a
   new source feature or run a historical track battery.

The packet refuses a source area containing moving assemblies, block entities,
inventories, entities, water, curves or unsupported track states. No rotation,
mirror, cross-world file copy, external `.nbt` ingestion, Create-specific
material accounting, survival workflow or large selection is claimed. A later
Create schematic UI/export adapter needs its own source/ownership design and
permission boundary; the existing host writer cannot be treated as that
adapter without work. If the project lead prefers a code packet first, this
qualification still serves as a measured host-service preflight rather than a
train or graph implementation.
