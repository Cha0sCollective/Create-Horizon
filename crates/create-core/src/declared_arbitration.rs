//! Experimental, closed three-slot declared-event arbitration; not a live world or Java clock.
//! Native orders/bookkeeping are authored assumptions. See P02_DECLARED_ARBITRATION_PACKET.md.
use crate::MotorSetting;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrialSpec {
    pub lower: u32,
    pub join: u32,
    pub upper: u32,
    pub capacity_per_rpm: u32,
    pub join_neighbor_order: [u32; 2],
    pub be_order: [u32; 3],
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    PlaceGenerator { node: u32, generated: MotorSetting },
    PlaceJoin,
    SetGenerated { node: u32, generated: MotorSetting },
    DestroyGenerator { node: u32 },
    Settle { label: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TrialError {
    DuplicateNode(u32),
    InvalidOrder,
    InvalidCapacity,
    TooManyEvents,
    UnknownNode(u32),
    WrongKind(u32),
    AlreadyPresent(u32),
    Absent(u32),
    JoinAdmission,
    OpposedMerge,
    EqualMerge,
    UnsupportedTransition(u32),
    LastGenerator,
    InvalidState(u32),
    UnsupportedPropagation(u32),
    NonConvergent,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Destruction {
    pub node: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum NodeView {
    Absent {
        node: u32,
    },
    Present {
        node: u32,
        generator: bool,
        generated_rpm: f32,
        theoretical_rpm: f32,
        effective_rpm: f32,
        source_parent: Option<u32>,
        network_present: bool,
        network_members: Vec<u32>,
        capacity_su: f32,
        stress_su: f32,
        flicker: u16,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Checkpoint {
    pub label: String,
    pub nodes: Vec<NodeView>,
    pub effects: Vec<Destruction>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TrialTrace {
    pub checkpoints: Vec<Checkpoint>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct State {
    present: bool,
    own: f32,
    speed: f32,
    parent: Option<u32>,
    group: Option<u32>,
    flicker: u16,
    attach: bool,
    reactivate: bool,
    dirty: bool,
    capacity: f32,
}

#[derive(Clone)]
struct Machine {
    spec: TrialSpec,
    nodes: BTreeMap<u32, State>,
    effects: Vec<Destruction>,
}

impl Machine {
    fn new(spec: &TrialSpec) -> Result<Self, TrialError> {
        let mut ids = BTreeSet::new();
        for id in [spec.lower, spec.join, spec.upper] {
            if !ids.insert(id) {
                return Err(TrialError::DuplicateNode(id));
            }
        }
        if spec.capacity_per_rpm != 16384 {
            return Err(TrialError::InvalidCapacity);
        }
        if spec.be_order.into_iter().collect::<BTreeSet<_>>() != ids
            || spec
                .join_neighbor_order
                .into_iter()
                .collect::<BTreeSet<_>>()
                != BTreeSet::from([spec.lower, spec.upper])
        {
            return Err(TrialError::InvalidOrder);
        }
        Ok(Self {
            spec: spec.clone(),
            nodes: ids.into_iter().map(|id| (id, State::default())).collect(),
            effects: vec![],
        })
    }
    fn generator(&self, id: u32) -> bool {
        id != self.spec.join
    }
    fn state(&self, id: u32) -> Result<State, TrialError> {
        self.nodes
            .get(&id)
            .copied()
            .ok_or(TrialError::UnknownNode(id))
    }
    fn present_generator(&self, id: u32) -> Result<State, TrialError> {
        let state = self.state(id)?;
        if !self.generator(id) {
            return Err(TrialError::WrongKind(id));
        }
        if !state.present {
            return Err(TrialError::Absent(id));
        }
        Ok(state)
    }
    fn neighbors(&self, id: u32) -> Vec<u32> {
        let candidates = if id == self.spec.join {
            self.spec.join_neighbor_order.to_vec()
        } else {
            vec![self.spec.join]
        };
        candidates
            .into_iter()
            .filter(|n| self.nodes[n].present)
            .collect()
    }
    fn adjacent(&self, a: u32, b: u32) -> bool {
        a != b
            && self.nodes.contains_key(&a)
            && self.nodes.contains_key(&b)
            && (a == self.spec.join || b == self.spec.join)
    }
    fn dirty_group(&mut self, group: Option<u32>) {
        if let Some(group) = group {
            for state in self
                .nodes
                .values_mut()
                .filter(|s| s.present && s.group == Some(group))
            {
                state.dirty = true;
            }
        }
    }
    fn network(&mut self, id: u32, group: Option<u32>) {
        let old = self.nodes[&id].group;
        if old == group {
            return;
        }
        let state = self.nodes.get_mut(&id).unwrap();
        state.group = group;
        state.capacity = 0.0;
        state.dirty = false;
        // Empty groups have no separate object/pending work to retain.
        self.dirty_group(old);
        self.dirty_group(group);
    }
    fn notify(&mut self, id: u32, before: f32) {
        let state = self.nodes.get_mut(&id).unwrap();
        if state.present
            && ((before == 0.0) != (state.speed == 0.0)
                || (before != 0.0 && state.speed != 0.0 && before.signum() != state.speed.signum()))
        {
            state.flicker += 5;
        }
    }
    fn parent(&mut self, id: u32, parent: u32) -> Result<(), TrialError> {
        if !self.adjacent(id, parent) || !self.nodes[&parent].present {
            return Err(TrialError::InvalidState(id));
        }
        self.nodes.get_mut(&id).unwrap().parent = Some(parent);
        self.network(id, self.nodes[&parent].group);
        let parent_speed = self.nodes[&parent].speed;
        let state = self.nodes.get_mut(&id).unwrap();
        if state.reactivate && parent_speed.abs() >= state.own.abs() {
            state.reactivate = false;
        }
        Ok(())
    }
    fn remove_source(&mut self, id: u32) {
        let before = self.nodes[&id];
        let generator = self.generator(id);
        let state = self.nodes.get_mut(&id).unwrap();
        if generator && before.parent.is_some() && before.own != 0.0 {
            state.reactivate = true;
        }
        state.speed = 0.0;
        state.parent = None;
        self.network(id, None);
        self.notify(id, before.speed);
    }
    fn propagate(&mut self, current: u32, depth: u8) -> Result<(), TrialError> {
        if depth >= 64 {
            return Err(TrialError::NonConvergent);
        }
        for neighbor in self.neighbors(current) {
            let c = self.nodes[&current];
            let n = self.nodes[&neighbor];
            if c.speed == 0.0 && n.speed == 0.0 {
                continue;
            }
            if c.speed.abs() > 256.0
                || n.speed.abs() > 256.0
                || c.flicker > 128
                || (c.speed != 0.0 && n.speed != 0.0 && c.speed.signum() != n.speed.signum())
            {
                return Err(TrialError::UnsupportedPropagation(current));
            }
            if n.speed.abs() > c.speed.abs() {
                self.parent(current, neighbor)?;
                self.nodes.get_mut(&current).unwrap().speed = n.speed;
                self.notify(current, c.speed);
                self.propagate(current, depth + 1)?;
                return Ok(());
            }
            if c.group.is_none() {
                return Err(TrialError::InvalidState(current));
            }
            if c.group == n.group {
                if c.speed.abs() > n.speed.abs() + n.speed.abs() / 256.0 / 256.0 {
                    return Err(TrialError::UnsupportedPropagation(current));
                }
                continue;
            }
            if c.parent == Some(neighbor) {
                return Err(TrialError::InvalidState(current));
            }
            self.parent(neighbor, current)?;
            self.nodes.get_mut(&neighbor).unwrap().speed = c.speed;
            self.notify(neighbor, n.speed);
            self.propagate(neighbor, depth + 1)?;
        }
        Ok(())
    }
    fn missing_source(&mut self, start: u32) -> Result<(), TrialError> {
        let missing = self.nodes[&start].parent;
        let mut frontier = VecDeque::from([start]);
        let mut visited = BTreeSet::new();
        let mut candidates = Vec::new();
        while let Some(current) = frontier.pop_front() {
            if !visited.insert(current) {
                return Err(TrialError::InvalidState(current));
            }
            self.remove_source(current);
            for neighbor in self.neighbors(current) {
                if Some(neighbor) == missing {
                    continue;
                }
                let n = self.nodes[&neighbor];
                let Some(parent) = n.parent else {
                    continue;
                };
                if parent != current {
                    candidates.push(neighbor);
                    continue;
                }
                if n.own != 0.0 {
                    candidates.push(neighbor);
                }
                frontier.push_back(neighbor);
            }
        }
        for candidate in candidates {
            let n = self.nodes[&candidate];
            if n.present && (n.parent.is_some() || n.own != 0.0) {
                self.propagate(candidate, 0)?;
                break;
            }
        }
        Ok(())
    }
    fn destroy(&mut self, id: u32) -> Result<(), TrialError> {
        self.present_generator(id)?;
        if self
            .nodes
            .iter()
            .filter(|(n, s)| self.generator(**n) && s.present)
            .count()
            <= 1
        {
            return Err(TrialError::LastGenerator);
        }
        self.network(id, None);
        *self.nodes.get_mut(&id).unwrap() = State::default();
        self.effects.push(Destruction { node: id });
        for neighbor in self.neighbors(id) {
            if self.nodes[&neighbor].parent == Some(id) {
                self.missing_source(neighbor)?;
            }
        }
        Ok(())
    }
    fn update_generator(&mut self, id: u32) -> Result<(), TrialError> {
        let before = self.nodes[&id];
        if before.speed != before.own {
            if before.speed == 0.0 {
                self.nodes.get_mut(&id).unwrap().speed = before.own;
                self.network(id, Some(id));
                self.propagate(id, 0)?;
            } else if before.parent.is_some() {
                if before.own.abs() > before.speed.abs() {
                    return Err(TrialError::UnsupportedTransition(id));
                }
                if before.own.signum() != before.speed.signum() {
                    self.destroy(id)?;
                }
            } else {
                // Admitted root changes are isolated, so detach has no descendants.
                if !self.neighbors(id).is_empty() {
                    return Err(TrialError::UnsupportedTransition(id));
                }
                self.nodes.get_mut(&id).unwrap().speed = before.own;
                self.propagate(id, 0)?;
            }
        }
        if self.nodes[&id].present {
            self.dirty_group(self.nodes[&id].group);
            self.notify(id, before.speed);
        }
        Ok(())
    }
    fn reconcile(&mut self, group: u32) {
        let capacity = self
            .nodes
            .values()
            .filter(|s| s.present && s.group == Some(group))
            .map(|s| s.own.abs() * self.spec.capacity_per_rpm as f32)
            .sum();
        for state in self
            .nodes
            .values_mut()
            .filter(|s| s.present && s.group == Some(group))
        {
            state.capacity = capacity;
            state.dirty = false;
        }
    }
    fn round(&mut self) -> Result<(), TrialError> {
        for id in self.spec.be_order {
            if !self.nodes[&id].present {
                continue;
            }
            if self.nodes[&id].attach {
                self.nodes.get_mut(&id).unwrap().attach = false;
                self.propagate(id, 0)?;
            }
            let state = self.nodes.get_mut(&id).unwrap();
            state.flicker = state.flicker.saturating_sub(1);
            if state.dirty {
                let group = state.group.ok_or(TrialError::InvalidState(id))?;
                self.reconcile(group);
            }
            if self.nodes[&id].reactivate {
                self.update_generator(id)?;
                self.nodes.get_mut(&id).unwrap().reactivate = false;
            }
        }
        Ok(())
    }
    fn quiet(&self) -> bool {
        self.nodes
            .values()
            .all(|s| !s.present || (!s.attach && !s.reactivate && !s.dirty && s.flicker == 0))
    }
    fn validate(&self, settled: bool) -> Result<(), TrialError> {
        for (&id, s) in &self.nodes {
            if !s.present {
                if *s != State::default() {
                    return Err(TrialError::InvalidState(id));
                }
                continue;
            }
            if !s.speed.is_finite()
                || !s.own.is_finite()
                || !s.capacity.is_finite()
                || s.speed.abs() > 256.0
                || s.own.abs() > 256.0
                || (self.generator(id) != (s.own != 0.0))
                || s.capacity < 0.0
                || (!self.generator(id) && s.reactivate)
                || (self.generator(id) && s.attach)
                || (s.dirty && s.group.is_none())
            {
                return Err(TrialError::InvalidState(id));
            }
            let mut cursor = id;
            let mut seen = BTreeSet::new();
            while let Some(parent) = self.nodes[&cursor].parent {
                if !seen.insert(cursor)
                    || !self.adjacent(cursor, parent)
                    || !self.nodes[&parent].present
                {
                    return Err(TrialError::InvalidState(id));
                }
                cursor = parent;
            }
            if let Some(group) = s.group {
                let root = self
                    .state(group)
                    .map_err(|_| TrialError::InvalidState(id))?;
                if !root.present
                    || !self.generator(group)
                    || cursor != group
                    || root.parent.is_some()
                    || root.group != Some(group)
                    || root.speed != root.own
                    || root.speed == 0.0
                    || s.speed != root.speed
                {
                    return Err(TrialError::InvalidState(id));
                }
                if settled {
                    let expected: f32 = self
                        .nodes
                        .values()
                        .filter(|p| p.present && p.group == Some(group))
                        .map(|p| p.own.abs() * self.spec.capacity_per_rpm as f32)
                        .sum();
                    if s.capacity != expected {
                        return Err(TrialError::InvalidState(id));
                    }
                }
            } else if s.speed != 0.0 || s.parent.is_some() || s.capacity != 0.0 {
                return Err(TrialError::InvalidState(id));
            }
            if settled && self.generator(id) && s.group.is_none() {
                return Err(TrialError::InvalidState(id));
            }
        }
        Ok(())
    }
    fn settle_limit(&mut self, limit: usize) -> Result<(), TrialError> {
        for _ in 0..limit {
            if self.quiet() {
                self.validate(true)?;
                return Ok(());
            }
            self.round()?;
        }
        if !self.quiet() {
            return Err(TrialError::NonConvergent);
        }
        self.validate(true)
    }
    fn snapshot(&self, label: String) -> Checkpoint {
        let nodes = self
            .nodes
            .iter()
            .map(|(&id, s)| {
                if !s.present {
                    return NodeView::Absent { node: id };
                }
                NodeView::Present {
                    node: id,
                    generator: self.generator(id),
                    generated_rpm: s.own,
                    theoretical_rpm: s.speed,
                    effective_rpm: s.speed,
                    source_parent: s.parent,
                    network_present: s.group.is_some(),
                    network_members: self
                        .nodes
                        .iter()
                        .filter(|(_, p)| p.present && p.group.is_some() && p.group == s.group)
                        .map(|(&n, _)| n)
                        .collect(),
                    capacity_su: s.capacity,
                    stress_su: 0.0,
                    flicker: s.flicker,
                }
            })
            .collect();
        Checkpoint {
            label,
            nodes,
            effects: self.effects.clone(),
        }
    }
    fn inner(&mut self, event: &Event) -> Result<Option<Checkpoint>, TrialError> {
        match event {
            Event::PlaceGenerator { node, generated } => {
                let s = self.state(*node)?;
                if !self.generator(*node) {
                    return Err(TrialError::WrongKind(*node));
                }
                if s.present {
                    return Err(TrialError::AlreadyPresent(*node));
                }
                if self.nodes[&self.spec.join].present {
                    return Err(TrialError::UnsupportedTransition(*node));
                }
                *self.nodes.get_mut(node).unwrap() = State {
                    present: true,
                    own: f32::from(generated.rpm()),
                    ..State::default()
                };
                self.update_generator(*node)?;
            }
            Event::PlaceJoin => {
                if self.nodes[&self.spec.join].present {
                    return Err(TrialError::AlreadyPresent(self.spec.join));
                }
                if !self.quiet() {
                    return Err(TrialError::JoinAdmission);
                }
                let a = self.nodes[&self.spec.lower];
                let b = self.nodes[&self.spec.upper];
                for (id, s) in [(self.spec.lower, a), (self.spec.upper, b)] {
                    if !s.present
                        || s.parent.is_some()
                        || s.group != Some(id)
                        || s.speed != s.own
                        || s.own == 0.0
                    {
                        return Err(TrialError::JoinAdmission);
                    }
                }
                if a.own.signum() != b.own.signum() {
                    return Err(TrialError::OpposedMerge);
                }
                if a.own.abs() == b.own.abs() {
                    return Err(TrialError::EqualMerge);
                }
                *self.nodes.get_mut(&self.spec.join).unwrap() = State {
                    present: true,
                    attach: true,
                    ..State::default()
                };
            }
            Event::SetGenerated { node, generated } => {
                let s = self.present_generator(*node)?;
                if (s.parent.is_some() && f32::from(generated.rpm()).abs() > s.speed.abs())
                    || (s.parent.is_none() && !self.neighbors(*node).is_empty())
                {
                    return Err(TrialError::UnsupportedTransition(*node));
                }
                self.nodes.get_mut(node).unwrap().own = f32::from(generated.rpm());
                self.update_generator(*node)?;
            }
            Event::DestroyGenerator { node } => self.destroy(*node)?,
            Event::Settle { label } => {
                self.settle_limit(64)?;
                return Ok(Some(self.snapshot(label.clone())));
            }
        }
        Ok(None)
    }
    fn apply(&mut self, event: &Event) -> Result<Option<Checkpoint>, TrialError> {
        self.validate(false)?;
        let mut next = self.clone();
        let result = next.inner(event)?;
        next.validate(false)?;
        *self = next;
        Ok(result)
    }
}

/// One fresh bounded trial per call. Names/labels never select behavior; unsupported
/// events return no partial trace and never alter the existing single-source solver.
pub fn run_declared_trial(spec: &TrialSpec, events: &[Event]) -> Result<TrialTrace, TrialError> {
    if events.len() > 16 {
        return Err(TrialError::TooManyEvents);
    }
    let mut machine = Machine::new(spec)?;
    let mut checkpoints = Vec::new();
    for event in events {
        if let Some(checkpoint) = machine.apply(event)? {
            checkpoints.push(checkpoint);
        }
    }
    Ok(TrialTrace { checkpoints })
}

#[cfg(test)]
mod tests;
