fn crushing_item_tick_pre(
    game: &mut crate::game::Game<'_>,
    dimension: &str,
    entity: u32,
    kind: &str,
    canceled: &mut bool,
) {
    crate::game::create_port_crushing::item_tick_pre(game, dimension, entity, kind, canceled);
}
