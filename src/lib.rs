use crate::game::app::App;
use axum::extract::ws::Message;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

pub mod game;
pub mod network;

pub type AppInstance = Arc<Mutex<App>>;

type Tx = mpsc::UnboundedSender<Message>;
