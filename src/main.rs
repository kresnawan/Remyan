use std::sync::{Arc, Mutex};
use remyan::{AppInstance, game::app::App, network};

#[tokio::main]
async fn main() {
    let instance = App::new();
    let pointer: AppInstance = Arc::new(Mutex::new(instance));
    network::network::init(pointer).await;
}