//! Development-only JSON bridge; the player library has no serialization dependency.
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

pub fn run(input: Value) -> Result<Value, String> {
    let mut results = Vec::new();
    let mut geometry_names = BTreeSet::new();
    for case in array(&input)? {
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
