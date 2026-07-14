use client::app::App;
use macroquad::prelude::*;

// a custom getrandom crate backend
// because we're not using wasm-bindgen to make the WASM binding
// so that we need a custom backend to get the game compiled
//
//
#[cfg(target_arch = "wasm32")]
fn get_random(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    for byte in buf.iter_mut() {
        *byte = (macroquad::rand::rand() % 256) as u8;
    }
    Ok(())
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
unsafe extern "Rust" fn __getrandom_v03_custom(
    dest: *mut u8,
    len: usize,
) -> Result<(), getrandom::Error> {
    let buf = unsafe {
        core::ptr::write_bytes(dest, 0, len);
        core::slice::from_raw_parts_mut(dest, len)
    };
    get_random(buf)
}

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
