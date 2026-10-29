//! Source-informed static dry X/Z geometry, not world discovery or propagation timing.
//!
//! The caller supplies every kinetic block in one closed fixture. Y, other motor
//! facings, equal-cog meshes and perpendicular large-cog meshes are unsupported.
//! See docs/design/P02_NATIVE_GEOMETRY_PACKET.md for measured versus source-only scope.
use crate::{Edge, MotorSetting, Node, Source, Transmission};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn index(self) -> usize {
        match self {
            Self::X => 0,
            Self::Y => 1,
            Self::Z => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Facing {
    East,
    West,
    Up,
    Down,
    South,
    North,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockKind {
    Motor {
        facing: Facing,
        setting: MotorSetting,
        capacity_per_rpm: f32,
    },
    Shaft {
        axis: Axis,
    },
    SmallCog {
        axis: Axis,
    },
    LargeCog {
        axis: Axis,
    },
}

impl BlockKind {
    fn passive_axis(self) -> Option<Axis> {
        match self {
            Self::Motor { .. } => None,
            Self::Shaft { axis } | Self::SmallCog { axis } | Self::LargeCog { axis } => Some(axis),
        }
    }

    fn is_small(self) -> bool {
        matches!(self, Self::SmallCog { .. })
    }

    fn is_large(self) -> bool {
        matches!(self, Self::LargeCog { .. })
    }

    fn exposes_shaft(self, direction: [i64; 3]) -> bool {
        match self {
            Self::Motor {
                facing: Facing::East,
                ..
            } => direction == [1, 0, 0],
            Self::Motor {
                facing: Facing::North,
                ..
            } => direction == [0, 0, -1],
            Self::Motor { .. } => false,
            _ => self.passive_axis().is_some_and(|axis| {
                direction[axis.index()].abs() == 1
                    && direction
                        .iter()
                        .enumerate()
                        .all(|(i, d)| i == axis.index() || *d == 0)
            }),
        }
    }
}

/// Dry, present block in a caller-owned complete static fixture, with no dimension/lifetime meaning.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Block {
    pub id: u32,
    pub position: [i32; 3],
    pub kind: BlockKind,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Topology {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeometryError {
    DuplicateId(u32),
    DuplicatePosition(u32, u32),
    UnsupportedAxis(u32, Axis),
    UnsupportedFacing(u32, Facing),
    InvalidCogPlacement(u32, u32),
    SmallCogMeshUnsupported(u32, u32),
    PerpendicularLargeCogsUnsupported(u32, u32),
}

fn difference(from: &Block, to: &Block) -> [i64; 3] {
    std::array::from_fn(|i| i64::from(to.position[i]) - i64::from(from.position[i]))
}

fn unit_axis(diff: [i64; 3]) -> Option<usize> {
    if diff.iter().map(|v| v.abs()).sum::<i64>() != 1 {
        return None;
    }
    diff.iter().position(|v| *v != 0)
}

fn legal_cog_placement(blocks: &[Block]) -> Result<(), GeometryError> {
    for cog in blocks
        .iter()
        .filter(|b| b.kind.is_small() || b.kind.is_large())
    {
        let cog_axis = cog.kind.passive_axis().unwrap().index();
        for other in blocks {
            let Some(face_axis) = unit_axis(difference(cog, other)) else {
                continue;
            };
            if face_axis == cog_axis
                || other
                    .kind
                    .passive_axis()
                    .is_some_and(|axis| axis.index() == face_axis)
            {
                continue;
            }
            if other.kind.is_large() || (cog.kind.is_large() && other.kind.is_small()) {
                return Err(GeometryError::InvalidCogPlacement(cog.id, other.id));
            }
        }
    }
    Ok(())
}

fn pair_edge(a: &Block, b: &Block) -> Result<Option<Edge>, GeometryError> {
    let diff = difference(a, b);
    if unit_axis(diff).is_some()
        && a.kind.exposes_shaft(diff)
        && b.kind.exposes_shaft(diff.map(|v| -v))
    {
        return Ok(Some(Edge {
            from: a.id,
            to: b.id,
            transmission: Transmission::Shaft,
        }));
    }
    let axes = (a.kind.passive_axis(), b.kind.passive_axis());
    if let (Some(axis_a), Some(axis_b)) = axes {
        if axis_a == axis_b {
            let axis = axis_a.index();
            if ((a.kind.is_large() && b.kind.is_small())
                || (a.kind.is_small() && b.kind.is_large()))
                && diff[axis] == 0
                && diff
                    .iter()
                    .enumerate()
                    .all(|(i, d)| i == axis || d.abs() == 1)
            {
                let (large, small) = if a.kind.is_large() {
                    (a.id, b.id)
                } else {
                    (b.id, a.id)
                };
                return Ok(Some(Edge {
                    from: large,
                    to: small,
                    transmission: Transmission::LargeToSmall,
                }));
            }
            if a.kind.is_small()
                && b.kind.is_small()
                && unit_axis(diff).is_some_and(|face| face != axis)
            {
                return Err(GeometryError::SmallCogMeshUnsupported(a.id, b.id));
            }
        } else if a.kind.is_large()
            && b.kind.is_large()
            && diff.iter().enumerate().all(|(i, d)| {
                if i == axis_a.index() || i == axis_b.index() {
                    d.abs() == 1
                } else {
                    *d == 0
                }
            })
        {
            return Err(GeometryError::PerpendicularLargeCogsUnsupported(a.id, b.id));
        }
    }
    Ok(None)
}

/// Derive canonical explicit topology, retaining the existing solver's facts and errors.
///
/// No Create source-parent, transition, destruction, world-load or arbitration behavior
/// is inferred. Call `crate::solve` on the result; unsupported solver cases remain errors.
pub fn derive_topology(blocks: &[Block]) -> Result<Topology, GeometryError> {
    let mut by_id = BTreeMap::new();
    let mut positions = BTreeMap::new();
    for block in blocks {
        if by_id.insert(block.id, *block).is_some() {
            return Err(GeometryError::DuplicateId(block.id));
        }
    }
    let blocks: Vec<_> = by_id.into_values().collect();
    for block in &blocks {
        if let Some(other) = positions.insert(block.position, block.id) {
            return Err(GeometryError::DuplicatePosition(other, block.id));
        }
    }
    for block in &blocks {
        match block.kind {
            BlockKind::Shaft { axis: Axis::Y }
            | BlockKind::SmallCog { axis: Axis::Y }
            | BlockKind::LargeCog { axis: Axis::Y } => {
                return Err(GeometryError::UnsupportedAxis(block.id, Axis::Y));
            }
            BlockKind::Motor { facing, .. } if !matches!(facing, Facing::East | Facing::North) => {
                return Err(GeometryError::UnsupportedFacing(block.id, facing));
            }
            _ => {}
        }
    }
    legal_cog_placement(&blocks)?;
    let mut edges = Vec::new();
    for (i, a) in blocks.iter().enumerate() {
        for b in &blocks[i + 1..] {
            if let Some(edge) = pair_edge(a, b)? {
                edges.push(edge);
            }
        }
    }
    // Enumeration is by the sorted unordered pair, including reversed size edges.
    let nodes = blocks
        .iter()
        .map(|block| {
            let source = match block.kind {
                BlockKind::Motor {
                    facing,
                    setting,
                    capacity_per_rpm,
                } => Some(Source {
                    rpm: f32::from(setting.rpm())
                        * if facing == Facing::North { -1.0 } else { 1.0 },
                    capacity_per_rpm,
                }),
                _ => None,
            };
            Node {
                id: block.id,
                impact_per_rpm: 0.0,
                source,
            }
        })
        .collect();
    Ok(Topology { nodes, edges })
}
