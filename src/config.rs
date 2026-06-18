use macroquad::{
    math::Vec2,
    window::{Conf, screen_height, screen_width},
};

/// Board layout configuration and dynamic sizing.
pub struct BoardConfig {
    /// Size of a single square on the chessboard.
    pub square_size: f32,
    /// X and Y offset to center the board in the available screen space.
    pub offset: Vec2,
    /// Fixed width reserved for the sidebar UI on the right.
    pub ui_width: f32,
}

impl BoardConfig {
    pub fn new() -> Self {
        Self::from_screen()
    }

    /// Updates the configuration based on the current window dimensions.
    /// Should be called every frame to handle window resizing smoothly.
    pub fn update(&mut self) {
        *self = Self::from_screen();
    }

    /// Calculates board dimensions and offsets based on the active screen size.
    fn from_screen() -> Self {
        // Reserve 300 pixels on the right side for the move history UI
        let ui_width = 300.0;
        let available_width = (screen_width() - ui_width).max(0.0);

        // The board takes up to 90% of the available vertical or horizontal space
        let size = screen_height().min(available_width) * 0.9;
        let square_size = size / 8.0;
        let board_size = square_size * 8.0;

        Self {
            square_size,
            offset: Vec2::new(
                (available_width - board_size) * 0.5,
                (screen_height() - board_size) * 0.5,
            ),
            ui_width,
        }
    }
}

impl Default for BoardConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns the window configuration used at application startup.
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Zugzwang".into(),
        window_width: 1024,
        window_height: 768,
        ..Default::default()
    }
}
