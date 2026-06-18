#![windows_subsystem = "windows"]

mod audio;
mod board;
mod config;
mod input;
mod piece;
mod state;

use crate::config::window_conf;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    // Initialize game configuration and state
    let mut config = config::BoardConfig::new();
    let mut state = state::GameState::default();

    // Load graphical and audio assets
    let textures = piece::load_piece_textures().await;
    state.load_audio().await;

    loop {
        clear_background(DARKGRAY);

        // Update screen dimensions and handle mouse logic/animations
        config.update();
        state.update(&config);

        // Handle keyboard shortcuts for move history
        if is_key_pressed(KeyCode::Left) {
            state.undo();
        }
        if is_key_pressed(KeyCode::Right) {
            state.redo();
        }

        // Render all visual elements
        board::draw_board(&config);
        board::draw_board_highlights(&config, state.current_position());
        piece::draw_pieces(
            state.current_position(),
            &textures,
            &config,
            state.animating.as_ref(),
        );
        input::draw_selection(state.selected, &config);
        board::draw_ui(&state, &config);

        next_frame().await;
    }
}
