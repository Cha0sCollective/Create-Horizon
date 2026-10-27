//! Development-only JSON bridge; the player library has no serialization dependency.
use create_horizon_core::{
    Edge, MotorSetting, Node, OperatingState, Sign, Source, Transmission, solve,
};
use serde_json::{Value, json};
use std::io::{self, Read};

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
fn run(input: Value) -> Result<Value, String> {
    let mut results = Vec::new();
    for case in array(&input)? {
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
fn main() {
    let result = (|| {
        let mut text = String::new();
        io::stdin()
            .read_to_string(&mut text)
            .map_err(|e| e.to_string())?;
        run(serde_json::from_str(&text).map_err(|e| e.to_string())?)
    })();
    match result {
        Ok(value) => println!("{value}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    }
}
