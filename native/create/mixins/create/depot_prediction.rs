// Predict the selected Depot/belt clicks and let a held FunnelItem bypass a
// chest's normal client use. Inventories and placement remain server-owned.
fn create_depot_prediction(
    game: &crate::game::Game<'_>,
    hit: &crate::game::Hit,
    held: Option<&crate::items::ItemStack>,
) -> std::ops::ControlFlow<(InteractionResult, DryUse)> {
    if game.native_on("create")
        && game.native_block(hit.pos) == "minecraft:chest"
        && held.is_some_and(|stack| stack.item.as_ref() == "create:andesite_funnel")
    {
        // The server lets BlockItem place this item before chest use.
        return std::ops::ControlFlow::Break((
            InteractionResult::Pass,
            DryUse::default(),
        ));
    }
    if game.native_on("create")
        && held.is_some_and(|s| {
            matches!(
                s.item.as_ref(),
                "create:vertical_gearbox" | "create:large_water_wheel"
            )
        })
        || crate::game::create_port::depot_reacts(game, hit.pos, hit.face)
        || game.native_on("create")
            && (game.native_block(hit.pos) == "create:belt"
                || held.is_some_and(|s| s.item.as_ref() == "create:belt_connector"))
    {
        std::ops::ControlFlow::Break((InteractionResult::Success, DryUse::default()))
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
