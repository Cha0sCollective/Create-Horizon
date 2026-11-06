fn create_burner_received(game: &mut crate::game::Game<'_>, channel: &str, data: &[u8]) {
    crate::game::create_port_burner_sync::from_server(game, channel, data);
}
