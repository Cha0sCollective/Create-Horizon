fn create_right_click(
    game: &mut crate::game::Game<'_>,
    pos: [i32; 3],
    face: [i32; 3],
    canceled: &mut bool,
) {
    if crate::game::create_s5_carrier::guard_proxy_use(game, pos)
        || crate::game::create_s5_piston::guard_proxy_use(game, pos)
    {
        *canceled = true;
        return;
    }
    crate::game::create_port::right_click(game, pos, face, canceled);
}
