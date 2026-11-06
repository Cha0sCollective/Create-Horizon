// SPDX-License-Identifier: MIT
// Selected from upstream236c87a8 particles.rs client_tick/animation_time.
use super::Game;
use std::cell::RefCell;
#[derive(Default)]
struct Clock {
    world: usize,
    game: usize,
    dimension: String,
    stamp: Option<i64>,
    ticks: u32,
}
thread_local! { static CLOCK: RefCell<Clock> = RefCell::new(Clock::default()); }
pub(super) fn reset() {
    CLOCK.with(|c| *c.borrow_mut() = Clock::default());
}
pub(super) fn time(partial: f32) -> f32 {
    CLOCK.with(|c| c.borrow().ticks as f32 + partial)
}
pub(super) fn tick(game: &Game<'_>) {
    if !game.native_window() || game.remote.is_none() || !game.native_on("create") {
        return;
    }
    CLOCK.with(|cell| {
        let mut c = cell.borrow_mut();
        let world = std::ptr::from_ref(&game.world) as usize;
        let id = std::ptr::from_ref(game) as usize;
        let time = game.world.game_time;
        if c.world != world
            || c.game != id
            || c.dimension != game.dim.id()
            || c.stamp.is_some_and(|old| old > time)
        {
            *c = Clock {
                world,
                game: id,
                dimension: game.dim.id().into(),
                ..Default::default()
            };
        }
        if !game.pause {
            c.ticks = (c.ticks + 1) % 1_728_000;
        }
        c.stamp = Some(time);
    });
}
