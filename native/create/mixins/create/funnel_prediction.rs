// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
// The remote DryLevel does not run native authoritative placement hooks.
// Let the server decide whether this FunnelItem places and changes its stack.
fn create_funnel_prediction(
    game: &crate::game::Game<'_>,
    stack: &crate::items::ItemStack,
) -> std::ops::ControlFlow<Option<DryPlace>> {
    if game.native_on("create") && stack.item.as_ref() == "create:andesite_funnel" {
        std::ops::ControlFlow::Break(None)
    } else {
        std::ops::ControlFlow::Continue(())
    }
}
