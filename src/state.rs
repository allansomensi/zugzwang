use crate::{audio::AudioSystem, config::BoardConfig, input::handle_mouse_input};
use shakmaty::{Chess, Square};

#[derive(Default)]
pub struct GameState {
    pub position: Chess,
    pub selected: Option<Square>,
    pub audio: AudioSystem,
}

impl GameState {
    pub fn update(&mut self, config: &BoardConfig) {
        let (new_selected, new_position) =
            handle_mouse_input(&self.position, self.selected, config, &self.audio);

        self.selected = new_selected;
        if let Some(pos) = new_position {
            self.position = pos;
        }
    }

    pub async fn load_audio(&mut self) {
        self.audio.load_sounds().await;
    }
}
