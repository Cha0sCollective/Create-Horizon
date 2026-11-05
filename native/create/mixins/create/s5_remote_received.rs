fn create_s5_remote_received(game: &mut crate::game::Game<'_>) {
    // poll_remote processes chunk/update tags before this client-level tick.
    // Reconstruct before its input handling or player/entity collision work.
    crate::game::create_s5_carrier::refresh(game);
}
