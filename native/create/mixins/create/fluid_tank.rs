// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
// Source-informed one-block water controller on the existing native capability.
use super::{
    Game, create_port_fluid_tank_codec as codec, native_block_entity_draw::BlockEntityDraw,
};
use crate::{
    containers::{BlockEntity, ModBlockEntity},
    items::ItemStack,
    mod_api::{
        NativeFluidAction, NativeFluidDrain, NativeFluidRenderOptions, NativeFluidStack,
        NativeFluidTankView,
    },
};
use horizon_core::geometry::Direction;
use horizon_world::nbt::Compound;

const ID: &str = "create:fluid_tank";

fn state(game: &Game<'_>, pos: [i32; 3]) -> Option<codec::Tank> {
    if game.native_block(pos) != ID {
        return None;
    }
    let BlockEntity::Mod(entity) = game.world.block_entities.get(&(pos[0], pos[1], pos[2]))? else {
        return None;
    };
    if entity.id != ID {
        return None;
    }
    let tank = codec::read(&entity.nbt).ok()?;
    if game.native_block_property(pos, "bottom").as_deref() != Some("true")
        || game.native_block_property(pos, "top").as_deref() != Some("true")
        || game.native_block_property(pos, "shape").as_deref()
            != Some(if tank.window { "window" } else { "plain" })
    {
        return None;
    }
    Some(tank)
}

fn water(amount: u32) -> NativeFluidStack {
    NativeFluidStack {
        fluid: "minecraft:water".into(),
        amount: amount as i32,
        components: Compound::new(),
    }
}

fn plain_water(stack: &NativeFluidStack) -> bool {
    stack.fluid == "minecraft:water" && stack.amount > 0 && stack.components.is_empty()
}

fn execute(game: &mut Game<'_>, pos: [i32; 3], old: u32, new: u32) -> bool {
    if old == new || game.native_block(pos) != ID {
        return false;
    }
    let changed = game
        .native_with_mod_block_entity(pos, |entity| {
            entity.id == ID
                && codec::read(&entity.nbt).is_ok_and(|current| current.amount == old)
                && codec::set_amount(&mut entity.nbt, new).unwrap_or(false)
        })
        .unwrap_or(false);
    if changed {
        game.native_block_entity_sync(pos);
    }
    changed
}

pub(super) fn custom_fluids(block: &str, custom: &mut bool) {
    if block == ID {
        *custom = true;
    }
}

pub(super) fn fluid_tanks(
    game: &Game<'_>,
    pos: [i32; 3],
    _side: Option<Direction>,
    out: &mut Option<Vec<NativeFluidTankView>>,
) {
    let Some(tank) = state(game, pos) else {
        return;
    };
    *out = Some(vec![NativeFluidTankView {
        tank: 0,
        content: (tank.amount > 0).then(|| water(tank.amount)),
        capacity: codec::CAPACITY as i32,
        can_fill: true,
        can_drain: true,
    }]);
}

pub(super) fn fluid_fill(
    game: &mut Game<'_>,
    pos: [i32; 3],
    _side: Option<Direction>,
    resource: &NativeFluidStack,
    action: NativeFluidAction,
    out: &mut Option<i32>,
) {
    if let Some(accepted) = transfer_fill(
        game,
        |game| state(game, pos),
        |game, old, new| execute(game, pos, old, new),
        resource,
        action,
    ) {
        *out = Some(accepted);
    }
}

pub(super) fn fluid_drain(
    game: &mut Game<'_>,
    pos: [i32; 3],
    _side: Option<Direction>,
    request: &NativeFluidDrain,
    action: NativeFluidAction,
    out: &mut Option<NativeFluidStack>,
) {
    if let Some(drained) = transfer_drain(
        game,
        |game| state(game, pos),
        |game, old, new| execute(game, pos, old, new),
        request,
        action,
    ) {
        *out = Some(drained);
    }
}

// The callback and the NBT fixture below share these exact transfer branches.
// `read` is the controller boundary; `commit` is invoked only by Execute.
fn transfer_fill<T>(
    source: &mut T,
    read: impl FnOnce(&T) -> Option<codec::Tank>,
    commit: impl FnOnce(&mut T, u32, u32) -> bool,
    resource: &NativeFluidStack,
    action: NativeFluidAction,
) -> Option<i32> {
    let tank = read(source)?;
    if !plain_water(resource) {
        return Some(0);
    }
    let accepted = (resource.amount as u32).min(codec::CAPACITY - tank.amount);
    if accepted == 0 {
        return Some(0);
    }
    #[allow(unreachable_patterns)]
    Some(match action {
        NativeFluidAction::Simulate => accepted as i32,
        NativeFluidAction::Execute => {
            if commit(source, tank.amount, tank.amount + accepted) {
                accepted as i32
            } else {
                0
            }
        }
        _ => 0,
    })
}

fn transfer_drain<T>(
    source: &mut T,
    read: impl FnOnce(&T) -> Option<codec::Tank>,
    commit: impl FnOnce(&mut T, u32, u32) -> bool,
    request: &NativeFluidDrain,
    action: NativeFluidAction,
) -> Option<NativeFluidStack> {
    let tank = read(source)?;
    let maximum = match request {
        NativeFluidDrain::Any(amount) if *amount > 0 => *amount as u32,
        NativeFluidDrain::Exact(stack) if plain_water(stack) => stack.amount as u32,
        _ => return None,
    };
    let accepted = maximum.min(tank.amount);
    if accepted == 0 {
        return None;
    }
    #[allow(unreachable_patterns)]
    match action {
        NativeFluidAction::Simulate => Some(water(accepted)),
        NativeFluidAction::Execute => {
            commit(source, tank.amount, tank.amount - accepted).then(|| water(accepted))
        }
        _ => None,
    }
}

fn bucket_item(held: Option<&ItemStack>) -> bool {
    held.is_some_and(|stack| {
        stack.count > 0
            && matches!(
                stack.item.as_ref(),
                "minecraft:water_bucket" | "minecraft:bucket" | "minecraft:lava_bucket"
            )
    })
}

/// Only Creative-player interaction is handled by Java's ordinary tank block.
pub(super) fn bucket_reacts(game: &Game<'_>, pos: [i32; 3]) -> bool {
    game.native_on("create")
        && game.creative
        && game.native_block(pos) == ID
        && bucket_item(game.held())
}

/// The eligible Java fluid-item path claims a failed exchange as well. An
/// unsupported bucket is claimed here rather than spilling into vanilla use.
pub(super) fn right_click(game: &mut Game<'_>, pos: [i32; 3]) -> bool {
    if !bucket_reacts(game, pos) {
        return false;
    }
    let Some(held) = game.held().cloned() else {
        return true;
    };
    if held.count > super::create_port::stack_maximum(game, &held)
        || held != ItemStack::new(&held.item, held.count)
    {
        return true;
    }
    let dim = game.dim.id().to_owned();
    match held.item.as_ref() {
        "minecraft:water_bucket" => {
            let Some((fluid, amount, _)) = game.native_item_fluid_drain(&held, 1_000) else {
                return true;
            };
            if fluid != "minecraft:water" || amount != 1_000 {
                return true;
            }
            let stack = water(1_000);
            if game.native_fluid_fill_stack(&dim, pos, None, &stack, NativeFluidAction::Simulate)
                == 1_000
            {
                let _ = game.native_fluid_fill_stack(
                    &dim,
                    pos,
                    None,
                    &stack,
                    NativeFluidAction::Execute,
                );
            }
        }
        "minecraft:bucket" => {
            let Some((amount, _)) = game.native_item_fluid_fill(&held, "minecraft:water", 1_000)
            else {
                return true;
            };
            if amount != 1_000 {
                return true;
            }
            let request = NativeFluidDrain::Exact(water(1_000));
            if game
                .native_fluid_drain_stack(&dim, pos, None, &request, NativeFluidAction::Simulate)
                .is_some_and(|s| s.amount == 1_000 && plain_water(&s))
            {
                let _ = game.native_fluid_drain_stack(
                    &dim,
                    pos,
                    None,
                    &request,
                    NativeFluidAction::Execute,
                );
            }
        }
        _ => {}
    }
    true // Creative keeps its held item, even after a successful exchange.
}

pub(super) fn wrench(game: &mut Game<'_>, pos: [i32; 3]) -> bool {
    if game.native_block(pos) != ID
        || game
            .held()
            .is_none_or(|s| s.item.as_ref() != "create:wrench")
    {
        return false;
    }
    // A malformed/connected tank is outside this packet and may not be
    // removed or reinterpreted by the generic wrench route.
    let Some(tank) = state(game, pos) else {
        return true;
    };
    if game.keys.contains(&winit::keyboard::KeyCode::ShiftLeft) {
        return false; // Existing source-backed sneak-wrench drop/remove path.
    }
    let changed = game
        .native_with_mod_block_entity(pos, |entity| {
            entity.id == ID && codec::toggle_window(&mut entity.nbt).unwrap_or(false)
        })
        .unwrap_or(false);
    if changed {
        if !game.native_set_block_property(
            pos,
            "shape",
            if tank.window { "plain" } else { "window" },
        ) {
            // Keep the controller and blockstate in agreement if the state
            // write was refused (for example during a non-server turn).
            let _ = game.native_with_mod_block_entity(pos, |entity| {
                if entity.id == ID {
                    let _ = codec::toggle_window(&mut entity.nbt);
                }
            });
            return true;
        }
        game.native_block_entity_sync(pos);
    }
    true
}

pub(super) fn guard_destroy(game: &Game<'_>, pos: [i32; 3], canceled: &mut bool) {
    if game.native_block(pos) == ID && state(game, pos).is_none() {
        *canceled = true;
    }
}

pub(super) fn draw(
    game: &Game<'_>,
    pos: [i32; 3],
    entity: &ModBlockEntity,
    draw: &mut BlockEntityDraw,
) -> bool {
    if game.native_block(pos) != ID || entity.id != ID {
        return false;
    }
    let Some(tank) = state(game, pos) else {
        return true;
    };
    if !tank.window || tank.amount == 0 {
        return true;
    }
    let top = 2.0 / 16.0 + 12.0 / 16.0 * tank.amount as f32 / codec::CAPACITY as f32;
    game.native_draw_fluid_box(
        draw,
        &water(tank.amount),
        [
            2.0 / 16.0,
            2.0 / 16.0,
            2.0 / 16.0,
            14.0 / 16.0,
            top,
            14.0 / 16.0,
        ],
        NativeFluidRenderOptions {
            argb: None,
            render_bottom: false,
            uv_shrink_ratio: 4.0 / 2048.0,
        },
    );
    true
}

#[cfg(test)]
mod transfer_tests {
    use super::*;
    use horizon_world::nbt::Tag;

    fn fixture_commit(nbt: &mut Compound, old: u32, new: u32, fail: bool) -> bool {
        !fail
            && codec::read(nbt).is_ok_and(|tank| tank.amount == old)
            && codec::set_amount(nbt, new).unwrap_or(false)
    }

    fn fill(
        nbt: &mut Compound,
        resource: &NativeFluidStack,
        action: NativeFluidAction,
        fail_commit: bool,
    ) -> (Option<i32>, usize) {
        let mut commits = 0;
        let result = transfer_fill(
            nbt,
            |nbt| codec::read(nbt).ok(),
            |nbt, old, new| {
                commits += 1;
                fixture_commit(nbt, old, new, fail_commit)
            },
            resource,
            action,
        );
        (result, commits)
    }

    fn drain(
        nbt: &mut Compound,
        request: &NativeFluidDrain,
        action: NativeFluidAction,
        fail_commit: bool,
    ) -> (Option<NativeFluidStack>, usize) {
        let mut commits = 0;
        let result = transfer_drain(
            nbt,
            |nbt| codec::read(nbt).ok(),
            |nbt, old, new| {
                commits += 1;
                fixture_commit(nbt, old, new, fail_commit)
            },
            request,
            action,
        );
        (result, commits)
    }

    #[test]
    fn fill_simulates_without_commit_and_executes_clipped_amount() {
        let mut nbt = codec::initial();
        let before = nbt.clone();
        assert_eq!(
            fill(&mut nbt, &water(9_000), NativeFluidAction::Simulate, false),
            (Some(8_000), 0)
        );
        assert_eq!(nbt, before);
        assert_eq!(
            fill(&mut nbt, &water(9_000), NativeFluidAction::Execute, false),
            (Some(8_000), 1)
        );
        assert_eq!(codec::read(&nbt).unwrap().amount, 8_000);
        assert_eq!(
            fill(&mut nbt, &water(1_000), NativeFluidAction::Execute, false),
            (Some(0), 0)
        );

        codec::set_amount(&mut nbt, 7_500).unwrap();
        let before = nbt.clone();
        assert_eq!(
            fill(&mut nbt, &water(1_000), NativeFluidAction::Simulate, false),
            (Some(500), 0)
        );
        assert_eq!(nbt, before);
        assert_eq!(
            fill(&mut nbt, &water(1_000), NativeFluidAction::Execute, false),
            (Some(500), 1)
        );
        assert_eq!(codec::read(&nbt).unwrap().amount, 8_000);
    }

    #[test]
    fn drain_simulates_without_commit_and_executes_any_and_exact() {
        for request in [
            NativeFluidDrain::Any(1_000),
            NativeFluidDrain::Exact(water(1_000)),
        ] {
            let mut nbt = codec::initial();
            codec::set_amount(&mut nbt, 750).unwrap();
            let before = nbt.clone();
            let (simulated, commits) =
                drain(&mut nbt, &request, NativeFluidAction::Simulate, false);
            assert_eq!(simulated.map(|stack| stack.amount), Some(750));
            assert_eq!(commits, 0);
            assert_eq!(nbt, before);
            let (executed, commits) = drain(&mut nbt, &request, NativeFluidAction::Execute, false);
            assert_eq!(executed.map(|stack| stack.amount), Some(750));
            assert_eq!(commits, 1);
            assert!(
                matches!(nbt.get(&"TankContent".into()), Some(Tag::Compound(c)) if c.is_empty())
            );
            let (empty, commits) = drain(&mut nbt, &request, NativeFluidAction::Execute, false);
            assert!(empty.is_none());
            assert_eq!(commits, 0);
        }
    }

    #[test]
    fn refuses_zero_foreign_components_lava_and_failed_commit() {
        let mut nbt = codec::initial();
        let before = nbt.clone();
        assert_eq!(
            fill(&mut nbt, &water(0), NativeFluidAction::Execute, false),
            (Some(0), 0)
        );
        let lava = NativeFluidStack {
            fluid: "minecraft:lava".into(),
            ..water(1_000)
        };
        assert_eq!(
            fill(&mut nbt, &lava, NativeFluidAction::Execute, false),
            (Some(0), 0)
        );
        let patched = NativeFluidStack {
            components: Compound::from([("foreign".into(), Tag::Byte(1))]),
            ..water(1_000)
        };
        assert_eq!(
            fill(&mut nbt, &patched, NativeFluidAction::Execute, false),
            (Some(0), 0)
        );
        for request in [
            NativeFluidDrain::Exact(lava),
            NativeFluidDrain::Exact(patched),
            NativeFluidDrain::Any(0),
        ] {
            let (returned, commits) = drain(&mut nbt, &request, NativeFluidAction::Execute, false);
            assert!(returned.is_none());
            assert_eq!(commits, 0);
        }
        assert_eq!(nbt, before);

        assert_eq!(
            fill(&mut nbt, &water(1_000), NativeFluidAction::Execute, true),
            (Some(0), 1)
        );
        assert_eq!(nbt, before);
        codec::set_amount(&mut nbt, 750).unwrap();
        let before = nbt.clone();
        let (returned, commits) = drain(
            &mut nbt,
            &NativeFluidDrain::Any(1_000),
            NativeFluidAction::Execute,
            true,
        );
        assert!(returned.is_none());
        assert_eq!(commits, 1);
        assert_eq!(nbt, before);

        nbt.insert("Controller".into(), Tag::Compound(Compound::new()));
        let before = nbt.clone();
        assert_eq!(
            fill(&mut nbt, &water(1_000), NativeFluidAction::Execute, false),
            (None, 0)
        );
        assert!(
            drain(
                &mut nbt,
                &NativeFluidDrain::Any(1_000),
                NativeFluidAction::Execute,
                false
            )
            .0
            .is_none()
        );
        assert_eq!(nbt, before);
    }
}
