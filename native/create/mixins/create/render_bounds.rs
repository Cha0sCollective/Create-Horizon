fn create_render_bounds(
    game: &crate::game::Game<'_>,
    pos: [i32; 3],
    entity: &crate::containers::ModBlockEntity,
    block: &str,
    bounds: &mut [f32; 6],
) {
    crate::game::create_port_belt::render_bounds(game, pos, entity, bounds);
    if block == "create:mechanical_mixer" {
        *bounds = [0.0, -1.5, 0.0, 1.0, 1.0, 1.0];
    }
    if block == "create:large_water_wheel" {
        *bounds = [-2.0, -2.0, -2.0, 3.0, 3.0, 3.0];
    }
}
