use crate::game::{app::App, card::Card, room_config::RoomConfig};
use axum::extract::ws::Message;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

pub mod game;
pub mod network;

pub type AppInstance = Arc<Mutex<App>>;

type Tx = mpsc::UnboundedSender<Message>;


