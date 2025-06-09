use crate::{config::BoardConfig, state::GameState};
use macroquad::{
    color::YELLOW,
    input::{MouseButton, is_mouse_button_pressed, mouse_position},
    shapes::draw_rectangle_lines,
};
use shakmaty::{Chess, File, Position, Rank, Square};

/// Converts mouse coordinates to chess square
pub fn mouse_square(config: &BoardConfig) -> Option<Square> {
    let (mx, my) = mouse_position();
    let x = mx - config.offset.x;
    let y = my - config.offset.y;

    if x < 0.0 || y < 0.0 || x >= config.square_size * 8.0 || y >= config.square_size * 8.0 {
        return None;
    }

    let file = File::ALL.get((x / config.square_size) as usize)?;
    let rank = Rank::ALL.get(7 - (y / config.square_size) as usize)?;
    Some(Square::from_coords(*file, *rank))
}

/// Handles mouse input - returns (new_selected, new_position)
pub fn handle_mouse_input(
    position: &Chess,
    selected: Option<Square>,
    config: &BoardConfig,
) -> (Option<Square>, Option<Chess>) {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return (selected, None);
    }

    let Some(square) = mouse_square(config) else {
        return (selected, None);
    };

    match selected {
        Some(from) => (None, try_move(position, from, square)),
        None => (try_select(position, square), None),
    }
}

/// Try to make a move
fn try_move(position: &Chess, from: Square, to: Square) -> Option<Chess> {
    position
        .legal_moves()
        .iter()
        .find(|mv| mv.from() == Some(from) && mv.to() == to)
        .map(|mv| position.clone().play(mv).unwrap())
}

/// Try to select a piece
fn try_select(position: &Chess, square: Square) -> Option<Square> {
    position
        .board()
        .piece_at(square)
        .filter(|piece| piece.color == position.turn())
        .map(|_| square)
}

/// Draw selection highlight
pub fn draw_selection(state: &GameState, config: &BoardConfig) {
    if let Some(square) = state.selected {
        let x = config.offset.x + square.file() as u8 as f32 * config.square_size;
        let y = config.offset.y + (7 - square.rank() as u8) as f32 * config.square_size;
        draw_rectangle_lines(x, y, config.square_size, config.square_size, 5.0, YELLOW);
    }
}
