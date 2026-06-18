use crate::config::BoardConfig;
use macroquad::{
    color::WHITE,
    math::Vec2,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture},
};
use shakmaty::{Chess, Color, Piece, Position, Role, Square};
use std::collections::HashMap;

/// Represents a chess piece currently in motion across the board.
pub struct AnimatingPiece {
    pub piece: Piece,
    pub to: Square,
    pub start_pos: Vec2,
    pub end_pos: Vec2,
    /// Animation progress from 0.0 (start) to 1.0 (finish).
    pub progress: f32,
}

impl AnimatingPiece {
    /// Initializes a new piece animation, calculating the exact screen coordinates
    /// for the starting and ending squares.
    pub fn new(piece: Piece, from: Square, to: Square, config: &BoardConfig) -> Self {
        let start_x = config.offset.x + from.file() as u8 as f32 * config.square_size;
        let start_y = config.offset.y + (7 - from.rank() as u8) as f32 * config.square_size;
        let end_x = config.offset.x + to.file() as u8 as f32 * config.square_size;
        let end_y = config.offset.y + (7 - to.rank() as u8) as f32 * config.square_size;

        Self {
            piece,
            to,
            start_pos: Vec2::new(start_x, start_y),
            end_pos: Vec2::new(end_x, end_y),
            progress: 0.0,
        }
    }

    /// Draws the moving piece at its current interpolated position.
    pub fn draw(&self, textures: &HashMap<Piece, Texture2D>, config: &BoardConfig) {
        if let Some(texture) = textures.get(&self.piece) {
            // Calculate the current position using linear interpolation
            let current_pos = self.start_pos.lerp(self.end_pos, self.progress);

            draw_texture_ex(
                texture,
                current_pos.x,
                current_pos.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(config.square_size, config.square_size)),
                    ..Default::default()
                },
            );
        }
    }
}

/// Loads all 12 chess piece textures.
pub async fn load_piece_textures() -> HashMap<Piece, Texture2D> {
    let mut textures = HashMap::new();
    for role in Role::ALL {
        for (color, prefix) in [(Color::White, "w"), (Color::Black, "b")] {
            let path = format!("assets/pieces/{prefix}{}.png", role.upper_char());
            let piece = Piece { role, color };
            textures.insert(piece, load_texture(&path).await.unwrap());
        }
    }
    textures
}

/// Draws all static pieces on the board and layers the animating piece on top if one exists.
pub fn draw_pieces(
    position: &Chess,
    textures: &HashMap<Piece, Texture2D>,
    config: &BoardConfig,
    animating: Option<&AnimatingPiece>,
) {
    for square in Square::ALL {
        // If there is an active animation, do not draw the static piece at the destination square
        // to prevent duplicate pieces appearing on the board.
        if let Some(anim) = animating
            && square == anim.to
        {
            continue;
        }

        if let Some(piece) = position.board().piece_at(square)
            && let Some(texture) = textures.get(&piece)
        {
            let x = config.offset.x + square.file() as u8 as f32 * config.square_size;
            let y = config.offset.y + (7 - square.rank() as u8) as f32 * config.square_size;

            draw_texture_ex(
                texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(config.square_size, config.square_size)),
                    ..Default::default()
                },
            );
        }
    }

    // Draw the moving piece last so it renders on top of all other pieces
    if let Some(anim) = animating {
        anim.draw(textures, config);
    }
}
