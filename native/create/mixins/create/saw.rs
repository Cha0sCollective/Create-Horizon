// Copyright (c) The Create Team / The Creators of Create
// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
// Selected upward-facing SawBlockEntity item path. Horizontal tree cutting,
// entity damage, belt/funnel exchange, filters and bulk recipes are absent.
use super::{Game, native_block_entity_draw::BlockEntityDraw};
use crate::{
    containers::{ModBlockEntity, ModContainer},
    items::ItemStack,
};
use create_engine::{
    kinetics::Axis,
    saw::{self, Action, SawCycle},
};
use horizon_assets::item_model::DisplayContext;
use horizon_world::nbt::{Compound, Tag};

const ID: &str = "create:mechanical_saw";
const BRIDGE: &str = "create_horizon:saw_inventory";

#[derive(Clone)]
struct Recipe {
    duration: i32,
    output: ItemStack,
}

/// Admit only the one source-backed recipe selected for this packet. Other
/// datapack cutting/stonecutting/assembly recipes remain unqualified.
fn recipe(game: &Game<'_>, input: &ItemStack) -> Option<Recipe> {
    if input.item.as_ref() != "create:andesite_alloy" || input.count != 1 {
        return None;
    }
    game.native_recipes_of_type("create:cutting")
        .into_iter()
        .find_map(|entry| {
            let json = &entry.json;
            let ingredients = json["ingredients"].as_array()?;
            if ingredients.len() != 1 || ingredients[0]["item"] != "create:andesite_alloy" {
                return None;
            }
            let results = json["results"].as_array()?;
            if results.len() != 1
                || results[0]["id"] != "create:shaft"
                || results[0]["count"] != 6
                || results[0]["chance"].as_f64().is_some_and(|v| v != 1.0)
            {
                return None;
            }
            let output = game.native_recipe_result(&results[0])?;
            if output.item.as_ref() != "create:shaft" || output.count != 6 {
                return None;
            }
            let duration = json["processing_time"].as_i64()?;
            (duration == 200).then_some(Recipe {
                duration: duration as i32,
                output,
            })
        })
}

pub(super) fn container(block: &str, container: &mut Option<ModContainer>) {
    if block == ID && container.is_none() {
        *container = Some(ModContainer {
            slots: saw::SLOTS,
            // The selected ordinary falling-item intake is handled below.
            // Do not accidentally expose unreviewed hopper/belt extraction.
            faces: std::array::from_fn(|_| Vec::new()),
            inputs: Vec::new(),
            outputs: Vec::new(),
            menu: None,
        });
    }
}

fn float(tag: &Tag, key: &str) -> Option<f32> {
    match tag.get(key) {
        Some(Tag::Float(bits)) => Some(f32::from_bits(*bits)),
        _ => None,
    }
}

fn cycle(entity: &ModBlockEntity) -> SawCycle {
    let Some(inventory) = entity.nbt.get(&"Inventory".into()) else {
        return SawCycle::default();
    };
    SawCycle {
        remaining_time: float(inventory, "ProcessingTime").unwrap_or(-1.0),
        recipe_duration: float(inventory, "RecipeTime").unwrap_or(0.0),
        applied_recipe: matches!(inventory.get("AppliedRecipe"), Some(Tag::Byte(1))),
    }
}

fn slots_nbt(items: &[Option<ItemStack>], cycle: SawCycle) -> Tag {
    let contents = items
        .iter()
        .enumerate()
        .filter_map(|(slot, item)| {
            let Tag::Compound(mut tag) = horizon_items::item_nbt::item_to_nbt(item.as_ref()?)
            else {
                return None;
            };
            tag.insert("Slot".into(), Tag::Int(slot as i32));
            Some(Tag::Compound(tag))
        })
        .collect();
    Tag::Compound(Compound::from([
        ("Size".into(), Tag::Int(saw::SLOTS as i32)),
        ("Items".into(), Tag::List(contents)),
        (
            "ProcessingTime".into(),
            Tag::Float(cycle.remaining_time.to_bits()),
        ),
        (
            "RecipeTime".into(),
            Tag::Float(cycle.recipe_duration.to_bits()),
        ),
        (
            "AppliedRecipe".into(),
            Tag::Byte(i8::from(cycle.applied_recipe)),
        ),
    ]))
}

fn restore(entity: &mut ModBlockEntity) {
    entity.items.resize(saw::SLOTS, None);
    let Some(current) = entity.nbt.get(&"Inventory".into()).cloned() else {
        return;
    };
    let previous = entity
        .nbt
        .get(&BRIDGE.into())
        .and_then(|tag| tag.get("Inventory"));
    if previous == Some(&current)
        || (previous.is_none() && entity.items.iter().any(Option::is_some))
    {
        return;
    }
    let Some(Tag::List(items)) = current.get("Items") else {
        return;
    };
    entity.items.fill(None);
    for entry in items {
        let slot = match entry.get("Slot") {
            Some(Tag::Int(value)) => usize::try_from(*value).ok(),
            Some(Tag::Byte(value)) => usize::try_from(*value).ok(),
            _ => None,
        };
        if let Some(destination) = slot.and_then(|slot| entity.items.get_mut(slot)) {
            *destination = horizon_items::item_nbt::item_from_nbt(entry);
        }
    }
}

fn snapshot(entity: &mut ModBlockEntity, cycle: SawCycle) -> bool {
    let value = slots_nbt(&entity.items, cycle);
    let changed_items = entity
        .nbt
        .get(&"Inventory".into())
        .and_then(|old| old.get("Items"))
        != value.get("Items");
    entity.nbt.insert("Inventory".into(), value.clone());
    // The one selected recipe has no alternative to cycle through.
    entity.nbt.insert("RecipeIndex".into(), Tag::Int(0));
    entity.nbt.insert(
        BRIDGE.into(),
        Tag::Compound(Compound::from([("Inventory".into(), value)])),
    );
    changed_items
}

/// The existing item-entity service replaces only the accepted one-item part
/// after a confirmed top contact. SawBlock's source callback also requires
/// nonzero speed and an empty processing inventory.
fn catch(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) -> bool {
    if entity.items.iter().any(Option::is_some) {
        return false;
    }
    const EPSILON: f64 = 1.0e-7;
    let dim = game.dim.id().to_owned();
    let min = pos.map(f64::from);
    let mut max = min.map(|v| v + 1.0);
    max[1] += EPSILON;
    let collision = game.native_block_collision(pos);
    for dropped in game.native_item_entities(&dim, min, max) {
        let touching = game
            .dropped
            .iter()
            .find(|item| item.id == dropped.id)
            .is_some_and(|item| {
                let (lo, hi) = (item.bounds().min_position(), item.bounds().max_position());
                item.on_ground
                    && collision.iter().any(|shape| {
                        (item.position.y - (min[1] + shape[4])).abs() <= EPSILON
                            && lo.x < min[0] + shape[3]
                            && hi.x > min[0] + shape[0]
                            && lo.z < min[2] + shape[5]
                            && hi.z > min[2] + shape[2]
                    })
            });
        if !touching
            || dropped.position[0].floor() != f64::from(pos[0])
            || dropped.position[2].floor() != f64::from(pos[2])
        {
            continue;
        }
        let Some((accepted_count, remainder_count)) = saw::take_one(dropped.stack.count) else {
            continue;
        };
        let mut accepted = dropped.stack.clone();
        accepted.count = accepted_count;
        let mut remainder = dropped.stack;
        remainder.count = remainder_count;
        if game.native_set_item_entity(&dim, dropped.id, remainder) {
            entity.items[0] = Some(accepted);
            return true;
        }
    }
    false
}

fn movement(game: &Game<'_>, pos: [i32; 3], rpm: f32) -> [f64; 3] {
    let along_first = game
        .native_block_property(pos, "axis_along_first")
        .as_deref()
        == Some("true");
    let sign = if rpm < 0.0 { -1.0 } else { 1.0 };
    if along_first {
        [0.0, 0.0, -sign]
    } else {
        [sign, 0.0, 0.0]
    }
}

pub(super) fn tick(
    game: &mut Game<'_>,
    pos: [i32; 3],
    entity: &mut ModBlockEntity,
    rpm: f32,
) -> bool {
    if entity.id != ID {
        return false;
    }
    restore(entity);
    let mut cycle = cycle(entity);
    let up = game.native_block_property(pos, "facing").as_deref() == Some("up");
    let caught = up && rpm != 0.0 && catch(game, pos, entity);
    let selected = entity.items[0]
        .as_ref()
        .and_then(|input| recipe(game, input));
    let occupied = entity.items.iter().any(Option::is_some);
    let action = if up {
        cycle.tick(rpm, occupied, selected.as_ref().map(|r| r.duration))
    } else {
        Action::None
    };
    match action {
        Action::Apply { supported: true } => {
            if let Some(selected) = selected {
                entity.items.fill(None);
                entity.items[1] = Some(selected.output);
            }
        }
        Action::Eject => {
            let direction = movement(game, pos, rpm);
            let center = pos.map(|v| f64::from(v) + 0.5);
            for item in entity.items.iter_mut().filter_map(Option::take) {
                let point = std::array::from_fn(|i| {
                    center[i] + direction[i] * 0.5 + if i == 1 { 0.5 } else { 0.0 }
                });
                let velocity = std::array::from_fn(|i| {
                    direction[i] * 0.0625 + if i == 1 { 0.125 } else { 0.0 }
                });
                game.spawn_item(item, point, velocity, 10);
            }
        }
        _ => {}
    }
    let changed_items = snapshot(entity, cycle);
    caught
        || changed_items
        || matches!(action, Action::Armed | Action::Apply { .. } | Action::Eject)
}

pub(super) fn reacts(game: &Game<'_>, pos: [i32; 3]) -> bool {
    game.native_on("create")
        && game.native_block(pos) == ID
        && game.native_block_property(pos, "facing").as_deref() == Some("up")
        && game
            .held()
            .is_none_or(|item| item.count == 0 || item.item.as_ref() == "minecraft:air")
}

pub(super) fn right_click(game: &mut Game<'_>, pos: [i32; 3]) -> bool {
    if !reacts(game, pos) {
        return false;
    }
    let Some(items) = game.native_with_mod_block_entity(pos, |entity| {
        restore(entity);
        let items = entity
            .items
            .iter_mut()
            .filter_map(Option::take)
            .collect::<Vec<_>>();
        snapshot(entity, SawCycle::default());
        items
    }) else {
        return false;
    };
    let actor = game.native_acting_id();
    for item in items {
        if let Some(rest) = game.native_give_stack(actor, item)
            && let Some(player) = game.native_player()
        {
            game.spawn_item(
                rest,
                [player.x, player.y + 1.0, player.z],
                [0.0, 0.2, 0.0],
                10,
            );
        }
    }
    game.native_block_entity_sync(pos);
    true
}

pub(super) fn draw(
    game: &Game<'_>,
    pos: [i32; 3],
    entity: &ModBlockEntity,
    draw: &mut BlockEntityDraw,
) -> bool {
    if entity.id != ID {
        return false;
    }
    if game.native_block_property(pos, "facing").as_deref() != Some("up") {
        return true;
    }
    let rpm = crate::game::create_port::number(&entity.nbt, "Speed");
    let axis = if game
        .native_block_property(pos, "axis_along_first")
        .as_deref()
        == Some("true")
    {
        Axis::X
    } else {
        Axis::Z
    };
    draw.push();
    if axis == Axis::X {
        draw.rotate_about([0.0, 1.0, 0.0], 90.0, [0.5; 3]);
    }
    let blade = if rpm > 0.0 {
        "create:block/mechanical_saw/blade_vertical_active"
    } else if rpm < 0.0 {
        "create:block/mechanical_saw/blade_vertical_reversed"
    } else {
        "create:block/mechanical_saw/blade_vertical_inactive"
    };
    game.native_draw_model(draw, blade, 0xFFFFFF);
    draw.pop();
    let time = draw.game_time as f64 + f64::from(draw.partial);
    let angle = create_engine::visual::angle(
        time,
        rpm,
        create_engine::visual::rotation_offset(pos, axis, false),
    );
    draw.push();
    draw.rotate_about(create_engine::visual::positive_axis(axis), angle, [0.5; 3]);
    if axis == Axis::X {
        draw.rotate_about([0.0, 1.0, 0.0], 90.0, [0.5; 3]);
    }
    game.native_draw_model(draw, "create:block/shaft", 0xFFFFFF);
    draw.pop();
    let stack = entity
        .nbt
        .get(&"Inventory".into())
        .and_then(|inventory| inventory.get("Items"))
        .and_then(|items| match items {
            Tag::List(items) => items.first(),
            _ => None,
        })
        .and_then(horizon_items::item_nbt::item_from_nbt);
    if let Some(stack) = stack {
        let (block_light, sky_light) = game.native_light([pos[0], pos[1] + 1, pos[2]]);
        draw.light =
            horizon_client::block_entity_render::vertex_light((sky_light << 4) | block_light);
        draw.push();
        draw.translate([0.5, 13.0 / 16.0, 0.5]);
        draw.scale([0.5; 3]);
        game.native_draw_item(draw, &stack, DisplayContext::Fixed, None, 0);
        draw.pop();
    }
    true
}
