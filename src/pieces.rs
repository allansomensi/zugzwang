use crate::config::BoardConfig;
use macroquad::{
    color::Color,
    math::Vec2,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex, load_texture},
};
use shakmaty::{Chess, Color as ShakmatyColor, Piece, Position, Role, Square};
use std::collections::HashMap;

/// Loads textures for all chess pieces asynchronously.
/// Returns a [`HashMap`] keyed by ([`Piece`], [`Color`]) with the corresponding [`Texture2D`].
pub async fn load_piece_textures() -> HashMap<(Piece, ShakmatyColor), Texture2D> {
    let mut textures = HashMap::new();

    for role in Role::ALL {
        for (color, prefix) in [(ShakmatyColor::White, "w"), (ShakmatyColor::Black, "b")] {
            let path = format!("assets/pieces/{}{}.png", prefix, role.upper_char());
            let piece = Piece { role, color };
            textures.insert((piece, color), load_texture(&path).await.unwrap());
        }
    }

    textures
}

/// Draws all pieces on the board according to the current position and configuration.
/// Uses loaded textures to render each piece in its square with proper scaling and centering.
pub fn draw_pieces(
    position: &Chess,
    textures: &HashMap<(Piece, ShakmatyColor), Texture2D>,
    config: &BoardConfig,
) {
    for square in Square::ALL {
        if let Some(piece) = position.board().piece_at(square) {
            if let Some(texture) = textures.get(&(piece, piece.color)) {
                let texture_size = config.square_size * 0.9;
                let file = square.file() as u8 as f32;
                let rank = square.rank() as u8 as f32;
                let display_rank = 7.0 - rank;

                let x = config.offset.x
                    + file * config.square_size
                    + (config.square_size - texture_size) / 2.0;
                let y = config.offset.y
                    + display_rank * config.square_size
                    + (config.square_size - texture_size) / 2.0;

                draw_texture_ex(
                    texture,
                    x,
                    y,
                    Color::new(1.0, 1.0, 1.0, 1.0),
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(texture_size, texture_size)),
                        ..Default::default()
                    },
                );
            }
        }
    }
}
