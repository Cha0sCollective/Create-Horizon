// Game-test area cleanup also removes unsimulated displays directly.
fn create_assembly_owner_guard_test_discard(
    game: &crate::game::Game<'_>,
    bounds: Aabb,
) -> std::ops::ControlFlow<()> {
    let would_delete_owner = game.others.iter().any(|entity| {
        let p = [entity.position.x, entity.position.y, entity.position.z];
        (0..3).all(|i| p[i] >= bounds.0[i] && p[i] < bounds.1[i])
            && crate::game::create_assembly_owner::guard_active_carrier(game, entity.id)
    });
    if would_delete_owner {
        std::ops::ControlFlow::Break(())
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
