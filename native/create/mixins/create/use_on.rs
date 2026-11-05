fn create_use_on(
    game: &mut crate::game::Game<'_>,
    stack: &crate::items::ItemStack,
    pos: [i32; 3],
    face: [i32; 3],
    claimed: &mut bool,
) {
    if stack.item.as_ref() == "create:track" && game.native_block(pos) == "create:track" {
        // TrackBlockItem starts curve selection here. This finite packet has no
        // curve owner, so do not consume an item or place a false straight.
        crate::game::create_port::use_on(game, stack, pos, face);
        *claimed = true;
        return;
    }
    if crate::game::create_s5_carrier::guard_wrench(game, &stack.item, pos)
        || crate::game::create_s5_piston::guard_wrench(game, &stack.item, pos)
    {
        *claimed = true;
        return;
    }
    crate::game::create_port::use_on(game, stack, pos, face);
    if crate::game::create_port_placement::alias(game, stack, pos, face) {
        *claimed = true;
    }
}
