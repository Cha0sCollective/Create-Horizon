// Copyright (c) The Create Team / The Creators of Create
// Copyright (c) 2026 Create Horizon contributors
// SPDX-License-Identifier: MIT
//! Finite east-facing portable-item encounter, source-informed from Create
//! 6.0.11 / 7ff2760e PortableStorageInterfaceBlockEntity and Movement.
//! Item custody and host inventory access remain the runtime adapter's job.

pub const ANIMATION: i32 = 4;
pub const TIMEOUT: i32 = 60;
pub const CONNECTING: i32 = 14;
pub const TRANSFERRED: i32 = TIMEOUT + ANIMATION;
pub const MAX_TIMER: i32 = TIMEOUT + ANIMATION * 2;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Connection {
    pub timer: i32,
    pub keep_alive: i32,
    pub working: bool,
    pub linked: bool,
    pub stalled: bool,
}

impl Connection {
    pub fn valid(self) -> bool {
        (0..=MAX_TIMER).contains(&self.timer)
            && (0..=2).contains(&self.keep_alive)
            && (!self.linked || self.timer > 0)
            && (!self.stalled || self.working)
    }

    /// First in the owner tick. A new encounter later in that tick is not aged.
    pub fn timer_tick(&mut self) {
        if self.keep_alive > 0 {
            self.keep_alive -= 1;
            if self.keep_alive == 0 {
                self.linked = false;
                self.timer = ANIMATION - 1;
                return;
            }
        }
        self.timer = self.timer.clamp(0, MAX_TIMER);
        if self.timer > ANIMATION || (self.timer > 0 && self.keep_alive == 0) {
            self.timer -= 1;
            if self.timer == 0 {
                self.linked = false;
            }
        }
    }

    /// A first/new-cell/explicit refresh visit. The caller checked the actual
    /// matching stationary block and power, never just a saved station link.
    pub fn encounter(&mut self, offset: f32, stationary_available: bool) -> bool {
        self.working = active_cell(offset) == Some(4) && stationary_available;
        if self.working && !self.linked {
            self.linked = true;
            self.timer = CONNECTING;
        }
        self.working
    }

    /// Actor tick follows safe motion. `motion` is the last actual X step,
    /// zero while already stalled. The target center is X+4.5 relative to base.
    pub fn actor_tick(&mut self, offset: f32, motion: f32, stationary_available: bool) {
        if !self.working || !stationary_available {
            self.working = false;
            self.stalled = false;
            return;
        }
        if !self.linked {
            self.linked = true;
            self.timer = CONNECTING;
        }
        self.keep_alive = 2;
        if let Some(stall) = nearest_step(offset, motion) {
            if !self.stalled && stall {
                self.stalled = true;
            }
        } else {
            self.close();
            return;
        }
        if self.stalled && self.timer <= ANIMATION {
            self.stalled = false;
        }
    }

    /// Live world/power/owner admission must also pass at the mutable hopper
    /// source boundary. This method grants no capability on its own.
    pub fn connected(self) -> bool {
        self.working && self.linked && (ANIMATION..=TRANSFERRED).contains(&self.timer)
    }

    pub fn content_transferred(&mut self) -> bool {
        if !self.connected() {
            return false;
        }
        self.timer = TRANSFERRED;
        true
    }

    pub fn close(&mut self) {
        self.working = false;
        self.linked = false;
        self.stalled = false;
        self.keep_alive = 0;
        self.timer = ANIMATION - 1;
    }

    pub fn animation(self) -> f32 {
        if (ANIMATION..=TRANSFERRED).contains(&self.timer) {
            1.0
        } else if self.timer >= TRANSFERRED {
            (MAX_TIMER - self.timer) as f32 / ANIMATION as f32
        } else {
            self.timer as f32 / ANIMATION as f32
        }
        .clamp(0.0, 1.0)
    }
}

/// The moving interface is at X+2, its center at X+2.5, active offset1.85.
pub fn active_cell(offset: f32) -> Option<i32> {
    (offset.is_finite() && (0.0..=1.0).contains(&offset)).then(|| (4.35 + offset).floor() as i32)
}

pub fn nearest_step(offset: f32, motion: f32) -> Option<bool> {
    active_cell(offset)?;
    if !motion.is_finite() || !(0.0..=0.49).contains(&motion) {
        return None;
    }
    let relative = offset - 0.15;
    Some(relative.abs() < (relative + motion).abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encounter_is_before_the_nearest_discrete_stall_and_times_out() {
        let mut c = Connection::default();
        assert_eq!(active_cell(0.0), Some(4));
        assert!(c.encounter(0.0, true));
        assert_eq!(c.timer, 14);
        assert!(c.connected());
        c.actor_tick(0.0, 0.125, true);
        assert!(!c.stalled);
        c.timer_tick();
        c.actor_tick(0.125, 0.125, true);
        assert!(c.stalled);
        assert_eq!(c.timer, 13);
        for _ in 0..9 {
            c.timer_tick();
            c.actor_tick(0.125, 0.0, true);
        }
        assert_eq!(c.timer, 4);
        assert!(c.connected() && !c.stalled);
    }

    #[test]
    fn actual_transfer_resets_timeout_and_closed_or_powered_link_cannot_transfer() {
        let mut c = Connection::default();
        assert!(!c.content_transferred());
        c.encounter(0.125, true);
        c.actor_tick(0.125, 0.125, true);
        assert!(c.content_transferred());
        assert_eq!(c.timer, 64);
        c.timer_tick();
        assert_eq!(c.timer, 63);
        c.actor_tick(0.125, 0.0, false);
        assert!(!c.connected() && !c.content_transferred());
        assert!(!c.stalled);
    }

    #[test]
    fn departure_closes_access_then_expires_keepalive_without_recreating_an_item() {
        let mut c = Connection::default();
        c.encounter(0.5, true);
        c.actor_tick(0.5, 0.001953125, true);
        c.encounter(0.75, true);
        c.actor_tick(0.75, 0.001953125, true);
        assert!(!c.connected());
        c.timer_tick();
        assert_eq!(c.keep_alive, 1);
        c.timer_tick();
        assert_eq!(c.timer, 3);
        assert!(!c.linked && !c.stalled);
        assert_eq!(c.animation(), 0.75);
    }

    #[test]
    fn malformed_ranges_and_reverse_or_nonfinite_motion_are_rejected() {
        assert!(
            !Connection {
                timer: 69,
                ..Default::default()
            }
            .valid()
        );
        assert!(
            !Connection {
                keep_alive: 3,
                ..Default::default()
            }
            .valid()
        );
        assert!(
            !Connection {
                linked: true,
                ..Default::default()
            }
            .valid()
        );
        assert_eq!(active_cell(f32::NAN), None);
        assert_eq!(nearest_step(0.1, -0.01), None);
        assert_eq!(nearest_step(0.1, f32::INFINITY), None);
    }
}
