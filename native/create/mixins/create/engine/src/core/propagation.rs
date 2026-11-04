// Copyright (c) The Create Team / The Creators of Create
// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
//! Loaded-node source propagation, following RotationPropagator and
//! GeneratingKineticBlockEntity. World drops remain the native host's job.
use crate::kinetics::{Kind, MAX_NODES, MAX_RPM, Node, Pos, ratio};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rotation {
    pub node: Node,
    pub speed: f32,
    pub source: Option<Pos>,
    pub root: Option<Pos>,
    pub flicker: u32,
    pub reactivate: bool,
}

pub fn generated(node: Node) -> f32 {
    match node.kind {
        Kind::Motor { rpm, .. } | Kind::WaterWheel { rpm, .. } => rpm,
        _ => 0.0,
    }
}

pub fn supported(node: Node) -> bool {
    matches!(
        node.kind,
        Kind::Motor { .. } | Kind::Shaft | Kind::Cog { .. } | Kind::Millstone
    )
}

/// Source identities are positions within one loaded dimension. This is a
/// kinetic calculation, not a player/item ownership or world-save protocol.
#[derive(Default)]
pub struct Propagation {
    pub nodes: BTreeMap<Pos, Rotation>,
    destroyed: Vec<Node>,
    pub limited: bool,
}

impl Propagation {
    pub fn take_destroyed(&mut self) -> Vec<Node> {
        std::mem::take(&mut self.destroyed)
    }

    pub fn upsert(&mut self, node: Node) {
        if !supported(node) || !generated(node).is_finite() {
            return;
        }
        if let Some(previous) = self.nodes.get(&node.pos).copied() {
            let same_shape = previous.node.axis == node.axis
                && match (previous.node.kind, node.kind) {
                    (Kind::Motor { facing: a, .. }, Kind::Motor { facing: b, .. }) => a == b,
                    (Kind::Shaft, Kind::Shaft) | (Kind::Millstone, Kind::Millstone) => true,
                    (Kind::Cog { large: a }, Kind::Cog { large: b }) => a == b,
                    _ => false,
                };
            if same_shape {
                self.nodes.get_mut(&node.pos).unwrap().node = node;
                if generated(previous.node) != generated(node) {
                    self.generated_changed(node.pos);
                }
                return;
            }
            self.remove(node.pos);
        }
        let speed = generated(node);
        self.nodes.insert(
            node.pos,
            Rotation {
                node,
                speed,
                source: None,
                root: (speed != 0.0).then_some(node.pos),
                flicker: 0,
                reactivate: false,
            },
        );
        self.propagate(node.pos);
    }

    pub fn remove(&mut self, pos: Pos) {
        if let Some(previous) = self.nodes.remove(&pos) {
            self.detached(previous);
        }
    }

    fn destroy(&mut self, pos: Pos) {
        if let Some(previous) = self.nodes.remove(&pos) {
            self.destroyed.push(previous.node);
            self.detached(previous);
        }
    }

    pub fn tick(&mut self) {
        for node in self.nodes.values_mut() {
            node.flicker = node.flicker.saturating_sub(1);
        }
        let pending: Vec<_> = self
            .nodes
            .iter()
            .filter_map(|(pos, node)| node.reactivate.then_some(*pos))
            .collect();
        for pos in pending {
            self.generated_changed(pos);
            if let Some(node) = self.nodes.get_mut(&pos) {
                node.reactivate = false;
            }
        }
    }

    fn speed(&mut self, pos: Pos, speed: f32) {
        if let Some(node) = self.nodes.get_mut(&pos) {
            let old = node.speed;
            if (old == 0.0) != (speed == 0.0)
                || (old != 0.0 && speed != 0.0 && old.signum() != speed.signum())
            {
                node.flicker += 5;
            }
            node.speed = speed;
        }
    }

    fn source(&mut self, pos: Pos, parent: Pos) {
        let Some(parent_node) = self.nodes.get(&parent).copied() else {
            return;
        };
        if let Some(node) = self.nodes.get_mut(&pos) {
            node.source = Some(parent);
            node.root = parent_node.root;
            if node.reactivate && parent_node.speed.abs() >= generated(node.node).abs() {
                node.reactivate = false;
            }
        }
    }

    fn clear_source(&mut self, pos: Pos) {
        if let Some(node) = self.nodes.get_mut(&pos) {
            if node.source.is_some() && generated(node.node) != 0.0 {
                node.reactivate = true;
            }
            node.source = None;
            node.root = None;
        }
        self.speed(pos, 0.0);
    }

    fn generated_changed(&mut self, pos: Pos) {
        let Some(old) = self.nodes.get(&pos).copied() else {
            return;
        };
        let new = generated(old.node);
        if old.speed == new {
            return;
        }
        if new == 0.0 && old.source.is_some() {
            return;
        }
        if old.speed != 0.0 && old.source.is_some() && old.speed.abs() >= new.abs() {
            if old.speed.signum() != new.signum() {
                self.destroy(pos);
            }
            return;
        }
        if old.speed != 0.0 {
            self.detached(old);
        }
        if let Some(node) = self.nodes.get_mut(&pos) {
            node.source = None;
            node.root = (new != 0.0).then_some(pos);
        }
        self.speed(pos, new);
        if new != 0.0 {
            self.propagate(pos);
        }
    }

    /// Direction order, then BlockPos.betweenClosed order (X fastest).
    fn neighbors(&self, node: Node) -> Vec<Pos> {
        let mut offsets = vec![
            [0, -1, 0],
            [0, 1, 0],
            [0, 0, -1],
            [0, 0, 1],
            [-1, 0, 0],
            [1, 0, 0],
        ];
        if matches!(node.kind, Kind::Cog { .. } | Kind::Millstone) {
            for z in -1i32..=1 {
                for y in -1i32..=1 {
                    for x in -1i32..=1 {
                        if x * x + y * y + z * z == 2
                            && (matches!(node.kind, Kind::Cog { large: true })
                                || [x, y, z][node.axis.index()] == 0)
                        {
                            offsets.push([x, y, z]);
                        }
                    }
                }
            }
        }
        offsets
            .into_iter()
            .filter_map(|offset| {
                let pos = std::array::from_fn(|i| node.pos[i] + offset[i]);
                self.nodes
                    .get(&pos)
                    .filter(|other| {
                        ratio(node, other.node).is_some() || ratio(other.node, node).is_some()
                    })
                    .map(|_| pos)
            })
            .collect()
    }

    fn detached(&mut self, removed: Rotation) {
        if removed.speed == 0.0 {
            return;
        }
        for neighbor in self.neighbors(removed.node) {
            if self
                .nodes
                .get(&neighbor)
                .is_some_and(|n| n.source == Some(removed.node.pos))
            {
                self.missing_source(neighbor);
            }
        }
    }

    fn missing_source(&mut self, start: Pos) {
        let missing = self.nodes.get(&start).and_then(|n| n.source);
        let mut frontier = VecDeque::from([start]);
        let mut seen = BTreeSet::new();
        let mut potential = Vec::new();
        while let Some(pos) = frontier.pop_front() {
            if !seen.insert(pos) {
                continue;
            }
            let Some(current) = self.nodes.get(&pos).copied() else {
                continue;
            };
            self.clear_source(pos);
            for neighbor in self.neighbors(current.node) {
                if Some(neighbor) == missing {
                    continue;
                }
                let next = self.nodes[&neighbor];
                if next.source.is_none() {
                    continue;
                }
                if next.source != Some(pos) {
                    potential.push(neighbor);
                    continue;
                }
                if generated(next.node) != 0.0 {
                    potential.push(neighbor);
                }
                frontier.push_back(neighbor);
            }
        }
        for pos in potential {
            if self
                .nodes
                .get(&pos)
                .is_some_and(|n| n.source.is_some() || generated(n.node) != 0.0)
            {
                self.propagate(pos);
                break;
            }
        }
    }

    /// Explicit continuation stack retains the Java recursive traversal without
    /// making a long shaft chain consume the server's native call stack.
    fn propagate(&mut self, start: Pos) {
        let Some(first) = self.nodes.get(&start) else {
            return;
        };
        let mut stack = vec![(start, self.neighbors(first.node), 0usize)];
        let mut steps = 0usize;
        while let Some((pos, neighbors, index)) = stack.pop() {
            steps += 1;
            if steps > MAX_NODES * 128 {
                self.limited = true;
                return;
            }
            let Some(current) = self.nodes.get(&pos).copied() else {
                continue;
            };
            let Some(&target) = neighbors.get(index) else {
                continue;
            };
            let Some(next) = self.nodes.get(&target).copied() else {
                stack.push((pos, neighbors, index + 1));
                continue;
            };
            let forward = ratio(current.node, next.node).unwrap_or(0.0);
            let backward = ratio(next.node, current.node).unwrap_or(0.0);
            let conveyed = current.speed * forward;
            let opposite = next.speed * backward;
            if conveyed == 0.0 && opposite == 0.0 {
                stack.push((pos, neighbors, index + 1));
                continue;
            }
            let incompatible =
                conveyed != 0.0 && next.speed != 0.0 && conveyed.signum() != next.speed.signum();
            if conveyed.abs() > MAX_RPM
                || opposite.abs() > MAX_RPM
                || current.flicker > 128
                || incompatible
            {
                self.destroy(pos);
                continue;
            }
            if opposite.abs() > current.speed.abs() {
                self.source(pos, target);
                self.speed(pos, opposite);
                stack.push((pos, self.neighbors(current.node), 0));
                continue;
            }
            if conveyed.abs() >= next.speed.abs() {
                if current.root.is_none() || current.root == next.root {
                    if conveyed.abs() > next.speed.abs() + next.speed.abs() / 256.0 / 256.0 {
                        self.destroy(pos);
                    } else {
                        stack.push((pos, neighbors, index + 1));
                    }
                    continue;
                }
                if current.source == Some(target) {
                    self.clear_source(pos);
                }
                self.source(target, pos);
                self.speed(target, self.nodes[&pos].speed * forward);
                stack.push((pos, neighbors, index + 1));
                stack.push((target, self.neighbors(next.node), 0));
                continue;
            }
            if (next.speed - conveyed).abs() > 1e-4 {
                self.speed(target, conveyed);
                self.source(target, pos);
                stack.push((pos, neighbors, index + 1));
                stack.push((target, self.neighbors(next.node), 0));
            } else {
                stack.push((pos, neighbors, index + 1));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kinetics::Axis;
    const A: Pos = [2, 2, 2];
    const JOIN: Pos = [2, 3, 2];
    const B: Pos = [2, 4, 2];
    fn motor(pos: Pos, rpm: f32) -> Node {
        Node {
            pos,
            axis: Axis::Y,
            kind: Kind::Motor {
                facing: if pos == A { [0, 1, 0] } else { [0, -1, 0] },
                rpm,
                capacity: 16384.0,
            },
        }
    }
    fn joined(a: f32, b: f32) -> Propagation {
        let mut state = Propagation::default();
        state.upsert(motor(A, a));
        state.upsert(motor(B, b));
        state.upsert(Node {
            pos: JOIN,
            axis: Axis::Y,
            kind: Kind::Shaft,
        });
        for _ in 0..20 {
            state.tick();
        }
        state
    }
    fn destroyed(state: &mut Propagation) -> Vec<Pos> {
        state
            .take_destroyed()
            .into_iter()
            .map(|node| node.pos)
            .collect()
    }
    #[test]
    fn equal_and_unequal_merge_choose_reference_sources() {
        for (speed, root) in [(16.0, A), (32.0, B)] {
            let state = joined(16.0, speed);
            for pos in [A, JOIN, B] {
                assert_eq!(state.nodes[&pos].speed, speed);
                assert_eq!(state.nodes[&pos].root, Some(root));
                assert_eq!(state.nodes[&pos].flicker, 0);
            }
            assert_eq!(state.nodes[&JOIN].source, Some(root));
        }
    }
    #[test]
    fn opposite_merge_destroys_the_joining_shaft_only() {
        let mut state = joined(16.0, -16.0);
        assert_eq!(destroyed(&mut state), [JOIN]);
        assert_eq!(state.nodes[&A].speed, 16.0);
        assert_eq!(state.nodes[&B].speed, -16.0);
    }
    #[test]
    fn removed_dominant_source_recovers_the_subordinate() {
        let mut state = joined(16.0, 32.0);
        state.remove(B);
        for _ in 0..20 {
            state.tick();
        }
        assert!(state.take_destroyed().is_empty());
        for pos in [A, JOIN] {
            assert_eq!(state.nodes[&pos].speed, 16.0);
            assert_eq!(state.nodes[&pos].root, Some(A));
        }
    }
    #[test]
    fn overpowered_reversal_destroys_that_motor() {
        let mut state = joined(16.0, 32.0);
        state.upsert(motor(A, -16.0));
        for _ in 0..20 {
            state.tick();
        }
        assert_eq!(destroyed(&mut state), [A]);
        assert_eq!(state.nodes[&JOIN].speed, 32.0);
        assert_eq!(state.nodes[&B].speed, 32.0);
    }
    #[test]
    fn overspeed_destroys_the_conveying_large_cog() {
        let mut state = Propagation::default();
        let large = Node {
            pos: JOIN,
            axis: Axis::Y,
            kind: Kind::Cog { large: true },
        };
        let small = Node {
            pos: [3, 3, 3],
            axis: Axis::Y,
            kind: Kind::Cog { large: false },
        };
        state.upsert(motor(A, 16.0));
        state.upsert(large);
        state.upsert(small);
        assert_eq!(state.nodes[&small.pos].speed, -32.0);
        state.upsert(motor(A, 256.0));
        for _ in 0..20 {
            state.tick();
        }
        assert_eq!(destroyed(&mut state), [large.pos]);
        assert_eq!(state.nodes[&A].speed, 256.0);
        assert_eq!(state.nodes[&small.pos].speed, 0.0);
        assert_eq!(state.nodes[&small.pos].root, None);
    }
    #[test]
    fn a_single_source_reverses_without_destroying_its_load() {
        let mut state = Propagation::default();
        state.upsert(motor(A, 16.0));
        state.upsert(Node {
            pos: JOIN,
            axis: Axis::Y,
            kind: Kind::Millstone,
        });
        state.upsert(motor(A, -16.0));
        assert!(state.take_destroyed().is_empty());
        assert_eq!(state.nodes[&JOIN].speed, -16.0);
        assert!(!state.limited);
    }
    #[test]
    fn destroyed_child_does_not_skip_the_surviving_sibling() {
        let mut state = Propagation::default();
        let mut drive = motor([-1, -1, 0], 16.0);
        drive.kind = Kind::Motor {
            facing: [0, 1, 0],
            rpm: 16.0,
            capacity: 16384.0,
        };
        state.upsert(drive);
        for (pos, kind) in [
            ([-1, 0, 0], Kind::Cog { large: false }),
            ([0, 0, 0], Kind::Cog { large: false }),
            ([0, -1, 0], Kind::Cog { large: true }),
            ([1, -1, 1], Kind::Cog { large: false }),
            ([0, 1, 0], Kind::Shaft),
        ] {
            state.upsert(Node {
                pos,
                axis: Axis::Y,
                kind,
            });
        }
        drive.kind = Kind::Motor {
            facing: [0, 1, 0],
            rpm: 256.0,
            capacity: 16384.0,
        };
        state.upsert(drive);
        assert_eq!(destroyed(&mut state), [[0, -1, 0]]);
        assert_eq!(state.nodes[&[0, 0, 0]].speed, -256.0);
        assert_eq!(state.nodes[&[0, 1, 0]].speed, -256.0);
    }
}
