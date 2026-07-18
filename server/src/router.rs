use std::{collections::HashMap, sync::Arc};

use axum::{Extension, Router, routing::get};
use remyan_core::AppInstance;
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{
    route::{auth, room, ws},
    server_room::ServerRoom,
};

#[derive(Deserialize)]
pub struct UserPayload {
    pub id: String,
}

#[derive(Deserialize)]
pub struct RoomIdQuery {
    pub room_id: String,
}

#[derive(Deserialize)]
pub struct RoomIdAndPlayerIdQuery {
    pub room_id: String,
    pub player_id: String,
}

pub struct Server {
    pub rooms: HashMap<[u8; 6], ServerRoom>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            rooms: HashMap::new(),
        }
    }
    pub async fn init(server: Arc<Mutex<Server>>, core_app: AppInstance) {
        let app: Router = Router::new()
            .route("/ping", get(|| async { "Pong" }))
            .merge(room())
            .merge(auth())
            .merge(ws())
            .layer(Extension(core_app))
            .layer(Extension(server));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:6767")
            .await
            .unwrap();

        println!("Server berjalan di port 6767");

        if let Err(err) = axum::serve(listener, app).await {
            println!("{:?}", err);
        };
    }
}
