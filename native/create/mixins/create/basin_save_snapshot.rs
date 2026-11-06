fn create_basin_save_snapshot(
    pos: (i32, i32, i32),
    entity: &crate::containers::BlockEntity,
) -> std::ops::ControlFlow<Tag> {
    use horizon_world::nbt::Tag;
    if let crate::containers::BlockEntity::Mod(entity) = entity
        && create_basin_inventory_codec::supported(entity)
    {
        // Exact pinned Mod branch, with only fresh Basin canonical mirrors.
        // HEAD preserves the host's nested imports and unrelated encoder body.
        let mut tag = entity.nbt.clone();
        create_basin_inventory_codec::snapshot(entity, &mut tag);
        tag.insert("id".into(), Tag::String(entity.id.as_str().into()));
        if entity.items.iter().any(Option::is_some) {
            tag.insert("Items".into(), crate::save::encode_items(&entity.items));
        }
        for (key, value) in [("x", pos.0), ("y", pos.1), ("z", pos.2)] {
            tag.insert(key.into(), Tag::Int(value));
        }
        return std::ops::ControlFlow::Break(Tag::Compound(tag));
    }
    std::ops::ControlFlow::Continue(())
}
