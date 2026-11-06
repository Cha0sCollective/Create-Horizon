// Copyright (c) The Create Team / The Creators of Create
// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
// Stationary straight TrackBlock subset. No rail graph or train owner is made.

use super::GameLevel;
use create_engine::track;
use horizon_core::position::BlockPos;
use horizon_world::{level::PlacementContext, registry::BlockStateId};

const ID: &str = "create:track";

fn block<'a>(game: &'a GameLevel<'_>, pos: [i32; 3]) -> &'a str {
    game.facts
        .get(game.world.block(pos[0], pos[1], pos[2]))
        .name
        .as_ref()
}

pub(super) fn placement_state(
    game: &GameLevel<'_>,
    state: BlockStateId,
    context: &PlacementContext,
    answer: &mut Option<Option<BlockStateId>>,
) {
    if game.native_state_block(state) != Some(ID) {
        return;
    }
    let Some(shape) = track::from_yaw(context.yaw) else {
        *answer = Some(None);
        return;
    };
    let pos = [context.pos.x, context.pos.y, context.pos.z];
    // This packet has no fluid-state adapter. Refuse water and other replaceable
    // contents instead of placing a falsely dry source track in them.
    if !matches!(
        block(game, pos),
        "minecraft:air" | "minecraft:cave_air" | "minecraft:void_air"
    ) || block(game, [pos[0], pos[1] - 1, pos[2]]) == ID
    {
        *answer = Some(None);
        return;
    }
    // TrackBlock promotes a straight shape to ascending beside a sturdy raised
    // neighbor. A nonreplaceable neighbor is a conservative refusal here; the
    // host's future full track placement can distinguish all sturdy faces.
    for delta in shape.neighbors() {
        let neighbor = [pos[0] + delta[0], pos[1], pos[2] + delta[2]];
        let neighbor_state = game.world.block(neighbor[0], neighbor[1], neighbor[2]);
        if game.facts.get(neighbor_state).name.as_ref() == ID {
            continue;
        }
        let above = game.world.block(neighbor[0], neighbor[1] + 1, neighbor[2]);
        if !game.facts.get(neighbor_state).replaceable && game.facts.get(above).replaceable {
            *answer = Some(None);
            return;
        }
    }
    let Some(state) = game.native_state_with_property(state, "shape", shape.state()) else {
        *answer = Some(None);
        return;
    };
    *answer = Some(Some(state));
}

pub(super) fn can_survive(
    game: &GameLevel<'_>,
    pos: BlockPos,
    state: BlockStateId,
    survives: &mut Option<bool>,
) {
    if game.facts.get(state).name.as_ref() == ID {
        // Source TrackBlock.canSurvive only rules out stacking track on track.
        *survives = Some(block(game, [pos.x, pos.y - 1, pos.z]) != ID);
    }
}
