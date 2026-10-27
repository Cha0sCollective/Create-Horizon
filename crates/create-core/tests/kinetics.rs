use create_horizon_core::*;

fn node(id: u32, impact: f32) -> Node {
    Node {
        id,
        impact_per_rpm: impact,
        source: None,
    }
}
fn source(id: u32, rpm: f32, capacity: f32) -> Node {
    Node {
        source: Some(Source {
            rpm,
            capacity_per_rpm: capacity,
        }),
        ..node(id, 0.0)
    }
}
fn edge(from: u32, to: u32, transmission: Transmission) -> Edge {
    Edge {
        from,
        to,
        transmission,
    }
}

#[test]
fn all_motor_settings_and_board_boundaries() {
    assert_eq!(MotorSetting::default().rpm(), 16);
    for rpm in -256..=256 {
        if rpm == 0 {
            assert!(MotorSetting::from_signed(rpm).is_err());
        } else {
            assert_eq!(MotorSetting::from_signed(rpm).unwrap().rpm(), rpm);
        }
    }
    for bad in [i16::MIN, -257, 257, i16::MAX] {
        assert!(MotorSetting::from_signed(bad).is_err());
    }
    for (magnitude, expected) in [
        (i32::MIN, 1),
        (-1, 1),
        (0, 1),
        (1, 1),
        (16, 16),
        (256, 256),
        (i32::MAX, 256),
    ] {
        assert_eq!(
            MotorSetting::from_board(Sign::Positive, magnitude).rpm(),
            expected
        );
        assert_eq!(
            MotorSetting::from_board(Sign::Negative, magnitude).rpm(),
            -expected
        );
    }
}

#[test]
fn signed_ratios_and_reciprocal_edges() {
    for (transmission, expected) in [
        (Transmission::Shaft, 16.0),
        (Transmission::EqualCogs, -16.0),
        (Transmission::LargeToSmall, -32.0),
        (Transmission::SmallToLarge, -8.0),
    ] {
        for sign in [-1.0, 1.0] {
            let answer = solve(
                &[source(0, sign * 16.0, 16384.0), node(1, 0.0)],
                &[edge(0, 1, transmission)],
            )
            .unwrap();
            assert_eq!(answer[1].theoretical_rpm, sign * expected);
            assert_eq!(answer[1].effective_rpm, sign * expected);
            let inverse = solve(
                &[node(0, 0.0), source(1, sign * expected, 16384.0)],
                &[edge(0, 1, transmission)],
            )
            .unwrap();
            assert_eq!(inverse[0].effective_rpm, sign * 16.0);
        }
    }
}

#[test]
fn finite_equality_overload_recovery_and_expiry() {
    let mut nodes = vec![source(0, -32.0, 8.0), node(1, 4.0)];
    let mut edges = vec![edge(0, 1, Transmission::Shaft)];
    let running = solve(&nodes, &edges).unwrap();
    assert_eq!(
        (running[1].stress_su, running[1].capacity_su),
        (128.0, 256.0)
    );
    nodes.push(node(2, 4.0));
    edges.push(edge(1, 2, Transmission::EqualCogs));
    let equality = solve(&nodes, &edges).unwrap();
    assert_eq!(equality[2].state, OperatingState::Running);
    assert_eq!(
        (
            equality[2].effective_rpm,
            equality[2].stress_su,
            equality[2].capacity_su
        ),
        (32.0, 256.0, 256.0)
    );
    nodes.push(node(3, 4.0));
    edges.push(edge(1, 3, Transmission::EqualCogs));
    let overload = solve(&nodes, &edges).unwrap();
    for machine in overload {
        assert_eq!(machine.state, OperatingState::Overstressed);
        assert_eq!(machine.theoretical_rpm.abs(), 32.0);
        assert_eq!(
            (
                machine.effective_rpm,
                machine.stress_su,
                machine.capacity_su
            ),
            (0.0, 384.0, 256.0)
        );
    }
    nodes.pop();
    edges.pop();
    assert_eq!(solve(&nodes, &edges).unwrap(), equality);
    nodes[0].source.as_mut().unwrap().rpm = 0.0;
    for machine in solve(&nodes, &edges).unwrap() {
        assert_eq!(machine.state, OperatingState::Unpowered);
        assert_eq!(
            (
                machine.theoretical_rpm,
                machine.effective_rpm,
                machine.stress_su,
                machine.capacity_su
            ),
            (0.0, 0.0, 0.0, 0.0)
        );
    }
}

#[test]
fn local_speed_sets_stress_not_driver_speed() {
    let answer = solve(
        &[source(0, 16.0, 8.0), node(1, 4.0)],
        &[edge(0, 1, Transmission::LargeToSmall)],
    )
    .unwrap();
    assert_eq!((answer[1].stress_su, answer[1].capacity_su), (128.0, 128.0));
    assert_eq!(answer[1].state, OperatingState::Running);
}

#[test]
fn split_merge_reversal_and_independent_components_do_not_retain_power() {
    let mut nodes = vec![
        source(0, 16.0, 8.0),
        node(1, 0.0),
        node(2, 4.0),
        source(3, -16.0, 8.0),
    ];
    let joined = vec![
        edge(0, 1, Transmission::Shaft),
        edge(1, 2, Transmission::Shaft),
    ];
    assert_eq!(solve(&nodes, &joined).unwrap()[2].effective_rpm, 16.0);
    let split = solve(&nodes, &joined[..1]).unwrap();
    assert_eq!(
        (split[2].state, split[2].capacity_su, split[2].effective_rpm),
        (OperatingState::Unpowered, 0.0, 0.0)
    );
    nodes[0].source.as_mut().unwrap().rpm = -16.0;
    assert_eq!(solve(&nodes, &joined).unwrap()[2].effective_rpm, -16.0);
    assert_eq!(solve(&nodes, &joined).unwrap()[3].effective_rpm, -16.0);
}

#[test]
fn input_order_does_not_change_successful_solution() {
    let mut nodes = vec![node(9, 4.0), source(4, 16.0, 8.0), node(1, 0.0)];
    let mut edges = vec![
        edge(4, 1, Transmission::Shaft),
        edge(1, 9, Transmission::EqualCogs),
    ];
    let expected = solve(&nodes, &edges).unwrap();
    nodes.reverse();
    edges.reverse();
    assert_eq!(solve(&nodes, &edges).unwrap(), expected);
    assert_eq!(
        expected.iter().map(|r| r.id).collect::<Vec<_>>(),
        vec![1, 4, 9]
    );
}

#[test]
fn rejects_unsupported_source_arbitration_and_ratio_conflicts() {
    let nodes = vec![source(0, 16.0, 8.0), node(1, 0.0), source(2, 16.0, 8.0)];
    assert_eq!(
        solve(
            &nodes,
            &[
                edge(0, 1, Transmission::Shaft),
                edge(1, 2, Transmission::Shaft)
            ]
        ),
        Err(SolveError::MultipleSourcesUnsupported(0))
    );
    let nodes = vec![source(0, 16.0, 8.0), node(1, 0.0), node(2, 0.0)];
    let edges = [
        edge(0, 1, Transmission::EqualCogs),
        edge(1, 2, Transmission::EqualCogs),
        edge(2, 0, Transmission::EqualCogs),
    ];
    assert!(matches!(
        solve(&nodes, &edges),
        Err(SolveError::ConflictingCycleUnsupported(..))
    ));
    let consistent = [
        edge(0, 1, Transmission::EqualCogs),
        edge(1, 2, Transmission::EqualCogs),
        edge(2, 0, Transmission::Shaft),
    ];
    assert_eq!(solve(&nodes, &consistent).unwrap()[2].effective_rpm, 16.0);
}

#[test]
fn rejects_invalid_topology_and_non_finite_facts() {
    assert_eq!(solve(&[], &[]), Ok(vec![]));
    assert_eq!(
        solve(&[node(1, 0.0), node(1, 0.0)], &[]),
        Err(SolveError::DuplicateNode(1))
    );
    assert_eq!(
        solve(&[node(1, 0.0)], &[edge(1, 2, Transmission::Shaft)]),
        Err(SolveError::UnknownNode(2))
    );
    assert_eq!(
        solve(&[node(1, 0.0)], &[edge(1, 1, Transmission::Shaft)]),
        Err(SolveError::InvalidEdge(1, 1))
    );
    let edges = [
        edge(0, 1, Transmission::Shaft),
        edge(1, 0, Transmission::Shaft),
    ];
    assert_eq!(
        solve(&[node(0, 0.0), node(1, 0.0)], &edges),
        Err(SolveError::InvalidEdge(1, 0))
    );
    for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -1.0] {
        assert_eq!(
            solve(&[node(0, bad)], &[]),
            Err(SolveError::InvalidFacts(0))
        );
        assert_eq!(
            solve(&[source(0, 16.0, bad)], &[]),
            Err(SolveError::InvalidFacts(0))
        );
    }
    for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        assert_eq!(
            solve(&[source(0, bad, 8.0)], &[]),
            Err(SolveError::InvalidFacts(0))
        );
    }
    assert_eq!(
        solve(&[source(0, 257.0, 8.0)], &[]),
        Err(SolveError::OverspeedUnsupported(0))
    );
    assert_eq!(
        solve(
            &[source(0, 256.0, 8.0), node(1, 0.0)],
            &[edge(0, 1, Transmission::LargeToSmall)]
        ),
        Err(SolveError::OverspeedUnsupported(1))
    );
    assert_eq!(
        solve(&[source(0, 16.0, f32::MAX)], &[]),
        Err(SolveError::NumericalRange(0))
    );
    assert_eq!(
        solve(
            &[source(0, 16.0, 8.0), node(1, f32::MAX)],
            &[edge(0, 1, Transmission::Shaft)]
        ),
        Err(SolveError::NumericalRange(1))
    );
}

#[test]
fn rejects_unrepresentable_ratio_chains_without_partial_solution() {
    let nodes: Vec<_> = (0..=150).map(|id| node(id, 0.0)).collect();
    let edges: Vec<_> = (0..150)
        .map(|id| edge(id, id + 1, Transmission::SmallToLarge))
        .collect();
    assert_eq!(solve(&nodes, &edges), Err(SolveError::NumericalRange(150)));
    let edges: Vec<_> = (0..128)
        .map(|id| edge(id, id + 1, Transmission::LargeToSmall))
        .collect();
    assert_eq!(
        solve(&nodes[..=128], &edges),
        Err(SolveError::NumericalRange(128))
    );
}

#[test]
fn rejects_positive_capacity_and_load_products_that_underflow_to_zero() {
    let tiny = f32::from_bits(1);
    assert_eq!(
        solve(&[source(0, tiny, 0.5)], &[]),
        Err(SolveError::NumericalRange(0))
    );
    let load = Node {
        impact_per_rpm: 0.5,
        ..source(0, tiny, 0.0)
    };
    assert_eq!(solve(&[load], &[]), Err(SolveError::NumericalRange(0)));
}
