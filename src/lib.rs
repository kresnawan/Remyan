use std::sync::{Arc, Mutex};

use crate::game::app::App;

pub mod game;
pub mod network;

pub type AppInstance = Arc<Mutex<App>>;