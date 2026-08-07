use remyan_core::{RoomConfig, protocol::{command::GameCommand, event::ServerEventPlayer}};

use crate::ui::widgets::switch_button::RoomConfigSwitchId;

#[derive(Clone, Debug)]
pub enum State {
    OpenDialogueBox(u8),
    OpenRoomConfigDialogueBox,
    CloseDialogueBox(u8),
    InputRoomId(String),
    CreateRoom,
    JoinRoom(String),
    RoomPlayers {
        players: Vec<Option<ServerEventPlayer>>,
        host_id: u32,
        self_id: u32,
    },
    LeaveRoom,
    ConfigUpdate(RoomConfig),
    ConfigInput(RoomConfigSwitchId),
    ApplyConfig,
    Reset,
    StartGame,
    InGameCommand(GameCommand),
}
