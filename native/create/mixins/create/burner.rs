// Copyright (c) The Create Team / The Creators of Create
// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
// BlazeBurnerBlock / BlazeBurnerBlockEntity, pinned Create6.0.10.
use super::{Game, GameLevel};
use crate::{
    containers::{BlockEntity, ModBlockEntity},
    items::ItemStack,
};
use create_engine::burner::{Burner, Candidate, Fuel, Heat};
use horizon_core::random::RandomSource;
use horizon_world::{
    nbt::{Compound, Tag},
    registry::BlockStateId,
};
type Pos = [i32; 3];
pub(super) const BLOCK: &str = "create:blaze_burner";
pub(super) const ENTITY: &str = "create:blaze_heater";
const FIELDS: [&str; 5] = [
    "fuelLevel",
    "burnTimeRemaining",
    "isCreative",
    "Goggles",
    "TrainHat",
];

pub(super) fn heat(name: &str) -> Option<Heat> {
    Some(match name {
        "none" => Heat::None,
        "smouldering" => Heat::Smouldering,
        "fading" => Heat::Fading,
        "kindled" => Heat::Kindled,
        "seething" => Heat::Seething,
        _ => return None,
    })
}
fn heat_name(heat: Heat) -> &'static str {
    match heat {
        Heat::None => "none",
        Heat::Smouldering => "smouldering",
        Heat::Fading => "fading",
        Heat::Kindled => "kindled",
        Heat::Seething => "seething",
    }
}
pub(super) fn stateful(block: &str, owns: &mut bool) {
    if block == BLOCK {
        *owns = true;
    }
}
pub(super) fn entity(
    block: &str,
    properties: &crate::mod_api::BlockProperties,
    out: &mut Option<(String, Compound)>,
) {
    if out.is_some()
        || block != BLOCK
        || properties
            .get("blaze")
            .and_then(|v| heat(v))
            .is_none_or(|v| v == Heat::None)
    {
        return;
    }
    let mut nbt = Compound::new();
    store(&mut nbt, Burner::default());
    *out = Some((ENTITY.into(), nbt));
}

fn load(nbt: &Compound, heat: Heat) -> Option<Burner> {
    // Preserve unsupported payloads untouched. Empty placement metadata is
    // produced by the host; no arbitrary fields or alternate numeric tags.
    if nbt.iter().any(|(key, value)| {
        let named = |name: &str| key.0.iter().copied().eq(name.encode_utf16());
        if named("fuelLevel") || named("burnTimeRemaining") {
            !matches!(value, Tag::Int(_))
        } else if named("isCreative") {
            !matches!(value, Tag::Byte(0))
        } else if named("components") {
            !matches!(value, Tag::Compound(c) if c.is_empty())
        } else {
            true
        }
    }) {
        return None;
    }
    let number = |key: &str| match nbt.get(&key.into()) {
        Some(Tag::Int(v)) => *v,
        _ => 0,
    };
    let fuel = match number("fuelLevel") {
        0 => Fuel::None,
        1 => Fuel::Normal,
        _ => return None,
    };
    let remaining = number("burnTimeRemaining");
    if !(0..=create_engine::burner::MAX_HEAT_CAPACITY).contains(&remaining)
        || (fuel == Fuel::None) != (remaining == 0)
        || !matches!(heat, Heat::Smouldering | Heat::Fading | Heat::Kindled)
    {
        return None;
    }
    Some(Burner {
        fuel,
        remaining,
        creative: false,
        heat,
    })
}
pub(super) fn supported(entity: &ModBlockEntity, heat: Heat) -> bool {
    entity.id == ENTITY
        && entity.items.iter().all(Option::is_none)
        && load(&entity.nbt, heat).is_some()
}
fn store(nbt: &mut Compound, state: Burner) {
    if state.creative {
        nbt.remove(&"fuelLevel".into());
        nbt.remove(&"burnTimeRemaining".into());
        nbt.insert("isCreative".into(), Tag::Byte(1));
    } else {
        nbt.remove(&"isCreative".into());
        nbt.insert("fuelLevel".into(), Tag::Int(state.fuel as i32));
        nbt.insert("burnTimeRemaining".into(), Tag::Int(state.remaining));
    }
}
fn current(game: &Game<'_>, pos: Pos) -> Option<Burner> {
    if game.native_block(pos) != BLOCK {
        return None;
    }
    let heat = heat(&game.native_block_property(pos, "blaze")?)?;
    let BlockEntity::Mod(entity) = game.world.block_entities.get(&(pos[0], pos[1], pos[2]))? else {
        return None;
    };
    if entity.id != ENTITY || entity.items.iter().any(Option::is_some) {
        return None;
    }
    load(&entity.nbt, heat)
}
fn candidate(game: &Game<'_>, stack: &ItemStack) -> Candidate {
    if game.remote.is_some() {
        // NeoForge synchronizes these merged server definitions. The native
        // client waits for the complete world snapshot rather than guessing
        // from its local resource pack or duplicating a furnace fuel table.
        return super::create_port_burner_sync::candidate(game, stack).unwrap_or(Candidate {
            fuel: Fuel::None,
            burn_time: 0,
        });
    }
    let data = |id| {
        game.native_data_map("minecraft:item", id, &stack.item)
            .and_then(|v| v.as_i64().or_else(|| v["burn_time"].as_i64()))
            .and_then(|v| i32::try_from(v).ok())
            .filter(|v| *v > 0)
    };
    let tag = |name| game.native_server_item_in_tag(stack, name).unwrap_or(false);
    Candidate::resolve(
        data("create:superheated_blaze_burner_fuels"),
        data("create:regular_blaze_burner_fuels"),
        tag("create:blaze_burner_fuel/special"),
        game.res.providers().burn_time(game.items(), stack, false),
        tag("create:blaze_burner_fuel/regular"),
    )
}
fn set_heat(game: &mut Game<'_>, pos: Pos, heat: Heat) {
    if game.native_set_block_property(pos, "blaze", heat_name(heat)) {
        game.native_block_entity_sync(pos);
    }
}
pub(super) fn tick(game: &mut Game<'_>, pos: Pos, entity: &mut ModBlockEntity) {
    if entity.id != ENTITY || entity.items.iter().any(Option::is_some) {
        return;
    }
    let Some(heat) = game
        .native_block_property(pos, "blaze")
        .and_then(|v| heat(&v))
    else {
        return;
    };
    let Some(mut state) = load(&entity.nbt, heat) else {
        return;
    };
    state.tick(|heat| set_heat(game, pos, heat));
    store(&mut entity.nbt, state);
}
fn play(game: &mut Game<'_>, pos: Pos, sound: &str, ambient: bool) {
    let volume = 0.125 + game.world.random.next_float() * 0.125;
    let pitch = if ambient { 1.15 } else { 0.75 } - game.world.random.next_float() * 0.25;
    game.with_level(|_, level, _| {
        level.native_play_sound(sound, pos.map(|v| f64::from(v) + 0.5), 4, volume, pitch);
    });
}
pub(super) fn reacts(game: &Game<'_>, pos: Pos) -> bool {
    if !game.native_on("create")
        || game.keys.contains(&winit::keyboard::KeyCode::ShiftLeft)
            && (game
                .inventory
                .get(game.selected)
                .is_some_and(Option::is_some)
                || game.offhand.is_some())
    {
        return false;
    }
    let Some(mut state) = current(game, pos) else {
        return false;
    };
    if state.heat == Heat::None {
        return false;
    }
    let Some(stack) = game.held().filter(|s| plain_coal(s)) else {
        return false;
    };
    let fuel = candidate(game, stack);
    supported_fuel(fuel) && state.insert(fuel, true, true, |_| {})
}
fn supported_fuel(candidate: Candidate) -> bool {
    candidate.fuel == Fuel::Normal
        && (1..=create_engine::burner::MAX_HEAT_CAPACITY).contains(&candidate.burn_time)
}
fn plain_coal(stack: &ItemStack) -> bool {
    stack.count > 0
        && stack.count <= 64
        && stack.item.as_ref() == "minecraft:coal"
        && *stack == ItemStack::new("minecraft:coal", stack.count)
}

/// Selected upstream prediction: authoritative inventory/block heat stay server-owned.
pub(super) fn predict_client(game: &mut Game<'_>, pos: Pos) -> bool {
    if game.remote.is_none() || !reacts(game, pos) {
        return false;
    }
    let held = game.held().cloned().unwrap();
    let Some(mut state) = current(game, pos) else {
        return false;
    };
    if !state.insert(candidate(game, &held), true, false, |_| {}) {
        return false;
    }
    game.native_with_mod_block_entity(pos, |entity| store(&mut entity.nbt, state));
    super::create_port_burner_presentation::burst(game, pos, false);
    true
}

pub(super) fn right_click(game: &mut Game<'_>, pos: Pos) -> bool {
    if !reacts(game, pos) {
        return false;
    }
    let held = game.held().cloned().unwrap();
    let Some(mut state) = current(game, pos) else {
        return false;
    };
    let old = state.heat;
    if !state.insert(candidate(game, &held), true, false, |_| {}) {
        return false;
    }
    play(game, pos, "minecraft:entity.blaze.shoot", false);
    set_heat(game, pos, state.heat);
    if old != state.heat {
        play(game, pos, "minecraft:entity.blaze.ambient", true);
    }
    game.native_with_mod_block_entity(pos, |e| store(&mut e.nbt, state));
    if old != state.heat {
        game.native_block_entity_sync(pos);
    }
    if !game.creative {
        game.consume_held();
    }
    true
}
pub(super) fn analog(game: &GameLevel<'_>, state: BlockStateId, out: &mut i32) {
    if game.native_state_block(state) == Some(BLOCK) {
        *out = game
            .registry
            .state(state)
            .and_then(|s| s.properties.get("blaze"))
            .and_then(|v| heat(v))
            .map_or(0, |h| i32::from(h.comparator()));
    }
}
/// Pending client load-tag boundary: raw adapter uses the exact validated
/// type/block/compound guards of Game::remote_block_entity_data. Java read
/// resets absent canonical fields, whereas Horizon's default merge retains them.
pub(super) fn packet(game: &mut Game<'_>, pos: Pos, kind: i32, tag: &Tag) {
    let Tag::Compound(fields) = tag else {
        return;
    };
    let Some(name) = game
        .remote
        .as_ref()
        .and_then(|r| r.builtin.get("minecraft:block_entity_type")?.name(kind))
    else {
        return;
    };
    if *game
        .res
        .facts()
        .get(game.world.block(pos[0], pos[1], pos[2]))
        .name
        != *name
        || name != BLOCK
    {
        return;
    }
    let Some(BlockEntity::Mod(entity)) =
        game.world.block_entities.get_mut(&(pos[0], pos[1], pos[2]))
    else {
        return;
    };
    if entity.id != ENTITY {
        return;
    }
    for field in FIELDS {
        if !fields.contains_key(&field.into()) {
            entity.nbt.remove(&field.into());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn occupied_factory_and_plain_state_refuse_unsupported_payloads() {
        let mut properties = crate::mod_api::BlockProperties::new();
        properties.insert("blaze".into(), "none".into());
        let mut out = None;
        entity(BLOCK, &properties, &mut out);
        assert!(out.is_none());
        properties.insert("blaze".into(), "smouldering".into());
        entity(BLOCK, &properties, &mut out);
        let (id, mut nbt) = out.unwrap();
        assert_eq!(id, ENTITY);
        assert_eq!(load(&nbt, Heat::Smouldering), Some(Burner::default()));
        nbt.insert("components".into(), Tag::Compound(Compound::new()));
        assert!(load(&nbt, Heat::Smouldering).is_some());
        let mut fueled = Burner::default();
        assert!(fueled.insert(
            Candidate {
                fuel: Fuel::Normal,
                burn_time: 1600
            },
            true,
            false,
            |_| {}
        ));
        store(&mut nbt, fueled);
        assert_eq!(load(&nbt, Heat::Kindled), Some(fueled));
        for (key, value) in [
            ("fuelLevel", Tag::Int(2)),
            ("burnTimeRemaining", Tag::Int(-1)),
            ("burnTimeRemaining", Tag::Int(10001)),
            ("burnTimeRemaining", Tag::Long(1600)),
            ("isCreative", Tag::Byte(1)),
            ("Goggles", Tag::Byte(1)),
            ("foreign", Tag::Int(0)),
            ("components", Tag::Int(0)),
        ] {
            let mut bad = nbt.clone();
            bad.insert(key.into(), value);
            let before = bad.clone();
            assert!(load(&bad, Heat::Kindled).is_none(), "{key}");
            assert_eq!(bad, before);
        }
        let mut nonempty = Compound::new();
        nonempty.insert("foreign".into(), Tag::Int(1));
        nbt.insert("components".into(), Tag::Compound(nonempty));
        assert!(load(&nbt, Heat::Kindled).is_none());
    }
    #[test]
    fn only_plain_coal_is_supported_and_fuel_ticks_keep_source_fields() {
        assert!(plain_coal(&ItemStack::new("minecraft:coal", 1)));
        for (id, count) in [
            ("minecraft:charcoal", 1),
            ("create:blaze_cake", 1),
            ("create:goggles", 1),
            ("minecraft:coal", 0),
            ("minecraft:coal", 65),
        ] {
            assert!(!plain_coal(&ItemStack::new(id, count)));
        }
        let mut state = Burner {
            fuel: Fuel::Normal,
            remaining: 125,
            creative: false,
            heat: Heat::Kindled,
        };
        let mut updates = Vec::new();
        state.tick(|heat| updates.push(heat));
        assert_eq!(updates, [Heat::Fading]);
        assert_eq!(state.remaining, 124);
        let mut nbt = Compound::new();
        store(&mut nbt, state);
        assert_eq!(load(&nbt, Heat::Fading), Some(state));
    }
}
