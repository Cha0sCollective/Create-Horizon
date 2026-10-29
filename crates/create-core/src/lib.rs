//! Settled single-source kinetics over explicit topology, measured by P01.
//!
//! This does not discover world connections or model propagation tick timing,
//! multiple-source arbitration, unloaded contributions, or overspeed destruction.
pub mod declared_arbitration;
pub mod geometry;

use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// A normal motor control never selects zero.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MotorSetting(i16);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
    Negative,
    Positive,
}

impl Default for MotorSetting {
    fn default() -> Self {
        Self(16)
    }
}

impl MotorSetting {
    pub fn from_signed(rpm: i16) -> Result<Self, InvalidMotorSetting> {
        if rpm == 0 || !(-256..=256).contains(&rpm) {
            return Err(InvalidMotorSetting);
        }
        Ok(Self(rpm))
    }
    /// Reference value-board rows select sign; magnitude is clamped to 1..256.
    pub fn from_board(sign: Sign, magnitude: i32) -> Self {
        let magnitude = magnitude.clamp(1, 256) as i16;
        Self(match sign {
            Sign::Negative => -magnitude,
            Sign::Positive => magnitude,
        })
    }
    pub fn rpm(self) -> i16 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidMotorSetting;

/// Ratio from an edge's `from` node to its `to` node. Geometry belongs to the adapter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transmission {
    Shaft,
    EqualCogs,
    LargeToSmall,
    SmallToLarge,
}

impl Transmission {
    fn ratio(self, reverse: bool) -> f32 {
        let ratio = match self {
            Self::Shaft => 1.0,
            Self::EqualCogs => -1.0,
            Self::LargeToSmall => -2.0,
            Self::SmallToLarge => -0.5,
        };
        if reverse { 1.0 / ratio } else { ratio }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Source {
    /// Signed RPM in the caller's declared axis convention; zero is an inactive source.
    pub rpm: f32,
    pub capacity_per_rpm: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Node {
    pub id: u32,
    pub impact_per_rpm: f32,
    pub source: Option<Source>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Edge {
    pub from: u32,
    pub to: u32,
    pub transmission: Transmission,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OperatingState {
    Unpowered,
    Running,
    Overstressed,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rotation {
    pub id: u32,
    pub theoretical_rpm: f32,
    pub effective_rpm: f32,
    pub stress_su: f32,
    pub capacity_su: f32,
    pub state: OperatingState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SolveError {
    DuplicateNode(u32),
    InvalidFacts(u32),
    UnknownNode(u32),
    InvalidEdge(u32, u32),
    MultipleSourcesUnsupported(u32),
    ConflictingCycleUnsupported(u32, u32),
    OverspeedUnsupported(u32),
    NumericalRange(u32),
}

/// Recompute a settled solution from the complete supplied topology.
///
/// Results and accumulation order are sorted by node ID. No previous solution is
/// retained, so a removed connection cannot leave cached power in this kernel.
/// Callers must retain unloaded-world accounting themselves. Validation errors
/// return no partial result. Unsupported cases are not guessed Create behavior.
pub fn solve(nodes: &[Node], edges: &[Edge]) -> Result<Vec<Rotation>, SolveError> {
    let mut by_id = BTreeMap::new();
    for node in nodes {
        if by_id.insert(node.id, *node).is_some() {
            return Err(SolveError::DuplicateNode(node.id));
        }
        if !node.impact_per_rpm.is_finite() || node.impact_per_rpm < 0.0 {
            return Err(SolveError::InvalidFacts(node.id));
        }
        if let Some(source) = node.source {
            if !source.rpm.is_finite()
                || !source.capacity_per_rpm.is_finite()
                || source.capacity_per_rpm < 0.0
            {
                return Err(SolveError::InvalidFacts(node.id));
            }
            if source.rpm.abs() > 256.0 {
                return Err(SolveError::OverspeedUnsupported(node.id));
            }
        }
    }
    let mut adjacency: BTreeMap<_, Vec<(u32, f32)>> =
        by_id.keys().map(|id| (*id, Vec::new())).collect();
    let mut pairs = BTreeSet::new();
    for edge in edges {
        for id in [edge.from, edge.to] {
            if !by_id.contains_key(&id) {
                return Err(SolveError::UnknownNode(id));
            }
        }
        if edge.from == edge.to || !pairs.insert((edge.from.min(edge.to), edge.from.max(edge.to))) {
            return Err(SolveError::InvalidEdge(edge.from, edge.to));
        }
        adjacency
            .get_mut(&edge.from)
            .unwrap()
            .push((edge.to, edge.transmission.ratio(false)));
        adjacency
            .get_mut(&edge.to)
            .unwrap()
            .push((edge.from, edge.transmission.ratio(true)));
    }
    for neighbors in adjacency.values_mut() {
        neighbors.sort_by_key(|(id, _)| *id);
    }
    let mut visited = BTreeSet::new();
    let mut result = Vec::with_capacity(nodes.len());
    for root in by_id.keys().copied() {
        if visited.contains(&root) {
            continue;
        }
        let mut component = BTreeSet::new();
        let mut pending = vec![root];
        while let Some(id) = pending.pop() {
            if !component.insert(id) {
                continue;
            }
            visited.insert(id);
            pending.extend(adjacency[&id].iter().map(|(other, _)| *other));
        }
        let sources: Vec<_> = component
            .iter()
            .filter(|id| by_id[id].source.is_some())
            .copied()
            .collect();
        if sources.len() > 1 {
            return Err(SolveError::MultipleSourcesUnsupported(root));
        }
        let anchor = sources.first().copied().unwrap_or(root);
        let mut weights = BTreeMap::from([(anchor, 1.0_f32)]);
        let mut pending = VecDeque::from([anchor]);
        while let Some(id) = pending.pop_front() {
            for (other, ratio) in &adjacency[&id] {
                let candidate = weights[&id] * ratio;
                if !candidate.is_finite() || candidate == 0.0 {
                    return Err(SolveError::NumericalRange(*other));
                }
                if let Some(existing) = weights.get(other) {
                    if *existing != candidate {
                        return Err(SolveError::ConflictingCycleUnsupported(id, *other));
                    }
                } else {
                    weights.insert(*other, candidate);
                    pending.push_back(*other);
                }
            }
        }
        let source = by_id[&anchor].source;
        let source_rpm = source.map_or(0.0, |source| source.rpm);
        let capacity = source.map_or(0.0, |source| source.rpm.abs() * source.capacity_per_rpm);
        if !capacity.is_finite()
            || source.is_some_and(|s| s.rpm != 0.0 && s.capacity_per_rpm != 0.0 && capacity == 0.0)
        {
            return Err(SolveError::NumericalRange(anchor));
        }
        let mut speeds = BTreeMap::new();
        let mut stress = 0.0_f32;
        for id in &component {
            let speed = source_rpm * weights[id];
            if !speed.is_finite() || (source_rpm != 0.0 && speed == 0.0) {
                return Err(SolveError::NumericalRange(*id));
            }
            if speed.abs() > 256.0 {
                return Err(SolveError::OverspeedUnsupported(*id));
            }
            speeds.insert(*id, speed);
            let contribution = by_id[id].impact_per_rpm * speed.abs();
            if by_id[id].impact_per_rpm != 0.0 && speed != 0.0 && contribution == 0.0 {
                return Err(SolveError::NumericalRange(*id));
            }
            stress += contribution;
            if !stress.is_finite() {
                return Err(SolveError::NumericalRange(*id));
            }
        }
        let state = if source_rpm == 0.0 {
            OperatingState::Unpowered
        } else if stress > capacity {
            OperatingState::Overstressed
        } else {
            OperatingState::Running
        };
        for id in component {
            result.push(Rotation {
                id,
                theoretical_rpm: speeds[&id],
                effective_rpm: if state == OperatingState::Running {
                    speeds[&id]
                } else {
                    0.0
                },
                stress_su: stress,
                capacity_su: capacity,
                state,
            });
        }
    }
    result.sort_by_key(|rotation| rotation.id);
    Ok(result)
}
