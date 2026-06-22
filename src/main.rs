use remyan::{AppInstance, game::app::App, network};
use std::{sync::Arc};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let instance = App::new();
    let pointer: AppInstance = Arc::new(Mutex::new(instance));
    network::router::init(pointer).await;
}
