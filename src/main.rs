use remyan::{AppInstance, game::{app::App, card::{Card, CardIcon, CardType, CourtType, SpotNumber}, room::Room}, network};
use std::{collections::HashSet, sync::Arc};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let instance = App::new();
    let pointer: AppInstance = Arc::new(Mutex::new(instance));
    network::router::init(pointer).await;
}
