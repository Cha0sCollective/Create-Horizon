// Copyright (c) 2026 Create Horizon contributors
// SPDX-License-Identifier: MIT
// Explicitly configured, one-cell S5 feasibility carrier. A motor's saved mod
// entity owns the payload; collision/render proxies contain no vanilla owner.
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

const KEY: &str = "create_horizon:s5_trial";
const PROXY: &str = "minecraft:moving_piston";

fn int(data: &Compound, key: &str) -> i32 {
    match data.get(&key.into()) {
        Some(Tag::Byte(v)) => i32::from(*v),
        Some(Tag::Int(v)) => *v,
        _ => 0,
    }
}

fn float(data: &Compound, key: &str) -> f32 {
    match data.get(&key.into()) {
        Some(Tag::Float(v)) => f32::from_bits(*v),
        Some(Tag::Int(v)) => *v as f32,
        _ => 0.0,
    }
}

#[derive(Clone)]
struct Trial {
    owner: [i32; 3],
    source: [i32; 3],
    data: Compound,
}

impl Trial {
    fn read(owner: [i32; 3], entity: &ModBlockEntity) -> Option<Self> {
        if entity.id != "create:motor" {
            return None;
        }
        let Tag::Compound(data) = entity.nbt.get(&KEY.into())? else {
            return None;
        };
        if int(data, "Enabled") != 1 && int(data, "Captured") != 1 {
            return None;
        }
        let source = [owner[0].checked_add(2)?, owner[1], owner[2]];
        let end = source[0].checked_add(1)?;
        if source[0] >> 4 != owner[0] >> 4 || end >> 4 != owner[0] >> 4 {
            return None;
        }
        if !float(data, "Offset").is_finite() || !(0.0..=1.0).contains(&float(data, "Offset")) {
            return None;
        }
        if !float(data, "Previous").is_finite() || !(0.0..=1.0).contains(&float(data, "Previous")) {
            return None;
        }
        if int(data, "Captured") == 1
            && data.get(&"Payload".into()) != Some(&Tag::String("minecraft:stone".into()))
        {
            return None;
        }
        Some(Self {
            owner,
            source,
            data: data.clone(),
        })
    }
    fn cells(&self) -> [[i32; 3]; 2] {
        [
            self.source,
            [self.source[0] + 1, self.source[1], self.source[2]],
        ]
    }
    fn captured(&self) -> bool {
        int(&self.data, "Captured") == 1
    }
    fn offset(&self) -> f32 {
        float(&self.data, "Offset")
    }
    fn set_float(&mut self, key: &str, value: f32) {
        self.data.insert(key.into(), Tag::Float(value.to_bits()));
    }
    fn status(&mut self, status: &str) {
        self.data
            .insert("Status".into(), Tag::String(status.into()));
    }
    fn store(self, entity: &mut ModBlockEntity) {
        entity.nbt.insert(KEY.into(), Tag::Compound(self.data));
    }
    fn bounds(&self, offset: f32) -> Aabb {
        let [x, y, z] = self.source.map(f64::from);
        Aabb::new(
            Vec3::new(x + f64::from(offset), y, z),
            Vec3::new(x + f64::from(offset) + 1.0, y + 1.0, z + 1.0),
        )
    }
    fn projection(
        &self,
        game: &Game<'_>,
        previous: f32,
        next: f32,
    ) -> Option<(BlockPos, MovingPiston)> {
        let step = carrier_step(previous, next)?;
        Some((
            BlockPos::new(
                self.source[0] + step.anchor_x,
                self.source[1],
                self.source[2],
            ),
            MovingPiston {
                moved_state: game.pack.assets.registry.default_state("minecraft:stone")?,
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

fn trials(game: &Game<'_>) -> Vec<Trial> {
    game.world
        .block_entities
        .iter()
        .filter_map(|(&(x, y, z), entity)| {
            let BlockEntity::Mod(entity) = entity else {
                return None;
            };
            Trial::read([x, y, z], entity)
        })
        .collect()
}

fn set(game: &mut Game<'_>, pos: [i32; 3], name: &str) -> bool {
    let Some(state) = game.pack.assets.registry.default_state(name) else {
        return false;
    };
    let dim = game.dim.id().to_owned();
    game.native_set_block(&dim, pos, state, 3);
    game.native_block(pos) == name
}

/// The two reserved cells may be empty while their solid payload is moving.
/// Inspect native collision boxes in the one-block halo as well: a fence below
/// the path, for example, can extend above its own cell into the swept cube.
fn static_path_clear(game: &Game<'_>, trial: &Trial, from: f32, to: f32) -> bool {
    static_path_clear_cells(game, trial.source, &trial.cells(), from, to)
}

/// Shared finite sweep for the trial stone and the real actuator's head/stone.
pub(super) fn static_path_clear_cells(
    game: &Game<'_>,
    source: [i32; 3],
    reserved: &[[i32; 3]],
    from: f32,
    to: f32,
) -> bool {
    if !from.is_finite()
        || !to.is_finite()
        || !(0.0..=1.0).contains(&from)
        || !(0.0..=1.0).contains(&to)
    {
        return false;
    }
    let [sx, sy, sz] = source;
    let (Some(x0), Some(x1), Some(y0), Some(y1), Some(z0), Some(z1)) = (
        sx.checked_sub(1),
        sx.checked_add(2),
        sy.checked_sub(1),
        sy.checked_add(1),
        sz.checked_sub(1),
        sz.checked_add(1),
    ) else {
        return false;
    };
    let swept = Aabb::new(
        Vec3::new(
            f64::from(sx) + f64::from(from.min(to)),
            f64::from(sy),
            f64::from(sz),
        ),
        Vec3::new(
            f64::from(sx) + 1.0 + f64::from(from.max(to)),
            f64::from(sy) + 1.0,
            f64::from(sz) + 1.0,
        ),
    );
    for x in x0..=x1 {
        for z in z0..=z1 {
            if !game.world.loaded(x, z) {
                return false;
            }
            for y in y0..=y1 {
                let pos = [x, y, z];
                if reserved.contains(&pos) {
                    continue;
                }
                for shape in game.native_block_collision(pos) {
                    if shape.iter().any(|v| !v.is_finite()) {
                        return false;
                    }
                    let obstacle = Aabb::new(
                        Vec3::new(
                            f64::from(x) + shape[0],
                            f64::from(y) + shape[1],
                            f64::from(z) + shape[2],
                        ),
                        Vec3::new(
                            f64::from(x) + shape[3],
                            f64::from(y) + shape[4],
                            f64::from(z) + shape[5],
                        ),
                    );
                    if swept.intersects(obstacle) {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn restore(game: &mut Game<'_>, trial: &mut Trial) -> bool {
    let cells = trial.cells();
    let endpoint = usize::from(trial.offset() >= 0.5);
    if cells
        .iter()
        .any(|p| !matches!(game.native_block(*p).as_str(), PROXY | "minecraft:air"))
        || !static_path_clear(game, trial, trial.offset(), endpoint as f32)
    {
        return false;
    }
    let Some((anchor, piston)) = trial.projection(game, trial.offset(), endpoint as f32) else {
        return false;
    };
    let step = carrier_step(trial.offset(), endpoint as f32).expect("validated endpoint");
    let Some(push) =
        game.create_s5_plan_push(anchor, piston, step.progress, trial.bounds(endpoint as f32))
    else {
        return false;
    };
    // No other callbacks/awaits or owner deletion between this finite transfer.
    let opposite = cells[1 - endpoint];
    if game.native_block(opposite) != "minecraft:air" && !set(game, opposite, "minecraft:air") {
        return false;
    }
    if !set(game, cells[endpoint], "minecraft:stone") {
        // The machine still owns the stone. Repair the cleared proxy if the
        // destination write fails, then retry the restoration on a full tick.
        if game.native_block(opposite) == "minecraft:air" {
            let _ = set(game, opposite, PROXY);
        }
        return false;
    }
    trial.data.insert("Captured".into(), Tag::Byte(0));
    trial.set_float("Offset", endpoint as f32);
    trial.set_float("Previous", endpoint as f32);
    trial.status("restored");
    game.create_s5_commit_push(push);
    true
}

pub(super) fn tick(game: &mut Game<'_>, pos: [i32; 3], entity: &mut ModBlockEntity) {
    if game.remote.is_some() || game.native_block(pos) != "create:creative_motor" {
        return;
    }
    let Some(mut trial) = Trial::read(pos, entity) else {
        return;
    };
    // Guest actions temporarily take LanHost out of Game and swap the actor
    // into player. Motion/restoration require the complete server hub.
    if game.lan.is_none() {
        trial.set_float("Previous", trial.offset());
        trial.status("awaiting-full-host");
        trial.store(entity);
        return;
    }
    if int(&trial.data, "RemovalReady") == 1 {
        return; // Do not recapture before the player's deletion retry.
    }
    if trial.captured() && int(&trial.data, "RemovalRequested") == 1 {
        trial.set_float("Previous", trial.offset());
        if restore(game, &mut trial) {
            trial.data.insert("RemovalRequested".into(), Tag::Byte(0));
            trial.data.insert("RemovalReady".into(), Tag::Byte(1));
        } else {
            trial.data.insert("Stalled".into(), Tag::Byte(1));
            trial.status("restore-blocked");
        }
        trial.store(entity);
        game.native_block_entity_sync(pos);
        return;
    }
    if trial.captured() && int(&trial.data, "RestorePending") == 1 {
        trial.set_float("Previous", trial.offset());
        if restore(game, &mut trial) {
            trial.data.remove(&"RestorePending".into());
        } else {
            trial.status("restore-blocked");
        }
        trial.store(entity);
        game.native_block_entity_sync(pos);
        return;
    }
    let rpm = float(&entity.nbt, "Speed");
    let Some(delta) = eastward_delta(rpm, trial.offset()) else {
        return;
    };
    trial
        .data
        .insert("Tick".into(), Tag::Long(game.world.game_time));
    trial
        .data
        .insert("OriginX".into(), Tag::Int(trial.source[0]));
    trial
        .data
        .insert("OriginY".into(), Tag::Int(trial.source[1]));
    trial
        .data
        .insert("OriginZ".into(), Tag::Int(trial.source[2]));
    if !trial.captured() && delta != 0.0 {
        let cells = trial.cells();
        let endpoint = usize::from(trial.offset() >= 0.5);
        if !trials(game).is_empty()
            || game.native_block(cells[endpoint]) != "minecraft:stone"
            || game.native_block(cells[1 - endpoint]) != "minecraft:air"
            || !game.world.loaded(cells[0][0], cells[0][2])
            || !static_path_clear(game, &trial, 0.0, 1.0)
        {
            trial.status("capture-refused");
            trial.store(entity);
            return;
        }
        trial.data.insert("Captured".into(), Tag::Byte(1));
        trial
            .data
            .insert("Payload".into(), Tag::String("minecraft:stone".into()));
        if !set(game, cells[endpoint], PROXY) || !set(game, cells[1 - endpoint], PROXY) {
            if set(game, cells[endpoint], "minecraft:stone") {
                trial.data.insert("Captured".into(), Tag::Byte(0));
            }
            trial.status("capture-write-refused");
            trial.store(entity);
            return;
        }
    }
    if trial.captured() {
        let original = trial.offset();
        let snapped = reversal_offset(
            original,
            float(&trial.data, "RPM"),
            rpm,
            int(&trial.data, "Stalled") == 1,
        )
        .unwrap_or(original);
        let next = (snapped + eastward_delta(rpm, snapped).unwrap_or(0.0)).clamp(0.0, 1.0);
        if trial.cells().iter().any(|p| game.native_block(*p) != PROXY) {
            trial.status("reserved-cell-changed");
            trial.data.insert("Stalled".into(), Tag::Byte(1));
            trial.set_float("Previous", original);
        } else if !static_path_clear(game, &trial, original, next) {
            trial.set_float("Previous", original);
            trial.status("world-blocked");
            trial.data.insert("Stalled".into(), Tag::Byte(1));
        } else if let Some((anchor, piston)) = trial.projection(game, original, next) {
            let step = carrier_step(original, next).expect("validated finite trial");
            if let Some(push) =
                game.create_s5_plan_push(anchor, piston, step.progress, trial.bounds(next))
            {
                trial.set_float("Previous", original);
                trial.set_float("Offset", next);
                trial.data.insert("Stalled".into(), Tag::Byte(0));
                trial.status(if next == original { "paused" } else { "moving" });
                game.create_s5_commit_push(push);
                if (next == 0.0 || next == 1.0) && next != original && !restore(game, &mut trial) {
                    trial.status("restore-blocked");
                    trial.data.insert("RestorePending".into(), Tag::Byte(1));
                }
            } else {
                trial.set_float("Previous", original);
                trial.status("body-blocked");
                trial.data.insert("Stalled".into(), Tag::Byte(1));
            }
        }
    }
    trial.set_float("RPM", rpm);
    trial.store(entity);
    game.native_block_entity_sync(pos);
}

/// Remove only derived entries, before the ordinary sync snapshots them as
/// finished vanilla pistons. Vanilla owners in block_ticks are never removed.
pub(super) fn before_sync(game: &mut Game<'_>) {
    let profiles = trials(game);
    game.world.moving_pistons.retain(|(pos, _)| {
        !profiles.iter().any(|t| t.cells().contains(pos))
            || game
                .block_ticks
                .moving_pistons
                .iter()
                .any(|(p, _)| [p.x, p.y, p.z] == *pos)
    });
}

pub(super) fn after_sync(game: &mut Game<'_>) {
    for trial in trials(game).into_iter().filter(|t| t.captured()) {
        if trial.cells().iter().any(|p| game.native_block(*p) != PROXY) {
            continue;
        }
        if let Some((anchor, mut piston)) =
            trial.projection(game, float(&trial.data, "Previous"), trial.offset())
        {
            piston.progress = carrier_step(float(&trial.data, "Previous"), trial.offset())
                .expect("validated offsets")
                .progress;
            game.world
                .moving_pistons
                .push(([anchor.x, anchor.y, anchor.z], piston.shape_state()));
        }
    }
}

pub(super) fn refresh(game: &mut Game<'_>) {
    if !trials(game).is_empty() {
        game.sync_moving_pistons();
    }
}

pub(super) fn owner_returned(game: &mut Game<'_>, pos: (i32, i32, i32)) {
    let enabled = game.world.block_entities.get(&pos).is_some_and(
        |entity| matches!(entity,BlockEntity::Mod(entity) if entity.nbt.contains_key(&KEY.into())),
    );
    if enabled {
        game.sync_moving_pistons();
    }
}

pub(super) fn guard_destroy(game: &mut Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    if *canceled {
        return;
    }
    let profiles = trials(game);
    for trial in profiles.into_iter().filter(|t| t.captured()) {
        if trial.cells().contains(&pos) {
            *canceled = true;
            return;
        }
        if trial.owner == pos {
            // This action lacks the other guests' host/body context. Retain
            // the actual owner, restore on a full tick, then require retry.
            let mut data = trial.data;
            data.insert("RemovalRequested".into(), Tag::Byte(1));
            game.native_with_mod_block_entity(pos, |entity| {
                entity.nbt.insert(KEY.into(), Tag::Compound(data));
            });
            game.native_block_entity_sync(pos);
            *canceled = true;
            return;
        }
    }
    // Corrupt finite state keeps its actual motor owner instead of deleting it.
    if let Some(BlockEntity::Mod(entity)) = game.world.block_entities.get(&(pos[0], pos[1], pos[2]))
    {
        if let Some(Tag::Compound(data)) = entity.nbt.get(&KEY.into())
            && int(data, "Captured") == 1
        {
            *canceled = true;
        }
    }
}

pub(super) fn guard_wrench(game: &Game<'_>, item: &str, pos: [i32; 3]) -> bool {
    item == "create:wrench"
        && trials(game)
            .iter()
            .any(|t| t.captured() && (t.owner == pos || t.cells().contains(&pos)))
}

pub(super) fn guard_proxy_use(game: &Game<'_>, pos: [i32; 3]) -> bool {
    trials(game)
        .iter()
        .any(|trial| trial.captured() && trial.cells().contains(&pos))
}
