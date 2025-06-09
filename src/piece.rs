use crate::config::BoardConfig;
use macroquad::{
    color::WHITE,
    math::Vec2,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture},
};
use shakmaty::{Chess, Color, Piece, Position, Role, Square};
use std::collections::HashMap;

/// Loads all chess piece textures
pub async fn load_piece_textures() -> HashMap<Piece, Texture2D> {
    let mut textures = HashMap::new();

    for role in Role::ALL {
        for (color, prefix) in [(Color::White, "w"), (Color::Black, "b")] {
            let path = format!("assets/pieces/{}{}.png", prefix, role.upper_char());
            let piece = Piece { role, color };
            textures.insert(piece, load_texture(&path).await.unwrap());
        }
    }

    textures
}

/// Draws all pieces on the board
pub fn draw_pieces(position: &Chess, textures: &HashMap<Piece, Texture2D>, config: &BoardConfig) {
    for square in Square::ALL {
        if let Some(piece) = position.board().piece_at(square) {
            if let Some(texture) = textures.get(&piece) {
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
    }
}
