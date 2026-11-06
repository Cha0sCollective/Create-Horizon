fn quill_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    crate::game::create_port_quill::key(game, key, pressed);
}
