use macroquad::{
    color::{BROWN, Color, LIGHTGRAY},
    math::Vec2,
    shapes::draw_rectangle,
    window::{screen_height, screen_width},
};

const DARK_SQUARE: Color = BROWN;
const LIGHT_SQUARE: Color = LIGHTGRAY;

/// Draws the chessboard.
pub fn draw_board() {
    // Calculate the size of each square based on screen dimensions
    let square_size = (screen_height().min(screen_width()) - 80.0) / 8.0;

    // Calculate the offset to center the board
    let offset = Vec2::new(
        (screen_width() - square_size * 8.0) / 2.0,
        (screen_height() - square_size * 8.0) / 2.0,
    );

    // Draw 8x8 grid of alternating light and dark squares
    for rank in 0..8 {
        for file in 0..8 {
            let is_light = (file + rank) % 2 == 0;
            let color = if is_light { LIGHT_SQUARE } else { DARK_SQUARE };
            draw_rectangle(
                offset.x + file as f32 * square_size,
                offset.y + rank as f32 * square_size,
                square_size,
                square_size,
                color,
            );
        }
    }
}
