use macroquad::{
    math::Vec2,
    window::{Conf, screen_height, screen_width},
};

/// Board layout configuration
pub struct BoardConfig {
    pub square_size: f32,
    pub offset: Vec2,
}

impl BoardConfig {
    pub fn new() -> Self {
        Self::from_screen()
    }

    pub fn update(&mut self) {
        *self = Self::from_screen();
    }

    fn from_screen() -> Self {
        let size = screen_height().min(screen_width()) * 0.9;
        let square_size = size / 8.0;
        let board_size = square_size * 8.0;

        Self {
            square_size,
            offset: Vec2::new(
                (screen_width() - board_size) * 0.5,
                (screen_height() - board_size) * 0.5,
            ),
        }
    }
}

impl Default for BoardConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns the window configuration.
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Zugzwang".into(),
        window_width: 1024,
        window_height: 768,
        ..Default::default()
    }
}
