// Copyright (c) The Create Team / The Creators of Create
// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
//! Create-specific particle controllers. Vanilla particle construction, lighting,
//! sprite-atlas lookup and rendering remain the platform adapter's responsibility.
use std::sync::OnceLock;
pub trait ParticleRandom {
    fn next_float(&mut self) -> f32;
    fn next_int(&mut self, bound: i32) -> i32;
}

pub fn offset_randomly(random: &mut impl ParticleRandom, range: f32) -> [f64; 3] {
    std::array::from_fn(|_| f64::from((random.next_float() - 0.5) * 2.0 * range))
}
fn sine_table() -> &'static [f32; 65536] {
    static TABLE: OnceLock<[f32; 65536]> = OnceLock::new();
    TABLE.get_or_init(|| {
        std::array::from_fn(|i| ((i as f64 * std::f64::consts::PI * 2.0 / 65536.0).sin()) as f32)
    })
}
/// Pinned Minecraft 1.21.1 `Mth.sin(float)`. Its float index multiplication is
/// different from the newer platform's double-index sine helper.
pub fn source_sin(angle: f32) -> f32 {
    sine_table()[((angle * 10430.378) as i32 & 65535) as usize]
}
