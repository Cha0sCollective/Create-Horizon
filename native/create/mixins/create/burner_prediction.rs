// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
// Exact-pinned adapter inside game/remote.rs. Recognized burner/Basin bucket actions
// own this turn; packets use the same methods/fields as the vanilla turn.
use super::{Game, Hand, S, face_direction, hand_id, play};

pub(in crate::game) fn block(game: &mut Game<'_>, hand: Hand) -> bool {
    if game.spectator() {
        return false;
    }
    let Some(hit) = game.pick() else { return false };
    let fuel = game.with_hand(hand, |g| {
        super::super::create_port_burner::reacts(g, hit.pos)
    });
    let basin_bucket = game.with_hand(hand, |g| {
        super::super::create_port_basin::bucket_reacts(g, hit.pos)
    });
    let tank_bucket = game.with_hand(hand, |g| {
        super::super::create_port_fluid_tank::bucket_reacts(g, hit.pos)
    });
    if !fuel && !basin_bucket && !tank_bucket {
        return false;
    }
    let sequence = game.next_sequence();
    let cursor = std::array::from_fn(|i| (hit.point[i] - f64::from(hit.pos[i])) as f32);
    game.send(S::UseItemOn(play::UseItemOn {
        hand: hand_id(hand),
        pos: hit.pos,
        face: i32::from(face_direction(hit.face)),
        cursor,
        inside: hit.face == [0; 3],
        world_border: false,
        sequence,
    }));
    if let Some(remote) = game.remote.as_mut() {
        remote.use_target = Some(hit.pos);
    }
    let predicted = game.with_hand(hand, |g| {
        if fuel {
            super::super::create_port_burner::predict_client(g, hit.pos)
        } else {
            // Basin inventory/fluid changes wait for the authoritative server.
            true
        }
    });
    debug_assert!(predicted);
    game.with_hand(hand, Game::swing_arm);
    true
}

impl Game<'_> {
    // client_level_unload also fires during a dimension transition, after the
    // same connection has installed its new level. Server fuel data remains
    // valid there; disconnect/reconfiguration removes heard_level first.
    pub(in crate::game) fn create_burner_sync_has_level(&self) -> bool {
        self.remote
            .as_ref()
            .is_some_and(|remote| remote.heard_level.is_some())
    }

    pub(in crate::game) fn create_burner_predict_block(&mut self, hand: Hand) -> bool {
        block(self, hand)
    }
}
