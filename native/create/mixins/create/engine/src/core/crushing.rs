// Copyright (c) The Create Team / The Creators of Create
// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
//! Selected one-item CrushingWheelControllerBlockEntity clock, without world IO.
//! The Java 6.0.11 controller is the behavioral reference for this bounded path.

pub const SLOTS: usize = 32;
pub const NOMINAL_TIME: f32 = 250.0;

/// The adapter has already established matching wheel axes and the middle gap.
/// Pair validity uses effective RPM, so overstressed wheels cannot process.
pub fn opposing(first: f32, second: f32) -> bool {
    first.is_finite()
        && second.is_finite()
        && first != 0.0
        && second != 0.0
        && first.is_sign_positive() != second.is_sign_positive()
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Cycle {
    pub remaining_time: f32,
    pub recipe_duration: f32,
    pub applied_recipe: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    None,
    Apply,
    Eject,
}

impl Cycle {
    /// Called only after the one cobblestone world item is confirmed in slot 0.
    pub fn insert_one(&mut self) {
        self.remaining_time = NOMINAL_TIME;
        self.recipe_duration = NOMINAL_TIME;
        self.applied_recipe = false;
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// The selected stack count is one. Java divides by log2(count) before
    /// clamping, so its pre-apply +infinity step becomes 20 time units/tick.
    /// Applying clears ProcessingInventory's clock and sets AppliedRecipe;
    /// the next valid powered tick ejects the resulting output.
    pub fn tick(&mut self, valid: bool, wheel_rpm: f32, occupied: bool) -> Action {
        if !valid
            || !occupied
            || !wheel_rpm.is_finite()
            || wheel_rpm == 0.0
            || !self.remaining_time.is_finite()
            || !self.recipe_duration.is_finite()
        {
            return Action::None;
        }
        let step = if self.applied_recipe {
            (wheel_rpm.abs() / 50.0 * 4.0).clamp(0.25, 20.0)
        } else {
            20.0
        };
        self.remaining_time -= step;
        if self.remaining_time < 20.0 && !self.applied_recipe {
            self.remaining_time = 0.0;
            self.recipe_duration = 0.0;
            self.applied_recipe = true;
            return Action::Apply;
        }
        if self.remaining_time > 0.0 {
            return Action::None;
        }
        self.remaining_time = 0.0;
        Action::Eject
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_opposite_nonzero_effective_speeds_form_a_processing_pair() {
        assert!(opposing(16.0, -16.0));
        assert!(opposing(-16.0, 32.0));
        assert!(!opposing(16.0, 16.0));
        assert!(!opposing(0.0, -16.0));
        assert!(!opposing(-0.0, 16.0));
        assert!(!opposing(f32::NAN, 16.0));
    }

    #[test]
    fn one_item_applies_after_twelve_valid_ticks_then_ejects_once() {
        let mut cycle = Cycle::default();
        cycle.insert_one();
        assert_eq!(cycle.recipe_duration, 250.0);
        for tick in 1..12 {
            assert_eq!(cycle.tick(true, 16.0, true), Action::None, "tick {tick}");
            assert_eq!(cycle.remaining_time, 250.0 - 20.0 * tick as f32);
        }
        assert_eq!(cycle.tick(true, 16.0, true), Action::Apply);
        assert_eq!(cycle.remaining_time, 0.0);
        assert_eq!(cycle.recipe_duration, 0.0);
        assert!(cycle.applied_recipe);
        assert_eq!(cycle.tick(true, 16.0, true), Action::Eject);
        cycle.clear();
        assert_eq!(cycle, Cycle::default());
        assert_eq!(cycle.tick(true, 16.0, false), Action::None);
    }

    #[test]
    fn stopped_and_invalid_pair_preserve_saved_clock_before_and_after_apply() {
        let mut cycle = Cycle::default();
        cycle.insert_one();
        for _ in 0..5 {
            assert_eq!(cycle.tick(true, 32.0, true), Action::None);
        }
        let saved = cycle;
        for _ in 0..35 {
            assert_eq!(cycle.tick(false, 32.0, true), Action::None);
            assert_eq!(cycle.tick(true, 0.0, true), Action::None);
        }
        assert_eq!(cycle, saved);
        let mut cold = saved;
        for _ in 0..6 {
            assert_eq!(cold.tick(true, -32.0, true), Action::None);
        }
        assert_eq!(cold.tick(true, -32.0, true), Action::Apply);
        assert_eq!(cold.tick(false, -32.0, true), Action::None);
        assert_eq!(cold.tick(true, -32.0, true), Action::Eject);
    }
}
