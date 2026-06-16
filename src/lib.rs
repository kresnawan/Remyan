use std::{collections::HashMap, sync::Arc};

use axum::extract::ws::Message;
use tokio::sync::{Mutex, RwLock, mpsc};

use crate::game::app::App;

pub mod game;
pub mod network;

pub type AppInstance = Arc<Mutex<App>>;

type Tx = mpsc::UnboundedSender<Message>;
type Connections = Arc<RwLock<HashMap<u32, Tx>>>;
