// Development-only observation of a real peer's received owner view.
fn create_assembly_owner_probe_frame(game: &crate::game::Game<'_>) {
    crate::game::create_assembly_owner::probe_client_frame(game);
}
