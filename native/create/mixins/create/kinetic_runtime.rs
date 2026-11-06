// SPDX-License-Identifier: MIT
// Per-server-tick connected-component cache. Reading only loaded blocks never
// tickets chunks; the two halves of an open clutch get separate cache entries.
use super::Game;
use create_engine::kinetics::{self, Network, Node, Pos};
use create_engine::propagation::{self, Propagation};
use horizon_world::nbt::{Compound, Tag};
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, VecDeque},
    rc::Rc,
};
#[derive(Default)]
struct Cache {
    world: Option<(usize, i64, String)>,
    components: BTreeMap<(Pos, u8), Rc<Network>>,
}
thread_local! { static CACHE: RefCell<Cache> = RefCell::new(Cache::default()); }

#[derive(Default)]
struct Level {
    tick: Option<i64>,
    previous: BTreeMap<Pos, Node>,
    rotation: Propagation,
}
#[derive(Default)]
struct Levels {
    owner: usize,
    dimensions: BTreeMap<String, Level>,
}
thread_local! {
    static LEVELS: RefCell<Levels> = RefCell::new(Levels::default());
    static CHANGED: RefCell<VecDeque<(&'static str, Pos)>> = const { RefCell::new(VecDeque::new()) };
}

pub(super) fn changed(dimension: &'static str, pos: Pos) {
    CHANGED.with(|cell| {
        let mut pending = cell.borrow_mut();
        // Client notifications also reach this hook. Bound their unused queue;
        // the authoritative snapshot catches edits absent from the queue.
        if pending.len() == kinetics::MAX_NODES * 16 {
            pending.pop_front();
        }
        pending.push_back((dimension, pos));
    });
    invalidate();
}

fn supported(game: &Game<'_>, pos: Pos) -> bool {
    matches!(
        game.native_block(pos).as_str(),
        "create:creative_motor"
            | "create:shaft"
            | "create:cogwheel"
            | "create:large_cogwheel"
            | "create:millstone"
    )
}

/// Apply source transitions before native block-entity ticks detach their entity
/// from the world's map. Read loaded nodes only; never acquire chunk tickets.
pub(super) fn level_tick(game: &mut Game<'_>) {
    if !game.native_on("create") || (game.serving.is_none() && game.options.dedicated.is_none()) {
        return;
    }
    let dimension = game.dim.id().to_owned();
    let events = CHANGED.with(|cell| {
        let mut queue = cell.borrow_mut();
        let mut events = Vec::new();
        queue.retain(|(dim, pos)| {
            if *dim == dimension {
                events.push(*pos);
                false
            } else {
                true
            }
        });
        events
    });
    let mut actual: BTreeMap<_, _> = game
        .world
        .block_entities
        .keys()
        .filter_map(|&(x, y, z)| {
            let pos = [x, y, z];
            crate::game::create_port::node(game, pos).map(|node| (pos, node))
        })
        .collect();
    LEVELS.with(|cell| {
        let mut levels = cell.borrow_mut();
        let owner = game as *const Game<'_> as usize;
        if levels.owner != owner {
            levels.owner = owner;
            levels.dimensions.clear();
        }
        let level = levels.dimensions.entry(dimension).or_default();
        if level
            .tick
            .is_some_and(|last| game.world.game_time != last + 1)
        {
            *level = Level::default();
        }
        level.tick = Some(game.world.game_time);
        let mut destroyed = Vec::<Node>::new();
        if level.previous != actual || !events.is_empty() {
            // Other imported machines retain their existing solver. Destructive
            // reference propagation applies only to wholly supported components.
            let mut seen = BTreeSet::new();
            let mut eligible = BTreeSet::new();
            for node in actual.values() {
                if seen.contains(&node.pos) {
                    continue;
                }
                let network = kinetics::solve(*node, |pos| actual.get(&pos).copied());
                seen.extend(network.vertices.iter().map(|(pos, _)| *pos));
                if network.status != kinetics::Status::TooLarge
                    && network.vertices.iter().all(|(pos, _)| {
                        supported(game, *pos)
                            && actual.get(pos).is_some_and(|n| propagation::supported(*n))
                    })
                {
                    eligible.extend(network.vertices.iter().map(|(pos, _)| *pos));
                }
            }
            // A mixed component is unqualified; discard its derived state
            // without treating that transition as destruction or source loss.
            level
                .rotation
                .nodes
                .retain(|pos, _| !actual.contains_key(pos) || eligible.contains(pos));
            let mut order = events;
            // Returning from an unqualified mixed/oversized component needs a
            // fresh derived node even when that block itself did not change.
            order.extend(
                eligible
                    .iter()
                    .filter(|pos| !level.rotation.nodes.contains_key(*pos))
                    .copied(),
            );
            order.extend(
                level
                    .previous
                    .keys()
                    .chain(actual.keys())
                    .filter(|pos| level.previous.get(*pos) != actual.get(*pos))
                    .copied(),
            );
            let mut applied = BTreeSet::new();
            for pos in order {
                if !applied.insert(pos) || destroyed.iter().any(|node| node.pos == pos) {
                    continue;
                }
                if let Some(node) = actual.get(&pos).filter(|_| eligible.contains(&pos)) {
                    level.rotation.upsert(*node);
                } else if !actual.contains_key(&pos) {
                    level.rotation.remove(pos);
                }
                destroyed.extend(level.rotation.take_destroyed());
            }
        }
        level.rotation.tick();
        destroyed.extend(level.rotation.take_destroyed());
        if level.rotation.limited {
            // Do not publish a partial destructive calculation at the work bound.
            log::warn!(
                "Create: kinetic propagation work bound reached; retaining conservative solver"
            );
            level.rotation = Propagation::default();
            level.previous.clear();
            return;
        }
        let mut retry = false;
        for expected in destroyed {
            // A queued notification may have been superseded. Never destroy a
            // different replacement block based on an obsolete calculation.
            if crate::game::create_port::node(game, expected.pos) != Some(expected) {
                retry = true;
                continue;
            }
            if let Err(error) = game.destroy_block_dropping(expected.pos) {
                log::warn!(
                    "Create: kinetic destruction failed at {:?}: {error:?}",
                    expected.pos
                );
            }
            if crate::game::create_port::node(game, expected.pos).is_none() {
                actual.remove(&expected.pos);
            } else {
                retry = true;
            }
            invalidate();
        }
        level.previous = actual;
        if retry {
            level.rotation = Propagation::default();
            level.previous.clear();
        }
    });
}

/// Derived native diagnostics; not a Java NBT or save-format compatibility claim.
pub(super) fn diagnostics(game: &Game<'_>, pos: Pos) -> Option<Compound> {
    LEVELS.with(|cell| {
        let levels = cell.borrow();
        if levels.owner != game as *const Game<'_> as usize {
            return None;
        }
        let state = levels
            .dimensions
            .get(game.dim.id())?
            .rotation
            .nodes
            .get(&pos)?;
        Some(Compound::from([
            ("TheoreticalSpeed".into(), Tag::Float(state.speed.to_bits())),
            (
                "Source".into(),
                Tag::IntArray(state.source.map(Vec::from).unwrap_or_default()),
            ),
            (
                "Root".into(),
                Tag::IntArray(state.root.map(Vec::from).unwrap_or_default()),
            ),
        ]))
    })
}
pub(super) fn invalidate() {
    CACHE.with(|c| c.borrow_mut().components.clear());
}
pub(super) fn network(game: &Game<'_>, start: Node, half: u8) -> Rc<Network> {
    CACHE.with(|cell| {
        let mut cache = cell.borrow_mut();
        let world = (
            game as *const Game<'_> as usize,
            game.world.game_time,
            game.dim.id().to_owned(),
        );
        if cache.world.as_ref() != Some(&world) {
            cache.world = Some(world);
            cache.components.clear();
        }
        if let Some(network) = cache.components.get(&(start.pos, half)) {
            return Rc::clone(network);
        }
        let network = Rc::new(kinetics::solve_half(start, half, |pos| {
            if pos == start.pos {
                Some(start)
            } else {
                crate::game::create_port::node(game, pos)
            }
        }));
        for key in &network.vertices {
            cache.components.insert(*key, Rc::clone(&network));
        }
        network
    })
}

// Compatible with the accepted five-type diagnostics and conservative mixed solver.
pub(super) fn theoretical(entity: &crate::containers::ModBlockEntity) -> f32 {
    if let Some(Tag::Compound(fields)) = entity.nbt.get(&"create_horizon:kinetic".into()) {
        if fields.contains_key(&"TheoreticalSpeed".into()) {
            return super::create_port::number(fields, "TheoreticalSpeed");
        }
    }
    super::create_port::number(&entity.nbt, "Speed")
}
