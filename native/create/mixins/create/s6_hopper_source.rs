// Copyright (c) 2026 Create Horizon contributors
// SPDX-License-Identifier: MIT
// Only the selected vanilla hopper path may expose the finite mounted slot.
fn create_s6_hopper_source(
    game: &mut crate::game::Game<'_>,
    above: (i32, i32, i32),
    hopper: &crate::game::machines::View,
) -> std::ops::ControlFlow<Option<bool>> {
    use crate::game::machines::{Kind, View};
    use horizon_core::geometry::Direction;
    use std::{ops::ControlFlow, sync::Arc};
    let station = [above.0, above.1, above.2];
    if !game.native_on("create")
        || game.native_block(station) != "create:portable_storage_interface"
    {
        return ControlFlow::Continue(());
    }
    let Some(&(pos, _)) = hopper.parts.first() else {
        return ControlFlow::Break(Some(false));
    };
    game.native_hopper_inventory_search([pos.0, pos.1, pos.2], Some(station), None);
    if hopper.kind != Kind::Hopper || game.native_block([pos.0, pos.1, pos.2]) != "minecraft:hopper"
    {
        return ControlFlow::Break(Some(false));
    }
    let Some(owner) = crate::game::create_s5_piston::portable_source(game, station) else {
        return ControlFlow::Break(Some(false));
    };
    let source = View {
        parts: vec![((owner[0], owner[1], owner[2]), 1)],
        kind: Kind::Mod,
        container: Some(Arc::new(crate::containers::ModContainer {
            slots: 1,
            faces: std::array::from_fn(|face| {
                if face == Direction::Down as usize {
                    vec![0]
                } else {
                    Vec::new()
                }
            }),
            inputs: Vec::new(),
            outputs: vec![0],
            menu: None,
        })),
    };
    ControlFlow::Break(Some(game.suck_from_slots(None, &source, hopper)))
}
