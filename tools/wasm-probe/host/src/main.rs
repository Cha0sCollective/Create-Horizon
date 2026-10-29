//! Development bridge to Horizon's real server-component sandbox, without a world.
use horizon_mods::{
    Capability, Manifest, Output, Package, Side,
    manifest::{component_imports, componentize},
    testing::TestGame,
};
use serde_json::{Value, json};
use std::{
    collections::BTreeSet,
    io::{self, Read},
    path::Path,
};

const MANIFEST: &str = "id='kinetic_probe'\nname='Kinetic probe'\nversion='0.0.0'\napi='0.3'\nside='server'\ncapabilities=['players']\n";

fn package(core: &Path, output: &Path) -> Result<(), String> {
    let bytes = std::fs::read(core).map_err(|e| e.to_string())?;
    let component = componentize(&bytes)?;
    Manifest::parse(MANIFEST)?
        .check_imports(component_imports(&component)?.iter().map(String::as_str))?;
    std::fs::write(output, Package::write(MANIFEST, &component)?).map_err(|e| e.to_string())
}

/// The guest receives one case per command, so batch-only geometry identity must
/// be checked here first. Legacy names do not participate in this policy.
fn preflight_geometry_names(cases: &[Value]) -> Result<(), String> {
    let mut names = BTreeSet::new();
    for case in cases {
        if case.get("geometry").is_none() {
            continue;
        }
        let name = case["case"]
            .as_str()
            .filter(|name| !name.is_empty())
            .ok_or("Expected nonempty case name")?;
        if !names.insert(name) {
            return Err("Duplicate geometry case name".into());
        }
    }
    Ok(())
}

fn evaluate() -> Result<Value, String> {
    let path =
        std::env::var_os("CREATE_KINETIC_COMPONENT").ok_or("Missing CREATE_KINETIC_COMPONENT")?;
    let path = Path::new(&path);
    let package = Package::read(path)?;
    if package.manifest.id != "kinetic_probe"
        || package.manifest.side != Side::Server
        || package.manifest.api != "0.3"
        || package.manifest.capabilities != [Capability::Players]
    {
        return Err("Expected the server probe with only the SDK players interface".into());
    }
    let mut game = TestGame::load(path)?;
    game.join("ReferenceFixture");
    let actor = game.players[0].clone();
    let _ = game.drain();
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| e.to_string())?;
    let input: Value = serde_json::from_str(&input).map_err(|e| e.to_string())?;
    let cases = input.as_array().ok_or("Expected cases")?;
    preflight_geometry_names(cases)?;
    let mut all = Vec::new();
    for case in cases {
        let text = json!([case]).to_string();
        if !game.player_command(&actor, &format!("evaluate {text}")) {
            return Err("Probe command missing".into());
        }
        let mut reply = None;
        for output in game.drain() {
            match output {
                Output::Stopped { reason, .. } => return Err(format!("Wasm stopped: {reason}")),
                Output::Reply {
                    text,
                    error,
                    player,
                } => {
                    if player != actor.id || error || reply.is_some() {
                        return Err(format!("Invalid Wasm reply: {text}"));
                    }
                    reply = Some(text);
                }
                _ => return Err("Unexpected Wasm side effect".into()),
            }
        }
        let rows: Value =
            serde_json::from_str(&reply.ok_or("No Wasm reply")?).map_err(|e| e.to_string())?;
        let rows = rows.as_array().ok_or("Expected result array")?;
        if rows.len() != 1 || rows[0]["case"] != case["case"] {
            return Err("Wrong Wasm result identity".into());
        }
        all.push(rows[0].clone());
    }
    Ok(json!(all))
}

fn main() {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let result = if args.len() == 3 && args[0] == "package" {
        package(Path::new(&args[1]), Path::new(&args[2])).map(|_| json!({"packaged":true}))
    } else if args.is_empty() {
        evaluate()
    } else {
        Err("Usage: [package CORE OUTPUT]".into())
    };
    match result {
        Ok(value) => println!("{value}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::preflight_geometry_names;
    use serde_json::{Value, json};

    fn geometry(name: &str) -> Value {
        json!({"case":name,"geometry":{"blocks":[
            {"id":0,"position":[0,0,0],"kind":"motor","facing":"east","setting":16,"capacity_per_rpm":16384},
            {"id":1,"position":[1,0,0],"kind":"shaft","axis":"x"}
        ]}})
    }

    fn legacy(name: &str) -> Value {
        json!({"case":name,"nodes":[],"edges":[]})
    }

    #[test]
    fn unique_geometry_and_repeated_legacy_names_are_accepted_without_mutation() {
        let cases = [geometry("a"), legacy("a"), legacy("a"), geometry("b")];
        let before = cases.clone();
        assert_eq!(preflight_geometry_names(&cases), Ok(()));
        assert_eq!(cases, before);
        assert_eq!(
            preflight_geometry_names(&[legacy("a"), geometry("a")]),
            Ok(())
        );
        assert_eq!(preflight_geometry_names(&[]), Ok(()));
    }

    #[test]
    fn repeated_valid_geometry_names_refuse_even_across_a_legacy_case() {
        let cases = [geometry("a"), legacy("a"), geometry("a")];
        let before = cases.clone();
        assert_eq!(
            preflight_geometry_names(&cases),
            Err("Duplicate geometry case name".into())
        );
        assert_eq!(cases, before);
    }

    #[test]
    fn geometry_name_shape_is_checked_only_for_geometry_cases() {
        assert_eq!(
            preflight_geometry_names(&[json!({"case":false,"nodes":[],"edges":[]})]),
            Ok(())
        );
        for case in [
            json!({"geometry":{"blocks":[]}}),
            json!({"case":false,"geometry":{"blocks":[]}}),
            json!({"case":"","geometry":{"blocks":[]}}),
        ] {
            assert_eq!(
                preflight_geometry_names(&[case]),
                Err("Expected nonempty case name".into())
            );
        }
    }
}
