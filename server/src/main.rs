use std::sync::Arc;

use remyan_core::{App, AppInstance};
use server::router::Server;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let core_instance = App::new();
    let core_arc: AppInstance = Arc::new(Mutex::new(core_instance));

    let server_instance = Server::new();
    let server_arc = Arc::new(Mutex::new(server_instance));
    
    Server::init(server_arc, core_arc).await;
}
