use remyan_core::RoomConfig;

use crate::ui::widgets::switch_button::{RoomConfigSwitchId, SwitchButtonId};

#[derive(Clone, Debug)]
pub enum State {
    OpenDialogueBox(u8),
    CloseDialogueBox(u8),
    InputRoomId(String),
    CreateRoom,
    JoinRoom(String),
    RoomPlayers{players: Vec<Option<PlayerJoinStruct>>, is_host: bool},
    LeaveRoom,
    ConfigUpdate(RoomConfig),
    ConfigInput(RoomConfigSwitchId),
    ApplyConfig,
    Reset
}

#[derive(Clone, Debug)]
pub struct PlayerJoinStruct {
    pub id: u32,
    pub name_alias: Option<String>,
    pub is_self: bool,
    pub is_room_host: bool
}