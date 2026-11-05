# S6 portable item interface — finite route audit

This read-only follow-up starts from the accepted mounted-depot checkpoint
`a77af67`. S6 remains OPEN. It proposes a finite next profile for the already
authorized mounted/portable family, without a general contraption manager.

## Current implementation and upstream reuse

The November5,2026 upstream check found the experimental port's main and
stage/parity-06 at
[`236c87a8`](https://github.com/Delete228/create-horizon/tree/236c87a889cd76f7e663d1d182c28dc1145f7b14).
Its [README](https://github.com/Delete228/create-horizon/blob/236c87a889cd76f7e663d1d182c28dc1145f7b14/README.md)
calls the preview0.7.0-alpha.2 and still lists moving contraptions as planned.
Its [current roadmap](https://github.com/Delete228/create-horizon/blob/236c87a889cd76f7e663d1d182c28dc1145f7b14/docs/roadmap.md)
leaves moving inventories and portable interfaces pending. The complete992-entry
source tree has no portable-interface/mounted-storage implementation. This
does not qualify or import the newer processing features wholesale.

Official Horizon main was checked at
[`696d0216`](https://github.com/horizon-rust/horizon/commit/696d021699104eee44386e4d39a1a45dff54ea7e).
Its [native-mod inventory documentation](https://github.com/horizon-rust/horizon/blob/696d021699104eee44386e4d39a1a45dff54ea7e/docs/native-mods.md#a-blocks-items)
continues to describe world-block container slots and item moves. No ready
contraption inventory capability follows from that API. Keep the tested
fc021038 pin: no engine upgrade or new public API is justified for this step.

The pinned host already supplies canonical mod item slots, ordinary hopper
extraction/insertion/cooldowns, chest storage, saves, block-entity sync,
redstone queries, block models and private Mixins targeting. Its
`game/machines.rs` `View` maps logical slots to block-entity positions through
`parts`; `get`, `slot_mut_at` and `view_changed` use that same map. The exact
`Game::suck_from_container` hopper entry has mutable Game before its container
lookup and delegates to `suck_from_slots`. One private adapter at that entry
can validate live power/geometry, construct a stack-local logical view of
the piston slot, then call the existing transfer function. This avoids both
an item copy and a global inventory alias. The station exposes no other
container callers. This source-supported route remains subject to
packet/source/game review; no public engine patch or duplicate hopper is needed.

An initial global `Containers::view` proposal was held in packet review:
that method is immutable, while `native_neighbor_signal` needs mutable Game.
A cached eligibility bit would need immediate invalidation and cold-load
rules. The selected hopper entry resolves the actual selected source query
with live mutable preflight and no eligibility cache or saved ready marker.

The accepted depot-only piston stores its one item in namespaced owner NBT.
That field cannot directly supply the host's borrowed `&ItemStack`/mutable
slot interfaces. Preserve that accepted representation for the existing
profile. The new two-block profile can instead use one canonical item slot
on the same piston block entity as its sole mounted-item owner. Its new
namespaced state records geometry/connection metadata, never an additional
saved item copy. Direct automation at the piston remains closed. This is a
new finite private profile, not a generic inventory-provider API or migration
of accepted saves.

## Reference behavior that must be retained

The Java reference checkout is unchanged at6.0.11 `7ff2760e`.
`PortableItemInterfaceBlockEntity` wraps the contraption's existing mounted
item handler; it does not own a second inventory. Its extraction/insertion
gate uses `canTransfer`, and successful transfer resets the timeout.
`PortableStorageInterfaceMovement` looks along the moving interface's facing
with active-area offset1.85, requires an opposing unpowered matching interface,
stalls at the selected encounter and maintains keep-alive2. The stationary
class defines animation4, initial connecting timer14, default timeout60 and
on-transfer timer64. Powered/disappeared interfaces must close the access path.
Source event order and units must be made explicit in the packet/tests; no
complete Java actor or tick-equivalence claim follows from these constants.

The smallest useful next assembly is a linear push chain: piston, depot,
portable storage interface, all on X. It avoids adding glue/chassis or an
arbitrary side attachment. One existing pole limits travel to one cell.
One opposing stationary interface is above a vanilla hopper/chest. Only
extraction of the depot's initial one plain ingot is qualified. After that
extraction the moving depot must be able to restore empty. Connection,
collision and safe restoration must cover both moving cargo blocks.

## Design boundary

Use the existing private piston/proxy/clearance route for this explicit
two-cargo chain, one canonical owner slot and one validated hopper-source view.
Keep the older stone and single-depot profiles readable and functional.
Prove that every alias operation reads/changes the same sole slot, that
disconnection immediately closes access and that retry cannot overwrite or
duplicate an item. Do not introduce a temporary stationary item buffer.

If source review or an actual focused trial disproves the alias/canonical-slot
route, stop before a general capability service, multi-world transaction
framework or shared contraption architecture. Request owner input for that
broader change. This audit alone authorizes no implementation or acceptance.
