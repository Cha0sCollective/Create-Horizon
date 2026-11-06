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

#[cfg(test)]
thread_local! {
    // A one-shot receipt fault after the real Basin Execute callback. No
    // production setting, command, environment knob or replacement provider.
    static ZERO_BASIN_EXECUTE_RECEIPT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static OBSERVED_REAL_EXECUTE: std::cell::Cell<Option<(i32, Volumes)>> = const { std::cell::Cell::new(None) };
}

fn execute_basin_fill(game: &mut Game<'_>, line: Line, dimension: &str, amount: u32) -> i32 {
    let receipt = game.native_fluid_fill_stack(
        dimension,
        line.basin,
        Some(line.facing.opposite()),
        &water(amount),
        NativeFluidAction::Execute,
    );
    #[cfg(test)]
    if ZERO_BASIN_EXECUTE_RECEIPT.with(|fault| fault.replace(false)) {
        let source = snapshot_tank(game, line.tank).expect("real source after Basin Execute");
        let target = snapshot_basin(game, line.basin).expect("real target after Basin Execute");
        OBSERVED_REAL_EXECUTE.with(|observed| {
            observed.set(Some((
                receipt,
                Volumes {
                    tank: source.amount,
                    basin_first_input: target.amount,
                },
            )));
        });
        return 0;
    }
    receipt
}

/// Called from the pump's ordinary server tick after the accepted kinetic
/// network has supplied its signed speed. The ticking pump BE is detached;
/// this function deliberately reads only its block state and the two endpoint
/// BEs still in the world map. No pipe or pump BE stores bulk fluid.
pub(super) fn tick(game: &mut Game<'_>, pump: Pos, rpm: f32) -> TickResult {
    // Stopped, nonfinite and out-of-range speed is ordinary refusal: no
    // endpoint callback runs and the restoration/fail-stop path is unreachable.
    if game.native_window() || create_engine::pump_transfer::rate(rpm) == 0 {
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
        execute_basin_fill(game, line, &dimension, step.amount)
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

#[cfg(test)]
mod restoration_tests {
    use super::*;
    use crate::game::{Audio, Options, Progress, Settings, load_resources};
    use horizon_client::{chunk::Chunk, containers::ModBlockEntity, world::World};
    use std::sync::Arc;

    const SOURCE: Pos = [1, 64, 1];
    const MACHINE: Pos = [2, 64, 1];
    const STRAIGHT: Pos = [3, 64, 1];
    const TARGET: Pos = [4, 64, 1];

    struct ClearReceiptFault;
    impl Drop for ClearReceiptFault {
        fn drop(&mut self) {
            ZERO_BASIN_EXECUTE_RECEIPT.with(|fault| fault.set(false));
            OBSERVED_REAL_EXECUTE.with(|observed| observed.set(None));
        }
    }

    // Same prepared-resource Game bootstrap used by Horizon's ignored
    // state_block_entity_tests. HORIZON_TEST_RESOURCES must be the composed
    // private Create-enabled resource root, not vanilla-only prepared assets.
    // The IDs/states and native Tank/Basin callbacks are the actual installed
    // Create content and hooks, not synthetic fixture registrations.
    fn with_game(run: impl FnOnce(&mut Game<'_>)) {
        let root = std::path::PathBuf::from(
            std::env::var_os("HORIZON_TEST_RESOURCES")
                .expect("prepared Create-enabled 26.3 test resources required"),
        );
        assert!(
            std::env::var("HORIZON_API_DISCOVERY")
                .unwrap_or_default()
                .starts_with("http://127.0.0.1:"),
            "headless fixture needs local-only discovery"
        );
        let settings = Settings::default();
        let loaded = load_resources(
            root.clone(),
            None,
            &root,
            &settings,
            std::time::Instant::now(),
            false,
            true,
            &Progress::new(),
            &mut |_, _| unreachable!("headless fixture has no menu"),
        )
        .unwrap();
        let registry = loaded.pack.assets.registry;
        let states = [TANK, PUMP, PIPE, BASIN]
            .map(|id| registry.default_state(id).expect("selected Create block state"));
        let mut options = Options::parse(std::iter::empty()).unwrap().unwrap();
        options.resources = root.clone();
        options.execution_role = crate::game::execution_role::ExecutionRole::IntegratedServer;
        options.split_guest = true;
        let mut game = Game::new(
            &loaded.resources,
            loaded.pack,
            loaded.gui.unwrap(),
            options,
            settings,
            Audio::silent(&root),
            None,
        )
        .unwrap();
        // The headless integrated-server fixture has no live serving handle;
        // native fluid APIs require the same explicit server-simulation flag
        // used by the pinned host's ignored Game tests.
        game.options.dedicated = Some(Vec::new());
        game.world = World::new(game.res.facts().air, 382);
        game.world
            .chunks
            .insert((0, 0), Arc::new(Chunk::empty(0, 0)));
        game.dim = crate::game::Dim::Overworld;
        for (pos, state) in [SOURCE, MACHINE, STRAIGHT, TARGET]
            .into_iter()
            .zip(states.iter().copied())
        {
            game.world.set(pos[0], pos[1], pos[2], state).unwrap();
        }
        let custom_fluids = game.native_custom_fluid_states();
        assert!(custom_fluids[states[0].0 as usize]);
        assert!(custom_fluids[states[3].0 as usize]);
        game.world.block_entities.set_custom_fluids(custom_fluids);
        let mut tank = ModBlockEntity::default();
        tank.id = TANK.into();
        tank.nbt = tank_codec::initial();
        assert!(tank_codec::set_amount(&mut tank.nbt, 1_000).unwrap());
        game.world
            .block_entities
            .insert((SOURCE[0], SOURCE[1], SOURCE[2]), BlockEntity::Mod(Box::new(tank)));
        let mut basin = ModBlockEntity::default();
        basin.id = BASIN.into();
        basin.items = vec![None; 18];
        crate::game::create_port_basin::initialize(&mut basin);
        game.world
            .block_entities
            .insert((TARGET[0], TARGET[1], TARGET[2]), BlockEntity::Mod(Box::new(basin)));
        game.world.modified.clear();
        game.world.block_log = Some(Vec::new());
        assert_eq!(snapshot_tank(&game, SOURCE).unwrap().amount, 1_000);
        assert_eq!(snapshot_basin(&game, TARGET).unwrap().amount, 0);
        assert!(views_match(
            &game,
            checked_line(&game, MACHINE).unwrap(),
            &snapshot_tank(&game, SOURCE).unwrap(),
            &snapshot_basin(&game, TARGET).unwrap()
        ));
        run(&mut game);
    }

    #[test]
    #[ignore = "prepared resources; run alone because Game/bootstrap uses process-wide content"]
    fn pump_execute_basin_zero_receipt_restores_real_owners() {
        let _clear_receipt_fault = ClearReceiptFault;
        with_game(|game| {
            crate::game::create_port_basin::reset(false);
            let tank_before = snapshot_tank(game, SOURCE).unwrap();
            let basin_before = snapshot_basin(game, TARGET).unwrap();
            assert_eq!(
                Step::maximum(
                    16.0,
                    Volumes {
                        tank: 1_000,
                        basin_first_input: 0,
                    }
                ),
                Some(8)
            );
            ZERO_BASIN_EXECUTE_RECEIPT.with(|fault| fault.set(true));
            OBSERVED_REAL_EXECUTE.with(|observed| observed.set(None));
            assert_eq!(tick(game, MACHINE, 16.0), TickResult::Restored);
            assert_eq!(
                OBSERVED_REAL_EXECUTE.with(|observed| observed.replace(None)),
                Some((
                    8,
                    Volumes {
                        tank: 992,
                        basin_first_input: 8,
                    }
                ))
            );
            assert!(!ZERO_BASIN_EXECUTE_RECEIPT.with(|fault| fault.get()));
            let tank_after = snapshot_tank(game, SOURCE).unwrap();
            let basin_after = snapshot_basin(game, TARGET).unwrap();
            assert_eq!(tank_after.nbt, tank_before.nbt);
            assert_eq!(tank_after.items, tank_before.items);
            assert_eq!(basin_after.nbt, basin_before.nbt);
            assert_eq!(basin_after.items, basin_before.items);
            assert_eq!((tank_after.amount, basin_after.amount), (1_000, 0));

            // The real Basin Execute raised contents_changed before restoration.
            // Its next ordinary tick consumes that flag; no Mixer exists at +2Y.
            assert_ne!(
                game.native_block([TARGET[0], TARGET[1] + 2, TARGET[2]]),
                "create:mechanical_mixer"
            );
            let key = (TARGET[0], TARGET[1], TARGET[2]);
            let Some(BlockEntity::Mod(mut detached)) = game.world.block_entities.remove(&key) else {
                panic!("Basin entity missing before ordinary tick");
            };
            crate::game::create_port_basin::tick(game, TARGET, &mut detached);
            game.world.block_entities.insert(key, BlockEntity::Mod(detached));
            let basin_after_tick = snapshot_basin(game, TARGET).unwrap();
            assert_eq!(basin_after_tick.amount, 0);
            assert!(basin_after_tick.items.iter().all(Option::is_none));
            assert_eq!(snapshot_tank(game, SOURCE).unwrap().amount, 1_000);
        });
    }
}
