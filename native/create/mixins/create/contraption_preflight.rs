// Copyright (c) The Create Team / The Creators of Create
// Copyright (c) 2026 Create Horizon contributors
// SPDX-License-Identifier: MIT
//! Read-only native bridge to the staged portable contraption core.
//! It plans no world edit and creates no moving owner or proxy.

use super::Game;
use create_engine::{
    contraption::{
        Adhesion, Assembly, BlockRules, CaptureError, CaptureView, Cell, LandingError, Rotation,
    },
    kinetics::{Axis, Pos},
    placement::Direction,
};
use horizon_world::chunk::BlockState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ReadError {
    OutsideWorld,
    Unloaded,
    InvalidState,
}

/// Narrow seam so the preflight decisions can be checked without a game run.
trait WorldRead {
    fn authoritative(&self) -> bool;
    fn block(&self, pos: Pos) -> Result<BlockState, ReadError>;
    fn has_block_entity(&self, pos: Pos) -> bool;
}

struct GameRead<'a, 'g>(&'a Game<'g>);

impl WorldRead for GameRead<'_, '_> {
    fn authoritative(&self) -> bool {
        !self.0.options.execution_role.is_client()
    }

    fn block(&self, [x, y, z]: Pos) -> Result<BlockState, ReadError> {
        let world = &self.0.world;
        if !(-30_000_000..30_000_000).contains(&x)
            || !(-30_000_000..30_000_000).contains(&z)
            || !(world.shape.min_y..world.shape.max_y()).contains(&y)
        {
            return Err(ReadError::OutsideWorld);
        }
        if !world.loaded(x, z) {
            return Err(ReadError::Unloaded);
        }
        self.0
            .pack
            .assets
            .registry
            .state(world.block(x, y, z))
            .cloned()
            .ok_or(ReadError::InvalidState)
    }

    fn has_block_entity(&self, [x, y, z]: Pos) -> bool {
        self.0.world.block_entities.contains_key(&(x, y, z))
    }
}

fn same_chunk(a: Pos, b: Pos) -> bool {
    a[0] >> 4 == b[0] >> 4 && a[2] >> 4 == b[2] >> 4
}

fn log_axis(state: &BlockState) -> Option<Axis> {
    if state.name != "minecraft:oak_log" || state.properties.len() != 1 {
        return None;
    }
    match state.properties.get("axis").map(String::as_str) {
        Some("x") => Some(Axis::X),
        Some("y") => Some(Axis::Y),
        Some("z") => Some(Axis::Z),
        _ => None,
    }
}

fn supported_rules(state: &BlockState) -> Option<BlockRules> {
    match state.name.as_str() {
        "minecraft:slime_block" if state.properties.is_empty() => Some(BlockRules {
            adhesion: Adhesion::Slime,
            ..BlockRules::default()
        }),
        "minecraft:oak_log" if log_axis(state).is_some() => Some(BlockRules::default()),
        _ => None,
    }
}

struct NativeCaptureView<'a, W> {
    world: &'a W,
    controller: Pos,
}

impl<W: WorldRead> CaptureView for NativeCaptureView<'_, W> {
    type Block = BlockState;

    fn inspect(&self, pos: Pos) -> Cell {
        if !same_chunk(self.controller, pos) {
            return Cell::Unloaded;
        }
        match self.world.block(pos) {
            Err(ReadError::OutsideWorld) => Cell::OutsideWorld,
            Err(ReadError::Unloaded) => Cell::Unloaded,
            Err(ReadError::InvalidState) => Cell::Unsupported,
            Ok(_) if self.world.has_block_entity(pos) => Cell::Unsupported,
            Ok(state) if state.name == "minecraft:air" && state.properties.is_empty() => {
                Cell::Empty
            }
            Ok(state) => supported_rules(&state).map_or(Cell::Unsupported, Cell::Block),
        }
    }

    fn snapshot(&self, pos: Pos) -> Option<BlockState> {
        matches!(self.inspect(pos), Cell::Block(_))
            .then(|| self.world.block(pos).ok())
            .flatten()
    }

    fn glued(&self, _pos: Pos, _face: Direction) -> bool {
        false
    }
}

fn rotated_state(
    state: &BlockState,
    turn: create_engine::contraption::QuarterTurn,
) -> Option<BlockState> {
    let mut state = state.clone();
    if state.name == "minecraft:oak_log" {
        let axis = turn.axis(log_axis(&state)?);
        let name = match axis {
            Axis::X => "x",
            Axis::Y => "y",
            Axis::Z => "z",
        };
        state.properties.insert("axis".into(), name.into());
    }
    Some(state)
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum PreflightError {
    NotAuthoritative,
    AxisMismatch,
    CoordinateOverflow,
    Controller(ReadError),
    InvalidController,
    Capture(CaptureError),
    WrongShape,
    InvalidAngle,
    Landing(LandingError),
}

/// Ephemeral evidence for a future bearing binding. Neither vector is custody.
#[derive(Debug, PartialEq)]
pub(super) struct Preflight {
    pub(super) sources: Vec<(Pos, BlockState)>,
    pub(super) landing: Vec<(Pos, BlockState)>,
    pub(super) centers: Vec<[f64; 3]>,
}

fn plan_scene(
    world: &impl WorldRead,
    controller: Pos,
    facing: Direction,
    axis: Axis,
    degrees: f32,
) -> Result<Preflight, PreflightError> {
    if !world.authoritative() {
        return Err(PreflightError::NotAuthoritative);
    }
    if facing.axis() != axis {
        return Err(PreflightError::AxisMismatch);
    }
    let controller_state = world
        .block(controller)
        .map_err(PreflightError::Controller)?;
    if controller_state.name != "create:mechanical_bearing"
        || controller_state
            .properties
            .get("facing")
            .map(String::as_str)
            != Some(facing.name())
        || !world.has_block_entity(controller)
    {
        return Err(PreflightError::InvalidController);
    }
    let step = facing.vector();
    let anchor = [
        controller[0]
            .checked_add(step[0])
            .ok_or(PreflightError::CoordinateOverflow)?,
        controller[1]
            .checked_add(step[1])
            .ok_or(PreflightError::CoordinateOverflow)?,
        controller[2]
            .checked_add(step[2])
            .ok_or(PreflightError::CoordinateOverflow)?,
    ];
    let view = NativeCaptureView { world, controller };
    let assembly = Assembly::capture(&view, anchor, Some(controller), None, 2)
        .map_err(PreflightError::Capture)?;
    let blocks = assembly.blocks();
    if blocks.len() != 2
        || !blocks
            .iter()
            .any(|(local, state)| *local == [0, 0, 0] && state.name == "minecraft:slime_block")
        || !blocks.iter().any(|(local, state)| {
            Direction::ALL.iter().any(|face| face.vector() == *local) && log_axis(state).is_some()
        })
    {
        return Err(PreflightError::WrongShape);
    }
    let sources: Vec<_> = blocks
        .iter()
        .map(|(local, state)| {
            let pos = [
                anchor[0].checked_add(local[0])?,
                anchor[1].checked_add(local[1])?,
                anchor[2].checked_add(local[2])?,
            ];
            Some((pos, state.clone()))
        })
        .collect::<Option<_>>()
        .ok_or(PreflightError::CoordinateOverflow)?;
    let rotation = Rotation::new(anchor, axis, degrees).ok_or(PreflightError::InvalidAngle)?;
    let centers = blocks
        .iter()
        .map(|(local, _)| {
            rotation.to_world([
                f64::from(local[0]) + 0.5,
                f64::from(local[1]) + 0.5,
                f64::from(local[2]) + 0.5,
            ])
        })
        .collect();
    let landing = assembly
        .landing(anchor, rotation.snapped(), rotated_state, |pos, _| {
            if !same_chunk(pos, controller) || world.has_block_entity(pos) {
                return false;
            }
            match world.block(pos) {
                Ok(state) if state.name == "minecraft:air" && state.properties.is_empty() => true,
                Ok(state) => sources
                    .iter()
                    .any(|(source, original)| *source == pos && *original == state),
                Err(_) => false,
            }
        })
        .map_err(PreflightError::Landing)?;
    Ok(Preflight {
        sources,
        landing,
        centers,
    })
}

/// Called only from a future read-only game hook, after root wires the module.
pub(super) fn bearing_preflight(
    game: &Game<'_>,
    controller: Pos,
    facing: Direction,
    axis: Axis,
    degrees: f32,
) -> Result<Preflight, PreflightError> {
    plan_scene(&GameRead(game), controller, facing, axis, degrees)
}

#[cfg(test)]
mod tests {
    use super::*;
    use horizon_world::chunk::BlockProperties;
    use std::collections::{HashMap, HashSet};

    const CONTROLLER: Pos = [8, 63, 8];
    const ROOT: Pos = [8, 64, 8];
    const LOG: Pos = [8, 64, 9];

    struct Scene {
        cells: HashMap<Pos, BlockState>,
        blocked: HashSet<Pos>,
        read_error: Option<(Pos, ReadError)>,
        authoritative: bool,
    }

    impl Default for Scene {
        fn default() -> Self {
            Self {
                cells: HashMap::new(),
                blocked: HashSet::new(),
                read_error: None,
                authoritative: true,
            }
        }
    }

    impl WorldRead for Scene {
        fn authoritative(&self) -> bool {
            self.authoritative
        }
        fn block(&self, pos: Pos) -> Result<BlockState, ReadError> {
            if let Some((at, error)) = self.read_error {
                if at == pos {
                    return Err(error);
                }
            }
            Ok(self
                .cells
                .get(&pos)
                .cloned()
                .unwrap_or_else(|| state("minecraft:air")))
        }
        fn has_block_entity(&self, pos: Pos) -> bool {
            self.blocked.contains(&pos)
        }
    }

    fn state(name: &str) -> BlockState {
        BlockState {
            name: name.into(),
            properties: BlockProperties::new(),
        }
    }

    fn scene() -> Scene {
        let mut scene = Scene::default();
        let mut bearing = state("create:mechanical_bearing");
        bearing.properties.insert("facing".into(), "up".into());
        scene.cells.insert(CONTROLLER, bearing);
        scene.blocked.insert(CONTROLLER);
        scene.cells.insert(ROOT, state("minecraft:slime_block"));
        let mut log = state("minecraft:oak_log");
        log.properties.insert("axis".into(), "z".into());
        scene.cells.insert(LOG, log);
        scene
    }

    #[test]
    fn actual_graph_and_rotation_share_two_snapshots_without_writes() {
        let scene = scene();
        let plan = plan_scene(&scene, CONTROLLER, Direction::Up, Axis::Y, 90.0).unwrap();
        assert_eq!(plan.sources.len(), 2);
        assert_eq!(plan.sources[0].0, ROOT);
        assert_eq!(plan.sources[1].0, LOG);
        assert_eq!(plan.landing[0].0, ROOT);
        assert_eq!(plan.landing[1].0, [9, 64, 8]);
        assert_eq!(
            plan.landing[1].1.properties.get("axis").map(String::as_str),
            Some("x")
        );
        assert_eq!(plan.centers[0], [8.5, 64.5, 8.5]);
        assert_eq!(scene.cells.len(), 3);
    }

    #[test]
    fn unknown_block_entity_unloaded_and_outside_cells_refuse_capture() {
        let mut scene = scene();
        scene
            .cells
            .insert([8, 65, 8], state("minecraft:diamond_block"));
        assert_eq!(
            plan_scene(&scene, CONTROLLER, Direction::Up, Axis::Y, 90.0),
            Err(PreflightError::Capture(CaptureError::Unsupported([
                8, 65, 8
            ])))
        );
        scene.cells.remove(&[8, 65, 8]);
        scene.blocked.insert(LOG);
        assert_eq!(
            plan_scene(&scene, CONTROLLER, Direction::Up, Axis::Y, 90.0),
            Err(PreflightError::Capture(CaptureError::Unsupported(LOG)))
        );
        scene.blocked.remove(&LOG);
        for (error, expected) in [
            (ReadError::Unloaded, CaptureError::Unloaded(LOG)),
            (ReadError::OutsideWorld, CaptureError::OutsideWorld(LOG)),
        ] {
            scene.read_error = Some((LOG, error));
            assert_eq!(
                plan_scene(&scene, CONTROLLER, Direction::Up, Axis::Y, 90.0),
                Err(PreflightError::Capture(expected))
            );
        }
    }

    #[test]
    fn empty_remote_misaligned_and_third_block_scenes_do_not_plan() {
        let mut empty = scene();
        empty.cells.remove(&ROOT);
        empty.cells.remove(&LOG);
        assert_eq!(
            plan_scene(&empty, CONTROLLER, Direction::Up, Axis::Y, 90.0),
            Err(PreflightError::WrongShape)
        );
        let mut scene = scene();
        scene.authoritative = false;
        assert_eq!(
            plan_scene(&scene, CONTROLLER, Direction::Up, Axis::Y, 90.0),
            Err(PreflightError::NotAuthoritative)
        );
        scene.authoritative = true;
        assert_eq!(
            plan_scene(&scene, CONTROLLER, Direction::Up, Axis::X, 90.0),
            Err(PreflightError::AxisMismatch)
        );
        let mut log = state("minecraft:oak_log");
        log.properties.insert("axis".into(), "z".into());
        scene.cells.insert([8, 64, 7], log);
        assert_eq!(
            plan_scene(&scene, CONTROLLER, Direction::Up, Axis::Y, 90.0),
            Err(PreflightError::Capture(CaptureError::TooLarge { limit: 2 }))
        );
    }

    #[test]
    fn controller_identity_facing_and_entity_are_required() {
        let mut scene = scene();
        scene.cells.insert(CONTROLLER, state("minecraft:stone"));
        assert_eq!(
            plan_scene(&scene, CONTROLLER, Direction::Up, Axis::Y, 90.0),
            Err(PreflightError::InvalidController)
        );
        let mut bearing = state("create:mechanical_bearing");
        bearing.properties.insert("facing".into(), "north".into());
        scene.cells.insert(CONTROLLER, bearing);
        assert_eq!(
            plan_scene(&scene, CONTROLLER, Direction::Up, Axis::Y, 90.0),
            Err(PreflightError::InvalidController)
        );
        scene
            .cells
            .get_mut(&CONTROLLER)
            .unwrap()
            .properties
            .insert("facing".into(), "up".into());
        scene.blocked.remove(&CONTROLLER);
        assert_eq!(
            plan_scene(&scene, CONTROLLER, Direction::Up, Axis::Y, 90.0),
            Err(PreflightError::InvalidController)
        );
    }

    #[test]
    fn crossing_a_chunk_edge_is_an_explicit_refusal() {
        let mut scene = Scene::default();
        let controller = [8, 63, 0];
        let mut bearing = state("create:mechanical_bearing");
        bearing.properties.insert("facing".into(), "up".into());
        scene.cells.insert(controller, bearing);
        scene.blocked.insert(controller);
        scene
            .cells
            .insert([8, 64, 0], state("minecraft:slime_block"));
        let mut log = state("minecraft:oak_log");
        log.properties.insert("axis".into(), "z".into());
        scene.cells.insert([8, 64, 1], log);
        assert_eq!(
            plan_scene(&scene, controller, Direction::Up, Axis::Y, 90.0),
            Err(PreflightError::Capture(CaptureError::Unloaded([8, 64, -1])))
        );
    }
}
