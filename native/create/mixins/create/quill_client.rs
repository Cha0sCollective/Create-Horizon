// Copyright (c) The Create Team / The Creators of Create
// Copyright (c) 2026 create-horizon contributors
// SPDX-License-Identifier: MIT
// Client-only Quill corner selection. No structure save or world mutation.

use super::Game;
use create_engine::quill::{Feedback, Pos, Selection};
use horizon_client::{first_person::Hand, hud::HudBatch};
use std::cell::RefCell;

const ITEM: &str = "create:schematic_and_quill";

#[derive(Default)]
struct Ui {
    selection: Selection,
    feedback: Option<Feedback>,
}

thread_local! {
    static UI: RefCell<Ui> = RefCell::new(Ui::default());
}

fn active(game: &Game<'_>) -> bool {
    game.native_window()
        && game.native_playing()
        && game.native_on("create")
        && game.held().is_some_and(|held| held.item.as_ref() == ITEM)
}

fn selected(game: &Game<'_>) -> Option<Pos> {
    let hit = game.pick()?;
    let block = game.world.block(hit.pos[0], hit.pos[1], hit.pos[2]);
    let replaceable = game.res.facts().get(block).replaceable;
    create_engine::quill::selected_cell(hit.pos, hit.face, replaceable)
}

pub(super) fn interaction(
    game: &mut Game<'_>,
    button: i32,
    hand: Hand,
    swing: &mut bool,
    canceled: &mut bool,
) {
    if *canceled || button != 1 || hand != Hand::Main || !active(game) {
        return;
    }
    let sneaking = game.keys.contains(&winit::keyboard::KeyCode::ShiftLeft);
    let target = if sneaking { None } else { selected(game) };
    let feedback = UI.with(|ui| {
        let mut ui = ui.borrow_mut();
        let feedback = ui.selection.click(target, sneaking);
        ui.feedback = Some(feedback);
        feedback
    });
    *swing = matches!(feedback, Feedback::First(_) | Feedback::Second(_));
    *canceled = true;
}

/// The frame hook also runs while a menu is open. Forget the two corners when
/// the Quill leaves the selected hand or the world is no longer playing.
pub(super) fn frame(game: &Game<'_>) {
    if !active(game) {
        clear();
    }
}

pub(super) fn clear() {
    UI.with(|ui| *ui.borrow_mut() = Ui::default());
}

fn point(pos: Pos) -> String {
    format!("{}, {}, {}", pos[0], pos[1], pos[2])
}

fn line(game: &Game<'_>, batch: &mut HudBatch, row: f32, value: &str, color: u32) {
    game.native_text(batch, value, [12.0, 42.0 + row * 12.0], color, true);
}

pub(super) fn hud(game: &Game<'_>, batch: &mut HudBatch) {
    if !active(game) {
        return;
    }
    UI.with(|ui| {
        let ui = ui.borrow();
        line(game, batch, 0.0, "Schematic and Quill", 0xFFE0A3);
        match (ui.selection.first(), ui.selection.second()) {
            (None, _) => line(
                game,
                batch,
                1.0,
                "Right-click a block: first corner",
                0xFFFFFF,
            ),
            (Some(first), None) => {
                line(
                    game,
                    batch,
                    1.0,
                    &format!("First: {}", point(first)),
                    0xFFFFFF,
                );
                if let Some(preview) = selected(game).and_then(|pos| ui.selection.preview(pos)) {
                    let color = if preview.within_limit() {
                        0xB8E986
                    } else {
                        0xFF7777
                    };
                    line(
                        game,
                        batch,
                        2.0,
                        &format!(
                            "Size: {} x {} x {} (max 16 each)",
                            preview.size[0], preview.size[1], preview.size[2]
                        ),
                        color,
                    );
                } else {
                    line(
                        game,
                        batch,
                        2.0,
                        "Right-click a block: second corner",
                        0xFFFFFF,
                    );
                }
            }
            (Some(first), Some(second)) => {
                let bounds = ui.selection.bounds().expect("complete selection");
                line(
                    game,
                    batch,
                    1.0,
                    &format!("First: {}", point(first)),
                    0xFFFFFF,
                );
                line(
                    game,
                    batch,
                    2.0,
                    &format!("Second: {}", point(second)),
                    0xFFFFFF,
                );
                line(
                    game,
                    batch,
                    3.0,
                    &format!(
                        "Size: {} x {} x {}",
                        bounds.size[0], bounds.size[1], bounds.size[2]
                    ),
                    0xB8E986,
                );
            }
        }
        let message = match ui.feedback {
            Some(Feedback::NoTarget) => Some("No block targeted"),
            Some(Feedback::TooLarge(_)) => Some("Selection exceeds 16 blocks on an axis"),
            Some(Feedback::Ready(_)) => Some("Selection ready; saving is not available yet"),
            Some(Feedback::Canceled) => Some("Selection canceled"),
            _ => None,
        };
        if let Some(message) = message {
            line(game, batch, 5.0, message, 0xFF7777);
        }
        line(game, batch, 6.0, "Shift + right-click: cancel", 0xC4C4C4);
    });
}
