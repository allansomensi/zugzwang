use crate::{audio::AudioSystem, config::BoardConfig};
use macroquad::{
    color::YELLOW,
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    shapes::draw_rectangle_lines,
};
use shakmaty::{Chess, File, Move, Position, Rank, Square, san::San};

/// Converts the current mouse screen coordinates into a chess board square.
pub fn mouse_square(config: &BoardConfig) -> Option<Square> {
    let (mx, my) = mouse_position();
    let x = mx - config.offset.x;
    let y = my - config.offset.y;

    // Ensure the click is within the playable board area
    if x < 0.0 || y < 0.0 || x >= config.square_size * 8.0 || y >= config.square_size * 8.0 {
        return None;
    }

    let file = File::ALL.get((x / config.square_size) as usize)?;
    let rank = Rank::ALL.get(7 - (y / config.square_size) as usize)?;
    Some(Square::from_coords(*file, *rank))
}

/// Handles mouse clicks to select pieces and execute moves.
pub fn handle_mouse_input(
    position: &Chess,
    selected: Option<Square>,
    config: &BoardConfig,
    audio: &AudioSystem,
) -> Option<(Option<Square>, Option<Move>, Option<String>)> {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    }

    let Some(square) = mouse_square(config) else {
        return Some((selected, None, None));
    };

    match selected {
        Some(from) => {
            if let Some(chess_move) = try_move(position, from, square) {
                let san = San::from_move(position, chess_move).to_string();
                audio.play_move_sound(chess_move, position);
                return Some((None, Some(chess_move), Some(san)));
            }
            // If the player clicked an invalid move square, try to select another piece instead
            Some((try_select(position, square), None, None))
        }
        None => Some((try_select(position, square), None, None)),
    }
}

/// Checks if a move from the starting square to the target square is legal.
fn try_move(position: &Chess, from: Square, to: Square) -> Option<Move> {
    position
        .legal_moves()
        .into_iter()
        .find(|&mv| mv.from() == Some(from) && mv.to() == to)
}

/// Attempts to select a piece at the given square.
fn try_select(position: &Chess, square: Square) -> Option<Square> {
    position
        .board()
        .piece_at(square)
        .filter(|piece| piece.color == position.turn())
        .map(|_| square)
}

/// Draws a visual highlight around the currently selected square.
pub fn draw_selection(selected: Option<Square>, config: &BoardConfig) {
    if let Some(square) = selected {
        let x = config.offset.x + square.file() as u8 as f32 * config.square_size;
        let y = config.offset.y + (7 - square.rank() as u8) as f32 * config.square_size;
        draw_rectangle_lines(x, y, config.square_size, config.square_size, 5.0, YELLOW);
    }
}
