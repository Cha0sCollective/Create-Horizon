fn create_basin_inventory_received(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    kind: i32,
    tag: &horizon_world::nbt::Tag,
) {
    crate::game::create_port_basin_inventory_packet::packet(game, pos, kind, tag);
}
