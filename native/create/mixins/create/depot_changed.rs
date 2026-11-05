// A successful automation transfer ending the exposed stack destroys its
// transported identity immediately, even if another hopper refills this tick.
fn create_depot_changed(game: &mut crate::game::Game<'_>, pos: (i32, i32, i32)) {
    crate::game::create_s5_piston::portable_changed(game, pos);
    crate::game::create_port_basin::notify_contents(game, [pos.0, pos.1, pos.2]);
    if let Some(crate::containers::BlockEntity::Mod(entity)) =
        game.world.block_entities.get_mut(&pos)
        && entity.id == "create:depot"
        && entity.items.first().is_none_or(Option::is_none)
    {
        entity.nbt.remove(&"HeldItem".into());
    }
}
