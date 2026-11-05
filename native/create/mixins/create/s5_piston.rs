// Copyright (c) 2026 Create Horizon contributors
// SPDX-License-Identifier: MIT
// One normal mechanical piston, one initially placed pole, one-cell eastward
// travel. The piston machine owns absent stone/pole payloads; a moving head is
// derived geometry, never a separately saved vanilla moving-piston owner.
use super::Game;
use crate::{
    containers::{BlockEntity, ModBlockEntity},
    items::ItemStack,
};
use create_engine::linear_carrier::{carrier_step, eastward_delta, reversal_offset};
use create_engine::portable_interface::Connection;
use horizon_core::{
    geometry::{Aabb, Direction, Vec3},
    position::BlockPos,
};
use horizon_world::{
    level::pistons::MovingPiston,
    nbt::{Compound, JavaString, Tag},
};

const KEY: &str = "create_horizon:s5_piston";
const BASE: &str = "create:mechanical_piston";
const POLE: &str = "create:piston_extension_pole";
const HEAD: &str = "create:mechanical_piston_head";
const STONE: &str = "minecraft:stone";
const DEPOT: &str = "create:depot";
const INTERFACE: &str = "create:portable_storage_interface";
const INGOT: &str = "minecraft:iron_ingot";
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cargo {
    Stone,
    Depot,
    Portable,
}

impl Cargo {
    fn block(self) -> &'static str {
        match self {
            Self::Stone => STONE,
            Self::Depot | Self::Portable => DEPOT,
        }
    }
}

fn depot_item(data: &Compound) -> Option<ItemStack> {
    let item = horizon_items::item_nbt::item_from_nbt(data.get(&"DepotItem".into())?)?;
    (item.item.as_ref() == INGOT && item.count == 1 && item.damage == 0 && item.extra.is_none())
        .then_some(item)
}

fn plain_ingot(item: &ItemStack) -> bool {
    item.item.as_ref() == INGOT && item.count == 1 && item.damage == 0 && item.extra.is_none()
}

fn read_connection(data: &Compound) -> Option<Connection> {
    let flag = |key: &str| match data.get(&key.into()) {
        Some(Tag::Int(0) | Tag::Byte(0)) => Some(false),
        Some(Tag::Int(1) | Tag::Byte(1)) => Some(true),
        _ => None,
    };
    let number = |key: &str| match data.get(&key.into()) {
        Some(Tag::Int(n)) => Some(*n),
        _ => None,
    };
    Some(Connection {
        timer: number("PortableTimer")?,
        keep_alive: number("PortableKeepAlive")?,
        working: flag("PortableWorking")?,
        linked: flag("PortableLinked")?,
        stalled: flag("PortableStalled")?,
    })
}

fn write_connection(data: &mut Compound, connection: Connection) {
    for (key, value) in [
        ("PortableTimer", connection.timer),
        ("PortableKeepAlive", connection.keep_alive),
        ("PortableWorking", i32::from(connection.working)),
        ("PortableLinked", i32::from(connection.linked)),
        ("PortableStalled", i32::from(connection.stalled)),
    ] {
        data.insert(key.into(), Tag::Int(value));
    }
}

fn depot_held(entity: &ModBlockEntity) -> Option<ItemStack> {
    if entity.id != DEPOT
        || entity.items.len() != 9
        || entity.items[1..].iter().any(Option::is_some)
    {
        return None;
    }
    let item = entity.items[0].as_ref()?;
    if item.item.as_ref() != INGOT || item.count != 1 || item.damage != 0 || item.extra.is_some() {
        return None;
    }
    // Only the ordinary held-stack presentation and empty processing buffer
    // belong to this profile. An active or unknown depot payload is refused.
    if entity
        .nbt
        .keys()
        .any(|key| key != &JavaString::from("HeldItem") && key != &JavaString::from("OutputBuffer"))
    {
        return None;
    }
    let Some(Tag::Compound(held)) = entity.nbt.get(&"HeldItem".into()) else {
        return None;
    };
    let shown = horizon_items::item_nbt::item_from_nbt(held.get(&"Item".into())?)?;
    if shown != *item
        || held.contains_key(&"FanProcessingType".into())
        || held.contains_key(&"FanProcessingTime".into())
    {
        return None;
    }
    match entity.nbt.get(&"OutputBuffer".into()) {
        None => {}
        Some(Tag::Compound(output))
            if output.get(&"Size".into()) == Some(&Tag::Int(8))
                && output.get(&"Items".into()) == Some(&Tag::List(Vec::new())) => {}
        _ => return None,
    }
    Some(item.clone())
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
    // Only the finite portable profile uses the canonical base container slot.
    // The accepted depot-only profile retains its original saved NBT codec.
    portable: bool,
    item: Option<ItemStack>,
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
        let depot = integer(&data, "CapturedDepot");
        let pole = integer(&data, "CapturedPole");
        let cargo = match data.get(&"CargoKind".into()) {
            None => Cargo::Stone,
            Some(Tag::String(s)) if s == &JavaString::from("stone") => Cargo::Stone,
            Some(Tag::String(s)) if s == &JavaString::from("depot") => Cargo::Depot,
            Some(Tag::String(s)) if s == &JavaString::from("portable_depot") => Cargo::Portable,
            _ => return None,
        };
        if !matches!(stone, 0 | 1)
            || !matches!(depot, 0 | 1)
            || !matches!(pole, 0 | 1)
            || stone + depot > 1
            || (stone == 1 && cargo != Cargo::Stone)
            || (depot == 1 && !matches!(cargo, Cargo::Depot | Cargo::Portable))
            || (stone == 1 && data.get(&"Stone".into()) != Some(&Tag::String(STONE.into())))
            || (depot == 1 && cargo == Cargo::Depot && depot_item(&data).is_none())
            || (pole == 1 && data.get(&"Pole".into()) != Some(&Tag::String(POLE.into())))
            || (matches!(phase, Phase::Retracted | Phase::Extended)
                && (stone != 0 || depot != 0 || pole != 0))
            || (phase == Phase::Retracted && offset != 0.0)
            || (phase == Phase::Extended && offset != 1.0)
        {
            return None;
        }
        let portable = cargo == Cargo::Portable;
        // Remote block-entity updates carry the wire snapshot, not canonical
        // slots. Old client slot contents may also be stale after extraction.
        let item = if game.remote.is_some() {
            if portable {
                wire_snapshot(&data)?
            } else {
                None
            }
        } else {
            entity.items.first().cloned().flatten()
        };
        let interface = integer(&data, "CapturedInterface");
        let transferred = integer(&data, "PortableTransferred");
        if (game.remote.is_none() && entity.items.iter().skip(1).any(Option::is_some))
            || (item.is_some() && (!portable || depot != 1))
            || item.as_ref().is_some_and(|item| !plain_ingot(item))
            || !matches!(interface, 0 | 1)
            || (interface == 1 && !portable)
            || (portable
                && (!phase.moving()
                    || base[0].checked_add(4)? >> 4 != base[0] >> 4
                    || data.contains_key(&"DepotItem".into())
                    || !matches!(
                        data.get(&"PortableTransferred".into()),
                        Some(Tag::Int(0 | 1) | Tag::Byte(0 | 1))
                    )
                    || (depot == 1 && item.is_none() != (transferred == 1))
                    || !read_connection(&data)?.valid()))
        {
            return None;
        }
        if portable
            && [
                "PortableDistance",
                "PortableArm",
                "PortablePreviousArm",
                "PortableStation",
                "PortablePreviousStation",
            ]
            .iter()
            .any(|key| {
                let v = float(&data, key);
                !v.is_finite() || !(0.0..=2.0).contains(&v)
            })
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
            portable,
            item,
        })
    }
    fn captured_stone(&self) -> bool {
        integer(&self.data, "CapturedStone") == 1
    }
    fn captured_depot(&self) -> bool {
        integer(&self.data, "CapturedDepot") == 1
    }
    fn cargo(&self) -> Cargo {
        match self.data.get(&"CargoKind".into()) {
            Some(Tag::String(s)) if s == &JavaString::from("depot") => Cargo::Depot,
            Some(Tag::String(s)) if s == &JavaString::from("portable_depot") => Cargo::Portable,
            _ => Cargo::Stone,
        }
    }
    fn captured_cargo(&self) -> bool {
        self.captured_stone() || self.captured_depot()
    }
    fn captured_pole(&self) -> bool {
        integer(&self.data, "CapturedPole") == 1
    }
    fn captured(&self) -> bool {
        self.captured_cargo() || self.captured_pole() || self.captured_interface()
    }
    fn captured_interface(&self) -> bool {
        integer(&self.data, "CapturedInterface") == 1
    }
    fn front(&self) -> [i32; 3] {
        [self.base[0] + 3, self.base[1], self.base[2]]
    }
    fn stationary(&self) -> [i32; 3] {
        [self.base[0] + 4, self.base[1], self.base[2]]
    }
    fn held_item(&self) -> Option<ItemStack> {
        if self.portable {
            self.item.clone()
        } else {
            depot_item(&self.data)
        }
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
    fn set_captured(&mut self, cargo: bool, pole: bool) {
        let kind = self.cargo();
        self.data.insert(
            "CapturedStone".into(),
            Tag::Byte(u8::from(cargo && kind == Cargo::Stone) as i8),
        );
        self.data.insert(
            "CapturedDepot".into(),
            Tag::Byte(u8::from(cargo && matches!(kind, Cargo::Depot | Cargo::Portable)) as i8),
        );
        self.data
            .insert("CapturedPole".into(), Tag::Byte(u8::from(pole) as i8));
    }
    fn status(&mut self, value: &str) {
        self.data.insert("Status".into(), Tag::String(value.into()));
    }
    fn store(self, entity: &mut ModBlockEntity) {
        if self.portable {
            entity.items.resize(1, None);
            entity.items[0] = self.item;
        }
        entity.nbt.insert(KEY.into(), Tag::Compound(self.data));
    }
    fn bounds(&self, source: [i32; 3], offset: f32) -> Aabb {
        let [x, y, z] = source.map(f64::from);
        Aabb::new(
            Vec3::new(x + f64::from(offset), y, z),
            Vec3::new(x + f64::from(offset) + 1.0, y + 1.0, z + 1.0),
        )
    }
    fn reserved(&self) -> Vec<[i32; 3]> {
        let mut cells = vec![self.base, self.pole, self.head, self.dest];
        if self.portable {
            cells.push(self.front());
        }
        cells
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

/// Capture transfers the depot's held stack into the piston owner. Horizon's
/// ordinary block replacement spills every ModContainer slot, so only this
/// depot-to-proxy write suppresses block-entity removal side effects.
fn capture_depot_block(game: &mut Game<'_>, pos: [i32; 3]) -> bool {
    let Some(state) = game.pack.assets.registry.default_state(PROXY) else {
        return false;
    };
    let dim = game.dim.id().to_owned();
    game.native_set_block(
        &dim,
        pos,
        state,
        3 | horizon_world::level::flags::UPDATE_SKIP_BLOCK_ENTITY_SIDEEFFECTS,
    );
    game.native_block(pos) == PROXY
}

fn depot_matches(game: &Game<'_>, pos: [i32; 3], item: &ItemStack) -> bool {
    game.native_block(pos) == DEPOT
        && game.world.block_entities.get(&(pos[0], pos[1], pos[2])).is_some_and(
            |entity| matches!(entity, BlockEntity::Mod(e) if depot_held(e).as_ref() == Some(item)),
        )
}

fn empty_depot_target(entity: &ModBlockEntity) -> bool {
    if entity.id != DEPOT
        || !matches!(entity.items.len(), 0 | 9)
        || entity.items.iter().any(Option::is_some)
        || entity
            .nbt
            .keys()
            .any(|key| key != &JavaString::from("OutputBuffer"))
    {
        return false;
    }
    match entity.nbt.get(&"OutputBuffer".into()) {
        None => true,
        Some(Tag::Compound(output)) => {
            output.get(&"Size".into()) == Some(&Tag::Int(8))
                && output.get(&"Items".into()) == Some(&Tag::List(Vec::new()))
        }
        _ => false,
    }
}

/// A depot already materialized by a failed prior attempt is retried in place.
/// The piston keeps its saved item claim until the actual target entity agrees.
fn materialize_cargo(game: &mut Game<'_>, piston: &mut Piston, pos: [i32; 3]) -> bool {
    if piston.cargo() == Cargo::Stone {
        return set(game, pos, STONE);
    }
    let item = piston.held_item();
    if item.is_none() && !piston.portable {
        return false;
    }
    if game.native_block(pos) == DEPOT && integer(&piston.data, "DepotMaterializing") != 1 {
        return false;
    }
    if cargo_in_depot(game, piston, pos) {
        return true;
    }
    if game.native_block(pos) != DEPOT {
        if !set(game, pos, DEPOT) {
            return false;
        }
        piston
            .data
            .insert("DepotMaterializing".into(), Tag::Byte(1));
    } else if integer(&piston.data, "DepotMaterializing") != 1 {
        // An independently placed depot is not the owner's restoration target.
        return false;
    }
    let installed = game.native_with_mod_block_entity(pos, |entity| {
        // Check the whole target before writing: otherwise a stale/foreign NBT
        // field could make post-write verification fail with two item copies.
        if !empty_depot_target(entity) {
            return false;
        }
        if entity.items.is_empty() {
            entity.items.resize(9, None);
        }
        entity.items[0] = item.clone();
        super::create_port::depot_data(entity);
        true
    }) == Some(true);
    if installed && cargo_in_depot(game, piston, pos) {
        game.native_block_entity_sync(pos);
        true
    } else {
        false
    }
}

fn cargo_in_depot(game: &Game<'_>, piston: &Piston, pos: [i32; 3]) -> bool {
    if let Some(item) = piston.held_item() {
        depot_matches(game, pos, &item)
    } else {
        piston.portable
            && game.native_block(pos) == DEPOT
            && game
                .world
                .block_entities
                .get(&(pos[0], pos[1], pos[2]))
                .is_some_and(
                    |entity| matches!(entity, BlockEntity::Mod(e) if empty_depot_target(e)),
                )
    }
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
        && (!piston.captured_cargo()
            || super::create_s5_carrier::static_path_clear_cells(
                game,
                piston.head,
                &reserved,
                from,
                to,
            ))
        && (!piston.captured_interface()
            || super::create_s5_carrier::static_path_clear_cells(
                game,
                piston.dest,
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
    let parts: Vec<_> = [
        (piston.captured_pole(), piston.base, HEAD),
        (piston.captured_cargo(), piston.head, piston.cargo().block()),
        (piston.captured_interface(), piston.dest, INTERFACE),
    ]
    .into_iter()
    .filter(|part| part.0)
    .collect();
    let &(_, source, block) = parts.first()?;
    let (anchor, projection) = piston.projection(game, source, block, from, to)?;
    let step = carrier_step(from, to)?;
    let boxes: Vec<_> = parts
        .iter()
        .map(|(_, pos, _)| {
            let x = f64::from(pos[0] - source[0]);
            Aabb::new(Vec3::new(x, 0.0, 0.0), Vec3::new(x + 1.0, 1.0, 1.0))
        })
        .collect();
    let mut final_bounds: Vec<_> = parts
        .iter()
        .map(|(_, pos, _)| piston.bounds(*pos, to))
        .collect();
    if returning_pole {
        final_bounds.push(piston.bounds(piston.pole, 0.0));
    }
    game.create_s5_plan_push_shapes(anchor, projection, step.progress, &boxes, &final_bounds)
}

fn empty_interface(game: &Game<'_>, pos: [i32; 3]) -> bool {
    game.native_block(pos) == INTERFACE
        && game.native_block_property(pos, "facing").as_deref() == Some("east")
        && game.world.block_entities.get(&(pos[0], pos[1], pos[2])).is_some_and(
            |entity| matches!(entity, BlockEntity::Mod(e) if e.id == INTERFACE && e.items.iter().all(Option::is_none) && e.nbt.is_empty()),
        )
}

fn materialize_interface(game: &mut Game<'_>, piston: &mut Piston, pos: [i32; 3]) -> bool {
    let block = game.native_block(pos);
    if block == INTERFACE {
        return integer(&piston.data, "InterfaceMaterializing") == 1 && empty_interface(game, pos);
    }
    if !matches!(block.as_str(), PROXY | "minecraft:air")
        || !game.create_s5_body_clear(
            BlockPos::new(pos[0], pos[1], pos[2]),
            piston.bounds(pos, 0.0),
        )
        || !set(game, pos, INTERFACE)
    {
        return false;
    }
    piston
        .data
        .insert("InterfaceMaterializing".into(), Tag::Byte(1));
    empty_interface(game, pos)
}

fn close_connection(piston: &mut Piston) {
    if piston.portable {
        write_connection(&mut piston.data, Connection::default());
    }
}

fn capture_portable_extension(game: &mut Game<'_>, piston: &mut Piston, item: ItemStack) -> bool {
    let mut candidate = piston.clone();
    candidate.portable = true;
    if candidate.base[0]
        .checked_add(4)
        .is_none_or(|x| x >> 4 != candidate.base[0] >> 4)
    {
        return false;
    }
    let cells = candidate.reserved();
    let dim = game.dim.id().to_owned();
    if piston.phase != Phase::Retracted || piston.offset() != 0.0
        || game.native_block(piston.pole) != POLE
        || game.native_block_property(piston.pole, "facing").as_deref() != Some("east")
        || !game.world.block_entities.get(&(piston.pole[0], piston.pole[1], piston.pole[2])).is_some_and(
            |entity| matches!(entity, BlockEntity::Mod(e) if e.id == POLE && e.items.is_empty()),
        )
        || !empty_interface(game, piston.dest)
        || game.native_neighbor_signal(&dim, piston.dest) > 0
        || game.native_block(candidate.front()) != "minecraft:air"
        || game.world.block_entities.contains_key(&(candidate.front()[0], candidate.front()[1], candidate.front()[2]))
        || !super::create_s5_carrier::static_path_clear_cells(game, piston.base, &cells, 0.0, 1.0)
        || !super::create_s5_carrier::static_path_clear_cells(game, piston.head, &cells, 0.0, 1.0)
        || !super::create_s5_carrier::static_path_clear_cells(game, piston.dest, &cells, 0.0, 1.0)
    {
        return false;
    }
    candidate.item = Some(item.clone());
    candidate.set_phase(Phase::Extending);
    candidate.set_float("Offset", 0.0);
    candidate.set_float("Previous", 0.0);
    candidate
        .data
        .insert("CargoKind".into(), Tag::String("portable_depot".into()));
    candidate.data.remove(&"Stone".into());
    candidate.data.remove(&"DepotItem".into());
    candidate
        .data
        .insert("Pole".into(), Tag::String(POLE.into()));
    candidate.data.insert("CapturePending".into(), Tag::Byte(1));
    candidate
        .data
        .insert("CapturedInterface".into(), Tag::Byte(0));
    candidate.set_captured(false, false);
    write_connection(&mut candidate.data, Connection::default());
    candidate.data.remove(&"PortableCell".into());
    candidate
        .data
        .insert("PortableTransferred".into(), Tag::Byte(0));
    for key in [
        "PortableDistance",
        "PortableArm",
        "PortablePreviousArm",
        "PortableStation",
        "PortablePreviousStation",
    ] {
        candidate.set_float(key, 0.0);
    }
    *piston = candidate;

    let depot_changed = capture_depot_block(game, piston.head);
    let depot_absent = !depot_matches(game, piston.head, &item);
    piston.set_captured(depot_absent, false);
    if !depot_absent {
        piston.item = None;
    }
    if !depot_changed {
        rollback_portable_capture(game, piston);
        return false;
    }
    let interface_changed = set(game, piston.dest, PROXY);
    piston.data.insert(
        "CapturedInterface".into(),
        Tag::Byte(i8::from(!empty_interface(game, piston.dest))),
    );
    if !interface_changed || !set(game, piston.front(), PROXY) {
        rollback_portable_capture(game, piston);
        return false;
    }
    let pole_changed = set(game, piston.pole, "minecraft:air");
    piston.set_captured(true, game.native_block(piston.pole) != POLE);
    if !pole_changed {
        rollback_portable_capture(game, piston);
        return false;
    }
    piston.data.remove(&"CapturePending".into());
    piston.status("extending-portable");
    let _ = base_state(game, piston.base, "moving");
    true
}

fn rollback_portable_capture(game: &mut Game<'_>, piston: &mut Piston) -> bool {
    close_connection(piston);
    // Flags are observed after each actual capture write. A delivered depot
    // clears the canonical slot immediately, even if another block must retry.
    if piston.captured_cargo()
        && matches!(
            game.native_block(piston.head).as_str(),
            PROXY | "minecraft:air" | DEPOT
        )
        && (game.native_block(piston.head) == DEPOT
            || game.create_s5_body_clear(
                BlockPos::new(piston.head[0], piston.head[1], piston.head[2]),
                piston.bounds(piston.head, 0.0),
            ))
        && materialize_cargo(game, piston, piston.head)
    {
        piston.set_captured(false, piston.captured_pole());
        piston.item = None;
    }
    if piston.captured_interface() {
        if materialize_interface(game, piston, piston.dest) {
            piston.data.insert("CapturedInterface".into(), Tag::Byte(0));
            piston.data.remove(&"InterfaceMaterializing".into());
        }
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
        piston.set_captured(piston.captured_cargo(), false);
    }
    if !piston.captured_interface() && game.native_block(piston.front()) == PROXY {
        let _ = set(game, piston.front(), "minecraft:air");
    }
    let restored = !piston.captured()
        && game.native_block(piston.head) == DEPOT
        && empty_interface(game, piston.dest)
        && game.native_block(piston.pole) == POLE
        && game.native_block(piston.front()) == "minecraft:air";
    if restored {
        piston.set_phase(Phase::Retracted);
        piston.data.remove(&"CapturePending".into());
        piston.data.remove(&"CargoKind".into());
        piston.data.remove(&"DepotMaterializing".into());
        piston.status("capture-rolled-back");
        let _ = base_state(game, piston.base, "retracted");
    } else {
        piston.data.insert("CapturePending".into(), Tag::Byte(1));
        piston.status("capture-restore-blocked");
    }
    restored
}

/// The real pole is initially placed. No payload is inferred from an empty
/// space or from the owner's configuration. All four cells must be loaded and
/// in the same chunk before the bounded capture can start.
fn capture_extension(game: &mut Game<'_>, piston: &mut Piston) -> bool {
    let reserved = piston.reserved();
    let head_block = game.native_block(piston.head);
    let cargo = match head_block.as_str() {
        STONE
            if !game.world.block_entities.contains_key(&(
                piston.head[0],
                piston.head[1],
                piston.head[2],
            )) =>
        {
            Cargo::Stone
        }
        DEPOT
            if game
                .native_block_property(piston.head, "waterlogged")
                .as_deref()
                != Some("true") =>
        {
            if game.native_block(piston.dest) == INTERFACE {
                Cargo::Portable
            } else {
                Cargo::Depot
            }
        }
        _ => return false,
    };
    let held = if matches!(cargo, Cargo::Depot | Cargo::Portable) {
        let Some(BlockEntity::Mod(entity)) =
            game.world
                .block_entities
                .get(&(piston.head[0], piston.head[1], piston.head[2]))
        else {
            return false;
        };
        let Some(item) = depot_held(entity) else {
            return false;
        };
        Some(item)
    } else {
        None
    };
    if cargo == Cargo::Portable {
        return capture_portable_extension(game, piston, held.expect("validated held depot"));
    }
    if piston.phase != Phase::Retracted
        || piston.offset() != 0.0
        || game.native_block(piston.pole) != POLE
        || game.native_block_property(piston.pole, "facing").as_deref() != Some("east")
        || game.native_block(piston.dest) != "minecraft:air"
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
    piston.data.insert(
        "CargoKind".into(),
        Tag::String(
            match cargo {
                Cargo::Stone => "stone",
                Cargo::Depot => "depot",
                Cargo::Portable => unreachable!("handled by finite portable capture"),
            }
            .into(),
        ),
    );
    if cargo == Cargo::Stone {
        piston
            .data
            .insert("Stone".into(), Tag::String(STONE.into()));
        piston.data.remove(&"DepotItem".into());
    } else if let Some(item) = held {
        piston.data.remove(&"Stone".into());
        piston.data.insert(
            "DepotItem".into(),
            horizon_items::item_nbt::item_to_nbt(&item),
        );
    }
    piston.data.insert("Pole".into(), Tag::String(POLE.into()));
    piston.data.insert("CapturePending".into(), Tag::Byte(1));
    if !(if cargo == Cargo::Depot {
        capture_depot_block(game, piston.head)
    } else {
        set(game, piston.head, PROXY)
    }) || !set(game, piston.dest, PROXY)
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
/// before retrying: a missing cargo or pole remains an NBT-owned payload;
/// a verified world original is no longer owned by the machine. The pending
/// flag keeps removal blocked even when both originals have returned but a
/// spare derived proxy still needs clearing.
fn rollback_capture(game: &mut Game<'_>, piston: &mut Piston) -> bool {
    if piston.portable {
        return rollback_portable_capture(game, piston);
    }
    let cargo_in_world = match piston.cargo() {
        Cargo::Stone => game.native_block(piston.head) == STONE,
        Cargo::Depot | Cargo::Portable => {
            depot_item(&piston.data).is_some_and(|item| depot_matches(game, piston.head, &item))
        }
    };
    piston.set_captured(!cargo_in_world, game.native_block(piston.pole) != POLE);
    let current = game.native_block(piston.head);
    if piston.captured_cargo()
        && (matches!(current.as_str(), PROXY | "minecraft:air")
            || (piston.cargo() == Cargo::Depot && current == DEPOT))
        && (current == DEPOT
            || game.create_s5_body_clear(
                BlockPos::new(piston.head[0], piston.head[1], piston.head[2]),
                piston.bounds(piston.head, 0.0),
            ))
        && materialize_cargo(game, piston, piston.head)
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
        piston.set_captured(piston.captured_cargo(), false);
    }
    if game.native_block(piston.dest) == PROXY {
        let _ = set(game, piston.dest, "minecraft:air");
    }
    let restored = !piston.captured()
        && match piston.cargo() {
            Cargo::Stone => game.native_block(piston.head) == STONE,
            Cargo::Depot | Cargo::Portable => {
                depot_item(&piston.data).is_some_and(|item| depot_matches(game, piston.head, &item))
            }
        }
        && game.native_block(piston.pole) == POLE
        && game.native_block(piston.dest) == "minecraft:air";
    if restored {
        piston.data.remove(&"DepotItem".into());
        piston.data.remove(&"DepotMaterializing".into());
        piston.data.remove(&"CargoKind".into());
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
    close_connection(piston);
    let from = piston.offset();
    if piston.captured() && !path_clear(game, piston, from, endpoint) {
        return false;
    }
    if piston.captured_cargo()
        && game.native_block(piston.dest) != PROXY
        && !(matches!(piston.cargo(), Cargo::Depot | Cargo::Portable)
            && game.native_block(piston.dest) == DEPOT)
    {
        return false;
    }
    if piston.captured_pole() && game.native_block(piston.head) != PROXY {
        return false;
    }
    if piston.captured_interface()
        && game.native_block(piston.front()) != PROXY
        && !(integer(&piston.data, "InterfaceMaterializing") == 1
            && game.native_block(piston.front()) == INTERFACE)
    {
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
    let cargo_target = if endpoint == 1.0 {
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
        if piston.portable {
            piston.front()
        } else {
            piston.dest
        }
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
        piston.set_captured(piston.captured_cargo(), false);
    }
    if piston.captured_interface() && endpoint == 1.0 {
        if !materialize_interface(game, piston, piston.front()) {
            piston.data.insert("RestorePending".into(), Tag::Byte(1));
            piston.data.insert("PendingEndpoint".into(), Tag::Int(1));
            return false;
        }
        piston.data.insert("CapturedInterface".into(), Tag::Byte(0));
        piston.data.remove(&"InterfaceMaterializing".into());
        if let Some(push) = push.take() {
            game.create_s5_commit_push(push);
        }
    }
    if extension && piston.captured_cargo() {
        if !materialize_cargo(game, piston, cargo_target) {
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
        if piston.portable {
            piston.item = None;
        }
        piston.data.remove(&"DepotItem".into());
        piston.data.remove(&"DepotMaterializing".into());
    }
    if piston.captured_interface() && endpoint == 0.0 {
        if !materialize_interface(game, piston, piston.dest) {
            piston.data.insert("RestorePending".into(), Tag::Byte(1));
            piston.data.insert("PendingEndpoint".into(), Tag::Int(0));
            return false;
        }
        piston.data.insert("CapturedInterface".into(), Tag::Byte(0));
        piston.data.remove(&"InterfaceMaterializing".into());
        if let Some(push) = push.take() {
            game.create_s5_commit_push(push);
        }
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
        piston.set_captured(piston.captured_cargo(), false);
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
    piston.data.remove(&"CargoKind".into());
    piston.data.remove(&"DepotItem".into());
    piston.data.remove(&"DepotMaterializing".into());
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

fn station_available(game: &mut Game<'_>, piston: &Piston) -> bool {
    let pos = piston.stationary();
    if game.native_block(pos) != INTERFACE
        || game.native_block_property(pos, "facing").as_deref() != Some("west")
        || !game.world.block_entities.get(&(pos[0], pos[1], pos[2])).is_some_and(
            |entity| matches!(entity, BlockEntity::Mod(e) if e.id == INTERFACE && e.items.iter().all(Option::is_none) && e.nbt.is_empty()),
        )
    { return false; }
    let dim = game.dim.id().to_owned();
    game.native_neighbor_signal(&dim, pos) == 0
}

fn portable_before_motion(game: &mut Game<'_>, piston: &mut Piston, rpm: f32) -> bool {
    if !piston.portable {
        return false;
    }
    let mut connection = read_connection(&piston.data).expect("validated portable state");
    if rpm >= 0.0 || !station_available(game, piston) {
        if connection.linked || connection.working || connection.stalled {
            connection.close();
        } else {
            connection.timer_tick();
        }
    } else {
        connection.timer_tick();
    }
    write_connection(&mut piston.data, connection);
    connection.stalled
}

fn portable_after_motion(game: &mut Game<'_>, piston: &mut Piston, motion: f32) {
    if !piston.portable || !piston.phase.moving() {
        return;
    }
    let mut connection = read_connection(&piston.data).expect("validated portable state");
    let available = station_available(game, piston);
    let cell =
        create_engine::portable_interface::active_cell(piston.offset()).expect("validated offset");
    if integer(&piston.data, "PortableCell") != cell
        || (integer(&piston.data, "PortableAvailable") == 1) != available
    {
        let was_linked = connection.linked;
        connection.encounter(piston.offset(), available);
        if connection.linked && !was_linked {
            piston.set_float(
                "PortableDistance",
                ((0.15 - piston.offset()).abs() + 0.85).min(2.0),
            );
        }
    }
    connection.actor_tick(piston.offset(), motion, available);
    let old_arm = float(&piston.data, "PortableArm");
    let distance = float(&piston.data, "PortableDistance");
    let target_arm = if connection.working && connection.timer > 4 {
        distance * 0.5
    } else {
        0.0
    };
    piston.set_float("PortablePreviousArm", old_arm);
    piston.set_float(
        "PortableArm",
        old_arm + (target_arm - old_arm).clamp(-0.25, 0.25),
    );
    piston.set_float(
        "PortablePreviousStation",
        float(&piston.data, "PortableStation"),
    );
    piston.set_float(
        "PortableStation",
        connection.animation().powi(2) * distance * 0.5,
    );
    piston.data.insert("PortableCell".into(), Tag::Int(cell));
    piston
        .data
        .insert("PortableAvailable".into(), Tag::Byte(i8::from(available)));
    if connection.stalled {
        piston.data.insert("Stalled".into(), Tag::Byte(1));
    }
    write_connection(&mut piston.data, connection);
}

/// The selected hopper source preflight calls this with a full mutable host.
/// It grants a stack-local view of the sole canonical slot, never a copy.
pub(super) fn portable_source(game: &mut Game<'_>, station: [i32; 3]) -> Option<[i32; 3]> {
    if game.remote.is_some() || game.lan.is_none() {
        return None;
    }
    let mut candidates = pistons(game)
        .into_iter()
        .filter(|p| p.portable && p.stationary() == station);
    let piston = candidates.next()?;
    if candidates.next().is_some() || piston.phase != Phase::Extending
        || !piston.captured_depot() || !piston.captured_interface() || !piston.captured_pole()
        || game.native_block(piston.pole) != "minecraft:air"
        || integer(&piston.data, "CapturePending") != 0
        || integer(&piston.data, "RestorePending") != 0
        || integer(&piston.data, "RemovalRequested") != 0
        || !read_connection(&piston.data)?.connected()
        || create_engine::portable_interface::active_cell(piston.offset()) != Some(4)
        || [piston.head, piston.dest, piston.front()].iter().any(|pos| game.native_block(*pos) != PROXY)
        || !game.world.block_entities.get(&(piston.base[0], piston.base[1], piston.base[2])).is_some_and(
            |entity| matches!(entity, BlockEntity::Mod(e) if { let rpm = float(&e.nbt, "Speed"); rpm.is_finite() && rpm < 0.0 }),
        )
        || !station_available(game, &piston)
    { return None; }
    static TRACE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if *TRACE.get_or_init(|| std::env::var_os("CREATE_HORIZON_TRACE").is_some()) {
        let owner_tick = match piston.data.get(&"Tick".into()) {
            Some(Tag::Long(t)) => *t,
            _ => 0,
        };
        let key = (
            piston.base,
            piston.item.is_some(),
            owner_tick == game.world.game_time,
        );
        thread_local! { static SEEN: std::cell::RefCell<std::collections::BTreeSet<([i32;3],bool,bool)>> = const { std::cell::RefCell::new(std::collections::BTreeSet::new()) }; }
        if SEEN.with(|seen| seen.borrow_mut().insert(key)) {
            log::info!(
                "Create portable hopper source: {}",
                serde_json::json!({"base":piston.base,
                "time":game.world.game_time,"owner_tick":owner_tick,"owner_already_ticked":key.2,
                "item_present":key.1,"timer":integer(&piston.data,"PortableTimer")})
            );
        }
    }
    Some(piston.base)
}

pub(super) fn portable_changed(game: &mut Game<'_>, pos: (i32, i32, i32)) {
    let Some(BlockEntity::Mod(entity)) = game.world.block_entities.get(&pos) else {
        return;
    };
    // The stock transfer changed the slot before this callback. Validate that
    // one allowed transient transition, then publish the empty custody state.
    if entity.items.first().is_some_and(Option::is_some) {
        return;
    }
    let mut pending = ModBlockEntity::default();
    pending.id = entity.id.clone();
    pending.nbt = entity.nbt.clone();
    pending.items = entity.items.clone();
    let Some(Tag::Compound(data)) = pending.nbt.get_mut(&KEY.into()) else {
        return;
    };
    if integer(data, "PortableTransferred") != 0 {
        return;
    }
    data.insert("PortableTransferred".into(), Tag::Byte(1));
    let Some(mut piston) = Piston::read(game, [pos.0, pos.1, pos.2], &pending) else {
        return;
    };
    if !piston.portable || !piston.captured_depot() || piston.item.is_some() {
        return;
    }
    let Some(mut connection) = read_connection(&piston.data) else {
        return;
    };
    // The stock move has already committed. Publish empty custody even if a
    // later side effect closed the link; only a still-connected timer resets.
    let _ = connection.content_transferred();
    piston
        .data
        .insert("PortableTransferred".into(), Tag::Byte(1));
    write_connection(&mut piston.data, connection);
    if let Some(BlockEntity::Mod(entity)) = game.world.block_entities.get_mut(&pos) {
        piston.store(entity);
    }
    game.native_block_entity_sync([pos.0, pos.1, pos.2]);
}

fn wire_item(data: &Compound) -> Option<ItemStack> {
    wire_snapshot(data).flatten()
}

fn wire_snapshot(data: &Compound) -> Option<Option<ItemStack>> {
    let Some(Tag::Compound(wire)) = data.get(&"PortableItemWire".into()) else {
        return None;
    };
    match wire.get(&"Present".into()) {
        Some(Tag::Int(0) | Tag::Byte(0)) => return Some(None),
        Some(Tag::Int(1) | Tag::Byte(1)) => {}
        _ => return None,
    }
    let item = horizon_items::item_nbt::item_from_nbt(wire.get(&"Item".into())?)?;
    plain_ingot(&item).then_some(Some(item))
}

pub(super) fn update_tag(entity: &ModBlockEntity, tag: &mut Option<Compound>) {
    if entity.id != BASE {
        return;
    }
    let snapshot = tag.get_or_insert_with(|| entity.nbt.clone());
    let Some(Tag::Compound(data)) = snapshot.get_mut(&KEY.into()) else {
        return;
    };
    let item = (data.get(&"CargoKind".into()) == Some(&Tag::String("portable_depot".into()))
        && integer(data, "CapturedDepot") == 1)
        .then(|| entity.items.first().cloned().flatten())
        .flatten()
        .filter(plain_ingot);
    let mut wire = Compound::new();
    wire.insert("Present".into(), Tag::Byte(i8::from(item.is_some())));
    if let Some(item) = item {
        wire.insert("Item".into(), horizon_items::item_nbt::item_to_nbt(&item));
    }
    data.insert("PortableItemWire".into(), Tag::Compound(wire));
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
    let motion_start = piston.offset();
    let interface_stalled = portable_before_motion(game, &mut piston, rpm);
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
        } else if interface_stalled {
            piston.set_float("Previous", from);
            piston.status("interface-stalled");
        } else if let Some(delta) = eastward_delta(rpm, from) {
            let to = (from + delta).clamp(0.0, 1.0);
            if !piston.captured()
                || (piston.captured_cargo() && game.native_block(piston.dest) != PROXY)
                || (piston.captured_pole() && game.native_block(piston.head) != PROXY)
                || (piston.captured_interface() && game.native_block(piston.front()) != PROXY)
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
    if piston.portable && piston.phase.moving() && rpm < 0.0 {
        let motion = piston.offset() - motion_start;
        portable_after_motion(game, &mut piston, motion);
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
        !profiles.iter().any(|p| p.reserved().contains(pos))
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
            (piston.captured_cargo(), piston.head, piston.cargo().block()),
            (piston.captured_interface(), piston.dest, INTERFACE),
        ] {
            let proxy = [source[0] + 1, source[1], source[2]];
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

/// The vanilla moving-piston renderer submits only the block model. The held
/// depot stack lives in the saved piston owner while the real depot is absent,
/// so draw that one item at the same interpolated block position on each peer.
pub(super) fn draw_mounted_item(
    game: &Game<'_>,
    camera: glam::DVec3,
    alpha: f64,
    out: &mut Vec<horizon_client::mesher::Vertex>,
) {
    if !game.native_window() || !game.native_on("create") {
        return;
    }
    let partial = (alpha as f32).clamp(0.0, 1.0);
    for piston in pistons(game).into_iter().filter(Piston::captured_depot) {
        if !piston.phase.moving() || game.native_block(piston.dest) != PROXY {
            continue;
        }
        let shown = if piston.portable {
            if game.remote.is_some() {
                wire_item(&piston.data)
            } else {
                piston.item.clone()
            }
        } else {
            depot_item(&piston.data)
        };
        if piston.portable {
            trace_portable_item(game, &piston, shown.as_ref());
        }
        let Some(item) = shown else {
            continue;
        };
        let previous = float(&piston.data, "Previous");
        let offset = previous + (piston.offset() - previous) * partial;
        let origin = glam::DVec3::new(
            f64::from(piston.head[0]) + f64::from(offset),
            f64::from(piston.head[1]),
            f64::from(piston.head[2]),
        ) - camera;
        let (block_light, sky_light) =
            game.native_light([piston.head[0], piston.head[1] + 1, piston.head[2]]);
        let mut draw = super::native_block_entity_draw::BlockEntityDraw::posed(
            partial,
            game.world.game_time,
            horizon_client::block_entity_render::vertex_light((sky_light << 4) | block_light),
            piston.head,
            glam::Mat4::from_translation(origin.as_vec3()),
            horizon_client::block_entity_render::diffuse_lights(game.dim == super::Dim::Nether),
            Vec::new(),
        );
        let mut entity = ModBlockEntity::default();
        entity.nbt.insert(
            "HeldItem".into(),
            Tag::Compound(Compound::from([
                ("Item".into(), horizon_items::item_nbt::item_to_nbt(&item)),
                ("Angle".into(), Tag::Int(0)),
            ])),
        );
        super::create_port_presentation::depot(game, piston.head, &entity, &mut draw);
        out.extend(draw.into_vertices());
    }
    for piston in pistons(game).into_iter().filter(Piston::captured_interface) {
        if game.native_block(piston.front()) != PROXY {
            continue;
        }
        let previous = float(&piston.data, "Previous");
        let offset = previous + (piston.offset() - previous) * partial;
        let origin = glam::DVec3::new(
            f64::from(piston.dest[0]) + f64::from(offset),
            f64::from(piston.dest[1]),
            f64::from(piston.dest[2]),
        ) - camera;
        let (block_light, sky_light) =
            game.native_light([piston.dest[0], piston.dest[1] + 1, piston.dest[2]]);
        let mut draw = super::native_block_entity_draw::BlockEntityDraw::posed(
            partial,
            game.world.game_time,
            horizon_client::block_entity_render::vertex_light((sky_light << 4) | block_light),
            piston.dest,
            glam::Mat4::from_translation(origin.as_vec3()),
            horizon_client::block_entity_render::diffuse_lights(game.dim == super::Dim::Nether),
            Vec::new(),
        );
        let arm = interpolate(&piston.data, "PortablePreviousArm", "PortableArm", partial);
        draw_interface_parts(
            game,
            &mut draw,
            "east",
            arm,
            read_connection(&piston.data).is_some_and(Connection::connected),
        );
        out.extend(draw.into_vertices());
    }
}

fn interpolate(data: &Compound, previous: &str, current: &str, partial: f32) -> f32 {
    let a = float(data, previous);
    a + (float(data, current) - a) * partial.clamp(0.0, 1.0)
}

fn draw_interface_parts(
    game: &Game<'_>,
    draw: &mut super::native_block_entity_draw::BlockEntityDraw,
    facing: &str,
    arm: f32,
    lit: bool,
) {
    draw.push();
    let (y, x) = match facing {
        "down" => (0.0, 180.0),
        "north" => (180.0, 90.0),
        "south" => (0.0, 90.0),
        "east" => (90.0, 90.0),
        "west" => (270.0, 90.0),
        _ => (0.0, 0.0),
    };
    draw.rotate_about([0.0, 1.0, 0.0], y, [0.5; 3]);
    draw.rotate_about([1.0, 0.0, 0.0], x, [0.5; 3]);
    draw.push();
    draw.translate([0.0, arm * 0.5 + 0.375, 0.0]);
    game.native_draw_model(
        draw,
        if lit {
            "create:block/portable_storage_interface/block_middle_powered"
        } else {
            "create:block/portable_storage_interface/block_middle"
        },
        0xFFFFFF,
    );
    draw.pop();
    draw.translate([0.0, arm, 0.0]);
    game.native_draw_model(
        draw,
        "create:block/portable_storage_interface/block_top",
        0xFFFFFF,
    );
    draw.pop();
}

pub(super) fn draw_stationary_interface(
    game: &Game<'_>,
    pos: [i32; 3],
    draw: &mut super::native_block_entity_draw::BlockEntityDraw,
) {
    let state = pistons(game)
        .into_iter()
        .find(|p| p.portable && p.stationary() == pos);
    let (arm, lit) = state.map_or((0.0, false), |p| {
        (
            interpolate(
                &p.data,
                "PortablePreviousStation",
                "PortableStation",
                draw.partial,
            ),
            read_connection(&p.data).is_some_and(Connection::connected),
        )
    });
    let facing = game
        .native_block_property(pos, "facing")
        .unwrap_or_else(|| "up".into());
    draw_interface_parts(game, draw, &facing, arm, lit);
}

fn trace_portable_item(game: &Game<'_>, piston: &Piston, item: Option<&ItemStack>) {
    static TRACE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    if !*TRACE.get_or_init(|| std::env::var_os("CREATE_HORIZON_TRACE").is_some()) {
        return;
    }
    thread_local! { static LAST: std::cell::RefCell<std::collections::BTreeMap<[i32;3],bool>> = const { std::cell::RefCell::new(std::collections::BTreeMap::new()) }; }
    let present = item.is_some();
    if LAST.with(|last| last.borrow_mut().insert(piston.base, present) != Some(present)) {
        log::info!(
            "Create portable item render: {}",
            serde_json::json!({"base":piston.base,"present":present,
            "item":item.map(|item| item.item.as_ref()),"count":item.map_or(0, |item| item.count),"time":game.world.game_time})
        );
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
        if piston.reserved().contains(&pos) && piston.base != pos {
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
            || integer(data, "CapturedDepot") == 1
            || integer(data, "CapturedInterface") == 1
            || integer(data, "CapturedPole") == 1
            || integer(data, "CapturePending") == 1
            || integer(data, "RetractionPending") == 1
            || integer(data, "RestorePending") == 1
            || entity.items.iter().any(Option::is_some))
    {
        *canceled = true;
    }
}

pub(super) fn guard_wrench(game: &Game<'_>, item: &str, pos: [i32; 3]) -> bool {
    item == "create:wrench"
        && pistons(game)
            .iter()
            .any(|p| p.phase.moving() && p.reserved().contains(&pos))
}

pub(super) fn guard_proxy_use(game: &Game<'_>, pos: [i32; 3]) -> bool {
    pistons(game)
        .iter()
        .any(|p| p.phase.moving() && p.reserved().contains(&pos) && p.base != pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn piston(data: Compound) -> Piston {
        Piston {
            base: [6, 65, 5],
            pole: [5, 65, 5],
            head: [7, 65, 5],
            dest: [8, 65, 5],
            phase: Phase::Extending,
            data,
            portable: false,
            item: None,
        }
    }

    fn held_depot() -> ModBlockEntity {
        let mut entity = ModBlockEntity::default();
        entity.id = DEPOT.into();
        entity.items = vec![None; 9];
        entity.items[0] = Some(ItemStack::new(INGOT, 1));
        super::super::create_port::depot_data(&mut entity);
        entity
    }

    #[test]
    fn old_stone_and_new_depot_use_distinct_custody_flags() {
        let mut old = piston(Compound::new());
        assert_eq!(old.cargo(), Cargo::Stone);
        old.set_captured(true, true);
        assert!(old.captured_stone() && !old.captured_depot());
        let mut data = Compound::new();
        data.insert("CargoKind".into(), Tag::String("depot".into()));
        data.insert(
            "DepotItem".into(),
            horizon_items::item_nbt::item_to_nbt(&ItemStack::new(INGOT, 1)),
        );
        let mut mounted = piston(data);
        mounted.set_captured(true, true);
        assert!(mounted.captured_depot() && !mounted.captured_stone());
        assert_eq!(depot_item(&mounted.data), Some(ItemStack::new(INGOT, 1)));
        mounted.set_captured(false, true);
        assert!(!mounted.captured_cargo() && mounted.captured_pole());
    }

    #[test]
    fn depot_refuses_output_and_malformed_payload_before_capture() {
        let mut depot = held_depot();
        assert_eq!(depot_held(&depot), Some(ItemStack::new(INGOT, 1)));
        depot.items[1] = Some(ItemStack::new("create:iron_sheet", 1));
        assert!(depot_held(&depot).is_none());
        depot.items[1] = None;
        depot.nbt.insert("OutputBuffer".into(), Tag::Int(8));
        assert!(depot_held(&depot).is_none());
        super::super::create_port::depot_data(&mut depot);
        depot.items[0] = Some(ItemStack::new(INGOT, 2));
        assert!(depot_held(&depot).is_none());
    }

    #[test]
    fn restoration_target_is_empty_and_known_before_item_installation() {
        let mut depot = ModBlockEntity::default();
        depot.id = DEPOT.into();
        assert!(empty_depot_target(&depot));
        super::super::create_port::depot_data(&mut depot);
        assert!(empty_depot_target(&depot));
        depot.nbt.insert("foreign".into(), Tag::Int(1));
        assert!(!empty_depot_target(&depot));
        depot.nbt.remove(&"foreign".into());
        depot.items = vec![None; 9];
        depot.items[0] = Some(ItemStack::new(INGOT, 1));
        assert!(!empty_depot_target(&depot));
    }

    #[test]
    fn portable_item_wire_is_derived_and_explicitly_cleared_without_saved_copy() {
        let mut owner = ModBlockEntity::default();
        owner.id = BASE.into();
        owner.items = vec![Some(ItemStack::new(INGOT, 1))];
        let data = Compound::from([
            ("CargoKind".into(), Tag::String("portable_depot".into())),
            ("CapturedDepot".into(), Tag::Byte(1)),
        ]);
        owner.nbt.insert(KEY.into(), Tag::Compound(data));
        let mut tag = None;
        update_tag(&owner, &mut tag);
        let Some(Tag::Compound(data)) = tag.as_ref().unwrap().get(&KEY.into()) else {
            panic!("owner wire missing");
        };
        assert_eq!(wire_item(data), Some(ItemStack::new(INGOT, 1)));
        let Some(Tag::Compound(saved)) = owner.nbt.get(&KEY.into()) else {
            unreachable!()
        };
        assert!(!saved.contains_key(&"PortableItemWire".into()));
        assert!(!saved.contains_key(&"DepotItem".into()));

        owner.items[0] = None;
        tag = None;
        update_tag(&owner, &mut tag);
        let Some(Tag::Compound(data)) = tag.as_ref().unwrap().get(&KEY.into()) else {
            unreachable!()
        };
        let Some(Tag::Compound(wire)) = data.get(&"PortableItemWire".into()) else {
            unreachable!()
        };
        assert_eq!(integer(wire, "Present"), 0);
        assert!(!wire.contains_key(&"Item".into()));
        assert!(wire_item(data).is_none());
    }
}
