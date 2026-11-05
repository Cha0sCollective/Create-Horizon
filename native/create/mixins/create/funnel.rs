// Copyright (c) The Create Team / The Creators of Create
// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
// Selected standing downward Andesite Funnel; Belt Funnel remains separate.
use super::Game;
use crate::containers::ModBlockEntity;
use horizon_core::geometry::Direction;
use horizon_world::nbt::Tag;

pub(super) const ID: &str = "create:andesite_funnel";
const COOLDOWN: i32 = 8;

fn cooldown(entity: &ModBlockEntity) -> i32 {
    super::create_port::integer(&entity.nbt, "TransferCooldown").max(0)
}

/// Tick only on the authoritative world, before the generic kinetic path.
pub(super) fn tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    if entity.id != ID
        || !game.native_on("create")
        || (game.serving.is_none() && game.options.dedicated.is_none())
    {
        return;
    }
    let dim = game.dim.id().to_owned();
    let powered = game.native_neighbor_signal(&dim, pos) > 0;
    game.native_set_block_property(pos, "powered", if powered { "true" } else { "false" });

    if powered {
        if cooldown(entity) != 0 {
            entity.nbt.insert("TransferCooldown".into(), Tag::Int(0));
            game.native_block_entity_sync(pos);
        }
        return;
    }
    let remaining = cooldown(entity);
    if remaining > 0 {
        entity
            .nbt
            .insert("TransferCooldown".into(), Tag::Int(remaining - 1));
        return;
    }
    if game.native_block_property(pos, "facing").as_deref() != Some("down")
        || game.native_block_property(pos, "extracting").as_deref() != Some("true")
    {
        return;
    }

    let chest = [pos[0], pos[1] + 1, pos[2]];
    if game.native_block(chest) != "minecraft:chest" {
        return;
    }
    let Some(slot) = game
        .native_container_slots(&dim, chest, Direction::Down)
        .and_then(|slots| {
            slots.into_iter().find(|slot| {
                slot.can_take && slot.stack.as_ref().is_some_and(|stack| stack.count > 0)
            })
        })
    else {
        return;
    };

    // Java scans the funnel block and the block below before committing the
    // extraction. The belt's existing world-item capture owns the next step.
    let min = [pos[0] as f64, (pos[1] - 1) as f64, pos[2] as f64];
    let max = [(pos[0] + 1) as f64, (pos[1] + 1) as f64, (pos[2] + 1) as f64];
    if !game.native_item_entities(&dim, min, max).is_empty() {
        return;
    }
    let Some(stack) = game.native_extract_item(&dim, chest, Direction::Down, slot.slot, 1) else {
        return;
    };
    debug_assert_eq!(stack.count, 1);
    // This host spawn has no acceptance result. The selected one-mod run
    // proves observed custody; arbitrary native spawn vetoes remain open.
    game.spawn_item(
        stack,
        [pos[0] as f64 + 0.5, pos[1] as f64, pos[2] as f64 + 0.5],
        [0.0, 0.0, 0.0],
        10,
    );
    entity
        .nbt
        .insert("TransferCooldown".into(), Tag::Int(COOLDOWN));
    game.native_block_entity_sync(pos);
}
