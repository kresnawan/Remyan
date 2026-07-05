use client::App;
use macroquad::prelude::*;

fn window_config() -> Conf {
    Conf {
        window_title: "Remyan".to_owned(),
        window_width: 1920,
        window_height: 1080,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    let mut app = App::new();
    app.init().await;
}
