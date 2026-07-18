#[derive(Debug, Clone, Copy)]
pub enum PlayerStatus {
    Online,
    Away,
    Offline,
}

#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub username: Option<String>,
    pub status: PlayerStatus,
}

impl Player {
    pub fn new(player_id: u32) -> Self {
        return Player {
            id: player_id,
            username: None,
            status: PlayerStatus::Online,
        };
    }
}
