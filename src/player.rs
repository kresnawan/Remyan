pub enum PlayerStatus {
    Online,
    Away,
    Offline,
}

pub struct Player {
    pub player_id: u32,
    pub status: PlayerStatus,
}
