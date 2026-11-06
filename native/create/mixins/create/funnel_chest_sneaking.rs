// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
// FunnelItem suppresses the clicked chest's use, without changing the
// player's actual Shift state used by BlockItem placement.
fn create_funnel_chest_sneaking(
    sneaking: bool,
    game: &crate::game::Game<'_>,
    block_hit: Option<&crate::game::Hit>,
) -> bool {
    sneaking
        || game.native_on("create")
            && game.hand == horizon_client::first_person::Hand::Main
            && game
                .held()
                .is_some_and(|stack| stack.item.as_ref() == "create:andesite_funnel")
            && block_hit.is_some_and(|hit| game.native_block(hit.pos) == "minecraft:chest")
}
