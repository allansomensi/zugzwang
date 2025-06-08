mod board;
mod config;

use crate::board::draw_board;
use crate::config::window_conf;
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(DARKGRAY);

        draw_board();

        next_frame().await;
    }
}
