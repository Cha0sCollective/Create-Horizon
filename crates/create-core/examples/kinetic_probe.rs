//! Native development entry point for the shared kinetic probe bridge.
#[path = "../../../tools/kinetic_probe_bridge.rs"]
mod bridge;
use std::io::{self, Read};

fn main() {
    let result = (|| {
        let mut text = String::new();
        io::stdin()
            .read_to_string(&mut text)
            .map_err(|e| e.to_string())?;
        bridge::run(serde_json::from_str(&text).map_err(|e| e.to_string())?)
    })();
    match result {
        Ok(value) => println!("{value}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    }
}
