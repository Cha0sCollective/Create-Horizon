fn quill_hud(game: &mut crate::game::Game<'_>, batch: &mut horizon_client::hud::HudBatch) {
    crate::game::create_port_quill::hud(game, batch);
}
