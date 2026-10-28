//! Quiet paired client: no commands, hooks or application-level host calls.
use horizon_mod_sdk::{Mod, Registration};

struct QuietClient;

impl Mod for QuietClient {
    fn init() -> Result<(Self, Registration), String> {
        Ok((Self, Registration::new()))
    }
}

horizon_mod_sdk::export_mod!(QuietClient);
