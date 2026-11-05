// The dry Basin cannot expose an inventory whose canonical schema is opaque.
// No mutable eligibility cache or alias is used; unrelated lookups continue.
fn create_basin_view(
    containers: &crate::game::machines::Containers<'_>,
    pos: (i32, i32, i32),
) -> std::ops::ControlFlow<Option<View>> {
    if let Some(crate::containers::BlockEntity::Mod(entity)) =
        containers.world.block_entities.get(&pos)
        && entity.id == "create:basin"
        && !horizon_client::save::create_basin_inventory_codec::supported(entity)
    {
        return std::ops::ControlFlow::Break(None);
    }
    std::ops::ControlFlow::Continue(())
}
