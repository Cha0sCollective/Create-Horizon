//! Development-only JSON bridge; the player library has no serialization dependency.
use create_horizon_core::declared_arbitration::{
    Event as TrialEvent, NodeView, TrialSpec, run_declared_trial,
};
use create_horizon_core::geometry::{Axis, Block, BlockKind, Facing, derive_topology};
use create_horizon_core::{
    Edge, MotorSetting, Node, OperatingState, Sign, Source, Transmission, solve,
};
use serde_json::{Value, json};
use std::collections::BTreeSet;

fn id(value: &Value) -> Result<u32, String> {
    value
        .as_u64()
        .and_then(|v| u32::try_from(v).ok())
        .ok_or_else(|| "Expected u32 ID".into())
}
fn number(value: &Value) -> Result<f32, String> {
    value
        .as_f64()
        .map(|v| v as f32)
        .ok_or_else(|| "Expected number".into())
}
fn array(value: &Value) -> Result<&Vec<Value>, String> {
    value.as_array().ok_or_else(|| "Expected array".into())
}
fn exact_fields(value: &Value, fields: &[&str]) -> Result<(), String> {
    let object = value.as_object().ok_or("Expected object")?;
    if object.len() != fields.len() || fields.iter().any(|key| !object.contains_key(*key)) {
        return Err(format!("Expected exactly fields {fields:?}"));
    }
    Ok(())
}
fn geometry_axis(value: &Value) -> Result<Axis, String> {
    match value.as_str() {
        Some("x") => Ok(Axis::X),
        Some("y") => Ok(Axis::Y),
        Some("z") => Ok(Axis::Z),
        _ => Err("Unknown geometry axis".into()),
    }
}
fn geometry_facing(value: &Value) -> Result<Facing, String> {
    match value.as_str() {
        Some("east") => Ok(Facing::East),
        Some("west") => Ok(Facing::West),
        Some("up") => Ok(Facing::Up),
        Some("down") => Ok(Facing::Down),
        Some("south") => Ok(Facing::South),
        Some("north") => Ok(Facing::North),
        _ => Err("Unknown geometry facing".into()),
    }
}
fn geometry_number(value: &Value) -> Result<f32, String> {
    let value = value.as_f64().ok_or("Expected finite geometry number")?;
    let converted = value as f32;
    if !value.is_finite() || !converted.is_finite() || (value != 0.0 && converted == 0.0) {
        return Err("Geometry number outside representable f32 range".into());
    }
    Ok(converted)
}
fn geometry_block(value: &Value) -> Result<Block, String> {
    let kind = match value["kind"].as_str() {
        Some("motor") => {
            exact_fields(
                value,
                &[
                    "id",
                    "position",
                    "kind",
                    "facing",
                    "setting",
                    "capacity_per_rpm",
                ],
            )?;
            let setting = value["setting"]
                .as_i64()
                .and_then(|v| i16::try_from(v).ok())
                .ok_or("Expected signed integer motor setting")?;
            BlockKind::Motor {
                facing: geometry_facing(&value["facing"])?,
                setting: MotorSetting::from_signed(setting)
                    .map_err(|_| "Invalid geometry motor setting")?,
                capacity_per_rpm: geometry_number(&value["capacity_per_rpm"])?,
            }
        }
        Some(kind @ ("shaft" | "small_cog" | "large_cog")) => {
            exact_fields(value, &["id", "position", "kind", "axis"])?;
            let axis = geometry_axis(&value["axis"])?;
            match kind {
                "shaft" => BlockKind::Shaft { axis },
                "small_cog" => BlockKind::SmallCog { axis },
                _ => BlockKind::LargeCog { axis },
            }
        }
        _ => return Err("Unknown geometry block kind".into()),
    };
    let coordinates = array(&value["position"])?;
    if coordinates.len() != 3 {
        return Err("Expected three position coordinates".into());
    }
    let mut position = [0; 3];
    for (i, coordinate) in coordinates.iter().enumerate() {
        position[i] = coordinate
            .as_i64()
            .and_then(|v| i32::try_from(v).ok())
            .ok_or("Expected i32 position coordinate")?;
    }
    Ok(Block {
        id: id(&value["id"])?,
        position,
        kind,
    })
}
fn geometry_case(case: &Value) -> Result<Value, String> {
    exact_fields(case, &["case", "geometry"])?;
    let name = case["case"]
        .as_str()
        .filter(|name| !name.is_empty())
        .ok_or("Expected nonempty case name")?;
    exact_fields(&case["geometry"], &["blocks"])?;
    let blocks = array(&case["geometry"]["blocks"])?
        .iter()
        .map(geometry_block)
        .collect::<Result<Vec<_>, _>>()?;
    let topology = derive_topology(&blocks).map_err(|e| format!("{name}: {e:?}"))?;
    let rotations =
        solve(&topology.nodes, &topology.edges).map_err(|e| format!("{name}: {e:?}"))?;
    let edges: Vec<_> = topology.edges.iter().map(|edge| {
        json!({"from":edge.from, "to":edge.to, "transmission":match edge.transmission {
            Transmission::Shaft => "shaft", Transmission::EqualCogs => "equal_cogs",
            Transmission::LargeToSmall => "large_to_small", Transmission::SmallToLarge => "small_to_large",
        }})
    }).collect();
    let rotations: Vec<_> = rotations.iter().map(|r| {
        json!({"id":r.id, "theoretical_rpm":r.theoretical_rpm, "effective_rpm":r.effective_rpm,
               "stress_su":r.stress_su, "capacity_su":r.capacity_su,
               "state":match r.state { OperatingState::Unpowered=>"unpowered",
                   OperatingState::Running=>"running", OperatingState::Overstressed=>"overstressed" }})
    }).collect();
    Ok(json!({"case":name, "edges":edges, "rotations":rotations}))
}

fn trial_setting(value: &Value) -> Result<MotorSetting, String> {
    let value = value
        .as_i64()
        .and_then(|v| i16::try_from(v).ok())
        .ok_or("Expected signed trial generation")?;
    MotorSetting::from_signed(value).map_err(|_| "Invalid trial generation".into())
}
fn trial_event(value: &Value) -> Result<TrialEvent, String> {
    match value["op"].as_str() {
        Some(op @ ("place_generator" | "set_generated")) => {
            exact_fields(value, &["op", "node", "generated"])?;
            let node = id(&value["node"])?;
            let generated = trial_setting(&value["generated"])?;
            Ok(if op == "place_generator" {
                TrialEvent::PlaceGenerator { node, generated }
            } else {
                TrialEvent::SetGenerated { node, generated }
            })
        }
        Some("place_join") => {
            exact_fields(value, &["op"])?;
            Ok(TrialEvent::PlaceJoin)
        }
        Some("destroy_generator") => {
            exact_fields(value, &["op", "node"])?;
            Ok(TrialEvent::DestroyGenerator {
                node: id(&value["node"])?,
            })
        }
        Some("settle") => {
            exact_fields(value, &["op", "label"])?;
            Ok(TrialEvent::Settle {
                label: value["label"]
                    .as_str()
                    .ok_or("Expected trial label")?
                    .into(),
            })
        }
        _ => Err("Unknown trial event".into()),
    }
}
fn trial_case(case: &Value) -> Result<Value, String> {
    exact_fields(case, &["case", "arbitration_trial"])?;
    let name = case["case"]
        .as_str()
        .filter(|n| !n.is_empty())
        .ok_or("Expected trial case metadata")?;
    let input = &case["arbitration_trial"];
    exact_fields(input, &["spec", "events"])?;
    let s = &input["spec"];
    exact_fields(
        s,
        &[
            "lower",
            "join",
            "upper",
            "capacity_per_rpm",
            "join_neighbor_order",
            "be_order",
        ],
    )?;
    let neighbors = array(&s["join_neighbor_order"])?;
    let order = array(&s["be_order"])?;
    if neighbors.len() != 2 || order.len() != 3 {
        return Err("Wrong trial order length".into());
    }
    let spec = TrialSpec {
        lower: id(&s["lower"])?,
        join: id(&s["join"])?,
        upper: id(&s["upper"])?,
        capacity_per_rpm: id(&s["capacity_per_rpm"])?,
        join_neighbor_order: [id(&neighbors[0])?, id(&neighbors[1])?],
        be_order: [id(&order[0])?, id(&order[1])?, id(&order[2])?],
    };
    let events = array(&input["events"])?;
    if events.len() > 16 {
        return Err("TooManyEvents".into());
    }
    let events = events
        .iter()
        .map(trial_event)
        .collect::<Result<Vec<_>, _>>()?;
    let trace = run_declared_trial(&spec, &events).map_err(|e| format!("{name}: {e:?}"))?;
    let checkpoints: Vec<_> = trace.checkpoints.into_iter().map(|checkpoint| {
        let nodes: Vec<_> = checkpoint.nodes.into_iter().map(|node| match node {
            NodeView::Absent { node } => json!({"id":node,"present":false}),
            NodeView::Present { node, generator, generated_rpm, theoretical_rpm, effective_rpm,
                source_parent, network_present, network_members, capacity_su, stress_su, flicker } =>
                json!({"id":node,"present":true,"kind":if generator {"generator"} else {"shaft"},
                    "generated_rpm":generated_rpm,"theoretical_rpm":theoretical_rpm,"effective_rpm":effective_rpm,
                    "source_parent":source_parent,"network_present":network_present,"network_members":network_members,
                    "capacity_su":capacity_su,"stress_su":stress_su,"flicker":flicker}),
        }).collect();
        let effects: Vec<_> = checkpoint.effects.iter().map(|effect|
            json!({"node":effect.node,"kind":"generator","with_drops":true})).collect();
        json!({"label":checkpoint.label,"nodes":nodes,"effects":effects})
    }).collect();
    Ok(json!({"case":name,"checkpoints":checkpoints}))
}

pub fn run(input: Value) -> Result<Value, String> {
    let mut results = Vec::new();
    let mut geometry_names = BTreeSet::new();
    for case in array(&input)? {
        if case.get("arbitration_trial").is_some() {
            results.push(trial_case(case)?);
            continue;
        }
        if case.get("geometry").is_some() {
            let result = geometry_case(case)?;
            if !geometry_names.insert(result["case"].as_str().unwrap().to_owned()) {
                return Err("Duplicate geometry case name".into());
            }
            results.push(result);
            continue;
        }
        let mut nodes = Vec::new();
        for n in array(&case["nodes"])? {
            let source = if n["source"].is_null() {
                None
            } else {
                let s = &n["source"];
                let rpm = if s.get("row").is_some() {
                    let sign = match s["row"].as_u64() {
                        Some(0) => Sign::Negative,
                        Some(1) => Sign::Positive,
                        _ => return Err("Invalid motor board row".into()),
                    };
                    let magnitude = s["magnitude"]
                        .as_i64()
                        .and_then(|v| i32::try_from(v).ok())
                        .ok_or("Invalid magnitude")?;
                    f32::from(MotorSetting::from_board(sign, magnitude).rpm())
                } else {
                    number(&s["rpm"])?
                };
                Some(Source {
                    rpm,
                    capacity_per_rpm: number(&s["capacity_per_rpm"])?,
                })
            };
            nodes.push(Node {
                id: id(&n["id"])?,
                impact_per_rpm: number(&n["impact_per_rpm"])?,
                source,
            });
        }
        let mut edges = Vec::new();
        for e in array(&case["edges"])? {
            let transmission = match e["transmission"].as_str() {
                Some("shaft") => Transmission::Shaft,
                Some("equal_cogs") => Transmission::EqualCogs,
                Some("large_to_small") => Transmission::LargeToSmall,
                Some("small_to_large") => Transmission::SmallToLarge,
                _ => return Err("Unknown transmission".into()),
            };
            edges.push(Edge {
                from: id(&e["from"])?,
                to: id(&e["to"])?,
                transmission,
            });
        }
        let rotations = solve(&nodes, &edges).map_err(|e| format!("{}: {e:?}", case["case"]))?;
        let rotations: Vec<_> = rotations
            .into_iter()
            .map(|r| {
                json!({"id":r.id,
            "theoretical_rpm":r.theoretical_rpm, "effective_rpm":r.effective_rpm,
            "stress_su":r.stress_su, "capacity_su":r.capacity_su,
            "state":match r.state { OperatingState::Unpowered=>"unpowered",
                OperatingState::Running=>"running", OperatingState::Overstressed=>"overstressed" }})
            })
            .collect();
        results.push(json!({"case":case["case"],"rotations":rotations}));
    }
    Ok(json!(results))
}
