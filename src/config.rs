use macroquad::{
    math::Vec2,
    window::{Conf, screen_height, screen_width},
};

/// Board layout configuration.
pub struct BoardConfig {
    pub square_size: f32,
    pub offset: Vec2,
}

impl BoardConfig {
    pub fn new() -> Self {
        let square_size = (screen_height().min(screen_width()) - 80.0) / 8.0;
        let offset = Vec2::new(
            (screen_width() - square_size * 8.0) / 2.0,
            (screen_height() - square_size * 8.0) / 2.0,
        );

        Self {
            square_size,
            offset,
        }
    }
}

/// Returns the window configuration.
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Zugzwang".into(),
        window_width: 1024,
        window_height: 768,
        window_resizable: false,
        ..Default::default()
    }
}
