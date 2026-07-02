/*
    to achieve result like CommandToken::RoomCommand::SendMessage { msg },
    we write the message as such:
    {
        "RoomCommand": {
            "SendMessage": {
                "message": "Your message"
            }
        }
    }

    CommandToken::RoomCommand::EditConfig { new_config }
    {
        "RoomCommand": {
            "EditConfig": {
                "new_config": {
                    "allow_court_stacking": true,
                    "free_hit": true,
                    "allow_railing": true,
                    "with_joker": true,
                    "hitter_scoring": true,
                    "number_of_jokers": "Two",
                    "joker_type": null
                }
            }
        }
    }
*/
use axum::extract::ws::Utf8Bytes;
use remyan_core::protocol::command::CommandToken;

pub fn parse_command(txt: Utf8Bytes) -> Result<CommandToken, serde_json::Error> {
    serde_json::from_slice::<CommandToken>(txt.as_bytes())
}
