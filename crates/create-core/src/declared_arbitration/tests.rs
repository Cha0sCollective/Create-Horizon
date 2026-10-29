use super::*;

fn spec() -> TrialSpec {
    TrialSpec {
        lower: 0,
        join: 1,
        upper: 2,
        capacity_per_rpm: 16384,
        join_neighbor_order: [0, 2],
        be_order: [0, 1, 2],
    }
}
fn rpm(value: i16) -> MotorSetting {
    MotorSetting::from_signed(value).unwrap()
}
fn settle(label: &str) -> Event {
    Event::Settle {
        label: label.into(),
    }
}
fn prefix(a: i16, b: i16) -> Vec<Event> {
    vec![
        Event::PlaceGenerator {
            node: 0,
            generated: rpm(16),
        },
        Event::PlaceGenerator {
            node: 2,
            generated: rpm(-16),
        },
        settle("initial"),
        Event::SetGenerated {
            node: 0,
            generated: rpm(a),
        },
        Event::SetGenerated {
            node: 2,
            generated: rpm(b),
        },
        settle("disconnected"),
        Event::PlaceJoin,
        settle("merged"),
    ]
}
fn merged() -> Machine {
    let mut m = Machine::new(&spec()).unwrap();
    for event in prefix(16, 32) {
        m.apply(&event).unwrap();
    }
    m
}
fn assert_node(
    c: &Checkpoint,
    id: u32,
    own: f32,
    speed: f32,
    parent: Option<u32>,
    members: &[u32],
    capacity: f32,
) {
    let found = c
        .nodes
        .iter()
        .find(|n| match n {
            NodeView::Absent { node } | NodeView::Present { node, .. } => *node == id,
        })
        .unwrap();
    let NodeView::Present {
        generated_rpm,
        theoretical_rpm,
        effective_rpm,
        source_parent,
        network_members,
        capacity_su,
        stress_su,
        flicker,
        network_present,
        ..
    } = found
    else {
        panic!("expected present {id}")
    };
    assert_eq!(
        (*generated_rpm, *theoretical_rpm, *effective_rpm),
        (own, speed, speed)
    );
    assert_eq!(*source_parent, parent);
    assert_eq!(network_members.as_slice(), members);
    assert_eq!(
        (*capacity_su, *stress_su, *flicker, *network_present),
        (capacity, 0.0, 0, true)
    );
}

#[test]
fn three_event_continuations_construct_literal_parents_groups_and_effects() {
    let mut programs = vec![prefix(16, 32); 3];
    programs[1].push(Event::DestroyGenerator { node: 2 });
    programs[2].push(Event::SetGenerated {
        node: 0,
        generated: rpm(-16),
    });
    for (index, program) in programs.iter_mut().enumerate() {
        program.push(settle("after"));
        let trace = run_declared_trial(&spec(), program).unwrap();
        assert_eq!(trace.checkpoints.len(), 4);
        let disconnected = &trace.checkpoints[1];
        assert_node(disconnected, 0, 16.0, 16.0, None, &[0], 262144.0);
        assert_node(disconnected, 2, 32.0, 32.0, None, &[2], 524288.0);
        assert_eq!(disconnected.nodes[1], NodeView::Absent { node: 1 });
        assert!(disconnected.effects.is_empty());
        let joined = &trace.checkpoints[2];
        assert_node(joined, 0, 16.0, 32.0, Some(1), &[0, 1, 2], 786432.0);
        assert_node(joined, 1, 0.0, 32.0, Some(2), &[0, 1, 2], 786432.0);
        assert_node(joined, 2, 32.0, 32.0, None, &[0, 1, 2], 786432.0);
        assert!(joined.effects.is_empty());
        let after = &trace.checkpoints[3];
        match index {
            0 => {
                assert_eq!(after.nodes, joined.nodes);
                assert!(after.effects.is_empty());
            }
            1 => {
                assert_node(after, 0, 16.0, 16.0, None, &[0, 1], 262144.0);
                assert_node(after, 1, 0.0, 16.0, Some(0), &[0, 1], 262144.0);
                assert_eq!(after.nodes[2], NodeView::Absent { node: 2 });
                assert_eq!(after.effects, vec![Destruction { node: 2 }]);
            }
            _ => {
                assert_eq!(after.nodes[0], NodeView::Absent { node: 0 });
                assert_node(after, 1, 0.0, 32.0, Some(2), &[1, 2], 524288.0);
                assert_node(after, 2, 32.0, 32.0, None, &[1, 2], 524288.0);
                assert_eq!(after.effects, vec![Destruction { node: 0 }]);
            }
        }
    }
}

#[test]
fn removal_clears_parent_descendants_before_deferred_candidate_reactivation() {
    let mut m = merged();
    m.apply(&Event::DestroyGenerator { node: 2 }).unwrap();
    assert_eq!(
        (m.nodes[&0].speed, m.nodes[&0].parent, m.nodes[&0].group),
        (0.0, None, None)
    );
    assert!(m.nodes[&0].reactivate);
    assert_eq!(
        (m.nodes[&1].speed, m.nodes[&1].parent, m.nodes[&1].group),
        (0.0, None, None)
    );
    assert!(!m.nodes[&1].reactivate);
    assert!(!m.quiet());
    // The candidate's own generation did not power the shaft during the parent walk.
    m.round().unwrap();
    assert_eq!(
        (m.nodes[&0].speed, m.nodes[&1].speed, m.nodes[&1].parent),
        (16.0, 16.0, Some(0))
    );
    assert!(!m.nodes[&0].reactivate);
    assert!(m.nodes[&0].flicker > 0 && m.nodes[&1].flicker > 0);
    assert!(!m.quiet());
    m.settle_limit(64).unwrap();
    assert!(m.quiet());
    m.validate(true).unwrap();
    assert_eq!(m.nodes[&1].capacity, 262144.0);
}

#[test]
fn generated_capacity_changes_without_overwriting_driven_speed_or_parent() {
    let mut m = merged();
    m.apply(&Event::SetGenerated {
        node: 0,
        generated: rpm(32),
    })
    .unwrap();
    m.apply(&settle("equal driven request")).unwrap();
    assert_eq!(
        (m.nodes[&0].own, m.nodes[&0].speed, m.nodes[&0].parent),
        (32.0, 32.0, Some(1))
    );
    assert_eq!(m.nodes[&1].capacity, 1048576.0);
    m.apply(&Event::SetGenerated {
        node: 0,
        generated: rpm(16),
    })
    .unwrap();
    m.apply(&settle("smaller same sign")).unwrap();
    assert_eq!(
        (m.nodes[&0].own, m.nodes[&0].speed, m.nodes[&0].parent),
        (16.0, 32.0, Some(1))
    );
    assert_eq!(m.nodes[&2].capacity, 786432.0);
    m.apply(&Event::SetGenerated {
        node: 0,
        generated: rpm(-16),
    })
    .unwrap();
    assert_eq!(m.nodes[&1].speed, 32.0);
    assert_eq!(m.nodes[&2].speed, 32.0);
    m.apply(&settle("leaf removed")).unwrap();
    assert_eq!(m.nodes[&1].capacity, 524288.0);
    assert_eq!(m.effects, vec![Destruction { node: 0 }]);
    assert_eq!(
        m.apply(&Event::DestroyGenerator { node: 0 }),
        Err(TrialError::Absent(0))
    );
    assert_eq!(m.effects.len(), 1);
}

#[test]
fn swapping_magnitudes_constructs_the_other_root_and_reactivates_the_other_motor() {
    let mut events = prefix(32, 16);
    events.push(Event::DestroyGenerator { node: 0 });
    events.push(settle("after"));
    let trace = run_declared_trial(&spec(), &events).unwrap();
    let joined = &trace.checkpoints[2];
    assert_node(joined, 0, 32.0, 32.0, None, &[0, 1, 2], 786432.0);
    assert_node(joined, 1, 0.0, 32.0, Some(0), &[0, 1, 2], 786432.0);
    assert_node(joined, 2, 16.0, 32.0, Some(1), &[0, 1, 2], 786432.0);
    assert_eq!(trace.checkpoints[3].nodes[0], NodeView::Absent { node: 0 });
    assert_node(
        &trace.checkpoints[3],
        1,
        0.0,
        16.0,
        Some(2),
        &[1, 2],
        262144.0,
    );
    assert_node(
        &trace.checkpoints[3],
        2,
        16.0,
        16.0,
        None,
        &[1, 2],
        262144.0,
    );
}

#[test]
fn identifiers_metadata_and_declared_orders_do_not_select_a_fixture_outcome() {
    let mut changed_spec = TrialSpec {
        lower: 90,
        join: 4,
        upper: 21,
        capacity_per_rpm: 16384,
        join_neighbor_order: [90, 21],
        be_order: [90, 4, 21],
    };
    let mut events = prefix(16, 32);
    for event in &mut events {
        match event {
            Event::PlaceGenerator { node, .. }
            | Event::SetGenerated { node, .. }
            | Event::DestroyGenerator { node } => {
                *node = match *node {
                    0 => 90,
                    1 => 4,
                    _ => 21,
                };
            }
            Event::Settle { label } => *label = "same arbitrary metadata".into(),
            _ => {}
        }
    }
    let first = run_declared_trial(&changed_spec, &events).unwrap();
    assert_node(
        &first.checkpoints[2],
        90,
        16.0,
        32.0,
        Some(4),
        &[4, 21, 90],
        786432.0,
    );
    assert_node(
        &first.checkpoints[2],
        4,
        0.0,
        32.0,
        Some(21),
        &[4, 21, 90],
        786432.0,
    );
    changed_spec.join_neighbor_order.reverse();
    changed_spec.be_order.reverse();
    let second = run_declared_trial(&changed_spec, &events).unwrap();
    assert_eq!(first, second); // source-derived software check, not Java order independence.
    assert_eq!(run_declared_trial(&changed_spec, &events).unwrap(), second);
}

#[test]
fn malformed_specs_and_event_admission_refuse_without_partial_state_or_effects() {
    let mut bad = spec();
    bad.upper = bad.lower;
    assert_eq!(Machine::new(&bad).err(), Some(TrialError::DuplicateNode(0)));
    bad = spec();
    bad.capacity_per_rpm = 8;
    assert_eq!(Machine::new(&bad).err(), Some(TrialError::InvalidCapacity));
    bad = spec();
    bad.join_neighbor_order = [0, 0];
    assert_eq!(Machine::new(&bad).err(), Some(TrialError::InvalidOrder));
    bad = spec();
    bad.be_order = [0, 1, 99];
    assert_eq!(Machine::new(&bad).err(), Some(TrialError::InvalidOrder));
    assert_eq!(
        run_declared_trial(&spec(), &vec![settle("label"); 17]),
        Err(TrialError::TooManyEvents)
    );
    let mut m = merged();
    let before_nodes = m.nodes.clone();
    let before_effects = m.effects.clone();
    for (event, error) in [
        (Event::PlaceJoin, TrialError::AlreadyPresent(1)),
        (
            Event::SetGenerated {
                node: 0,
                generated: rpm(64),
            },
            TrialError::UnsupportedTransition(0),
        ),
        (
            Event::SetGenerated {
                node: 2,
                generated: rpm(16),
            },
            TrialError::UnsupportedTransition(2),
        ),
        (
            Event::SetGenerated {
                node: 99,
                generated: rpm(16),
            },
            TrialError::UnknownNode(99),
        ),
        (
            Event::DestroyGenerator { node: 1 },
            TrialError::WrongKind(1),
        ),
        (
            Event::PlaceGenerator {
                node: 0,
                generated: rpm(16),
            },
            TrialError::AlreadyPresent(0),
        ),
    ] {
        assert_eq!(m.apply(&event), Err(error));
        assert_eq!(m.nodes, before_nodes);
        assert_eq!(m.effects, before_effects);
    }
    m.apply(&Event::DestroyGenerator { node: 2 }).unwrap();
    assert_eq!(
        m.apply(&Event::DestroyGenerator { node: 0 }),
        Err(TrialError::LastGenerator)
    );
}

#[test]
fn opposed_equal_unsettled_and_missing_source_merges_are_explicit_boundaries() {
    for (a, b, expected) in [
        (16, -32, TrialError::OpposedMerge),
        (16, 16, TrialError::EqualMerge),
    ] {
        let mut m = Machine::new(&spec()).unwrap();
        for event in prefix(a, b).into_iter().take(6) {
            m.apply(&event).unwrap();
        }
        assert_eq!(m.apply(&Event::PlaceJoin), Err(expected));
    }
    let mut m = Machine::new(&spec()).unwrap();
    assert_eq!(m.apply(&Event::PlaceJoin), Err(TrialError::JoinAdmission));
    m.apply(&Event::PlaceGenerator {
        node: 0,
        generated: rpm(16),
    })
    .unwrap();
    m.apply(&Event::PlaceGenerator {
        node: 2,
        generated: rpm(32),
    })
    .unwrap();
    assert_eq!(m.apply(&Event::PlaceJoin), Err(TrialError::JoinAdmission));
    assert!(m.effects.is_empty());
}

#[test]
fn corrupted_parent_group_speed_and_pending_state_never_become_settled_evidence() {
    let m = merged();
    let mut bad = m.clone();
    bad.nodes.get_mut(&0).unwrap().parent = Some(99);
    assert_eq!(bad.validate(true), Err(TrialError::InvalidState(0)));
    bad = m.clone();
    bad.nodes.get_mut(&1).unwrap().parent = Some(0);
    assert!(matches!(
        bad.validate(true),
        Err(TrialError::InvalidState(_))
    ));
    bad = m.clone();
    bad.nodes.get_mut(&0).unwrap().group = Some(0);
    assert_eq!(bad.validate(true), Err(TrialError::InvalidState(0)));
    for value in [f32::NAN, f32::INFINITY, 257.0] {
        bad = m.clone();
        bad.nodes.get_mut(&0).unwrap().speed = value;
        assert_eq!(bad.validate(true), Err(TrialError::InvalidState(0)));
    }
    bad = m.clone();
    bad.nodes.get_mut(&0).unwrap().capacity = 0.0;
    assert_eq!(bad.validate(true), Err(TrialError::InvalidState(0)));
    bad = m.clone();
    bad.nodes.get_mut(&0).unwrap().attach = true;
    assert_eq!(bad.validate(true), Err(TrialError::InvalidState(0)));
    bad = m.clone();
    bad.nodes.get_mut(&1).unwrap().reactivate = true;
    assert_eq!(bad.validate(true), Err(TrialError::InvalidState(1)));
}

#[test]
fn settle_barrier_and_propagation_work_limits_refuse_instead_of_guessing() {
    let mut m = Machine::new(&spec()).unwrap();
    for event in prefix(16, 32).into_iter().take(7) {
        m.apply(&event).unwrap();
    }
    assert_eq!(m.settle_limit(0), Err(TrialError::NonConvergent));
    assert_eq!(m.propagate(1, 64), Err(TrialError::NonConvergent));
    m.nodes.get_mut(&1).unwrap().flicker = 129;
    assert_eq!(
        m.apply(&settle("blocked")),
        Err(TrialError::UnsupportedPropagation(1))
    );
    assert!(m.nodes[&1].attach);
    assert!(m.effects.is_empty());
}
