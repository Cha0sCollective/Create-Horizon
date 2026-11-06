// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
// Bounded server-side Tank -> facing Pump -> straight Pipe -> Basin transfer.
// Canonical water remains only in the existing TankContent/InputTanks saves.
use super::{Game, create_port_fluid_codec as basin_codec, create_port_fluid_tank_codec as tank_codec};
use crate::{
    containers::{BlockEntity, ModRevision},
    items::ItemStack,
    mod_api::{NativeFluidAction, NativeFluidDrain, NativeFluidStack, NativeFluidTankView},
};
use create_engine::pump_transfer::{Step, Volumes};
use horizon_core::geometry::Direction;
use horizon_world::nbt::{Compound, Tag};

type Pos = [i32; 3];
const TANK: &str = "create:fluid_tank";
const PUMP: &str = "create:mechanical_pump";
const PIPE: &str = "create:fluid_pipe";
const BASIN: &str = "create:basin";

#[derive(Clone, Copy)]
struct Line {
    tank: Pos,
    pump: Pos,
    pipe: Pos,
    basin: Pos,
    facing: Direction,
}

#[derive(Clone)]
struct Snapshot {
    nbt: Compound,
    items: Vec<Option<ItemStack>>,
    revision: ModRevision,
    amount: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TickResult {
    Refused,
    Moved(u32),
    Restored,
}

fn displaced(pos: Pos, direction: Direction, distance: i32) -> Option<Pos> {
    let delta = direction.normal();
    let mut result = pos;
    for axis in 0..3 {
        result[axis] = pos[axis].checked_add(delta[axis] * distance)?;
    }
    Some(result)
}

fn loaded(game: &Game<'_>, pos: Pos) -> bool {
    game.world.loaded(pos[0], pos[2])
}

fn facing(name: &str) -> Option<Direction> {
    Some(match name {
        "down" => Direction::Down,
        "up" => Direction::Up,
        "north" => Direction::North,
        "south" => Direction::South,
        "west" => Direction::West,
        "east" => Direction::East,
        _ => return None,
    })
}

fn pipe_axis(direction: Direction) -> &'static str {
    match direction {
        Direction::East | Direction::West => "x",
        Direction::Up | Direction::Down => "y",
        Direction::North | Direction::South => "z",
    }
}

fn checked_line(game: &Game<'_>, pump: Pos) -> Option<Line> {
    if !loaded(game, pump) || game.native_block(pump) != PUMP {
        return None;
    }
    let direction = facing(&game.native_block_property(pump, "facing")?)?;
    let tank = displaced(pump, direction.opposite(), 1)?;
    let pipe = displaced(pump, direction, 1)?;
    let basin = displaced(pump, direction, 2)?;
    let line = Line {
        tank,
        pump,
        pipe,
        basin,
        facing: direction,
    };
    let positions = [tank, pump, pipe, basin];
    if positions.iter().any(|&pos| !loaded(game, pos))
        || game.native_block(tank) != TANK
        || game.native_block(pipe) != PIPE
        || game.native_block(basin) != BASIN
        || game.native_block_property(pipe, "axis").as_deref() != Some(pipe_axis(direction))
    {
        return None;
    }

    // Only the three intended adjacent pairs can be fluid connections. Refuse
    // side branches/loops and any other exposed native fluid target, including
    // an ordinary tank or cauldron. Check chunk loading before every lookup.
    let dimension = game.dim.id();
    for (index, &pos) in positions.iter().enumerate() {
        for side in Direction::ALL {
            let neighbor = displaced(pos, side, 1)?;
            if !loaded(game, neighbor) {
                return None;
            }
            if positions
                .iter()
                .enumerate()
                .any(|(other, &inside)| inside == neighbor && index.abs_diff(other) == 1)
            {
                continue;
            }
            let block = game.native_block(neighbor);
            if [TANK, PUMP, PIPE, BASIN].contains(&block.as_str())
                || game
                    .native_fluid_view(dimension, neighbor, Some(side.opposite()))
                    .is_some()
            {
                return None;
            }
        }
    }
    let mixer = displaced(basin, Direction::Up, 2)?;
    if !loaded(game, mixer) || game.native_block(mixer) == "create:mechanical_mixer" {
        return None;
    }
    Some(line)
}

fn snapshot_tank(game: &Game<'_>, pos: Pos) -> Option<Snapshot> {
    if !loaded(game, pos) || game.native_block(pos) != TANK {
        return None;
    }
    let BlockEntity::Mod(entity) = game.world.block_entities.get(&(pos[0], pos[1], pos[2]))? else {
        return None;
    };
    if entity.id != TANK || !entity.items.is_empty() {
        return None;
    }
    let tank = tank_codec::read(&entity.nbt).ok()?;
    if game.native_block_property(pos, "bottom").as_deref() != Some("true")
        || game.native_block_property(pos, "top").as_deref() != Some("true")
        || game.native_block_property(pos, "shape").as_deref()
            != Some(if tank.window { "window" } else { "plain" })
    {
        return None;
    }
    Some(Snapshot {
        nbt: entity.nbt.clone(),
        items: entity.items.clone(),
        revision: entity.revision(),
        amount: tank.amount,
    })
}

fn snapshot_basin(game: &Game<'_>, pos: Pos) -> Option<Snapshot> {
    if !loaded(game, pos) || game.native_block(pos) != BASIN {
        return None;
    }
    let BlockEntity::Mod(entity) = game.world.block_entities.get(&(pos[0], pos[1], pos[2]))? else {
        return None;
    };
    if entity.id != BASIN
        || entity.items.iter().any(Option::is_some)
        || super::create_port_basin::unsupported(game, pos, entity).is_some()
    {
        return None;
    }
    let input = basin_codec::read_half(entity.nbt.get(&"InputTanks".into())).ok()?;
    let output = basin_codec::read_half(entity.nbt.get(&"OutputTanks".into())).ok()?;
    if input[1].is_some() || output.iter().any(Option::is_some) {
        return None;
    }
    let amount = match &input[0] {
        None => 0,
        Some(fluid) if fluid.key.id == "minecraft:water" && fluid.key.components.is_empty() => {
            fluid.amount
        }
        _ => return None,
    };
    Some(Snapshot {
        nbt: entity.nbt.clone(),
        items: entity.items.clone(),
        revision: entity.revision(),
        amount,
    })
}

fn view_amount(view: &NativeFluidTankView) -> Option<u32> {
    match &view.content {
        None => Some(0),
        Some(stack) if stack.fluid == "minecraft:water" && stack.components.is_empty() => {
            u32::try_from(stack.amount).ok()
        }
        _ => None,
    }
}

fn views_match(game: &Game<'_>, line: Line, tank: &Snapshot, basin: &Snapshot) -> bool {
    let dimension = game.dim.id();
    let Some(source) = game.native_fluid_view(dimension, line.tank, Some(line.facing)) else {
        return false;
    };
    let Some(target) = game.native_fluid_view(
        dimension,
        line.basin,
        Some(line.facing.opposite()),
    ) else {
        return false;
    };
    source.len() == 1
        && source[0].tank == 0
        && source[0].can_drain
        && source[0].capacity == tank_codec::CAPACITY as i32
        && view_amount(&source[0]) == Some(tank.amount)
        && target.len() == 4
        && target[2].tank == 2
        && target[2].can_fill
        && target[2].capacity == 1_000
        && view_amount(&target[2]) == Some(basin.amount)
        && [0, 1, 3].into_iter().all(|index| view_amount(&target[index]) == Some(0))
}

fn same_snapshot(before: &Snapshot, after: &Snapshot) -> bool {
    before.revision.same_entity(after.revision)
        && before.nbt == after.nbt
        && before.items == after.items
        && before.amount == after.amount
}

fn unchanged_simulation(before: &Snapshot, after: &Snapshot) -> bool {
    same_snapshot(before, after) && before.revision == after.revision
}

fn only_expected_nbt_change(before: &Snapshot, after: &Snapshot, key: &str) -> bool {
    if !before.revision.same_entity(after.revision) || before.items != after.items {
        return false;
    }
    let mut old = before.nbt.clone();
    let mut new = after.nbt.clone();
    old.remove(&key.into());
    new.remove(&key.into());
    old == new
}

fn restore_one(game: &mut Game<'_>, pos: Pos, block: &str, saved: &Snapshot) -> bool {
    if !loaded(game, pos) || game.native_block(pos) != block {
        return false;
    }
    let done = game
        .native_with_mod_block_entity(pos, |entity| {
            if entity.id != block || !entity.revision().same_entity(saved.revision) {
                return false;
            }
            entity.nbt = saved.nbt.clone();
            entity.items = saved.items.clone();
            true
        })
        .unwrap_or(false);
    if done {
        game.native_block_entity_sync(pos);
    }
    done
}

fn restore_or_fault(game: &mut Game<'_>, line: Line, tank: &Snapshot, basin: &Snapshot) {
    // Attempt both restorations even if the first fails. A callback can have
    // changed either owner despite its returned amount.
    let source = restore_one(game, line.tank, TANK, tank);
    let target = restore_one(game, line.basin, BASIN, basin);
    let verified = checked_line(game, line.pump).is_some_and(|current| {
        current.tank == line.tank && current.pipe == line.pipe && current.basin == line.basin
    }) && match (
        snapshot_tank(game, line.tank),
        snapshot_basin(game, line.basin),
    ) {
        (Some(actual_tank), Some(actual_basin)) => {
            same_snapshot(tank, &actual_tank)
                && same_snapshot(basin, &actual_basin)
                && (Volumes {
                    tank: actual_tank.amount,
                    basin_first_input: actual_basin.amount,
                })
                .restored_to(Volumes {
                    tank: tank.amount,
                    basin_first_input: basin.amount,
                })
        }
        _ => false,
    };
    if !(source && target && verified) {
        panic!("FLUIDS-005 hard custody fault: Tank/Basin restoration could not verify");
    }
}

fn water(amount: u32) -> NativeFluidStack {
    NativeFluidStack {
        fluid: "minecraft:water".into(),
        amount: amount as i32,
        components: Compound::new(),
    }
}

fn drained_amount(stack: Option<NativeFluidStack>) -> Option<u32> {
    let stack = stack?;
    if stack.fluid != "minecraft:water" || !stack.components.is_empty() {
        return None;
    }
    u32::try_from(stack.amount).ok()
}

/// Called from the pump's ordinary server tick after the accepted kinetic
/// network has supplied its signed speed. The ticking pump BE is detached;
/// this function deliberately reads only its block state and the two endpoint
/// BEs still in the world map. No pipe or pump BE stores bulk fluid.
pub(super) fn tick(game: &mut Game<'_>, pump: Pos, rpm: f32) -> TickResult {
    if game.native_window() {
        return TickResult::Refused;
    }
    let Some(line) = checked_line(game, pump) else {
        return TickResult::Refused;
    };
    let (Some(tank), Some(basin)) = (
        snapshot_tank(game, line.tank),
        snapshot_basin(game, line.basin),
    ) else {
        return TickResult::Refused;
    };
    if !views_match(game, line, &tank, &basin) {
        return TickResult::Refused;
    }
    let before = Volumes {
        tank: tank.amount,
        basin_first_input: basin.amount,
    };
    let Some(maximum) = Step::maximum(rpm, before) else {
        return TickResult::Refused;
    };
    let dimension = game.dim.id().to_owned();
    let target_face = Some(line.facing.opposite());
    let source_face = Some(line.facing);
    let accepted = game.native_fluid_fill_stack(
        &dimension,
        line.basin,
        target_face,
        &water(maximum),
        NativeFluidAction::Simulate,
    );
    let source_simulated = (accepted > 0 && accepted <= maximum as i32).then(|| {
        game.native_fluid_drain_stack(
            &dimension,
            line.tank,
            source_face,
            &NativeFluidDrain::Exact(water(accepted as u32)),
            NativeFluidAction::Simulate,
        )
    });
    let simulated = Step::from_simulation(
        before,
        maximum,
        accepted,
        drained_amount(source_simulated.flatten()),
    );
    let still_unchanged = snapshot_tank(game, line.tank)
        .is_some_and(|actual| unchanged_simulation(&tank, &actual))
        && snapshot_basin(game, line.basin)
            .is_some_and(|actual| unchanged_simulation(&basin, &actual));
    if !still_unchanged {
        log::warn!("Create pump transfer: endpoint simulation changed owner state; restoring");
        restore_or_fault(game, line, &tank, &basin);
        return TickResult::Restored;
    }
    let Some(step) = simulated else {
        return TickResult::Refused;
    };

    let drained = drained_amount(game.native_fluid_drain_stack(
        &dimension,
        line.tank,
        source_face,
        &NativeFluidDrain::Exact(water(step.amount)),
        NativeFluidAction::Execute,
    ));
    let filled = if drained == Some(step.amount) {
        game.native_fluid_fill_stack(
            &dimension,
            line.basin,
            target_face,
            &water(step.amount),
            NativeFluidAction::Execute,
        )
    } else {
        0
    };
    let after = (snapshot_tank(game, line.tank), snapshot_basin(game, line.basin));
    let committed = if let (Some(source), Some(target)) = &after {
        step.committed(
            Volumes {
                tank: source.amount,
                basin_first_input: target.amount,
            },
            drained,
            filled,
        ) && only_expected_nbt_change(&tank, source, "TankContent")
            && only_expected_nbt_change(&basin, target, "InputTanks")
            && views_match(game, line, source, target)
    } else {
        false
    };
    if committed {
        TickResult::Moved(step.amount)
    } else {
        log::warn!("Create pump transfer: endpoint receipt/census mismatch; restoring");
        restore_or_fault(game, line, &tank, &basin);
        TickResult::Restored
    }
}
