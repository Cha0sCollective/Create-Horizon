//! Import-free counter fixture; the host owns the committed counter.
#[cfg(any(
    all(feature = "trap", feature = "reject"),
    all(feature = "trap", feature = "invalid"),
    all(feature = "reject", feature = "invalid"),
    all(
        feature = "fuel",
        any(feature = "trap", feature = "reject", feature = "invalid")
    )
))]
compile_error!("select at most one counter fixture fault feature");

wit_bindgen::generate!({
    path: "wit",
    world: "guest-counter-fixture",
});

use exports::{
    counter_trial::{Proposal, Rejection, Request},
    horizon::mods::{players, server_events},
};

struct Fixture;

// These are fixture-owned exports, not calls to the host's players capability.
impl players::Guest for Fixture {
    fn online() -> Vec<players::Player> {
        Vec::new()
    }

    fn tell(_player: String, _text: String) -> bool {
        false
    }

    fn broadcast(_text: String) {}
}

impl server_events::Guest for Fixture {
    fn init() -> Result<server_events::Registration, String> {
        Ok(server_events::Registration {
            commands: Vec::new(),
            tick: false,
            joins: false,
            chat: false,
            blocks: false,
        })
    }

    fn tick(_tick: u64) {}

    fn player_joined(_player: players::Player) {}

    fn player_left(_player: players::Player) {}

    fn chat(_player: players::Player, _text: String) {}

    fn command(
        _player: players::Player,
        _name: String,
        _args: String,
        _values: Vec<(String, server_events::ArgumentValue)>,
    ) -> Result<Option<String>, String> {
        Ok(None)
    }

    fn block_broken(_player: players::Player, _pos: server_events::BlockPos, _state: String) {}

    fn block_placed(_player: players::Player, _pos: server_events::BlockPos, _state: String) {}
}

impl exports::counter_trial::Guest for Fixture {
    fn step(input: Request) -> Result<Proposal, Rejection> {
        #[cfg(feature = "trap")]
        if input.counter >= 3 {
            panic!("counter fixture trap after three committed steps");
        }
        #[cfg(feature = "reject")]
        if input.counter >= 3 {
            return Err(Rejection::Rejected);
        }
        #[cfg(feature = "fuel")]
        if input.counter >= 3 {
            loop {
                core::hint::spin_loop();
            }
        }
        let next_counter = input.counter.checked_add(1).ok_or(Rejection::Rejected)?;
        #[cfg(feature = "invalid")]
        let next_counter = if input.counter >= 3 {
            input.counter.checked_add(2).ok_or(Rejection::Rejected)?
        } else {
            next_counter
        };
        Ok(Proposal {
            world_instance: input.world_instance,
            instance: input.instance,
            game_time: input.game_time,
            expected_counter: input.counter,
            next_counter,
        })
    }
}

#[cfg(target_arch = "wasm32")]
export!(Fixture);
