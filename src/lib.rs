
use axum::extract::ws::Message;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

use crate::core::app::App;

pub mod core;
pub mod network;

pub type AppInstance = Arc<Mutex<App>>;

type Tx = mpsc::UnboundedSender<Message>;
