#[derive(Debug, Clone, Copy)]
pub enum PlayerStatus {
    Online,
    Away,
    Offline,
}

#[derive(Debug)]
pub struct Player {
    pub username: String,
    pub status: PlayerStatus,
}

impl Player {
    pub fn new(username: String) -> Self {
        return Player {
            username: username,
            status: PlayerStatus::Offline,
        };
    }
}
