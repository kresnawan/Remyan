use crate::page::Pages;

#[derive(Clone)]
pub enum State {
    MovePage(Pages),
    OpenDialogueBox(u8),
    CloseDialogueBox(u8),
    InputRoomId(String),
    CreateRoom,
    JoinRoom(String),
    PlayerJoin(Vec<Option<PlayerJoinStruct>>),
    LeaveRoom
}

#[derive(Clone)]
pub struct PlayerJoinStruct {
    pub id: u32,
    pub name_alias: Option<String>,
    pub is_self: bool,
    pub is_room_host: bool
}