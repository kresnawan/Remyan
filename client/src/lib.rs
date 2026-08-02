pub mod page;
pub mod ui;
pub mod state;
pub mod app;
pub mod wrapper;

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    unsafe fn js_console_log(ptr: *const u8, len: usize);
}

#[cfg(target_arch = "wasm32")]
pub fn console_log(text: &str) {
    let ptr = text.as_ptr();
    let len = text.len();

    unsafe {
        js_console_log(ptr, len);
    }
}