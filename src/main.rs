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
    let mut config = config::BoardConfig::new();
    let mut state = state::GameState::default();
    let textures = piece::load_piece_textures().await;
    state.load_audio().await;

    loop {
        clear_background(DARKGRAY);

        config.update();
        state.update(&config);

        board::draw_board(&config);
        piece::draw_pieces(&state.position, &textures, &config);
        input::draw_selection(&state, &config);

        next_frame().await;
    }
}
