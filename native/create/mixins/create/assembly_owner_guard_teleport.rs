// Prevent a command from moving an active assembly's display carrier alone.
fn create_assembly_owner_guard_teleport(
    game: &crate::game::Game<'_>,
    target: Target,
) -> std::ops::ControlFlow<()> {
    let Target::Other(id) = target else {
        return std::ops::ControlFlow::Continue(());
    };
    if crate::game::create_assembly_owner::guard_active_carrier(game, id) {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
