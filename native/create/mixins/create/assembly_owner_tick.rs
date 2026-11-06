// Discovery only: no entity spawn, movement, block capture, or world write.
fn create_assembly_owner_tick(game: &mut crate::game::Game<'_>, dimension: &str) {
    crate::game::create_assembly_owner::level_tick_post(game, dimension);
}
