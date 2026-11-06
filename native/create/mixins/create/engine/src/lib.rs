//! Game-independent parts, also compiled as the native bundle's engine crate.
#[path = "core/basin.rs"]
pub mod basin;
#[path = "core/contraption/mod.rs"]
pub mod contraption;
#[path = "core/crushing.rs"]
pub mod crushing;
#[path = "core/kinetics.rs"]
pub mod kinetics;
#[path = "core/placement.rs"]
pub mod placement;
#[path = "core/pressing.rs"]
pub mod pressing;
#[path = "core/propagation.rs"]
pub mod propagation;
#[path = "core/pump_transfer.rs"]
pub mod pump_transfer;
#[path = "core/quill.rs"]
pub mod quill;
#[path = "core/session_cache.rs"]
pub mod session_cache;
#[path = "core/track.rs"]
pub mod track;

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

#[path = "core/saw.rs"]
pub mod saw;

#[path = "core/burner.rs"]
pub mod burner;
#[path = "core/burner_visual.rs"]
pub mod burner_visual;
#[path = "core/particles.rs"]
pub mod particles;

#[path = "core/transport.rs"]
pub mod transport;
