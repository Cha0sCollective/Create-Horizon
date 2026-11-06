fn create_track_survival(
    game: &crate::game::GameLevel<'_>,
    pos: horizon_core::position::BlockPos,
    state: horizon_world::registry::BlockStateId,
    survives: &mut Option<bool>,
) {
    crate::game::create_port_track::can_survive(game, pos, state, survives);
}
