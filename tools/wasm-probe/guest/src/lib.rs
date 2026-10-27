//! Development math probe. SDK command context links the players interface;
//! this component calls no player, world, storage or item operation.
#[path = "../../../kinetic_probe_bridge.rs"]
mod bridge;
use horizon_mod_sdk::server::{Args, Argument, Player, Registration, ServerMod};

struct Probe;
impl ServerMod for Probe {
    fn init() -> Result<(Self, Registration), String> {
        Ok((
            Self,
            Registration::new().command_with("evaluate", [Argument::text("json")]),
        ))
    }
    fn command(
        &mut self,
        _player: &Player,
        name: &str,
        args: &Args,
    ) -> Result<Option<String>, String> {
        if name != "evaluate" {
            return Err("Unknown probe command".into());
        }
        let input = serde_json::from_str(args.string("json").ok_or("Missing JSON")?)
            .map_err(|e| e.to_string())?;
        Ok(Some(bridge::run(input)?.to_string()))
    }
}
horizon_mod_sdk::export_server_mod!(Probe);
