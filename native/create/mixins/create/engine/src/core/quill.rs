// Copyright (c) The Create Team / The Creators of Create
// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
// Create 6.0.11 SchematicAndQuillHandler: bounded, client-only corner selection.

pub const MAX_AXIS: u64 = 16;
pub type Pos = [i32; 3];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bounds {
    pub min: Pos,
    pub max: Pos,
    pub size: [u64; 3],
}

impl Bounds {
    pub fn between(first: Pos, second: Pos) -> Self {
        let min = std::array::from_fn(|axis| first[axis].min(second[axis]));
        let max = std::array::from_fn(|axis| first[axis].max(second[axis]));
        let size =
            std::array::from_fn(|axis| (i64::from(max[axis]) - i64::from(min[axis]) + 1) as u64);
        Self { min, max, size }
    }

    pub fn within_limit(self) -> bool {
        self.size.iter().all(|size| *size <= MAX_AXIS)
    }
}

/// Java selects the adjacent cell on a vertical nonreplaceable block face.
/// The native packet uses the ordinary client block hit, without Java's
/// extended 75-block ray or midair-tool mode.
pub fn selected_cell(hit: Pos, face: Pos, replaceable: bool) -> Option<Pos> {
    if face[1] == 0 || replaceable {
        return Some(hit);
    }
    Some([hit[0], hit[1].checked_add(face[1])?, hit[2]])
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Selection {
    first: Option<Pos>,
    second: Option<Pos>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Feedback {
    Canceled,
    NoTarget,
    First(Pos),
    Second(Bounds),
    TooLarge(Bounds),
    Ready(Bounds),
}

impl Selection {
    pub fn first(self) -> Option<Pos> {
        self.first
    }

    pub fn second(self) -> Option<Pos> {
        self.second
    }

    pub fn bounds(self) -> Option<Bounds> {
        Some(Bounds::between(self.first?, self.second?))
    }

    pub fn preview(self, target: Pos) -> Option<Bounds> {
        Some(Bounds::between(self.first?, target))
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    /// A completed pair stays selected. Saving/prompting is a later packet.
    pub fn click(&mut self, target: Option<Pos>, sneaking: bool) -> Feedback {
        if sneaking {
            self.clear();
            return Feedback::Canceled;
        }
        if let Some(bounds) = self.bounds() {
            return Feedback::Ready(bounds);
        }
        let Some(target) = target else {
            return Feedback::NoTarget;
        };
        let Some(first) = self.first else {
            self.first = Some(target);
            return Feedback::First(target);
        };
        let bounds = Bounds::between(first, target);
        if !bounds.within_limit() {
            return Feedback::TooLarge(bounds);
        }
        self.second = Some(target);
        Feedback::Second(bounds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reversed_corners_have_inclusive_dimensions() {
        let mut selection = Selection::default();
        assert_eq!(
            selection.click(Some([12, 70, -2]), false),
            Feedback::First([12, 70, -2])
        );
        assert_eq!(
            selection.click(Some([9, 68, -4]), false),
            Feedback::Second(Bounds {
                min: [9, 68, -4],
                max: [12, 70, -2],
                size: [4, 3, 3],
            })
        );
        assert_eq!(selection.second(), Some([9, 68, -4]));
    }

    #[test]
    fn one_to_sixteen_accepts_and_seventeen_preserves_first() {
        let mut selection = Selection::default();
        selection.click(Some([0, 64, 0]), false);
        assert!(matches!(
            selection.click(Some([16, 64, 0]), false),
            Feedback::TooLarge(Bounds {
                size: [17, 1, 1],
                ..
            })
        ));
        assert_eq!(selection.first(), Some([0, 64, 0]));
        assert_eq!(selection.second(), None);
        assert!(matches!(
            selection.click(Some([-15, 64, 0]), false),
            Feedback::Second(Bounds {
                size: [16, 1, 1],
                ..
            })
        ));
    }

    #[test]
    fn no_target_cancel_and_third_click_do_not_change_unrelated_state() {
        let mut selection = Selection::default();
        assert_eq!(selection.click(None, false), Feedback::NoTarget);
        selection.click(Some([1, 2, 3]), false);
        assert_eq!(selection.click(None, false), Feedback::NoTarget);
        selection.click(Some([2, 3, 4]), false);
        let complete = selection;
        assert!(matches!(selection.click(None, false), Feedback::Ready(_)));
        assert_eq!(selection, complete);
        assert_eq!(selection.click(None, true), Feedback::Canceled);
        assert_eq!(selection, Selection::default());
    }

    #[test]
    fn vertical_nonreplaceable_hit_selects_adjacent_cell() {
        assert_eq!(
            selected_cell([2, 64, 3], [0, 1, 0], false),
            Some([2, 65, 3])
        );
        assert_eq!(
            selected_cell([2, 64, 3], [0, -1, 0], false),
            Some([2, 63, 3])
        );
        assert_eq!(
            selected_cell([2, 64, 3], [1, 0, 0], false),
            Some([2, 64, 3])
        );
        assert_eq!(selected_cell([2, 64, 3], [0, 1, 0], true), Some([2, 64, 3]));
        assert_eq!(selected_cell([2, i32::MAX, 3], [0, 1, 0], false), None);
    }
}
