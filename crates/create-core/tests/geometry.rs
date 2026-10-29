use create_horizon_core::geometry::{
    Axis, Block, BlockKind, Facing, GeometryError, derive_topology,
};
use create_horizon_core::{Edge, MotorSetting, OperatingState, SolveError, Transmission, solve};

fn motor(id: u32, position: [i32; 3], facing: Facing, setting: i16) -> Block {
    Block {
        id,
        position,
        kind: BlockKind::Motor {
            facing,
            setting: MotorSetting::from_signed(setting).unwrap(),
            capacity_per_rpm: 16384.0,
        },
    }
}
fn shaft(id: u32, position: [i32; 3], axis: Axis) -> Block {
    Block {
        id,
        position,
        kind: BlockKind::Shaft { axis },
    }
}
fn cog(id: u32, position: [i32; 3], axis: Axis, large: bool) -> Block {
    Block {
        id,
        position,
        kind: if large {
            BlockKind::LargeCog { axis }
        } else {
            BlockKind::SmallCog { axis }
        },
    }
}
fn edge(from: u32, to: u32, transmission: Transmission) -> Edge {
    Edge {
        from,
        to,
        transmission,
    }
}
fn horizontal(x: bool, connected: bool, generated: i16) -> Vec<Block> {
    let axis = if x { Axis::X } else { Axis::Z };
    vec![
        motor(
            0,
            if x { [2, 3, 3] } else { [3, 3, 4] },
            if x { Facing::East } else { Facing::North },
            if x { generated } else { -generated },
        ),
        shaft(1, [3, 3, 3], axis),
        cog(2, if x { [4, 3, 3] } else { [3, 3, 2] }, axis, true),
        cog(
            3,
            if x { [4, 4, 4] } else { [4, 4, 2] },
            if connected {
                axis
            } else if x {
                Axis::Z
            } else {
                Axis::X
            },
            false,
        ),
    ]
}

#[test]
fn four_measured_layouts_both_signs_have_literal_topology_and_settled_values() {
    for x in [false, true] {
        for connected in [false, true] {
            for generated in [-16, 16] {
                let topology = derive_topology(&horizontal(x, connected, generated)).unwrap();
                let mut expected = vec![
                    edge(0, 1, Transmission::Shaft),
                    edge(1, 2, Transmission::Shaft),
                ];
                if connected {
                    expected.push(edge(2, 3, Transmission::LargeToSmall));
                }
                assert_eq!(topology.edges, expected);
                assert_eq!(topology.nodes[0].source.unwrap().rpm, f32::from(generated));
                assert!(topology.nodes.iter().all(|n| n.impact_per_rpm == 0.0));
                assert!(topology.nodes[1..].iter().all(|n| n.source.is_none()));
                let rotations = solve(&topology.nodes, &topology.edges).unwrap();
                assert_eq!(
                    rotations.iter().map(|r| r.id).collect::<Vec<_>>(),
                    vec![0, 1, 2, 3]
                );
                for r in rotations {
                    let powered = r.id != 3 || connected;
                    let expected_rpm = if r.id != 3 {
                        f32::from(generated)
                    } else if connected {
                        -2.0 * f32::from(generated)
                    } else {
                        0.0
                    };
                    assert_eq!(r.theoretical_rpm, expected_rpm);
                    assert_eq!(r.effective_rpm, expected_rpm);
                    assert_eq!(r.stress_su, 0.0);
                    assert_eq!(r.capacity_su, if powered { 262144.0 } else { 0.0 });
                    assert_eq!(
                        r.state,
                        if powered {
                            OperatingState::Running
                        } else {
                            OperatingState::Unpowered
                        }
                    );
                }
            }
        }
    }
}

#[test]
fn motor_face_rear_side_and_gap_are_distinct_for_each_supported_facing() {
    for (facing, axis, front, rear, side, gap) in [
        (
            Facing::East,
            Axis::X,
            [1, 0, 0],
            [-1, 0, 0],
            [0, 0, 1],
            [2, 0, 0],
        ),
        (
            Facing::North,
            Axis::Z,
            [0, 0, -1],
            [0, 0, 1],
            [1, 0, 0],
            [0, 0, -2],
        ),
    ] {
        for (position, expected) in [(front, true), (rear, false), (side, false), (gap, false)] {
            let blocks = [motor(0, [0, 0, 0], facing, 16), shaft(1, position, axis)];
            let topology = derive_topology(&blocks).unwrap();
            assert_eq!(
                topology.edges,
                if expected {
                    vec![edge(0, 1, Transmission::Shaft)]
                } else {
                    vec![]
                }
            );
        }
        let wrong_axis = if axis == Axis::X { Axis::Z } else { Axis::X };
        assert!(
            derive_topology(&[motor(0, [0, 0, 0], facing, 16), shaft(1, front, wrong_axis)])
                .unwrap()
                .edges
                .is_empty()
        );
    }
}

#[test]
fn coaxial_cogs_and_shafts_use_mutual_shaft_faces_before_gears() {
    for axis in [Axis::X, Axis::Z] {
        let unit = if axis == Axis::X {
            [1, 0, 0]
        } else {
            [0, 0, 1]
        };
        for a_large in [false, true] {
            for b_large in [false, true] {
                let blocks = [
                    cog(4, [0, 0, 0], axis, a_large),
                    cog(9, unit, axis, b_large),
                ];
                assert_eq!(
                    derive_topology(&blocks).unwrap().edges,
                    vec![edge(4, 9, Transmission::Shaft)]
                );
            }
        }
        let blocks = [shaft(4, [0, 0, 0], axis), cog(9, unit, axis, true)];
        assert_eq!(
            derive_topology(&blocks).unwrap().edges,
            vec![edge(4, 9, Transmission::Shaft)]
        );
    }
}

#[test]
fn source_derived_all_diagonal_quadrants_use_large_to_small_orientation() {
    // Source-derived checks; the horizontal corpus observes only one quadrant per axis.
    for axis in [Axis::X, Axis::Z] {
        for first in [-1, 1] {
            for second in [-1, 1] {
                let target = if axis == Axis::X {
                    [0, first, second]
                } else {
                    [first, second, 0]
                };
                let mut blocks = [cog(9, [0, 0, 0], axis, true), cog(2, target, axis, false)];
                let expected = vec![edge(9, 2, Transmission::LargeToSmall)];
                assert_eq!(derive_topology(&blocks).unwrap().edges, expected);
                blocks.reverse();
                assert_eq!(derive_topology(&blocks).unwrap().edges, expected);
            }
        }
    }
}

#[test]
fn missing_perpendicular_component_is_illegal_but_distant_and_axial_offsets_do_not_mesh() {
    for axis in [Axis::X, Axis::Z] {
        let invalid = if axis == Axis::X {
            [0, 1, 0]
        } else {
            [1, 0, 0]
        };
        assert_eq!(
            derive_topology(&[cog(0, [0, 0, 0], axis, true), cog(1, invalid, axis, false)]),
            Err(GeometryError::InvalidCogPlacement(0, 1))
        );
        let positions = if axis == Axis::X {
            [[0, 2, 1], [0, 1, 2], [1, 1, 1], [2, 0, 0], [0, 0, 2]]
        } else {
            [[2, 1, 0], [1, 2, 0], [1, 1, 1], [0, 0, 2], [2, 0, 0]]
        };
        for position in positions {
            assert!(
                derive_topology(&[cog(0, [0, 0, 0], axis, true), cog(1, position, axis, false)])
                    .unwrap()
                    .edges
                    .is_empty()
            );
        }
        let diagonal = if axis == Axis::X {
            [0, 1, 1]
        } else {
            [1, 1, 0]
        };
        let other_axis = if axis == Axis::X { Axis::Z } else { Axis::X };
        assert!(
            derive_topology(&[
                cog(0, [0, 0, 0], axis, true),
                cog(1, diagonal, other_axis, false)
            ])
            .unwrap()
            .edges
            .is_empty()
        );
        assert!(
            derive_topology(&[cog(0, [0, 0, 0], axis, true), cog(1, diagonal, axis, true)])
                .unwrap()
                .edges
                .is_empty()
        );
    }
}

#[test]
fn small_drives_large_through_existing_reciprocal_without_a_second_edge() {
    for generated in [-16, 16] {
        let blocks = [
            motor(0, [-2, 0, 0], Facing::East, generated),
            shaft(1, [-1, 0, 0], Axis::X),
            cog(2, [0, 0, 0], Axis::X, false),
            cog(3, [0, 1, 1], Axis::X, true),
        ];
        let topology = derive_topology(&blocks).unwrap();
        assert_eq!(
            topology.edges,
            vec![
                edge(0, 1, Transmission::Shaft),
                edge(1, 2, Transmission::Shaft),
                edge(3, 2, Transmission::LargeToSmall)
            ]
        );
        let rotations = solve(&topology.nodes, &topology.edges).unwrap();
        assert_eq!(rotations[3].effective_rpm, -0.5 * f32::from(generated));
    }
}

#[test]
fn input_order_id_permutations_and_translations_preserve_physical_results() {
    let original = horizontal(true, true, 16);
    let expected = derive_topology(&original).unwrap();
    let mut reversed = original.clone();
    reversed.reverse();
    assert_eq!(derive_topology(&reversed).unwrap(), expected);
    let mut translated = original.clone();
    for b in &mut translated {
        b.position = b.position.map(|v| v - 1000);
    }
    assert_eq!(derive_topology(&translated).unwrap(), expected);
    let mut permuted = original;
    for (b, id) in permuted.iter_mut().zip([9, 2, 7, 1]) {
        b.id = id;
    }
    let topology = derive_topology(&permuted).unwrap();
    assert_eq!(
        topology.edges,
        vec![
            edge(7, 1, Transmission::LargeToSmall),
            edge(2, 7, Transmission::Shaft),
            edge(2, 9, Transmission::Shaft)
        ]
    );
    let rotations = solve(&topology.nodes, &topology.edges).unwrap();
    assert_eq!(
        rotations
            .iter()
            .map(|r| (r.id, r.effective_rpm))
            .collect::<Vec<_>>(),
        vec![(1, -32.0), (2, 16.0), (7, 16.0), (9, 16.0)]
    );
}

#[test]
fn extreme_i32_positions_cannot_wrap_or_create_false_neighbors() {
    let blocks = [
        shaft(0, [i32::MIN, 0, 0], Axis::X),
        shaft(1, [i32::MAX, 0, 0], Axis::X),
    ];
    assert!(derive_topology(&blocks).unwrap().edges.is_empty());
    for base in [i32::MIN, i32::MAX - 1] {
        let blocks = [
            shaft(0, [base, 0, 0], Axis::X),
            shaft(1, [base + 1, 0, 0], Axis::X),
        ];
        assert_eq!(
            derive_topology(&blocks).unwrap().edges,
            vec![edge(0, 1, Transmission::Shaft)]
        );
    }
    let blocks = [
        cog(0, [i32::MIN, i32::MIN, i32::MIN], Axis::X, true),
        cog(1, [i32::MIN, i32::MAX, i32::MAX], Axis::X, false),
    ];
    assert!(derive_topology(&blocks).unwrap().edges.is_empty());
    assert_eq!(derive_topology(&[]).unwrap().nodes, vec![]);
}

#[test]
fn identity_domain_placement_and_unimplemented_physical_couplings_have_specific_errors() {
    let a = shaft(0, [0, 0, 0], Axis::X);
    assert_eq!(
        derive_topology(&[a, shaft(0, [2, 0, 0], Axis::X)]),
        Err(GeometryError::DuplicateId(0))
    );
    assert_eq!(
        derive_topology(&[a, shaft(1, [0, 0, 0], Axis::X)]),
        Err(GeometryError::DuplicatePosition(0, 1))
    );
    assert_eq!(
        derive_topology(&[shaft(0, [0, 0, 0], Axis::Y)]),
        Err(GeometryError::UnsupportedAxis(0, Axis::Y))
    );
    for facing in [Facing::West, Facing::South, Facing::Up, Facing::Down] {
        assert_eq!(
            derive_topology(&[motor(0, [0, 0, 0], facing, 16)]),
            Err(GeometryError::UnsupportedFacing(0, facing))
        );
    }
    assert_eq!(
        derive_topology(&[
            cog(0, [0, 0, 0], Axis::X, true),
            cog(1, [0, 1, 0], Axis::X, true)
        ]),
        Err(GeometryError::InvalidCogPlacement(0, 1))
    );
    assert_eq!(
        derive_topology(&[
            cog(0, [0, 0, 0], Axis::X, false),
            cog(1, [0, 1, 0], Axis::X, false)
        ]),
        Err(GeometryError::SmallCogMeshUnsupported(0, 1))
    );
    assert_eq!(
        derive_topology(&[
            cog(0, [0, 0, 0], Axis::X, true),
            cog(1, [1, 0, 1], Axis::Z, true)
        ]),
        Err(GeometryError::PerpendicularLargeCogsUnsupported(0, 1))
    );
    // Neighbor AXIS parallel to the face direction exempts otherwise large-cog exclusion.
    assert!(
        derive_topology(&[
            cog(0, [0, 0, 0], Axis::X, true),
            cog(1, [0, 0, 1], Axis::Z, true)
        ])
        .unwrap()
        .edges
        .is_empty()
    );
}

#[test]
fn geometry_retains_solver_errors_instead_of_inventing_arbitration_or_destruction() {
    let mut blocks = vec![
        motor(0, [-2, 0, 0], Facing::East, 16),
        shaft(1, [-1, 0, 0], Axis::X),
        cog(2, [0, 0, 0], Axis::X, true),
        cog(3, [0, 1, 1], Axis::X, false),
        shaft(4, [-1, 1, 1], Axis::X),
        motor(5, [-2, 1, 1], Facing::East, 16),
    ];
    let topology = derive_topology(&blocks).unwrap();
    assert_eq!(
        solve(&topology.nodes, &topology.edges),
        Err(SolveError::MultipleSourcesUnsupported(0))
    );
    blocks.truncate(4);
    blocks[0] = motor(0, [-2, 0, 0], Facing::East, 256);
    let topology = derive_topology(&blocks).unwrap();
    assert_eq!(
        solve(&topology.nodes, &topology.edges),
        Err(SolveError::OverspeedUnsupported(3))
    );
    for capacity in [f32::NAN, f32::INFINITY, -1.0] {
        blocks[0].kind = BlockKind::Motor {
            facing: Facing::East,
            setting: MotorSetting::default(),
            capacity_per_rpm: capacity,
        };
        let topology = derive_topology(&blocks).unwrap();
        assert_eq!(
            solve(&topology.nodes, &topology.edges),
            Err(SolveError::InvalidFacts(0))
        );
    }
}

#[path = "../../../tools/kinetic_probe_bridge.rs"]
mod probe_bridge;

fn geometry_request() -> serde_json::Value {
    serde_json::json!([{"case":"geometry-control", "geometry":{"blocks":[
        {"id":0,"position":[0,0,0],"kind":"motor","facing":"east","setting":16,"capacity_per_rpm":16384},
        {"id":1,"position":[1,0,0],"kind":"shaft","axis":"x"}
    ]}}])
}

#[test]
fn geometry_probe_decodes_exact_records_and_keeps_legacy_result_shape() {
    let result = probe_bridge::run(geometry_request()).unwrap();
    assert_eq!(
        result[0]["edges"],
        serde_json::json!([{"from":0,"to":1,"transmission":"shaft"}])
    );
    assert_eq!(result[0]["rotations"][1]["effective_rpm"], 16.0);
    assert_eq!(result[0].as_object().unwrap().len(), 3);
    let legacy = serde_json::json!([{"case":"legacy", "nodes":[
        {"id":0,"impact_per_rpm":0,"source":{"rpm":16,"capacity_per_rpm":16384}}
    ], "edges":[]}]);
    let result = probe_bridge::run(legacy).unwrap();
    assert_eq!(result[0].as_object().unwrap().len(), 2);
    assert!(result[0].get("edges").is_none());
    assert_eq!(result[0]["rotations"][0]["effective_rpm"], 16.0);
}

#[test]
fn geometry_probe_refuses_malformed_extra_mixed_and_unsupported_input() {
    use serde_json::json;
    let base = geometry_request();
    let mut controls = Vec::new();
    for (key, value) in [
        ("id", json!(true)),
        ("position", json!([false, 0, 0])),
        ("position", json!([0, 0])),
        ("position", json!([2147483648_i64, 0, 0])),
        ("position", json!([0.0, 0, 0])),
        ("setting", json!(false)),
        ("setting", json!(0)),
        ("setting", json!(257)),
        ("capacity_per_rpm", json!(true)),
        ("capacity_per_rpm", json!("16384")),
        ("capacity_per_rpm", json!(1.0e100)),
        ("capacity_per_rpm", json!(1.0e-100)),
        ("capacity_per_rpm", json!(-1)),
        ("facing", json!("west")),
        ("facing", json!("unknown")),
        ("kind", json!("millstone")),
        ("waterlogged", json!(true)),
    ] {
        let mut request = base.clone();
        request[0]["geometry"]["blocks"][0][key] = value;
        controls.push(request);
    }
    for value in [json!("y"), json!("unknown"), json!(true)] {
        let mut request = base.clone();
        request[0]["geometry"]["blocks"][1]["axis"] = value;
        controls.push(request);
    }
    for (key, value) in [("id", json!(0)), ("position", json!([0, 0, 0]))] {
        let mut request = base.clone();
        request[0]["geometry"]["blocks"][1][key] = value;
        controls.push(request);
    }
    let mut request = base.clone();
    request[0]["geometry"]["blocks"][0]
        .as_object_mut()
        .unwrap()
        .remove("setting");
    controls.push(request);
    let mut request = base.clone();
    request[0]["edges"] = json!([]);
    controls.push(request);
    let mut request = base.clone();
    request[0]["geometry"]["extra"] = json!(0);
    controls.push(request);
    let mut request = base.clone();
    request[0]["case"] = json!(false);
    controls.push(request);
    let mut request = base.clone();
    request.as_array_mut().unwrap().push(base[0].clone());
    controls.push(request);
    for (index, request) in controls.into_iter().enumerate() {
        assert!(
            probe_bridge::run(request).is_err(),
            "malformed geometry control {index} accepted"
        );
    }
}
