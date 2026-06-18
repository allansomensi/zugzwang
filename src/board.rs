use crate::config::BoardConfig;
use crate::state::GameState;
use macroquad::{
    color::{BROWN, Color, LIGHTGRAY, ORANGE, RED, WHITE},
    shapes::draw_rectangle,
    text::draw_text,
    window::screen_width,
};
use shakmaty::{Chess, Color as ShakColor, Position};

const DARK_SQUARE: Color = BROWN;
const LIGHT_SQUARE: Color = LIGHTGRAY;

/// Draws the 8x8 chessboard grid with alternating colors.
pub fn draw_board(config: &BoardConfig) {
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

/// Highlights the king's square if the current position is in check or checkmate.
pub fn draw_board_highlights(config: &BoardConfig, position: &Chess) {
    if position.is_check()
        && let Some(king_sq) = position.board().king_of(position.turn())
    {
        let x = config.offset.x + king_sq.file() as u8 as f32 * config.square_size;
        let y = config.offset.y + (7 - king_sq.rank() as u8) as f32 * config.square_size;

        let color = if position.is_checkmate() {
            Color::new(1.0, 0.0, 0.0, 0.7)
        } else {
            Color::new(1.0, 0.5, 0.0, 0.7)
        };

        draw_rectangle(x, y, config.square_size, config.square_size, color);
    }
}

/// Renders the sidebar user interface.
pub fn draw_ui(state: &GameState, config: &BoardConfig) {
    let ui_x = screen_width() - config.ui_width + 20.0;
    let pos = state.current_position();

    let turn_text = if pos.turn() == ShakColor::White {
        "White to move"
    } else {
        "Black to move"
    };
    let status_color = if pos.is_checkmate() { RED } else { WHITE };

    draw_text(turn_text, ui_x, 40.0, 30.0, status_color);

    if pos.is_checkmate() {
        draw_text("CHECKMATE!", ui_x, 70.0, 24.0, RED);
    } else if pos.is_check() {
        draw_text("Check!", ui_x, 70.0, 24.0, ORANGE);
    }

    draw_text("Move History:", ui_x, 110.0, 24.0, LIGHTGRAY);
    draw_text(
        "Left/Right arrows to undo/redo",
        ui_x,
        135.0,
        16.0,
        LIGHTGRAY,
    );

    let mut y = 170.0;
    let moves_to_show = state.move_list[..state.current_index].to_vec();

    // Prevent vertical overflow by only showing the last 40 half-moves
    let start_idx = if moves_to_show.len() > 40 {
        moves_to_show.len() - 40
    } else {
        0
    };
    let chunks: Vec<_> = moves_to_show[start_idx..].chunks(2).collect();
    let base_turn = (start_idx / 2) + 1;

    for (i, chunk) in chunks.iter().enumerate() {
        let white_move = &chunk[0];
        let black_move = if chunk.len() > 1 { &chunk[1] } else { "" };
        let text = format!("{}. {white_move}  {black_move}", base_turn + i);
        draw_text(&text, ui_x, y, 20.0, WHITE);
        y += 25.0;
    }
}
