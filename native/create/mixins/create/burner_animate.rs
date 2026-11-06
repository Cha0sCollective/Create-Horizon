// Source Block.animateTick, invoked by the ordinary ClientLevel random sampler.
// This client-library boundary needs no Game, block-entity scan or engine-crate dependency.
fn create_burner_animate(
    engine: &mut crate::particles::ParticleEngine,
    ctx: &crate::animate_tick::AnimateContext<'_>,
    state: horizon_world::registry::BlockStateId,
    pos: [i32; 3],
) {
    use horizon_core::random::RandomSource;
    let Some(state) = ctx.registry.state(state) else {
        return;
    };
    let center = pos.map(|v| f64::from(v as f32 + 0.5));
    if state.name == "create:blaze_burner" {
        if engine.random().next_int_bound(10).unwrap_or(0) != 0 {
            return;
        }
        if state.properties.get("blaze").is_none_or(|v| v == "none") {
            return;
        }
        let random = engine.random();
        let volume = 0.5 + random.next_float();
        let pitch = random.next_float() * 0.7 + 0.6;
        engine.sounds.push((
            "minecraft:block.campfire.crackle",
            center,
            volume,
            pitch,
            false,
        ));
        return;
    }
}
