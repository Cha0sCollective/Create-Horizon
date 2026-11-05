// Copyright (c) 2026 Create Horizon contributors
// SPDX-License-Identifier: MIT
// One normal mechanical piston, one initially placed pole, one-cell eastward
// travel. The piston machine owns absent stone/pole payloads; a moving head is
// derived geometry, never a separately saved vanilla moving-piston owner.
use super::Game;
use crate::containers::{BlockEntity, ModBlockEntity};
use create_engine::linear_carrier::{carrier_step, eastward_delta, reversal_offset};
use horizon_core::{
    geometry::{Aabb, Direction, Vec3},
    position::BlockPos,
};
use horizon_world::{
    level::pistons::MovingPiston,
    nbt::{Compound, Tag},
};

const KEY: &str = "create_horizon:s5_piston";
const BASE: &str = "create:mechanical_piston";
const POLE: &str = "create:piston_extension_pole";
const HEAD: &str = "create:mechanical_piston_head";
const STONE: &str = "minecraft:stone";
const PROXY: &str = "minecraft:moving_piston";

fn integer(data: &Compound, key: &str) -> i32 {
    match data.get(&key.into()) {
        Some(Tag::Int(v)) => *v,
        Some(Tag::Byte(v)) => i32::from(*v),
        _ => 0,
    }
}

fn float(data: &Compound, key: &str) -> f32 {
    match data.get(&key.into()) {
        Some(Tag::Float(v)) => f32::from_bits(*v),
        _ => 0.0,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Retracted = 0,
    Extending = 1,
    Extended = 2,
    Retracting = 3,
}

impl Phase {
    fn read(value: i32) -> Option<Self> {
        match value {
            0 => Some(Self::Retracted),
            1 => Some(Self::Extending),
            2 => Some(Self::Extended),
            3 => Some(Self::Retracting),
            _ => None,
        }
    }
    fn moving(self) -> bool {
        matches!(self, Self::Extending | Self::Retracting)
    }
}

#[derive(Clone)]
struct Piston {
    base: [i32; 3],
    pole: [i32; 3],
    head: [i32; 3],
    dest: [i32; 3],
    phase: Phase,
    data: Compound,
}

impl Piston {
    fn read(game: &Game<'_>, base: [i32; 3], entity: &ModBlockEntity) -> Option<Self> {
        if entity.id != BASE
            || game.native_block(base) != BASE
            || game.native_block_property(base, "facing").as_deref() != Some("east")
            || game
                .native_block_property(base, "axis_along_first")
                .as_deref()
                != Some("false")
        {
            return None;
        }
        let pole_x = base[0].checked_sub(1)?;
        let head_x = base[0].checked_add(1)?;
        let dest_x = base[0].checked_add(2)?;
        if [pole_x, head_x, dest_x]
            .iter()
            .any(|x| *x >> 4 != base[0] >> 4)
        {
            return None;
        }
        let data = match entity.nbt.get(&KEY.into()) {
            Some(Tag::Compound(data)) => data.clone(),
            None => Compound::new(),
            _ => return None,
        };
        let phase = Phase::read(integer(&data, "Phase"))?;
        let offset = float(&data, "Offset");
        let previous = float(&data, "Previous");
        if !offset.is_finite()
            || !previous.is_finite()
            || !(0.0..=1.0).contains(&offset)
            || !(0.0..=1.0).contains(&previous)
        {
            return None;
        }
        let stone = integer(&data, "CapturedStone");
        let pole = integer(&data, "CapturedPole");
        if !matches!(stone, 0 | 1)
            || !matches!(pole, 0 | 1)
            || (stone == 1 && data.get(&"Stone".into()) != Some(&Tag::String(STONE.into())))
            || (pole == 1 && data.get(&"Pole".into()) != Some(&Tag::String(POLE.into())))
            || (matches!(phase, Phase::Retracted | Phase::Extended) && (stone != 0 || pole != 0))
            || (phase == Phase::Retracted && offset != 0.0)
            || (phase == Phase::Extended && offset != 1.0)
        {
            return None;
        }
        Some(Self {
            base,
            pole: [pole_x, base[1], base[2]],
            head: [head_x, base[1], base[2]],
            dest: [dest_x, base[1], base[2]],
            phase,
            data,
        })
    }
    fn captured_stone(&self) -> bool {
        integer(&self.data, "CapturedStone") == 1
    }
    fn captured_pole(&self) -> bool {
        integer(&self.data, "CapturedPole") == 1
    }
    fn captured(&self) -> bool {
        self.captured_stone() || self.captured_pole()
    }
    fn offset(&self) -> f32 {
        float(&self.data, "Offset")
    }
    fn set_float(&mut self, key: &str, value: f32) {
        self.data.insert(key.into(), Tag::Float(value.to_bits()));
    }
    fn set_phase(&mut self, phase: Phase) {
        self.phase = phase;
        self.data.insert("Phase".into(), Tag::Int(phase as i32));
    }
    fn set_captured(&mut self, stone: bool, pole: bool) {
        self.data
            .insert("CapturedStone".into(), Tag::Byte(u8::from(stone) as i8));
        self.data
            .insert("CapturedPole".into(), Tag::Byte(u8::from(pole) as i8));
    }
    fn status(&mut self, value: &str) {
        self.data.insert("Status".into(), Tag::String(value.into()));
    }
    fn store(self, entity: &mut ModBlockEntity) {
        entity.nbt.insert(KEY.into(), Tag::Compound(self.data));
    }
    fn bounds(&self, source: [i32; 3], offset: f32) -> Aabb {
        let [x, y, z] = source.map(f64::from);
        Aabb::new(
            Vec3::new(x + f64::from(offset), y, z),
            Vec3::new(x + f64::from(offset) + 1.0, y + 1.0, z + 1.0),
        )
    }
    fn reserved(&self) -> [[i32; 3]; 4] {
        [self.base, self.pole, self.head, self.dest]
    }
    fn projection(
        &self,
        game: &Game<'_>,
        source: [i32; 3],
        block: &str,
        previous: f32,
        next: f32,
    ) -> Option<(BlockPos, MovingPiston)> {
        let step = carrier_step(previous, next)?;
        Some((
            BlockPos::new(source[0] + step.anchor_x, source[1], source[2]),
            MovingPiston {
                moved_state: game.pack.assets.registry.default_state(block)?,
                direction: if step.direction_x > 0 {
                    Direction::East
                } else {
                    Direction::West
                },
                extending: true,
                source: false,
                progress: step.previous_progress,
                progress_o: step.previous_progress,
                last_ticked: game.world.game_time,
                death_ticks: 0,
                id: 0,
            },
        ))
    }
}

fn pistons(game: &Game<'_>) -> Vec<Piston> {
    game.world
        .block_entities
        .iter()
        .filter_map(|(&(x, y, z), entity)| {
            let BlockEntity::Mod(entity) = entity else {
                return None;
            };
            Piston::read(game, [x, y, z], entity)
        })
        .collect()
}

fn set(game: &mut Game<'_>, pos: [i32; 3], block: &str) -> bool {
    let Some(state) = game.pack.assets.registry.default_state(block) else {
        return false;
    };
    let dim = game.dim.id().to_owned();
    game.native_set_block(&dim, pos, state, 3);
    game.native_block(pos) == block
}

fn base_state(game: &mut Game<'_>, pos: [i32; 3], state: &str) -> bool {
    if game.native_block_property(pos, "state").as_deref() == Some(state) {
        return true;
    }
    game.native_set_block_property(pos, "state", state);
    game.native_block_property(pos, "state").as_deref() == Some(state)
}

fn path_clear(game: &Game<'_>, piston: &Piston, from: f32, to: f32) -> bool {
    let reserved = piston.reserved();
    super::create_s5_carrier::static_path_clear_cells(game, piston.base, &reserved, from, to)
        && (!piston.captured_stone()
            || super::create_s5_carrier::static_path_clear_cells(
                game,
                piston.head,
                &reserved,
                from,
                to,
            ))
}

fn planned_push(
    game: &mut Game<'_>,
    piston: &Piston,
    from: f32,
    to: f32,
    returning_pole: bool,
) -> Option<super::piston_push::create_s5_clearance::S5PushPlan> {
    let (source, block) = if piston.captured_pole() {
        (piston.base, HEAD)
    } else {
        (piston.head, STONE)
    };
    let (anchor, projection) = piston.projection(game, source, block, from, to)?;
    let step = carrier_step(from, to)?;
    let cube = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
    let mut boxes = vec![cube];
    let mut final_bounds = vec![piston.bounds(source, to)];
    if piston.captured_pole() && piston.captured_stone() {
        boxes.push(Aabb::new(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(2.0, 1.0, 1.0),
        ));
        final_bounds.push(piston.bounds(piston.head, to));
    }
    if returning_pole {
        final_bounds.push(piston.bounds(piston.pole, 0.0));
    }
    game.create_s5_plan_push_shapes(anchor, projection, step.progress, &boxes, &final_bounds)
}

/// The real pole is initially placed. No payload is inferred from an empty
/// space or from the owner's configuration. All four cells must be loaded and
/// in the same chunk before the bounded capture can start.
fn capture_extension(game: &mut Game<'_>, piston: &mut Piston) -> bool {
    let reserved = piston.reserved();
    if piston.phase != Phase::Retracted
        || piston.offset() != 0.0
        || game.native_block(piston.pole) != POLE
        || game.native_block_property(piston.pole, "facing").as_deref() != Some("east")
        || game.native_block(piston.head) != STONE
        || game.native_block(piston.dest) != "minecraft:air"
        || game
            .world
            .block_entities
            .contains_key(&(piston.head[0], piston.head[1], piston.head[2]))
        || game
            .world
            .block_entities
            .contains_key(&(piston.dest[0], piston.dest[1], piston.dest[2]))
        || !path_clear(game, piston, 0.0, 1.0)
        || !super::create_s5_carrier::static_path_clear_cells(
            game,
            piston.head,
            &reserved,
            0.0,
            1.0,
        )
    {
        return false;
    }
    // A pole may have the port's empty mod entity, but no additional payload.
    if !game
        .world
        .block_entities
        .get(&(piston.pole[0], piston.pole[1], piston.pole[2]))
        .is_some_and(
            |entity| matches!(entity, BlockEntity::Mod(e) if e.id == POLE && e.items.is_empty()),
        )
    {
        return false;
    }
    piston.set_phase(Phase::Extending);
    piston.set_float("Offset", 0.0);
    piston.set_float("Previous", 0.0);
    piston
        .data
        .insert("Stone".into(), Tag::String(STONE.into()));
    piston.data.insert("Pole".into(), Tag::String(POLE.into()));
    piston.data.insert("CapturePending".into(), Tag::Byte(1));
    if !set(game, piston.head, PROXY)
        || !set(game, piston.dest, PROXY)
        || !set(game, piston.pole, "minecraft:air")
    {
        rollback_capture(game, piston);
        return false;
    }
    piston.set_captured(true, true);
    piston.data.remove(&"CapturePending".into());
    piston.status("extending");
    let _ = base_state(game, piston.base, "moving");
    true
}

/// Capture's writes can fail independently. Classify each physical original
/// before retrying: a missing stone or pole remains an NBT-owned payload;
/// a verified world original is no longer owned by the machine. The pending
/// flag keeps removal blocked even when both originals have returned but a
/// spare derived proxy still needs clearing.
fn rollback_capture(game: &mut Game<'_>, piston: &mut Piston) -> bool {
    piston.set_captured(
        game.native_block(piston.head) != STONE,
        game.native_block(piston.pole) != POLE,
    );
    if piston.captured_stone()
        && matches!(
            game.native_block(piston.head).as_str(),
            PROXY | "minecraft:air"
        )
        && game.create_s5_body_clear(
            BlockPos::new(piston.head[0], piston.head[1], piston.head[2]),
            piston.bounds(piston.head, 0.0),
        )
        && set(game, piston.head, STONE)
    {
        piston.set_captured(false, piston.captured_pole());
    }
    if piston.captured_pole()
        && matches!(
            game.native_block(piston.pole).as_str(),
            PROXY | "minecraft:air"
        )
        && game.create_s5_body_clear(
            BlockPos::new(piston.pole[0], piston.pole[1], piston.pole[2]),
            piston.bounds(piston.pole, 0.0),
        )
        && set(game, piston.pole, POLE)
    {
        piston.set_captured(piston.captured_stone(), false);
    }
    if game.native_block(piston.dest) == PROXY {
        let _ = set(game, piston.dest, "minecraft:air");
    }
    let restored = !piston.captured()
        && game.native_block(piston.head) == STONE
        && game.native_block(piston.pole) == POLE
        && game.native_block(piston.dest) == "minecraft:air";
    if restored {
        piston.data.remove(&"CapturePending".into());
        piston.set_phase(Phase::Retracted);
        piston.status("capture-rolled-back");
        let _ = base_state(game, piston.base, "retracted");
    } else {
        piston.data.insert("CapturePending".into(), Tag::Byte(1));
        piston.status("capture-restore-blocked");
    }
    restored
}

fn capture_retraction(game: &mut Game<'_>, piston: &mut Piston) -> bool {
    if piston.phase != Phase::Extended
        || piston.offset() != 1.0
        || game.native_block(piston.pole) != "minecraft:air"
        || game.native_block(piston.head) != HEAD
        || game.native_block_property(piston.head, "facing").as_deref() != Some("east")
        || !path_clear(game, piston, 1.0, 0.0)
    {
        return false;
    }
    piston.data.insert("Pole".into(), Tag::String(POLE.into()));
    piston.set_phase(Phase::Retracting);
    piston.set_float("Previous", 1.0);
    piston.data.insert("RetractionPending".into(), Tag::Byte(1));
    if !set(game, piston.head, PROXY) {
        rollback_retraction(game, piston);
        return false;
    }
    piston.set_captured(false, true);
    piston.data.remove(&"RetractionPending".into());
    piston.status("retracting");
    let _ = base_state(game, piston.base, "moving");
    true
}

fn rollback_retraction(game: &mut Game<'_>, piston: &mut Piston) -> bool {
    piston.set_captured(false, game.native_block(piston.head) != HEAD);
    if piston.captured_pole()
        && matches!(
            game.native_block(piston.head).as_str(),
            PROXY | "minecraft:air"
        )
        && game.create_s5_body_clear(
            BlockPos::new(piston.head[0], piston.head[1], piston.head[2]),
            piston.bounds(piston.head, 0.0),
        )
        && set(game, piston.head, HEAD)
    {
        piston.set_captured(false, false);
    }
    let restored = !piston.captured() && game.native_block(piston.head) == HEAD;
    if restored {
        piston.data.remove(&"RetractionPending".into());
        piston.set_phase(Phase::Extended);
        piston.status("retraction-rolled-back");
        let _ = base_state(game, piston.base, "extended");
    } else {
        piston.data.insert("RetractionPending".into(), Tag::Byte(1));
        piston.status("retraction-restore-blocked");
    }
    restored
}

/// Materialize the nearest reference grid state. Failed writes retain the
/// corresponding item in the piston NBT and are retried from a full-host tick.
fn settle(game: &mut Game<'_>, piston: &mut Piston, endpoint: f32) -> bool {
    if !piston.phase.moving() || !matches!(endpoint, 0.0 | 1.0) {
        return false;
    }
    let from = piston.offset();
    if piston.captured() && !path_clear(game, piston, from, endpoint) {
        return false;
    }
    if piston.captured_stone() && game.native_block(piston.dest) != PROXY {
        return false;
    }
    if piston.captured_pole() && game.native_block(piston.head) != PROXY {
        return false;
    }
    let mut push = if piston.captured() {
        let Some(push) = planned_push(game, piston, from, endpoint, endpoint == 0.0) else {
            return false;
        };
        Some(push)
    } else {
        None
    };
    let extension = piston.phase == Phase::Extending;
    let stone_target = if endpoint == 1.0 {
        piston.dest
    } else {
        piston.head
    };
    let pole_target = if endpoint == 1.0 {
        piston.head
    } else {
        piston.pole
    };
    let pole_form = if endpoint == 1.0 { HEAD } else { POLE };
    let unused = if endpoint == 1.0 {
        piston.pole
    } else if extension {
        piston.dest
    } else {
        piston.head
    };
    let pole_first = endpoint == 0.0;
    if pole_first && piston.captured_pole() {
        if !game.create_s5_body_clear(
            BlockPos::new(pole_target[0], pole_target[1], pole_target[2]),
            piston.bounds(pole_target, 0.0),
        ) {
            return false;
        }
        if !set(game, pole_target, pole_form) {
            return false;
        }
        if let Some(push) = push.take() {
            game.create_s5_commit_push(push);
        }
        piston.set_captured(piston.captured_stone(), false);
    }
    if extension && piston.captured_stone() {
        if !set(game, stone_target, STONE) {
            piston.data.insert("RestorePending".into(), Tag::Byte(1));
            piston
                .data
                .insert("PendingEndpoint".into(), Tag::Int(endpoint as i32));
            return false;
        }
        if let Some(push) = push.take() {
            game.create_s5_commit_push(push);
        }
        piston.set_captured(false, piston.captured_pole());
    }
    if !pole_first && piston.captured_pole() {
        if !set(game, pole_target, pole_form) {
            piston.data.insert("RestorePending".into(), Tag::Byte(1));
            piston
                .data
                .insert("PendingEndpoint".into(), Tag::Int(endpoint as i32));
            return false;
        }
        if let Some(push) = push.take() {
            game.create_s5_commit_push(push);
        }
        piston.set_captured(piston.captured_stone(), false);
    }
    if game.native_block(unused) == PROXY && !set(game, unused, "minecraft:air") {
        piston.status("proxy-clear-blocked");
        piston.data.insert("RestorePending".into(), Tag::Byte(1));
        piston
            .data
            .insert("PendingEndpoint".into(), Tag::Int(endpoint as i32));
        return false;
    }
    piston.set_float("Previous", endpoint);
    piston.set_float("Offset", endpoint);
    piston.set_phase(if endpoint == 0.0 {
        Phase::Retracted
    } else {
        Phase::Extended
    });
    piston.data.remove(&"RestorePending".into());
    piston.data.remove(&"PendingEndpoint".into());
    piston.status("placed");
    let _ = base_state(
        game,
        piston.base,
        if endpoint == 0.0 {
            "retracted"
        } else {
            "extended"
        },
    );
    true
}

pub(super) fn tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    if game.remote.is_some() || game.native_block(pos) != BASE {
        return;
    }
    let Some(mut piston) = Piston::read(game, pos, entity) else {
        return;
    };
    // Guest actions temporarily remove the shared hub from Game. Body plans,
    // materialization and removal run only with the complete authoritative hub.
    if game.lan.is_none() {
        piston.status("awaiting-full-host");
        piston.store(entity);
        return;
    }
    if integer(&piston.data, "RemovalReady") == 1 {
        return;
    }
    if integer(&piston.data, "CapturePending") == 1 {
        rollback_capture(game, &mut piston);
        piston.store(entity);
        game.native_block_entity_sync(pos);
        return;
    }
    if integer(&piston.data, "RetractionPending") == 1 {
        rollback_retraction(game, &mut piston);
        piston.store(entity);
        game.native_block_entity_sync(pos);
        return;
    }
    if integer(&piston.data, "RestorePending") == 1 {
        let endpoint = integer(&piston.data, "PendingEndpoint") as f32;
        if !settle(game, &mut piston, endpoint) {
            piston.status("restore-blocked");
        }
        piston.store(entity);
        game.native_block_entity_sync(pos);
        return;
    }
    if integer(&piston.data, "RemovalRequested") == 1 {
        let endpoint = f32::from(piston.offset() >= 0.5);
        if !piston.phase.moving() || settle(game, &mut piston, endpoint) {
            piston.data.insert("RemovalReady".into(), Tag::Byte(1));
            piston.status("removal-ready");
        } else {
            piston.status("removal-blocked");
        }
        piston.store(entity);
        game.native_block_entity_sync(pos);
        return;
    }

    let rpm = float(&entity.nbt, "Speed");
    if !rpm.is_finite() {
        return;
    }
    if piston.phase == Phase::Retracted && rpm < 0.0 {
        if !capture_extension(game, &mut piston) {
            piston.status("assembly-refused");
            piston.store(entity);
            game.native_block_entity_sync(pos);
            return;
        }
    } else if piston.phase == Phase::Extended && rpm > 0.0 {
        if !capture_retraction(game, &mut piston) {
            piston.status("retraction-refused");
            piston.store(entity);
            game.native_block_entity_sync(pos);
            return;
        }
    }
    if piston.phase.moving() {
        let from = piston.offset();
        let previous_rpm = float(&piston.data, "RPM");
        let opposite = previous_rpm != 0.0 && previous_rpm.signum() != rpm.signum();
        if rpm == 0.0 || opposite {
            let snapped = reversal_offset(
                from,
                previous_rpm,
                rpm,
                integer(&piston.data, "Stalled") == 1,
            )
            .unwrap_or(from);
            let endpoint = f32::from(snapped >= 0.5);
            if !settle(game, &mut piston, endpoint) {
                piston.data.insert("Stalled".into(), Tag::Byte(1));
                piston.status("stop-blocked");
            }
        } else if let Some(delta) = eastward_delta(rpm, from) {
            let to = (from + delta).clamp(0.0, 1.0);
            if !piston.captured()
                || (piston.captured_stone() && game.native_block(piston.dest) != PROXY)
                || (piston.captured_pole() && game.native_block(piston.head) != PROXY)
                || !path_clear(game, &piston, from, to)
            {
                piston.data.insert("Stalled".into(), Tag::Byte(1));
                piston.status("path-blocked");
            } else if let Some(push) = planned_push(game, &piston, from, to, false) {
                piston.set_float("Previous", from);
                piston.set_float("Offset", to);
                piston.data.insert("Stalled".into(), Tag::Byte(0));
                piston.status("moving");
                game.create_s5_commit_push(push);
                if (to == 0.0 || to == 1.0) && !settle(game, &mut piston, to) {
                    piston.data.insert("RestorePending".into(), Tag::Byte(1));
                    piston
                        .data
                        .insert("PendingEndpoint".into(), Tag::Int(to as i32));
                    piston.status("endpoint-blocked");
                }
            } else {
                piston.data.insert("Stalled".into(), Tag::Byte(1));
                piston.status("body-blocked");
            }
        }
    }
    piston.data.insert("RPM".into(), Tag::Float(rpm.to_bits()));
    piston
        .data
        .insert("Tick".into(), Tag::Long(game.world.game_time));
    piston.store(entity);
    game.native_block_entity_sync(pos);
}

/// The world vector contains derived moving-piston geometry only. It must be
/// pruned before the host snapshots finished vanilla piston entries, then
/// rebuilt from the saved Create owner on both server and peers.
pub(super) fn before_sync(game: &mut Game<'_>) {
    let profiles = pistons(game);
    game.world.moving_pistons.retain(|(pos, _)| {
        !profiles
            .iter()
            .any(|p| [p.base, p.head, p.dest].contains(pos))
            || game
                .block_ticks
                .moving_pistons
                .iter()
                .any(|(native, _)| [native.x, native.y, native.z] == *pos)
    });
}

pub(super) fn after_sync(game: &mut Game<'_>) {
    for piston in pistons(game).into_iter().filter(Piston::captured) {
        let previous = float(&piston.data, "Previous");
        let current = piston.offset();
        for (captured, source, block) in [
            (piston.captured_pole(), piston.base, HEAD),
            (piston.captured_stone(), piston.head, STONE),
        ] {
            let proxy = if block == HEAD {
                piston.head
            } else {
                piston.dest
            };
            if !captured || game.native_block(proxy) != PROXY {
                continue;
            }
            if let Some((anchor, mut projection)) =
                piston.projection(game, source, block, previous, current)
            {
                projection.progress = carrier_step(previous, current)
                    .expect("validated finite offsets")
                    .progress;
                game.world
                    .moving_pistons
                    .push(([anchor.x, anchor.y, anchor.z], projection.shape_state()));
            }
        }
    }
}

pub(super) fn refresh(game: &mut Game<'_>) {
    if !pistons(game).is_empty() {
        game.sync_moving_pistons();
    }
}

pub(super) fn owner_returned(game: &mut Game<'_>, pos: (i32, i32, i32)) {
    if game.world.block_entities.get(&pos).is_some_and(
        |entity| matches!(entity, BlockEntity::Mod(entity) if entity.id == BASE && entity.nbt.contains_key(&KEY.into())),
    ) {
        game.sync_moving_pistons();
    }
}

pub(super) fn guard_destroy(game: &mut Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    if *canceled {
        return;
    }
    for piston in pistons(game).into_iter().filter(|p| {
        p.phase.moving()
            || p.captured()
            || integer(&p.data, "CapturePending") == 1
            || integer(&p.data, "RetractionPending") == 1
            || integer(&p.data, "RestorePending") == 1
    }) {
        if [piston.pole, piston.head, piston.dest].contains(&pos) {
            *canceled = true;
            return;
        }
        if piston.base == pos {
            let mut data = piston.data;
            data.insert("RemovalRequested".into(), Tag::Byte(1));
            game.native_with_mod_block_entity(pos, |entity| {
                entity.nbt.insert(KEY.into(), Tag::Compound(data));
            });
            game.native_block_entity_sync(pos);
            *canceled = true;
            return;
        }
    }
    // A corrupt owner cannot be mined while it could still hold either item.
    if let Some(BlockEntity::Mod(entity)) = game.world.block_entities.get(&(pos[0], pos[1], pos[2]))
        && entity.id == BASE
        && let Some(Tag::Compound(data)) = entity.nbt.get(&KEY.into())
        && (integer(data, "CapturedStone") == 1
            || integer(data, "CapturedPole") == 1
            || integer(data, "CapturePending") == 1
            || integer(data, "RetractionPending") == 1
            || integer(data, "RestorePending") == 1)
    {
        *canceled = true;
    }
}

pub(super) fn guard_wrench(game: &Game<'_>, item: &str, pos: [i32; 3]) -> bool {
    item == "create:wrench"
        && pistons(game)
            .iter()
            .any(|p| p.phase.moving() && [p.base, p.pole, p.head, p.dest].contains(&pos))
}

pub(super) fn guard_proxy_use(game: &Game<'_>, pos: [i32; 3]) -> bool {
    pistons(game)
        .iter()
        .any(|p| p.phase.moving() && [p.pole, p.head, p.dest].contains(&pos))
}
