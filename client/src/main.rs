#[cfg(target_arch = "wasm32")]
use std::panic;

#[cfg(target_arch = "wasm32")]
use client::console_log;

use client::app::App;
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
    #[cfg(target_arch = "wasm32")]
    console_log("Hello from Rust! Small update");

    #[cfg(target_arch = "wasm32")]
    panic::set_hook(Box::new(|panic_info| {
        console_log("🚨 Custom Panic Listener Triggered!");

        if let Some(location) = panic_info.location() {
            console_log(&format!(
                "Panic occurred in file '{}' at line {}",
                location.file(),
                location.line()
            ));
        }

        if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
            console_log(&format!("Panic message: {}", message));
        } else if let Some(message) = panic_info.payload().downcast_ref::<String>() {
            console_log(&format!("Panic message: {}", message));
        } else {
            console_log(&format!("Panic message is unknown."));
        }
    }));

    let mut app = App::new();
    app.init().await;
}
