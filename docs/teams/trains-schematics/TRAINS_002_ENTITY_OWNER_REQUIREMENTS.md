# TRAINS-002 — requirements for a reusable contraption owner

Status: source-backed design handoff only, 5 November 2026. The accepted native
source is `55339ce3347b2ce667281e8a3e6318ec856144e4` (`.11`, asset cache 14);
this team's source baseline is `99d97fb187c3e1b129d5d8ae8908220b5acd2622`.
The accepted stationary `create:track` X/Z profile remains unchanged. No train,
rail graph, entity owner, save codec or new acceptance claim is implemented here.

Source: pinned Create Java 6.0.11 at `7ff2760e4924d35d4bc2e22c4b19923f0178cd16`
under `references/create/src/main/java/com/simibubi/create/`; native host pin
Horizon `9c7f9b2215dfed497aba5522c44f38926628648d`, Mixins
`e8cd0aa38cffef49cb2e088ef716baa107148c02`. The staged Create 6.0.10
source and local JAR are distinct art/port inputs, not proof of full parity.

## Source ownership model

Java saves a train and its graph in `RailwaySavedData.java:24-26,33-66`, loading
graphs before trains. `Train.java:107-127,1158-1214` gives a train its stable
UUID, graph reference and carriages. A carriage entity stores `TrainId` and
`CarriageIndex`, rebinds through the railway manager
(`entity/CarriageContraptionEntity.java:319-330,469-479`), and explicitly returns
`false` from `shouldBeSaved` (`:719-723`). It is therefore a live projection and
link, not the sole durable train record. Bogey travelling points resolve pose
against a graph (`entity/Carriage.java:148-220,322-413`;
`entity/CarriageBogey.java:100-166,192-209`). Cargo is held by `Carriage.storage`
(`entity/Carriage.java:75-93,652-665`); `entity/CarriageContraption.java:131-143`
transfers storage to the carriage rather than keeping a duplicate inventory in
the projection.

The first Java mechanical bearing instead captures blocks and spawns a
controlled contraption entity (`content/contraptions/bearing/MechanicalBearingBlockEntity.java:167-205`).
Its controller angle and entity structure/link/axis are saved separately
(`MechanicalBearingBlockEntity.java:77-98`;
`content/contraptions/AbstractContraptionEntity.java:611-651`;
`content/contraptions/ControlledContraptionEntity.java:80-97`). This describes
the reference lifecycle; it does not require the native first bearing to use a
new entity kind.

## Contract to preserve in the first live bearing design

| Contract | Immediate bounded bearing requirement | Train-specific addition, deferred |
| --- | --- | --- |
| Authority and identity | One stable assembly ID names the exact captured block owner, independent of controller position, chunk key or render object. The versioned payload records exact source cells and verified capture/restore progress. Only the server record can advance custody. | A stable train ID maps to ordered carriages; a carriage's live entity ID is a rebindable projection. Reject duplicate or orphaned owners across dimensions and chunks. |
| Transform and tracking | Keep local block coordinates, anchor/pivot, axis, prior/current angle and tick epoch independent of backing storage. Server pose drives contact; the client receives a read-only tracked pose. | Graph UUID and bogey travelling points determine each carriage anchor, orientation and cross-dimension presentation. Graph edits, derailment and multi-carriage tracking need their own contracts. |
| Contact | Define the two-block swept envelope and player/entity refusal or safe response before each bearing pose advance and restoration. | Continuous moving-solid, coupling, passenger/rider and station contact are separate train work; a display entity alone provides no physical contact. |
| Storage | Refuse inventory-bearing/actor blocks in the inert first bearing. Keep captured block snapshots and any future storage pointer out of a second client-owned copy. | Carriage cargo becomes one canonical storage owner, with interfaces to portable transfer, stations and schedule filtering. |
| Normal save/reopen | On ordinary unload/reopen, recover the single owner, its pose and progress, then reconcile the controller and exact world cells before movement or restore. Missing/invalid ownership must hold for repair rather than recapture or delete cargo. | A global graph/train record must load before carriage rebind, with a defined policy for absent graph, missing entity, chunk unload and duplicate projections. Test cross-file/cross-chunk ordering before a train claim. |

The primary project's Contraptions-002 proposal uses one saved, synced chunk attachment for
its same-chunk two-block bearing (`docs/teams/contraptions/CONTRAPTIONS_002_LIVE_BEARING_PROPOSAL.md`).
That is a bounded backing. Its ID, payload, checked custody and pose/contact
semantics should remain portable; one-record-per-chunk, same-chunk clearance and
controller adjacency must not become requirements of the assembly codec. The
entity-owner addendum (`docs/teams/contraptions/CONTRAPTIONS_002_ENTITY_OWNER_ADDENDUM.md`)
finds that pinned Horizon can save typed entity attachments and tracked displays,
but an unsimulated marker/display does not provide the independent tick, removal
guard, rediscovery and moving-solid contact needed for sole ownership. Train
authority therefore needs a later explicit owner/lifetime decision; no current
content declaration or marker is a complete train owner.

## Gate for a later train packet

Before implementing assembly, graph travel or carriage entities, specify and
review one canonical train/graph save owner, stable IDs and rebind rules; exact
bogey/graph pose and client tracking; single cargo custody; contact; and ordinary
save/reopen across the selected files and dimensions. The first bearing may
validate portable record, checked capture/restore, pose and conservative contact
without implementing any of those train-only services. No Java schematic/world
import, public host change or train runtime follows from this handoff.
