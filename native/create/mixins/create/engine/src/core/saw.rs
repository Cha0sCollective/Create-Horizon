// Copyright (c) The Create Team / The Creators of Create
// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
//! Stationary SawBlockEntity's item-processing clock. Inventory and recipe
//! lookup remain in the native adapter; horizontal block breaking is absent.

pub const SLOTS: usize = 32;
pub const POST_PROCESS_UNITS: f32 = 20.0;
pub const NO_RECIPE_UNITS: f32 = 10.0;

/// ProcessingInventory's saved float fields and the saw's initial -1 sentinel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SawCycle {
    pub remaining_time: f32,
    pub recipe_duration: f32,
    pub applied_recipe: bool,
}

impl Default for SawCycle {
    fn default() -> Self {
        Self {
            remaining_time: -1.0,
            recipe_duration: 0.0,
            applied_recipe: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    None,
    Armed,
    Processing,
    /// Recipe matching is rechecked when the saw applies it, as in Java.
    Apply {
        supported: bool,
    },
    Eject,
}

pub fn processing_speed(rpm: f32) -> f32 {
    (rpm.abs() / 24.0).clamp(1.0, 128.0)
}

/// Default bulkCutting=false: accept exactly one from a landed stack and
/// leave the remaining items in the original world entity.
pub fn take_one(count: u32) -> Option<(u32, u32)> {
    (count > 0).then_some((1, count.saturating_sub(1)))
}

impl SawCycle {
    pub fn tick(&mut self, rpm: f32, occupied: bool, recipe_duration: Option<i32>) -> Action {
        if rpm == 0.0 {
            return Action::None;
        }
        if self.remaining_time == -1.0 {
            if !occupied || self.applied_recipe {
                return Action::None;
            }
            self.recipe_duration = recipe_duration.map_or(NO_RECIPE_UNITS, |v| v as f32);
            self.remaining_time = self.recipe_duration;
            return Action::Armed;
        }
        self.remaining_time -= processing_speed(rpm);
        if self.remaining_time < 5.0 && !self.applied_recipe {
            self.applied_recipe = true;
            self.recipe_duration = POST_PROCESS_UNITS;
            self.remaining_time = POST_PROCESS_UNITS;
            return Action::Apply {
                supported: recipe_duration.is_some(),
            };
        }
        if self.remaining_time <= 0.0 {
            *self = Self::default();
            return Action::Eject;
        }
        Action::Processing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_hundred_and_twenty_processing_units_scale_with_signed_rpm() {
        assert_eq!(processing_speed(0.0), 1.0);
        assert_eq!(processing_speed(24.0), 1.0);
        assert_eq!(processing_speed(-48.0), 2.0);
        assert_eq!(processing_speed(256.0), 256.0 / 24.0);
        assert_eq!(processing_speed(4096.0), 128.0);
        let mut cycle = SawCycle::default();
        assert_eq!(cycle.tick(24.0, true, Some(200)), Action::Armed);
        for _ in 0..195 {
            assert_eq!(cycle.tick(24.0, true, Some(200)), Action::Processing);
        }
        assert_eq!(
            cycle.tick(24.0, true, Some(200)),
            Action::Apply { supported: true }
        );
        assert_eq!(cycle.remaining_time, 20.0);
        for _ in 0..19 {
            assert_eq!(cycle.tick(24.0, true, Some(200)), Action::Processing);
        }
        assert_eq!(cycle.tick(24.0, true, Some(200)), Action::Eject);
        assert_eq!(cycle, SawCycle::default());
    }

    #[test]
    fn stopped_saw_preserves_input_phase_and_resumes_without_rearming() {
        let mut cycle = SawCycle::default();
        assert_eq!(cycle.tick(48.0, true, Some(200)), Action::Armed);
        assert_eq!(cycle.tick(48.0, true, Some(200)), Action::Processing);
        let saved = cycle;
        for _ in 0..35 {
            assert_eq!(cycle.tick(0.0, true, Some(200)), Action::None);
        }
        assert_eq!(cycle, saved);
        assert_eq!(cycle.tick(-48.0, true, Some(200)), Action::Processing);
        assert_eq!(cycle.remaining_time, saved.remaining_time - 2.0);
    }

    #[test]
    fn unsupported_input_runs_short_clock_and_exits_without_recipe_application() {
        let mut cycle = SawCycle::default();
        assert_eq!(cycle.tick(24.0, true, None), Action::Armed);
        assert_eq!(cycle.recipe_duration, NO_RECIPE_UNITS);
        for _ in 0..5 {
            assert_eq!(cycle.tick(24.0, true, None), Action::Processing);
        }
        assert_eq!(
            cycle.tick(24.0, true, None),
            Action::Apply { supported: false }
        );
        for _ in 0..19 {
            assert_eq!(cycle.tick(24.0, true, None), Action::Processing);
        }
        assert_eq!(cycle.tick(24.0, true, None), Action::Eject);
        assert_eq!(cycle, SawCycle::default());
    }

    #[test]
    fn no_input_does_not_arm_and_changed_recipe_is_checked_at_application() {
        let mut cycle = SawCycle::default();
        assert_eq!(cycle.tick(24.0, false, Some(200)), Action::None);
        assert_eq!(cycle.tick(256.0, true, Some(200)), Action::Armed);
        for _ in 0..18 {
            assert_eq!(cycle.tick(256.0, true, None), Action::Processing);
        }
        assert_eq!(
            cycle.tick(256.0, true, None),
            Action::Apply { supported: false }
        );
    }

    #[test]
    fn one_item_intake_preserves_the_world_remainder() {
        assert_eq!(take_one(0), None);
        assert_eq!(take_one(1), Some((1, 0)));
        assert_eq!(take_one(3), Some((1, 2)));
    }
}
