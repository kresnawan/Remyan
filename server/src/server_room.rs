use axum::extract::ws::{Message, Utf8Bytes};
use remyan_core::{Room, protocol::event::{EventToken, ServerEvent}};
use std::collections::HashMap;

use crate::Tx;

pub struct ServerRoom {
    pub room_id: [u8; 6],
    pub txs: HashMap<u32, Option<Tx>>,
}

impl ServerRoom {
    pub fn new(room_id: [u8; 6]) -> Self {
        ServerRoom { room_id, txs: HashMap::new() }
    }
    pub async fn broadcast(&mut self, all: bool, sender: u32, token: EventToken) {
        let serialized = serde_json::to_string(&token).unwrap();
        
        let payload = Utf8Bytes::from(serialized);

        let mut txs_iter = self.txs.iter_mut();
        while let Some((pid, tx)) = txs_iter.next() {
            if all || pid != &sender {
                if let Err(err) = tx.as_ref().unwrap().send(Message::Text(payload.clone())) {
                    println!("{}", err);
                    continue;
                }
            }
        }
    }

    pub async fn broadcast_card(&mut self, room: &Room) {
        let mut iter = self.txs.iter_mut();
        while let Some((pid, pd)) = iter.next() {

            let core_player = room.players.get(&pid).unwrap();

            let token = EventToken::ServerEvent(ServerEvent::PlayerCard(core_player.hand_cards.clone()));
            let serialized = serde_json::to_string(&token).unwrap();
            let payload = Utf8Bytes::from(serialized);

            if let Err(err) = pd.as_ref().unwrap().send(Message::Text(payload.clone())) {
                println!("{}", err);
                continue;
            }
        }
    }

    pub async fn send_player(
        &self,
        token: EventToken,
        player_id: u32,
    ) {
        let tx = self.txs.get(&player_id).unwrap();
        let serialized = serde_json::to_string(&token).unwrap();
        let payload = Utf8Bytes::from(serialized);

        if let Some(ptx) = tx.as_ref() {
            if let Err(err) = ptx.send(Message::Text(payload.clone())) {
                println!("{}", err);
            }
        }
    }
}
