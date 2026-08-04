use std::sync::Arc;

use axum::extract::ws::Message;
use remyan_core::App;
use tokio::sync::{Mutex, mpsc};

use crate::router::Server;

pub mod handler;
pub mod route;
pub mod ws;
pub mod router;
pub mod server_room;

pub type Tx = mpsc::UnboundedSender<Message>;
pub type ServerInstance = Arc<Mutex<Server>>;
pub type AppInstance = Arc<Mutex<App>>;