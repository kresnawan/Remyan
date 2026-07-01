use axum::extract::ws::{Message, Utf8Bytes};

use crate::core::{protocol::{EventToken, ServerEvent}, room::Room};

pub async fn broadcast(
    room: &Room,
    token: EventToken,
    player_id: u32,
    all: bool,
) -> Result<(), String> {
    let serialized = serde_json::to_string(&token).unwrap();
    let payload = Utf8Bytes::from(serialized);

    for (&pid, pd) in room.players.iter() {
        if all || pid != player_id {
            let ptx = match pd.tx.as_ref() {
                Some(tx) => tx,
                None => {
                    println!("broadcast: terjadi error pada tx.as_ref()");
                    continue;
                }
            };

            if let Err(_) = ptx.send(Message::Text(payload.clone())) {
                println!("broadcast: terjadi error saat mengirim ke pesan websocket ke client");
                continue;
            }
        }
    }

    Ok(())
}

pub async fn broadcast_card(room: &Room) -> Result<(), String> {
    for (_, pd) in room.players.iter() {
        let ptx = match pd.tx.as_ref() {
            Some(tx) => tx,
            None => {
                println!("broadcast_card: terjadi error pada tx.as_ref()");
                continue;
            }
        };

        let token = EventToken::ServerEvent(ServerEvent::PlayerCard(pd.hand_cards.clone()));
        let serialized = serde_json::to_string(&token).unwrap();
        let payload = Utf8Bytes::from(serialized);

        if let Err(_) = ptx.send(Message::Text(payload.clone())) {
            println!("broadcast_card: terjadi error saat mengirim ke pesan websocket ke client");
            continue;
        }
    }

    Ok(())
}

pub async fn ws_send_player(room: &Room, token: EventToken, player_id: u32) -> Result<(), String> {
    let player = room.players.get(&player_id).unwrap();
    let serialized = serde_json::to_string(&token).unwrap();
    let payload = Utf8Bytes::from(serialized);

    if let Some(ptx) = player.tx.as_ref() {
        match ptx.send(Message::Text(payload.clone())) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e.to_string()),
        }
    }

    Err(String::from(
        "ws_send_player: terjadi error pada tx.as_ref()",
    ))
}
