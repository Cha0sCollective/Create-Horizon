fn create_burner_unload(game: &crate::game::Game<'_>) {
    crate::game::create_port_burner_sync::clear(game);
    crate::game::create_port_burner_presentation::reset();
    crate::game::create_port_burner_clock::reset();
}
