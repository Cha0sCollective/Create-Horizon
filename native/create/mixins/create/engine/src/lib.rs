//! Game-independent parts, also compiled as the native bundle's engine crate.
#[path = "core/basin.rs"]
pub mod basin;
#[path = "core/kinetics.rs"]
pub mod kinetics;
#[path = "core/placement.rs"]
pub mod placement;
#[path = "core/pressing.rs"]
pub mod pressing;
#[path = "core/propagation.rs"]
pub mod propagation;
#[path = "core/session_cache.rs"]
pub mod session_cache;

#[path = "core/linear_carrier.rs"]
pub mod linear_carrier;

#[path = "core/portable_interface.rs"]
pub mod portable_interface;

#[path = "core/depot.rs"]
pub mod depot;

#[path = "core/controls.rs"]
pub mod controls;

#[path = "core/visual.rs"]
pub mod visual;

#[path = "core/waterwheel.rs"]
pub mod waterwheel;

#[path = "core/milling.rs"]
pub mod milling;

#[path = "core/fan.rs"]
pub mod fan;

#[path = "core/drilling.rs"]
pub mod drilling;

#[path = "core/burner.rs"]
pub mod burner;
#[path = "core/burner_visual.rs"]
pub mod burner_visual;
#[path = "core/particles.rs"]
pub mod particles;
