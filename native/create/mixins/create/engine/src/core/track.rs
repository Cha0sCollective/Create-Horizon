// Copyright (c) The Create Team / The Creators of Create
// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
// Create 6.0.11 TrackShape/TrackBlock: selected flat orthogonal placement only.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Straight {
    X,
    Z,
}

impl Straight {
    pub fn state(self) -> &'static str {
        match self {
            Self::X => "xo",
            Self::Z => "zo",
        }
    }

    pub fn neighbors(self) -> [[i32; 3]; 2] {
        match self {
            Self::X => [[-1, 0, 0], [1, 0, 0]],
            Self::Z => [[0, 0, -1], [0, 0, 1]],
        }
    }
}

/// Accept only headings where the reference chooses an orthogonal shape.
/// Diagonal headings need a different source shape and are left for rail graphs.
pub fn from_yaw(yaw: f32) -> Option<Straight> {
    if !yaw.is_finite() {
        return None;
    }
    let degrees = yaw.rem_euclid(360.0);
    let quarter = (degrees / 90.0).round();
    if (degrees - quarter * 90.0).abs() >= 22.5 {
        return None;
    }
    if (quarter as i64).rem_euclid(2) == 0 {
        Some(Straight::Z)
    } else {
        Some(Straight::X)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flat_reference_headings_and_diagonal_refusal() {
        for yaw in [0.0, 180.0, -180.0, 360.0] {
            assert_eq!(from_yaw(yaw), Some(Straight::Z));
        }
        for yaw in [90.0, -90.0, 270.0] {
            assert_eq!(from_yaw(yaw), Some(Straight::X));
        }
        for yaw in [45.0, 135.0, 22.5, -22.5, f32::NAN] {
            assert_eq!(from_yaw(yaw), None);
        }
        assert_eq!(from_yaw(22.0), Some(Straight::Z));
        assert_eq!(from_yaw(68.0), Some(Straight::X));
    }
}
