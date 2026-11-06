# TRAINS-002 — structure-service reuse and Create selection decision

Status: read-only design decision, 5 November 2026. No source, schema, host pin
or gameplay outcome changes here. Accepted straight X/Z track evidence on the
first cohort remains accepted and is not repeated. Horizon's existing **vanilla
structure-block** Save/Load is a reusable host workflow, not a newly implemented
Create schematic, quill, Schematicannon, Java/world import or train system.

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
`ignore_air=false` (`:299-307`), so air inside a selected box is captured;
`ignore_entities=true` excludes entities. The UI size and offset limit
is 48 blocks per axis (`crates/horizon-client-render/src/structure_block.rs:10-14`).
On a dedicated LAN guest, structure editing requires Creative and permission
at least 2 (`structure_blocks.rs:568-582`). The peer can remain an observer.
The template writer is private and the area save method is `pub(super)`; this
does not already expose a Create-specific schematic/quill workflow.

## Smallest Create-specific follow-on decision

Select a `create:schematic_and_quill` **two-corner selection UX only** as the
first implementable Create-facing packet. Pinned Java 6.0.11
`content/schematics/client/SchematicAndQuillHandler.java:93-136,180-205`
defines first/second click, status, selection box and cancel behavior;
`SchematicPromptScreen.java:45-74,108-110` names a later saved schematic.
The native lane already has client interaction and HUD injection seams in
`native/create/mixins/create/horizon-mixin.toml:392-405,489-498`, with a working
client pattern in `motor_control.rs:39-86,306-323`; item registration/content
generation is in `native/create/tools/catalog.py` and `content.py`. A bounded
implementation would add the item and selected art, client-only first/second
corner state and coordinate/size HUD, with a selected maximum of 16 blocks
per axis, plus cancel/reset on sneaking, item change and world/screen exit.
It would make **no world write**. Java's
world-space selection outline needs a separately verified native draw seam;
this first packet would not claim it.

The selected exit checks would cover normal and reversed corner order,
inclusive dimensions within the 16-block-per-axis bound, HUD visibility,
cancel/reset and noninterference when another item is held. Focused pure
selection tests and one project-lead-scheduled integrated/dedicated peer UI
milestone would be appropriate if that packet is allocated. No driver, build
or vanilla structure-block replay is started by this design handoff.

Java `content/schematics/SchematicExport.java:38-63` writes a separate
compressed client file and `SchematicAndQuillHandler.java:207-230` may upload
it. Export, upload, server validation, placement, transforms, material
checklist, cannon, inventories, actors, entities, curves, moving assemblies,
cross-world transfer and external `.nbt` ingestion remain separate decisions.
A later server bridge could use Horizon's internal
`structure_blocks.rs:309-347` for world-local static Save after an explicit
authority/permission design; that method is not a public mod API and ordinary
structure-block editing is Creative/operator-gated. Do not equate the
two-corner UX with a saved schematic or complete workflow.
