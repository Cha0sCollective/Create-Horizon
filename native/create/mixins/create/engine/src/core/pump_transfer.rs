// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
// One Tank -> one Pump -> one straight Pipe -> one Basin. These are the
// arithmetic/custody decisions used by the native endpoint adapter, not a
// general fluid network or another owner of stored water.

pub const FIRST_INPUT_CAPACITY: u32 = 1_000;
pub const TANK_CAPACITY: u32 = 8_000;
const MAX_RPM: f32 = 256.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Volumes {
    pub tank: u32,
    pub basin_first_input: u32,
}

impl Volumes {
    pub fn total(self) -> Option<u32> {
        self.tank.checked_add(self.basin_first_input)
    }

    pub fn restored_to(self, before: Self) -> bool {
        self == before && self.total().is_some() && self.total() == before.total()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Step {
    pub before: Volumes,
    pub amount: u32,
}

/// Pump facing sets direction; signed speed only supplies pressure magnitude.
/// The selected single path takes floor(abs(speed)/2), at least 1 mB per active
/// tick. Invalid/nonfinite/out-of-kinetic-range speed refuses without a write.
pub fn rate(rpm: f32) -> u32 {
    if !rpm.is_finite() || rpm == 0.0 || rpm.abs() > MAX_RPM {
        return 0;
    }
    ((rpm.abs() / 2.0).floor() as u32).max(1)
}

impl Step {
    pub fn maximum(rpm: f32, before: Volumes) -> Option<u32> {
        if before.tank > TANK_CAPACITY || before.total().is_none() {
            return None;
        }
        let room = FIRST_INPUT_CAPACITY.checked_sub(before.basin_first_input)?;
        let amount = rate(rpm).min(before.tank).min(room);
        (amount > 0).then_some(amount)
    }

    /// Destination simulation can accept less than the maximum. The source
    /// must simulate the exact amount selected by that destination.
    pub fn from_simulation(
        before: Volumes,
        maximum: u32,
        destination_accepted: i32,
        source_exact: Option<u32>,
    ) -> Option<Self> {
        let accepted = u32::try_from(destination_accepted).ok()?;
        if accepted == 0
            || accepted > maximum
            || accepted > before.tank
            || before.basin_first_input.checked_add(accepted)? > FIRST_INPUT_CAPACITY
            || before.total().is_none()
            || source_exact != Some(accepted)
        {
            return None;
        }
        Some(Self {
            before,
            amount: accepted,
        })
    }

    /// API receipts are insufficient: actual canonical deltas and census must
    /// agree with both receipts before the transfer can be counted.
    pub fn committed(self, after: Volumes, drained: Option<u32>, filled: i32) -> bool {
        let Ok(expected_receipt) = i32::try_from(self.amount) else {
            return false;
        };
        let (Some(tank), Some(basin), Some(total)) = (
            self.before.tank.checked_sub(self.amount),
            self.before.basin_first_input.checked_add(self.amount),
            self.before.total(),
        ) else {
            return false;
        };
        drained == Some(self.amount)
            && filled == expected_receipt
            && after.tank == tank
            && after.basin_first_input == basin
            && after.total() == Some(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facing_not_speed_sign_sets_the_direction_and_rate() {
        assert_eq!(rate(16.0), 8);
        assert_eq!(rate(-16.0), 8);
        assert_eq!(rate(1.0), 1);
        assert_eq!(rate(0.0), 0);
        assert_eq!(rate(f32::NAN), 0);
        assert_eq!(rate(f32::INFINITY), 0);
        assert_eq!(rate(257.0), 0);
    }

    #[test]
    fn first_input_room_limits_the_step_and_full_target_refuses_with_water_left() {
        assert_eq!(
            Step::maximum(
                -16.0,
                Volumes {
                    tank: 1_000,
                    basin_first_input: 995,
                }
            ),
            Some(5)
        );
        assert_eq!(
            Step::maximum(
                16.0,
                Volumes {
                    tank: 1_000,
                    basin_first_input: 1_000,
                }
            ),
            None
        );
    }

    #[test]
    fn simulation_requires_matching_exact_source_and_destination() {
        let before = Volumes {
            tank: 1_000,
            basin_first_input: 0,
        };
        assert_eq!(
            Step::from_simulation(before, 8, 5, Some(5)).unwrap().amount,
            5
        );
        assert!(Step::from_simulation(before, 8, 5, Some(4)).is_none());
        assert!(Step::from_simulation(before, 8, 9, Some(9)).is_none());
        assert!(Step::from_simulation(before, 8, 0, None).is_none());
    }

    #[test]
    fn receipts_and_real_owners_must_agree_or_restore() {
        let before = Volumes {
            tank: 1_000,
            basin_first_input: 400,
        };
        let step = Step::from_simulation(before, 8, 8, Some(8)).unwrap();
        let valid = Volumes {
            tank: 992,
            basin_first_input: 408,
        };
        assert!(step.committed(valid, Some(8), 8));
        assert!(!step.committed(valid, Some(7), 8));
        assert!(!step.committed(valid, Some(8), 7));
        assert!(!step.committed(
            Volumes {
                tank: 992,
                basin_first_input: 407,
            },
            Some(8),
            8
        ));
        assert!(before.restored_to(before));
        assert!(!valid.restored_to(before));
    }

    #[test]
    fn destination_rejection_after_drain_requires_restoration() {
        let before = Volumes {
            tank: 1_000,
            basin_first_input: 400,
        };
        let step = Step::from_simulation(before, 8, 8, Some(8)).unwrap();
        let source_drained_only = Volumes {
            tank: 992,
            basin_first_input: 400,
        };
        assert!(!step.committed(source_drained_only, Some(8), 0));
        assert!(!source_drained_only.restored_to(before));
        assert!(before.restored_to(before));
    }

    #[test]
    fn malformed_public_amounts_refuse_without_overflow() {
        assert_eq!(
            Step::maximum(
                16.0,
                Volumes {
                    tank: u32::MAX,
                    basin_first_input: 1,
                }
            ),
            None
        );
        let malformed = Step {
            before: Volumes {
                tank: 1,
                basin_first_input: 1_000,
            },
            amount: 2,
        };
        assert!(!malformed.committed(malformed.before, Some(2), 2));
    }
}
