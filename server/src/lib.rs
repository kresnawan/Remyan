use std::sync::Arc;

use axum::extract::ws::Message;
use tokio::sync::{Mutex, mpsc};

use crate::router::Server;

pub mod handler;
pub mod route;
pub mod ws;
pub mod router;
pub mod server_room;

type Tx = mpsc::UnboundedSender<Message>;
type ServerInstance = Arc<Mutex<Server>>;