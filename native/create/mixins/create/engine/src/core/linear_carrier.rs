// Copyright (c) The Create Team / The Creators of Create
// Copyright (c) 2026 Create Horizon contributors
// SPDX-License-Identifier: MIT
//! One-cell east-facing actuator math for the bounded S5 feasibility adapter.
//! Source-informed from Create 6.0.11 / 7ff2760e: KineticBlockEntity's
//! convertToLinear, MechanicalPistonBlockEntity's getMovementSpeed, and
//! LinearActuatorBlockEntity's onSpeedChanged. This is not an assembly engine.

fn valid_offset(offset: f32) -> bool {
    offset.is_finite() && (0.0..=1.0).contains(&offset)
}

/// Blocks per server tick for an east-facing, one-cell mechanical-piston trial.
/// East's positive X facing negates the RPM. Clamp speed before endpoint travel,
/// as in the selected Java reference; caller supplies authoritative kinetic RPM.
pub fn eastward_delta(rpm: f32, offset: f32) -> Option<f32> {
    if !rpm.is_finite() || !valid_offset(offset) {
        return None;
    }
    Some((-(rpm / 512.0).clamp(-0.49, 0.49)).clamp(-offset, 1.0 - offset))
}

/// The reference snaps to a sixteenth on a nonstalled sign change from nonzero
/// RPM, including a change to zero. Collision validation must include this snap,
/// rather than silently using it as an already accepted physical position.
pub fn reversal_offset(offset: f32, previous_rpm: f32, rpm: f32, stalled: bool) -> Option<f32> {
    if !valid_offset(offset) || !previous_rpm.is_finite() || !rpm.is_finite() {
        return None;
    }
    let sign = |value: f32| {
        if value > 0.0 {
            1
        } else if value < 0.0 {
            -1
        } else {
            0
        }
    };
    Some(
        if !stalled && previous_rpm != 0.0 && sign(previous_rpm) != sign(rpm) {
            (offset * 16.0).round() / 16.0
        } else {
            offset
        },
    )
}

/// A derived vanilla-piston projection, measured relative to the payload's
/// original cell. It owns no block/item. Both directions use nondecreasing
/// progress because Horizon's existing entity push ignores a negative delta.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CarrierStep {
    pub anchor_x: i32,
    pub direction_x: i32,
    pub previous_progress: f32,
    pub progress: f32,
}

impl CarrierStep {
    /// Translation from the payload's original cell, matching MovingPistonState
    /// for an extending, non-source projection. No real piston lifecycle implied.
    pub fn offset_at(self, progress: f32) -> f32 {
        self.anchor_x as f32 + self.direction_x as f32 * (progress - 1.0)
    }

    pub fn advances(self) -> bool {
        self.progress > self.previous_progress
    }
}

pub fn carrier_step(previous_offset: f32, offset: f32) -> Option<CarrierStep> {
    if !valid_offset(previous_offset) || !valid_offset(offset) {
        return None;
    }
    Some(if offset < previous_offset {
        CarrierStep {
            anchor_x: 0,
            direction_x: -1,
            previous_progress: 1.0 - previous_offset,
            progress: 1.0 - offset,
        }
    } else {
        CarrierStep {
            anchor_x: 1,
            direction_x: 1,
            previous_progress: previous_offset,
            progress: offset,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn near(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 0.000_001,
            "{actual} != {expected}"
        );
    }

    #[test]
    fn selected_reference_rpm_units_facing_cap_and_endpoint_order() {
        for (rpm, offset, expected) in [
            (-64.0, 0.5, 0.125),
            (64.0, 0.5, -0.125),
            (-1024.0, 0.0, 0.49),
            (1024.0, 1.0, -0.49),
            (-64.0, 0.9375, 0.0625),
            (64.0, 0.0625, -0.0625),
            (-64.0, 1.0, 0.0),
            (64.0, 0.0, 0.0),
            (0.0, 0.375, 0.0),
        ] {
            near(eastward_delta(rpm, offset).unwrap(), expected);
        }
    }

    #[test]
    fn sixty_four_rpm_traverses_one_cell_in_eight_ticks() {
        let mut offset = 0.0;
        for _ in 0..8 {
            offset += eastward_delta(-64.0, offset).unwrap();
        }
        assert_eq!(offset, 1.0);
        assert_eq!(eastward_delta(-64.0, offset), Some(0.0));
    }

    #[test]
    fn reversal_and_zero_speed_snap_match_reference_without_snapping_stalls() {
        assert_eq!(reversal_offset(0.34, -64.0, 64.0, false), Some(0.3125));
        assert_eq!(reversal_offset(0.34, -64.0, 0.0, false), Some(0.3125));
        assert_eq!(reversal_offset(0.53125, 64.0, -64.0, false), Some(0.5625));
        assert_eq!(reversal_offset(0.34, -64.0, 64.0, true), Some(0.34));
        assert_eq!(reversal_offset(0.34, 0.0, 64.0, false), Some(0.34));
        assert_eq!(reversal_offset(0.34, 64.0, 32.0, false), Some(0.34));
        assert_eq!(reversal_offset(0.34, -0.0, 64.0, false), Some(0.34));
    }

    #[test]
    fn both_directions_map_to_the_same_swept_world_positions() {
        for (from, to) in [(0.25, 0.375), (0.375, 0.25), (0.0, 0.125), (1.0, 0.875)] {
            let step = carrier_step(from, to).unwrap();
            assert!(step.advances());
            near(step.offset_at(step.previous_progress), from);
            near(step.offset_at(step.progress), to);
            near(
                step.offset_at((step.previous_progress + step.progress) / 2.0),
                (from + to) / 2.0,
            );
        }
        let reverse = carrier_step(0.375, 0.25).unwrap();
        assert_eq!(reverse.anchor_x, 0);
        assert_eq!(reverse.direction_x, -1);
        assert_eq!(reverse.previous_progress, 0.625);
        assert_eq!(reverse.progress, 0.75);
    }

    #[test]
    fn a_stationary_projection_cannot_request_an_entity_push() {
        let step = carrier_step(0.375, 0.375).unwrap();
        assert!(!step.advances());
        assert_eq!(step.offset_at(step.progress), 0.375);
    }

    #[test]
    fn invalid_machine_data_cannot_create_a_carrier() {
        for offset in [f32::NAN, f32::INFINITY, -0.01, 1.01] {
            assert!(eastward_delta(64.0, offset).is_none());
            assert!(reversal_offset(offset, 64.0, -64.0, false).is_none());
            assert!(carrier_step(offset, 0.5).is_none());
            assert!(carrier_step(0.5, offset).is_none());
        }
        assert!(eastward_delta(f32::NAN, 0.5).is_none());
        assert!(reversal_offset(0.5, f32::INFINITY, 0.0, false).is_none());
    }
}
