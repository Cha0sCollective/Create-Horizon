fn create_destroy(game: &mut crate::game::Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    if crate::game::create_port::inventory_blocked(game, pos) {
        *canceled = true;
        return;
    }
    crate::game::create_s5_carrier::guard_destroy(game, pos, canceled);
    crate::game::create_s5_piston::guard_destroy(game, pos, canceled);
    crate::game::create_port_fluid_tank::guard_destroy(game, pos, canceled);
    if !*canceled {
        crate::game::create_port_placement::destroy(game, pos, canceled);
        if !*canceled && game.native_block(pos) == "create:basin" {
            crate::game::create_port_basin::removed(game, pos);
        }
    }
}
