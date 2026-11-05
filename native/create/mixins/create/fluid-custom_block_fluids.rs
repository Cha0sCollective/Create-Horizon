fn create_custom_block_fluids(block: &str, custom: &mut bool) {
    crate::game::create_port_basin::custom_fluids(block, custom);
    crate::game::create_port_fluid_tank::custom_fluids(block, custom);
}
