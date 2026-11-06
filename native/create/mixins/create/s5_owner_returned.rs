fn create_s5_owner_returned(result: (), game: &mut crate::game::Game<'_>, pos: (i32, i32, i32)) {
    crate::game::create_s5_carrier::owner_returned(game, pos);
    crate::game::create_s5_piston::owner_returned(game, pos);
    result
}
