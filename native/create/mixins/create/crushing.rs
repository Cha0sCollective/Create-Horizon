// Copyright (c) The Create Team / The Creators of Create
// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
// CrushingWheelBlock/ControllerBlockEntity: selected stationary X-axis pair,
// one cobblestone item, milling fallback, and downward world output only.
use super::{Game, native_block_entity_draw::BlockEntityDraw};
use crate::{
    containers::{BlockEntity, ModBlockEntity, ModContainer},
    items::ItemStack,
};
use create_engine::{
    crushing::{self, Action, Cycle},
    kinetics::{Axis, Pos},
};
use horizon_assets::item_model::DisplayContext;
use horizon_world::nbt::{Compound, Tag};
use std::{cell::RefCell, collections::{BTreeSet, VecDeque}};

const WHEEL: &str = "create:crushing_wheel";
const CONTROLLER: &str = "create:crushing_wheel_controller";
const BRIDGE: &str = "create_horizon:crushing_inventory";
const INPUT: &str = "minecraft:cobblestone";
const OUTPUT: &str = "minecraft:gravel";
const PENDING_LIMIT: usize = 4096;

thread_local! {
    static CHANGED: RefCell<VecDeque<(&'static str, Pos)>> = const { RefCell::new(VecDeque::new()) };
    static CONTACT: RefCell<Option<Contact>> = const { RefCell::new(None) };
}

#[derive(Clone)]
struct Contact {
    owner: usize,
    dimension: String,
    middle: Pos,
    item: u32,
    tick: i64,
    pickup_delay: u32,
}

fn at(pos: Pos, dz: i32) -> Pos { [pos[0], pos[1], pos[2] + dz] }

/// Block changes can remove both wheels before either wheel's next entity tick.
/// Reconcile from the pre-entity-tick level hook while the controller's canonical
/// ModBlockEntity is still attached for the host container spill path.
pub(super) fn changed(dimension: &'static str, pos: Pos) {
    CHANGED.with(|cell| {
        let mut queue = cell.borrow_mut();
        if queue.len() == PENDING_LIMIT { queue.pop_front(); }
        queue.push_back((dimension, pos));
    });
}

pub(super) fn level_tick(game: &mut Game<'_>) {
    if !game.native_on("create") || (game.serving.is_none() && game.options.dedicated.is_none()) {
        return;
    }
    let dimension = game.dim.id().to_owned();
    let mut candidates = BTreeSet::new();
    CHANGED.with(|cell| {
        cell.borrow_mut().retain(|(dim, pos)| {
            if *dim != dimension { return true; }
            candidates.insert(*pos);
            candidates.insert(at(*pos, -1));
            candidates.insert(at(*pos, 1));
            false
        });
    });
    // A bounded client-side queue can omit notifications. Loaded wheels also
    // discover a newly formed pair, while controllers are checked for orphans.
    for &(x, y, z) in game.world.block_entities.keys() {
        let pos = [x, y, z];
        match game.native_block(pos).as_str() {
            WHEEL => { candidates.insert(at(pos, -1)); candidates.insert(at(pos, 1)); }
            CONTROLLER => { candidates.insert(pos); }
            _ => {}
        }
    }
    for middle in candidates { reconcile(game, middle, None); }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pair { first_rpm: f32, second_rpm: f32 }

impl Pair {
    fn valid(self) -> bool {
        crushing::opposing(self.first_rpm, self.second_rpm)
            && self.first_rpm > 0.0 && self.second_rpm < 0.0
    }
}

fn pair(game: &Game<'_>, middle: Pos, current: Option<(Pos, f32)>) -> Option<Pair> {
    let first = at(middle, -1);
    let second = at(middle, 1);
    if ![first, second].iter().all(|&pos| {
        game.native_block(pos) == WHEEL
            && game.native_block_property(pos, "axis").as_deref() == Some("x")
    }) { return None; }
    let speed = |pos| {
        current.filter(|(p, _)| *p == pos).map(|(_, rpm)| rpm).unwrap_or_else(|| {
            let Some(start) = super::create_port::node(game, pos) else { return 0.0; };
            super::create_port_kinetic_runtime::network(game, start, 0).speed(pos)
        })
    };
    Some(Pair { first_rpm: speed(first), second_rpm: speed(second) })
}

fn reconcile(game: &mut Game<'_>, middle: Pos, current: Option<(Pos, f32)>) {
    let block = game.native_block(middle);
    let actual = pair(game, middle, current);
    match actual {
        Some(pair) if block == CONTROLLER => {
            let valid = if pair.valid() { "true" } else { "false" };
            game.native_set_block_property(middle, "valid", valid);
        }
        Some(pair) if super::create_port_generators::replaceable(game, middle) => {
            let dim = game.dim.id().to_owned();
            if let Some(state) = game.pack.assets.registry.default_state(CONTROLLER)
                && game.native_set_block(&dim, middle, state, 3)
            {
                game.native_set_block_property(middle, "valid", if pair.valid() { "true" } else { "false" });
            }
        }
        None if block == CONTROLLER => {
            // Re-read identity immediately before destruction: a stale queued
            // wheel notification must never destroy a replacement middle block.
            if game.native_block(middle) == CONTROLLER
                && pair(game, middle, current).is_none()
                && let Err(error) = game.destroy_block_dropping(middle)
            {
                log::warn!("Create: crushing controller removal failed at {middle:?}: {error:?}");
            }
        }
        _ => {}
    }
}

/// Called by the wheel's kinetic tick after its speed is resolved. The current
/// wheel entity is detached by that tick, so its effective RPM is explicit.
pub(super) fn wheel_changed(game: &mut Game<'_>, pos: Pos, effective_rpm: f32) {
    if game.native_block(pos) != WHEEL { return; }
    for middle in [at(pos, -1), at(pos, 1)] {
        reconcile(game, middle, Some((pos, effective_rpm)));
    }
}

pub(super) fn container(block: &str, container: &mut Option<ModContainer>) {
    if block == CONTROLLER && container.is_none() {
        *container = Some(ModContainer {
            slots: crushing::SLOTS,
            faces: std::array::from_fn(|_| Vec::new()),
            inputs: Vec::new(),
            outputs: Vec::new(),
            menu: None,
        });
    }
}

fn float(tag: &Tag, key: &str) -> Option<f32> {
    match tag.get(key) { Some(Tag::Float(bits)) => Some(f32::from_bits(*bits)), _ => None }
}

fn cycle(entity: &ModBlockEntity) -> Cycle {
    let Some(inventory) = entity.nbt.get(&"Inventory".into()) else { return Cycle::default(); };
    Cycle {
        remaining_time: float(inventory, "ProcessingTime").unwrap_or(-1.0),
        recipe_duration: float(inventory, "RecipeTime").unwrap_or(0.0),
        applied_recipe: matches!(inventory.get("AppliedRecipe"), Some(Tag::Byte(1))),
    }
}

fn inventory_tag(items: &[Option<ItemStack>], cycle: Cycle) -> Tag {
    let contents = items.iter().enumerate().filter_map(|(slot, item)| {
        let Tag::Compound(mut tag) = horizon_items::item_nbt::item_to_nbt(item.as_ref()?) else { return None; };
        tag.insert("Slot".into(), Tag::Int(slot as i32));
        Some(Tag::Compound(tag))
    }).collect();
    Tag::Compound(Compound::from([
        ("Size".into(), Tag::Int(crushing::SLOTS as i32)),
        ("Items".into(), Tag::List(contents)),
        ("ProcessingTime".into(), Tag::Float(cycle.remaining_time.to_bits())),
        ("RecipeTime".into(), Tag::Float(cycle.recipe_duration.to_bits())),
        ("AppliedRecipe".into(), Tag::Byte(i8::from(cycle.applied_recipe))),
    ]))
}

fn restore(entity: &mut ModBlockEntity) {
    let has_live_slots = entity.items.len() >= crushing::SLOTS;
    entity.items.resize(crushing::SLOTS, None);
    let Some(current) = entity.nbt.get(&"Inventory".into()).cloned() else { return; };
    let previous = entity.nbt.get(&BRIDGE.into()).and_then(|tag| tag.get("Inventory"));
    if (has_live_slots && previous == Some(&current))
        || (previous.is_none() && entity.items.iter().any(Option::is_some)) {
        return;
    }
    let Some(Tag::List(items)) = current.get("Items") else { return; };
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

fn snapshot(entity: &mut ModBlockEntity, cycle: Cycle) -> bool {
    let value = inventory_tag(&entity.items, cycle);
    let changed_items = entity.nbt.get(&"Inventory".into()).and_then(|old| old.get("Items")) != value.get("Items");
    entity.nbt.insert("Inventory".into(), value.clone());
    entity.nbt.insert(BRIDGE.into(), Tag::Compound(Compound::from([("Inventory".into(), value)])));
    changed_items
}

fn recipe(game: &Game<'_>) -> Option<ItemStack> {
    game.native_recipes_of_type("create:milling").into_iter().find_map(|entry| {
        let json = &entry.json;
        let ingredients = json["ingredients"].as_array()?;
        if ingredients.len() != 1 || ingredients[0] != INPUT || json["processing_time"] != 250 { return None; }
        let results = json["results"].as_array()?;
        if results.len() != 1 || results[0]["id"] != OUTPUT
            || results[0]["chance"].as_f64().is_some_and(|chance| chance != 1.0)
        { return None; }
        let output = game.native_recipe_result(&results[0])?;
        (output.item.as_ref() == OUTPUT && output.count == 1).then_some(output)
    })
}

/// API 146 must opt in before the host asks for individual item Pre events.
pub(super) fn item_tick_listened(listened: &mut bool) { *listened = true; }

/// Keep just the selected world-owned item out of ordinary collision physics
/// while it crosses the controller. Custody changes only in `catch` below.
pub(super) fn item_tick_pre(
    game: &mut Game<'_>, dimension: &str, id: u32, kind: &str, canceled: &mut bool,
) {
    if *canceled || kind != "minecraft:item" || dimension != game.dim.id()
        || !game.native_on("create")
        || (game.serving.is_none() && game.options.dedicated.is_none())
    { return; }
    let owner = game as *const Game<'_> as usize;
    let now = game.world.game_time;
    let active = CONTACT.with(|cell| {
        let mut slot = cell.borrow_mut();
        if slot.as_ref().is_some_and(|contact| {
            contact.owner != owner || contact.dimension != dimension
                || now < contact.tick || now > contact.tick + 1
                || !game.dropped.iter().any(|item| item.id == contact.item)
        }) { *slot = None; }
        slot.clone()
    });
    if active.as_ref().is_some_and(|contact| contact.item != id) { return; }
    let Some(item) = game.dropped.iter().find(|item| item.id == id) else { return; };
    let middle = active.as_ref().map_or_else(|| [
        item.position.x.floor() as i32,
        item.position.y.floor() as i32,
        item.position.z.floor() as i32,
    ], |contact| contact.middle);
    let position = item.position;
    let selected = item.stack.item.as_ref() == INPUT && item.stack.count == 1
        && position.x >= f64::from(middle[0]) + 0.25
        && position.x <= f64::from(middle[0]) + 0.75
        && position.z >= f64::from(middle[2]) + 0.25
        && position.z <= f64::from(middle[2]) + 0.75;
    let top = f64::from(middle[1]) + 0.8125;
    let just_landed = item.on_ground && (position.y - top).abs() <= 1.0e-7;
    let pickup_delay = active.as_ref().map_or(item.pickup_delay, |contact| contact.pickup_delay);
    let empty = matches!(game.world.block_entities.get(&(middle[0], middle[1], middle[2])),
        Some(BlockEntity::Mod(entity)) if entity.id == CONTROLLER
            && entity.items.iter().all(Option::is_none)
            && !matches!(entity.nbt.get(&"Inventory".into()).and_then(|v| v.get("Items")),
                Some(Tag::List(items)) if !items.is_empty()));
    let valid = game.native_block(middle) == CONTROLLER
        && game.native_block_property(middle, "facing").as_deref() == Some("down")
        && game.native_block_property(middle, "valid").as_deref() == Some("true")
        && pair(game, middle, None).is_some_and(Pair::valid);
    if !selected || !empty || !valid || recipe(game).is_none()
        || (active.is_none() && !just_landed)
    {
        if active.is_some() {
            if let Some(item) = game.dropped.iter_mut().find(|item| item.id == id) {
                item.position.y = top + 1.0e-4;
                item.previous = item.position;
                item.velocity.y = 0.0;
                item.on_ground = false;
                item.pickup_delay = pickup_delay;
            }
            CONTACT.with(|cell| *cell.borrow_mut() = None);
        }
        return;
    }
    let Some(item) = game.dropped.iter_mut().find(|item| item.id == id) else { return; };
    item.previous = item.position;
    item.position.y = (item.position.y - 0.16).max(f64::from(middle[1]) + 0.20);
    item.velocity.y = 0.0;
    item.on_ground = false;
    item.pickup_delay = u32::MAX;
    *canceled = true;
    CONTACT.with(|cell| *cell.borrow_mut() = Some(Contact {
        owner, dimension: dimension.to_owned(), middle, item: id, tick: now, pickup_delay,
    }));
}

fn catch(game: &mut Game<'_>, pos: Pos, entity: &mut ModBlockEntity) -> bool {
    if entity.items.iter().any(Option::is_some) { return false; }
    let contact = CONTACT.with(|cell| cell.borrow().clone());
    let Some(contact) = contact.filter(|contact| {
        contact.owner == game as *const Game<'_> as usize
            && contact.dimension == game.dim.id() && contact.middle == pos
    }) else { return false; };
    let dim = game.dim.id().to_owned();
    let min = pos.map(f64::from);
    let max = min.map(|v| v + 1.0);
    for dropped in game.native_item_entities(&dim, min, max) {
        let crossing = dropped.id == contact.item
            && game.dropped.iter().find(|item| item.id == dropped.id).is_some_and(|item| {
                item.stack == dropped.stack && !item.on_ground
                    && item.position.y >= min[1] + 0.20
                    && item.position.y < min[1] + 0.25
            });
        if dropped.stack.item.as_ref() != INPUT || dropped.stack.count != 1 || !crossing
            || dropped.position[0] < min[0] + 0.25 || dropped.position[0] > min[0] + 0.75
            || dropped.position[2] < min[2] + 0.25 || dropped.position[2] > min[2] + 0.75
        { continue; }
        let mut rest = dropped.stack.clone();
        rest.count = 0;
        if game.native_set_item_entity(&dim, dropped.id, rest) {
            entity.items[0] = Some(dropped.stack);
            CONTACT.with(|cell| *cell.borrow_mut() = None);
            return true;
        }
    }
    false
}

pub(super) fn tick_controller(game: &mut Game<'_>, pos: Pos, entity: &mut ModBlockEntity) -> bool {
    if entity.id != CONTROLLER { return false; }
    restore(entity);
    let mut clock = cycle(entity);
    let pair = pair(game, pos, None);
    let valid = pair.is_some_and(Pair::valid)
        && game.native_block_property(pos, "valid").as_deref() == Some("true");
    let rpm = if valid { pair.map_or(0.0, |p| p.first_rpm) } else { 0.0 };
    let old_rpm = crate::game::create_port::number(&entity.nbt, "Speed");
    entity.nbt.insert("Speed".into(), Tag::Float(rpm.to_bits()));
    let selected = if valid { recipe(game) } else { None };
    let caught = valid && selected.is_some() && catch(game, pos, entity);
    if caught { clock.insert_one(); }
    let occupied = entity.items.iter().any(Option::is_some);
    let input_ready = entity.items[0].as_ref().is_some_and(|input| input.item.as_ref() == INPUT && input.count == 1)
        && entity.items[1..].iter().all(Option::is_none);
    let output_ready = entity.items[0].is_none()
        && entity.items[1].as_ref().is_some_and(|output| output.item.as_ref() == OUTPUT && output.count == 1)
        && entity.items[2..].iter().all(Option::is_none);
    let can_tick = valid && if clock.applied_recipe { output_ready } else { input_ready && selected.is_some() };
    let action = if caught { Action::None } else { clock.tick(can_tick, rpm, occupied) };
    match action {
        Action::Apply => {
            if input_ready {
                if let Some(output) = selected {
                    entity.items[0] = None;
                    entity.items[1] = Some(output);
                }
            }
        }
        Action::Eject => {
            if let Some(output) = entity.items[1].take() {
                clock.clear();
                game.spawn_item(output, [f64::from(pos[0]) + 0.5, f64::from(pos[1]) - 0.125, f64::from(pos[2]) + 0.5], [0.0, -0.05, 0.0], 10);
            }
        }
        Action::None => {}
    }
    let changed = snapshot(entity, clock);
    caught || changed || old_rpm != rpm || action != Action::None
}

pub(super) fn models(models: &mut Vec<String>) {
    models.push("create:block/crushing_wheel/block".into());
}

pub(super) fn draw(game: &Game<'_>, pos: Pos, entity: &ModBlockEntity, draw: &mut BlockEntityDraw) -> bool {
    if entity.id == WHEEL {
        if game.native_block_property(pos, "axis").as_deref() != Some("x") { return true; }
        let time = draw.game_time as f64 + f64::from(draw.partial);
        let angle = create_engine::visual::angle(time, crate::game::create_port::number(&entity.nbt, "Speed"), create_engine::visual::rotation_offset(pos, Axis::X, false));
        draw.push();
        draw.rotate_about([1.0, 0.0, 0.0], angle, [0.5; 3]);
        // Both the authored fallback and Create's OBJ have a Y-axis hub.
        // Orient that hub along the selected X shaft after applying its spin.
        draw.rotate_about([0.0, 0.0, 1.0], -90.0, [0.5; 3]);
        game.native_draw_model(draw, "create:block/crushing_wheel/block", 0xFFFFFF);
        draw.pop();
        return true;
    }
    if entity.id != CONTROLLER { return false; }
    let stack = entity.items.iter().find_map(Option::as_ref).cloned().or_else(|| {
        entity.nbt.get(&"Inventory".into())?.get("Items").and_then(|items| match items { Tag::List(items) => items.first(), _ => None }).and_then(horizon_items::item_nbt::item_from_nbt)
    });
    if let Some(stack) = stack {
        draw.push();
        draw.translate([0.5; 3]);
        draw.scale([0.45; 3]);
        game.native_draw_item(draw, &stack, DisplayContext::Fixed, None, 0);
        draw.pop();
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_pair_positions_and_counterrotation() {
        let middle = [4, 70, -2];
        assert_eq!(at(middle, -1), [4, 70, -3]);
        assert_eq!(at(middle, 1), [4, 70, -1]);
        assert!(Pair { first_rpm: 16.0, second_rpm: -16.0 }.valid());
        assert!(!Pair { first_rpm: -16.0, second_rpm: 16.0 }.valid());
        assert!(!Pair { first_rpm: 16.0, second_rpm: 16.0 }.valid());
        assert!(!Pair { first_rpm: 0.0, second_rpm: -16.0 }.valid());
    }

    #[test]
    fn inventory_clock_survives_cold_restore_without_resurrecting_spent_input() {
        let mut live = ModBlockEntity::default();
        live.id = CONTROLLER.into();
        live.items = vec![None; crushing::SLOTS];
        live.items[0] = Some(ItemStack::new(INPUT, 1));
        let mut running = Cycle::default();
        running.insert_one();
        for _ in 0..3 { assert_eq!(running.tick(true, 16.0, true), Action::None); }
        assert!(snapshot(&mut live, running));

        let mut cold = ModBlockEntity::default();
        cold.id = CONTROLLER.into();
        cold.items.clear();
        cold.nbt = live.nbt.clone();
        restore(&mut cold);
        assert_eq!(cold.items.len(), crushing::SLOTS);
        assert_eq!(cold.items[0], Some(ItemStack::new(INPUT, 1)));
        assert_eq!(cycle(&cold), running);

        // The host spills canonical slots. A stale mirror must not put a
        // removed item back into those slots on a subsequent adapter tick.
        cold.items[0] = None;
        restore(&mut cold);
        assert!(cold.items.iter().all(Option::is_none));
    }
}
