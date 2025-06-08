use macroquad::window::Conf;

/// Returns the window configuration.
pub fn window_conf() -> Conf {
    Conf {
        window_title: "Zugzwang".into(),
        window_width: 1024,
        window_height: 768,
        ..Default::default()
    }
}
