# Reviewed next-feature decisions

These are proposals, not implementation authority. Current already-approved
wheel, water-line, Drill and Tank checks continue independently. Preserve the
frozen007 build and its failed attempts. No proposal changes host pins or
raises the 512 bundled-content-file cap by implication.

| Feature | Concrete bounded proposal | Decision boundary |
| --- | --- | --- |
| Stationary capture | [Capture005](../contraptions/CONTRAPTIONS_005_STATIONARY_CAPTURE_DECISION.md): upward diagnostic bearing; capture and explicitly restore one slime block and one oak log; static peer drawing and ordinary reopen; existing V1 owner,6 new resources. | Prior owner foundation explicitly excluded world-block capture. Owner question is pending; no capture implementation. |
| Downward Chute | [Decision](../../../orchestration/teams/transport-006-chute-decision.md) plus controlling [parity refinement](../../../orchestration/teams/transport-006-chute-parity-refinement.md): Creative, fixed dry normal chute between plain single chests; canonical slot plus saved ItemPosition; Java probe below0.5/commit below0; up to16 from one source slot; cold blocked restore and single spill. | New block-local owner/position needs approval. Single-source-slot extraction and retention without supported lower output are explicit parity limits. No Chute implementation. |
| Quill Save | [World-local proposal](../trains-schematics/TRAINS_003_WORLD_LOCAL_SAVE_BRIDGE_PROPOSAL.md): Creative operator holding Quill; up to16 blocks per axis; blocks/air, no entities/BE; safe slug and no overwrite; world-local native GZIP template only. | New save authority and separately reviewed private safe-writer repair need approval. No schematic item, Java export/import, placement or Save implementation. |

Capture's 6 plus Chute's 5 would bring frozen007's 499 authored bundle entries to 510
if both were approved and composed without other additions. This is a content
file budget, not a world block-state palette or save schema. Original art
continues to come only from the separately pinned local JAR cache.

The [live upstream reuse check](../trains-schematics/TRAINS_003_LIVE_UPSTREAM_REFRESH_2026-11-06.md)
records official Horizon main 0d21d552 at 07:34 UTC on November 6. Its template
writer blob is identical to the pinned writer and still directly overwrites,
then caches even after write failure. The public Create port has inspect/paste
services, but no ready Quill Save bridge was found at dd0f3141. No upgrade was
adopted. The Chute supplement reuses the pinned chest slot/capacity services
to preserve Java timing without a new host API.
