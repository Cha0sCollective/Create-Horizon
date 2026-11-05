fn create_s5_after_sync(result: (), game: &mut crate::game::Game<'_>) {
    crate::game::create_s5_carrier::after_sync(game);
    result
}
