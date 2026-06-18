use crate::{
    audio::AudioSystem, config::BoardConfig, input::handle_mouse_input, piece::AnimatingPiece,
};
use shakmaty::{Chess, Move, Position, Square};

/// Manages the core logic, history, and active status of the chess game.
pub struct GameState {
    /// Full history of board positions for undo/redo functionality.
    pub history: Vec<Chess>,
    /// SAN representations of played moves.
    pub move_list: Vec<String>,
    /// The current position in the timeline (used for undo/redo).
    pub current_index: usize,
    /// The square currently selected by the player.
    pub selected: Option<Square>,
    /// Audio manager for sound effects.
    pub audio: AudioSystem,
    /// The piece currently being animated, if any. Blocks input while active.
    pub animating: Option<AnimatingPiece>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            history: vec![Chess::default()],
            move_list: Vec::new(),
            current_index: 0,
            selected: None,
            audio: AudioSystem::default(),
            animating: None,
        }
    }
}

impl GameState {
    /// Returns the active board position based on the current timeline index.
    pub fn current_position(&self) -> &Chess {
        &self.history[self.current_index]
    }

    /// Called every frame to progress animations and handle user inputs.
    pub fn update(&mut self, config: &BoardConfig) {
        // If a piece is moving, progress the animation and block new inputs
        if let Some(anim) = &mut self.animating {
            anim.progress += macroquad::time::get_frame_time() * 8.0;
            if anim.progress >= 1.0 {
                self.animating = None;
            }
            return;
        }

        // Process mouse input if no animation is currently blocking
        if let Some((new_selected, chess_move, san_str)) =
            handle_mouse_input(self.current_position(), self.selected, config, &self.audio)
        {
            self.selected = new_selected;
            if let Some(m) = chess_move {
                self.make_move(m, san_str.unwrap(), config);
            }
        }
    }

    /// Executes a valid move, creating the animation and updating the game history.
    pub fn make_move(&mut self, m: Move, san: String, config: &BoardConfig) {
        let pos = self.current_position();

        // Pass the move by reference to avoid consuming it before calculating the animation
        let new_pos = pos.clone().play(m).unwrap();

        // Setup the visual animation from the starting square to the destination
        if let (Some(from), to) = (m.from(), m.to())
            && let Some(piece) = pos.board().piece_at(from)
        {
            self.animating = Some(AnimatingPiece::new(piece, from, to, config));
        }

        // If a move is made after an undo, truncate the alternate future timeline
        self.history.truncate(self.current_index + 1);
        self.move_list.truncate(self.current_index);

        self.history.push(new_pos);
        self.move_list.push(san);
        self.current_index += 1;
    }

    /// Reverts the board to the previous move state. Blocked during animations.
    pub fn undo(&mut self) {
        if self.current_index > 0 && self.animating.is_none() {
            self.current_index -= 1;
            self.selected = None;
        }
    }

    /// Advances the board forward in the history timeline. Blocked during animations.
    pub fn redo(&mut self) {
        if self.current_index + 1 < self.history.len() && self.animating.is_none() {
            self.current_index += 1;
            self.selected = None;
        }
    }

    /// Initializes and caches the audio files.
    pub async fn load_audio(&mut self) {
        self.audio.load_sounds().await;
    }
}
