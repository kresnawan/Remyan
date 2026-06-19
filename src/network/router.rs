use axum::{Extension, Router};
use serde::Deserialize;

use crate::{
    AppInstance,
    network::route::{auth, room, ws},
};

#[derive(Deserialize)]
pub struct UserPayload {
    pub id: String,
}

#[derive(Deserialize)]
pub struct RoomIdQuery {
    pub room_id: String,
}

pub async fn init(game_app: AppInstance) {
    let app: Router = Router::new()
        .merge(room())
        .merge(auth())
        .merge(ws())
        .layer(Extension(game_app));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:6767")
        .await
        .unwrap();

    println!("Server berjalan di port 6767");

    if let Err(err) = axum::serve(listener, app).await {
        println!("{:?}", err);
    };
}
