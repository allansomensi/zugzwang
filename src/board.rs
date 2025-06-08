use crate::config::BoardConfig;
use macroquad::{
    color::{BROWN, Color, LIGHTGRAY},
    shapes::draw_rectangle,
};

const DARK_SQUARE: Color = BROWN;
const LIGHT_SQUARE: Color = LIGHTGRAY;

/// Draws the chessboard.
pub fn draw_board(config: &BoardConfig) {
    // Draw 8x8 grid of alternating light and dark squares
    for rank in 0..8 {
        for file in 0..8 {
            let is_light = (file + rank) % 2 == 0;
            let color = if is_light { LIGHT_SQUARE } else { DARK_SQUARE };
            draw_rectangle(
                config.offset.x + file as f32 * config.square_size,
                config.offset.y + rank as f32 * config.square_size,
                config.square_size,
                config.square_size,
                color,
            );
        }
    }
}
