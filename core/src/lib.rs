mod app;
mod card;
mod card_game;
mod deck;
mod player;
mod player_turn;
pub mod protocol;
mod room;

pub use app::App;
pub use card::*;
pub use card_game::CardGame;
pub use deck::*;
pub use player::*;
pub use player_turn::PlayerTurn;
pub use room::*;

use std::sync::Arc;
use tokio::sync::Mutex;

pub type AppInstance = Arc<Mutex<App>>;
