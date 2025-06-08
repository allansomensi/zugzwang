mod board;
mod config;
mod pieces;

use crate::config::window_conf;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    let config = config::BoardConfig::new();

    let textures = pieces::load_piece_textures().await;
    let position = shakmaty::Chess::default();

    loop {
        clear_background(DARKGRAY);

        board::draw_board(&config);
        pieces::draw_pieces(&position, &textures, &config);

        next_frame().await;
    }
}
