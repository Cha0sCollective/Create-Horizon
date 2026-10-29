# Detached cursor ownership audit

Read-only prerequisite audit at local engine base 51e2882, with pinned Minecraft
26.3 client bytecode inspected using installed Java 25 javap. No cursor fix is
implemented or accepted by this note. H06d player-cell work is independent.

Horizon Hub::poll removes a Guest before emitting HostEvent::Left. The current
Left producer normalizes crafting inputs, saves the detached guest and removes
its play state. Neither lan_crafting_left nor playerdata::store includes the
carried cursor. Thus cursor-only disconnect can lose ownership on successful IO.

An initial suggestion to append the cursor to place_back inputs was rejected
before any implementation. The pinned AbstractContainerMenu.removed delegates to
dropOrPlaceInInventory, which drops the entire stack for a disconnected player;
it only places back for the applicable still-connected case. Inventory room and
creative mode do not select inventory return on disconnect. Connected ContainerClose
handling must remain separate.

ServerGamePacketListenerImpl.removePlayerFromWorld calls ServerPlayer.disconnect
before PlayerList.remove. disconnect sets the flag returned by hasDisconnected.
PlayerList.remove saves before removePlayerImmediately; Player.remove then invokes
inventoryMenu.removed and closes another open menu. The intervening ServerLevel
removePlayerImmediately delegation has not yet been inspected independently.
Horizon conn.rs emits Left after its read loop ends and Out::Close is queued.

A possible later bounded packet is cursor-only native drop after the existing
player save, in the guest's original dimension, restoring the prior active level.
It needs exact existing throw_from_eyes motion/RNG reuse with detached guest pose,
checked prepublication work and refusal retaining the original cursor. Merely
calling drop_guest_items omits spread RNG and is not full native throw parity.
Crafting/workstation/merchant behavior is a separate review scope.

Required tests would exercise the real NetEvent::Left/Hub::poll/Game handler,
room/partial/full and creative inventories, rich metadata, parked source dimension,
empty cursor and connected-close regression. Do not expect cursor in playerdata;
verify inventory plus native dropped-entity ownership. A helper preserving cursor
on refusal does not preserve the detached Guest across enclosing handler teardown.
Failed save/source-entry retention, reconnect ordering, active guest capture and
coherent checkpoints remain open. No broad retirement queue or recovery service
is authorized by this note.

Raw javap outputs stay outside Git at workspace-root .local/h08a14-*-javap.log.
The attempted REA query failed because a Windows executable received a WSL path;
plain Linux Java 25 javap supplied the primary evidence. No game/account data,
bytecode or decompiled source is copied into this private authored audit.
