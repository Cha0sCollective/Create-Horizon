// Prevent the native entity API from moving an active assembly's carrier alone.
fn create_assembly_owner_guard_move(
    game: &crate::game::Game<'_>,
    id: u32,
) -> std::ops::ControlFlow<bool> {
    if crate::game::create_assembly_owner::guard_active_carrier(game, id) {
        std::ops::ControlFlow::Break(false)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
