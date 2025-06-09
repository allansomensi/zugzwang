use macroquad::audio::{Sound, load_sound, play_sound_once};
use shakmaty::{Chess, Move, Position};

/// Audio system for playing sound effects.
pub struct AudioSystem {
    move_sound: Option<Sound>,
    capture_sound: Option<Sound>,
    castling_sound: Option<Sound>,
    check_sound: Option<Sound>,
    checkmate_sound: Option<Sound>,
}

impl AudioSystem {
    pub fn new() -> Self {
        Self {
            move_sound: None,
            capture_sound: None,
            castling_sound: None,
            check_sound: None,
            checkmate_sound: None,
        }
    }

    /// Loads all chess sound effects.
    pub async fn load_sounds(&mut self) {
        self.move_sound = load_sound("assets/audio/move-self.ogg").await.ok();
        self.capture_sound = load_sound("assets/audio/capture.ogg").await.ok();
        self.castling_sound = load_sound("assets/audio/castle.ogg").await.ok();
        self.check_sound = load_sound("assets/audio/move-check.ogg").await.ok();
        self.checkmate_sound = load_sound("assets/audio/game-end.ogg").await.ok();
    }

    pub fn play_move_sound(&self, chess_move: &Move, position: &Chess) {
        // Create the position after the move to check for check/checkmate
        let new_position = position.clone();
        if let Ok(new_pos) = new_position.play(chess_move) {
            // Check for checkmate first (highest priority)
            if new_pos.is_checkmate() {
                if let Some(sound) = &self.checkmate_sound {
                    play_sound_once(sound);
                    return;
                }
            }

            // Check for check
            if new_pos.is_check() {
                if let Some(sound) = &self.check_sound {
                    play_sound_once(sound);
                    return;
                }
            }
        }

        // Check for castling
        if chess_move.is_castle() {
            if let Some(sound) = &self.castling_sound {
                play_sound_once(sound);
                return;
            }
        }

        // Check if it's a capture
        if position.board().piece_at(chess_move.to()).is_some() || chess_move.is_en_passant() {
            if let Some(sound) = &self.capture_sound {
                play_sound_once(sound);
            }
        } else {
            // Regular move
            if let Some(sound) = &self.move_sound {
                play_sound_once(sound);
            }
        }
    }
}

impl Default for AudioSystem {
    fn default() -> Self {
        Self::new()
    }
}
