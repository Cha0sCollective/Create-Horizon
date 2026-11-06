// An existing native tracking callback queues only an already validated VIEW.
fn create_assembly_owner_tracking(
    game: &mut crate::game::Game<'_>,
    entity: u32,
    uuid: u128,
    kind: &str,
) {
    crate::game::create_assembly_owner::player_start_tracking(game, entity, uuid, kind);
}
