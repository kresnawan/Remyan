use std::sync::{
    Arc,
    mpsc::{Receiver, Sender, channel},
};

use macroquad::{
    color::{BLACK, BLANK, Color, WHITE},
    experimental::coroutines::start_coroutine,
    window::{next_frame, screen_height, screen_width},
};
use quad_net::{
    http_request::{Method, Request, RequestBuilder},
    web_socket::WebSocket,
};
use remyan_core::{
    Player,
    protocol::{
        command::{CommandToken, RoomCommand},
        event::{EventToken, ServerEvent},
    },
};

use crate::{
    state::PlayerJoinStruct,
    ui::{
        config::dimension::DynamicDimension,
        widgets::{
            button::Button,
            container::Direction,
            switch_button::SwitchButton,
            text::{HEADING_5, TextConfig},
        },
    },
};

use crate::{
    page::{Page, Pages},
    state::State,
    ui::{
        config::{
            dimension::ObjectDimension,
            font::Nunito,
            gradient::Gradient,
            parent::ParentState,
            position::{DynamicPosition, ObjectPosition},
        },
        traits::object::Object,
        widgets::{
            button::regular_button::RegularButton,
            container::Container,
            dialogue_box::DialogueBox,
            player_slot::PlayerSlot,
            rectangle::{Rectangle, RectangleConfig},
            text::{HEADING_2, Text},
        },
    },
    wrapper::draw::draw_rectangle_extended,
};

pub struct Room {
    players: Vec<Player>,
    room_id: String,
    objects: Vec<Box<dyn Object + Send>>,
    ws: Option<WebSocket>,
    player_id: u32
}

impl Room {
    pub fn new(font: Arc<Nunito>, ws: WebSocket, room_id: String, player_id: u32) -> Self {
        let wrapper = load_room_objects(font.clone());
        let dialogue = load_config_dialogue(font.clone());
        Self {
            players: Vec::new(),
            objects: vec![wrapper, dialogue],
            ws: Some(ws),
            room_id,
            player_id
        }
    }
}

impl Page for Room {
    fn update(&mut self, state: &Option<State>) -> Option<State> {
        for i in &mut self.objects {
            if let Some(n) = i.update(None, None, None, None, state) {
                if let State::LeaveRoom = n {
                    if let Some(ws) = &self.ws {
                        let msg = serde_json::to_string(&CommandToken::RoomCommand(
                            RoomCommand::LeaveRoom,
                        ))
                        .unwrap();
                        ws.send_text(&msg);
                    }
                }

                return Some(n);
            }
        }

        if let Some(ws) = &mut self.ws {
            if let Some(value) = ws.try_recv() {
                let deserialized =
                    serde_json::from_str::<EventToken>(str::from_utf8(&value).unwrap());

                if let Ok(token) = deserialized {
                    match token {
                        EventToken::ServerEvent(e) => match e {
                            ServerEvent::RoomPlayer { players, host_id } => {
                                let mut arr: Vec<Option<PlayerJoinStruct>> = Vec::new();
                                for i in 0..4 {
                                    if let Some(value) = players.get(i) {
                                        arr.push(Some(PlayerJoinStruct {
                                            id: *value,
                                            name_alias: None,
                                            is_self: *value == self.player_id,
                                            is_room_host: *value == host_id,
                                        }));
                                    } else {
                                        arr.push(None);
                                    }
                                }

                                return Some(State::PlayerJoin(arr));
                            }

                            _ => {}
                        },

                        EventToken::RoomEvent(e) => {}
                        EventToken::GameEvent(e) => {}
                    }
                }
            }
        }

        return None;
    }
    fn draw(&self) {
        draw_rectangle_extended(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            0.0,
            Color::from_hex(0x7d0202),
            Color::from_hex(0x2b0000),
            30.0,
            0.0,
            BLACK,
        );
        for i in &self.objects {
            i.draw();
        }
    }
}

fn load_room_objects(font: Arc<Nunito>) -> Box<dyn Object + Send> {
    let mut quit_room_dialog = DialogueBox::new(
        ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Center),
        ObjectDimension::absolute(800.0, 400.0),
        RectangleConfig::new(
            5.0,
            Gradient::new(0.0, vec![Color::from_hex(0x2e2e2e)]),
            2.0,
            Color::from_hex(0x5e5e5e),
        ),
        1,
    );

    let mut room_code = Text::new("Kode Room: 2919391379919339557", font.clone());

    let mut wrapper_3_top_top = Container::new(
        ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Start),
        ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Percent(10.0)),
        ParentState::new(),
        None,
    );

    let mut wrapper_3_top_bottom_marginer = Container::new(
        ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Center),
        ObjectDimension::dynamic(
            DynamicDimension::Percent(70.0),
            DynamicDimension::Percent(90.0),
        ),
        ParentState::new(),
        None,
    );

    let mut wrapper_3_top_bottom = Container::new(
        ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::End),
        ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Percent(90.0)),
        ParentState::new(),
        None,
    );

    let mut wrapper_3_top = Container::new(
        ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Start),
        ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Percent(80.0)),
        ParentState::new(),
        None,
    );

    let mut wrapper_3_bottom = Container::new(
        ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::End),
        ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Percent(15.0)),
        ParentState::new(),
        None,
    );

    let mut wrapper_2 = Container::new(
        ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Center),
        ObjectDimension::dynamic(
            DynamicDimension::Custom(Arc::new(|_, _, p_width, _| p_width - 70.5 * 2.0)),
            DynamicDimension::Custom(Arc::new(|_, _, _, p_height| p_height - 70.5 * 2.0)),
        ),
        ParentState::new(),
        None,
    );

    let mut wrapper = Container::new(
        ObjectPosition::absolute(0.0, 0.0),
        ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Full),
        ParentState::new(),
        None,
    );

    let start_game_btn = RegularButton::new(
        ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Start),
        None,
        "Mulai Game",
        TextConfig::default(font.clone()),
        RectangleConfig::new(5.0, Gradient::primary(), 0.0, BLANK),
        6.0,
        font.clone(),
    )
    .on_click(|| return Some(State::MovePage(Pages::MainMenu)))
    .set_padding(100.0, 50.0);

    let room_config_btn = RegularButton::new(
        ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::Start),
        None,
        "Konfigurasi",
        TextConfig::default(font.clone()),
        RectangleConfig::new(5.0, Gradient::primary(), 0.0, BLANK),
        6.0,
        font.clone(),
    )
    .on_click(|| return Some(State::OpenDialogueBox(3)))
    .set_padding(75.0, 25.0);

    let left_room_btn = RegularButton::new(
        ObjectPosition::dynamic(DynamicPosition::End, DynamicPosition::Start),
        None,
        "Keluar",
        TextConfig::default(font.clone()),
        RectangleConfig::new(5.0, Gradient::primary(), 0.0, BLANK),
        6.0,
        font.clone(),
    )
    .on_click(|| return Some(State::OpenDialogueBox(1)))
    .set_padding(75.0, 25.0);

    let rectang = Rectangle::new(
        ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Start),
        ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Full),
        ParentState::new(),
        RectangleConfig {
            corner_radius: 10.0,
            color: Gradient {
                colors: vec![Color::from_rgba(0, 0, 0, 100)],
                angle: 0.0,
            },
            outline: 0.0,
            outline_color: BLACK,
        },
    );

    let player_slot_1 = PlayerSlot::new(
        ObjectPosition::dynamic(DynamicPosition::Grid, DynamicPosition::Center),
        ObjectDimension::dynamic(DynamicDimension::Grid, DynamicDimension::Full),
        font.clone(),
    )
    .set_index(0);

    let player_slot_2 = PlayerSlot::new(
        ObjectPosition::dynamic(DynamicPosition::Grid, DynamicPosition::Center),
        ObjectDimension::dynamic(DynamicDimension::Grid, DynamicDimension::Full),
        font.clone(),
    )
    .set_index(1);

    let player_slot_3 = PlayerSlot::new(
        ObjectPosition::dynamic(DynamicPosition::Grid, DynamicPosition::Center),
        ObjectDimension::dynamic(DynamicDimension::Grid, DynamicDimension::Full),
        font.clone(),
    )
    .set_index(2);

    let player_slot_4 = PlayerSlot::new(
        ObjectPosition::dynamic(DynamicPosition::Grid, DynamicPosition::Center),
        ObjectDimension::dynamic(DynamicDimension::Grid, DynamicDimension::Full),
        font.clone(),
    )
    .set_index(3);

    let quit_room_dialog_heading = Text::new("Keluar Dari Room?", font.clone())
        .set_position(ObjectPosition::dynamic(
            DynamicPosition::Center,
            DynamicPosition::Start,
        ))
        .set_font_size(HEADING_2);

    let quit_room_dialog_p = Text::new(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum et tincidunt arcu. Curabitur libero sapien, tristique nec elementum sed, rhoncus sed sapien. Donec ut urna at sem aliquet tempor ac et tortor.", font.clone()
        )
            .wrap_text()
            .set_font_size(HEADING_5)
            .set_position(ObjectPosition::new(0.0, 60.0, Some(DynamicPosition::Center), None));

    let y_btn = RegularButton::new(
        ObjectPosition::dynamic(DynamicPosition::Grid, DynamicPosition::Center),
        None,
        "Ya",
        TextConfig::default(font.clone()),
        RectangleConfig::new(5.0, Gradient::primary(), 0.0, BLANK),
        6.0,
        font.clone(),
    )
    .set_dimensions(ObjectDimension::new(
        0.0,
        0.0,
        Some(DynamicDimension::Grid),
        Some(DynamicDimension::Full),
    ))
    .on_click(|| return Some(State::LeaveRoom))
    .set_is_on_dialogue(1);
    // .set_padding(75.0, 25.0);

    let n_btn = RegularButton::new(
        ObjectPosition::dynamic(DynamicPosition::Flex, DynamicPosition::Center),
        None,
        "Tidak",
        TextConfig::default(font.clone()),
        RectangleConfig::new(5.0, Gradient::primary(), 0.0, BLANK),
        6.0,
        font.clone(),
    )
    .set_dimensions(ObjectDimension::new(
        0.0,
        0.0,
        Some(DynamicDimension::Grid),
        Some(DynamicDimension::Full),
    ))
    .on_click(|| {
        return Some(State::CloseDialogueBox(1));
    })
    .set_is_on_dialogue(1);
    // .set_padding(75.0, 25.0);

    let mut quit_room_dialog_btn_wrapper = Container::new(
        ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::End),
        ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Percent(20.0)),
        ParentState::new(),
        None,
    );

    quit_room_dialog_btn_wrapper.add_child_ref(Box::new(y_btn));
    quit_room_dialog_btn_wrapper.add_child_ref(Box::new(n_btn));
    quit_room_dialog_btn_wrapper.set_is_grid_ref(Direction::X, 20.0);

    quit_room_dialog.add_object_ref(Box::new(quit_room_dialog_heading));
    quit_room_dialog.add_object_ref(Box::new(quit_room_dialog_p));
    quit_room_dialog.add_object_ref(Box::new(quit_room_dialog_btn_wrapper));

    room_code.set_position_ref(ObjectPosition::new(
        20.0,
        0.0,
        None,
        Some(DynamicPosition::Center),
    ));

    wrapper_3_top_top.add_child_ref(Box::new(room_code));

    wrapper_3_top_bottom_marginer.add_child_ref(Box::new(player_slot_1));
    wrapper_3_top_bottom_marginer.add_child_ref(Box::new(player_slot_2));
    wrapper_3_top_bottom_marginer.add_child_ref(Box::new(player_slot_3));
    wrapper_3_top_bottom_marginer.add_child_ref(Box::new(player_slot_4));
    wrapper_3_top_bottom_marginer.set_is_grid_ref(Direction::X, 25.0);

    wrapper_3_top_bottom.add_child_ref(Box::new(wrapper_3_top_bottom_marginer));

    wrapper_3_top.add_child_ref(Box::new(rectang));
    wrapper_3_top.add_child_ref(Box::new(wrapper_3_top_top));
    wrapper_3_top.add_child_ref(Box::new(wrapper_3_top_bottom));

    wrapper_3_bottom.add_child_ref(Box::new(start_game_btn));
    wrapper_3_bottom.add_child_ref(Box::new(room_config_btn));
    wrapper_3_bottom.add_child_ref(Box::new(left_room_btn));

    wrapper_2.add_child_ref(Box::new(wrapper_3_top));
    wrapper_2.add_child_ref(Box::new(wrapper_3_bottom));

    wrapper.add_child_ref(Box::new(wrapper_2));
    wrapper.add_child_ref(Box::new(quit_room_dialog));

    return Box::new(wrapper);
}

fn load_config_dialogue(font: Arc<Nunito>) -> Box<dyn Object + Send> {
    let header = Text::new("Konfigurasi", font.clone()).set_position(ObjectPosition::dynamic(
        DynamicPosition::Center,
        DynamicPosition::Start,
    ));

    let top_container = Container::new(
        ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Start),
        ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Percent(15.0)),
        ParentState::new(),
        None,
    )
    .add_child(Box::new(header));

    let mut edit_config_dialogue = DialogueBox::new(
        ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Center),
        ObjectDimension::absolute(800.0, 500.0),
        RectangleConfig::new(5.0, Gradient::new(0.0, vec![BLACK]), 2.0, WHITE),
        3,
    );

    let switch_1 = load_config_option_switch("Boleh nge-rail", font.clone());
    let switch_2 = load_config_option_switch("Boleh tumpuk londo", font.clone());
    let switch_3 = load_config_option_switch("Pukulan bebas", font.clone());
    let switch_4 = load_config_option_switch("Skor pemukul", font.clone());

    let left_container = Container::new(
        ObjectPosition::dynamic(
            DynamicPosition::Start,
            DynamicPosition::Custom(Arc::new(|px, py, pw, ph| ph * 0.15)),
        ),
        ObjectDimension::dynamic(
            DynamicDimension::Percent(50.0),
            DynamicDimension::Percent(75.0),
        ),
        ParentState::new(),
        None,
    )
    .add_child(switch_1)
    .add_child(switch_2)
    .add_child(switch_3)
    .add_child(switch_4)
    .set_is_flex(Direction::Y, 25.0)
    .set_padding_all(0.0, 20.0, 0.0, 0.0);

    let switch_5 = load_config_option_switch("Joker", font.clone());

    let right_container = Container::new(
        ObjectPosition::dynamic(
            DynamicPosition::End,
            DynamicPosition::Custom(Arc::new(|px, py, pw, ph| ph * 0.15)),
        ),
        ObjectDimension::dynamic(
            DynamicDimension::Percent(50.0),
            DynamicDimension::Percent(75.0),
        ),
        ParentState::new(),
        None,
    )
    .add_child(switch_5)
    .set_is_flex(Direction::Y, 25.0)
    .set_is_flex(Direction::Y, 25.0)
    .set_padding_all(0.0, 0.0, 0.0, 20.0);

    let apply_btn = RegularButton::new(
        ObjectPosition::dynamic(DynamicPosition::Grid, DynamicPosition::Center),
        Some(ObjectDimension::dynamic(
            DynamicDimension::Grid,
            DynamicDimension::Full,
        )),
        "Terapkan",
        TextConfig::default(font.clone()),
        RectangleConfig::new(5.0, Gradient::primary(), 0.0, BLANK),
        6.0,
        font.clone(),
    )
    .set_is_on_dialogue(3);

    let cancel_btn = RegularButton::new(
        ObjectPosition::dynamic(DynamicPosition::Grid, DynamicPosition::Center),
        Some(ObjectDimension::dynamic(
            DynamicDimension::Grid,
            DynamicDimension::Full,
        )),
        "Batal",
        TextConfig::default(font.clone()),
        RectangleConfig::new(5.0, Gradient::gray(), 0.0, BLANK),
        6.0,
        font.clone(),
    )
    .set_is_on_dialogue(3)
    .on_click(|| return Some(State::CloseDialogueBox(3)));

    let btn_container = Container::new(
        ObjectPosition::dynamic(DynamicPosition::End, DynamicPosition::End),
        ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Percent(15.0)),
        ParentState::new(),
        None,
    )
    .add_child(Box::new(apply_btn))
    .add_child(Box::new(cancel_btn))
    .set_is_grid(Direction::X, 15.0);

    edit_config_dialogue.add_object_ref(Box::new(top_container));
    edit_config_dialogue.add_object_ref(Box::new(left_container));
    edit_config_dialogue.add_object_ref(Box::new(right_container));
    edit_config_dialogue.add_object_ref(Box::new(btn_container));

    return Box::new(edit_config_dialogue);
}

fn load_config_option_switch(text: &str, font: Arc<Nunito>) -> Box<dyn Object + Send + Sync> {
    let switch = SwitchButton::new(
        ObjectPosition::new(
            0.0,
            0.0,
            Some(DynamicPosition::End),
            Some(DynamicPosition::Center),
        ),
        50.0,
    );
    let desc = Text::new(text, font.clone())
        .set_config(TextConfig::new(font.regular.clone(), WHITE, HEADING_5))
        .set_position(ObjectPosition::dynamic(
            DynamicPosition::Start,
            DynamicPosition::Center,
        ));

    let container = Container::new(
        ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::Grid),
        ObjectDimension::new(0.0, 50.0, Some(DynamicDimension::Full), None),
        ParentState::new(),
        None,
    )
    .add_child(Box::new(switch))
    .add_child(Box::new(desc));

    return Box::new(container);
}
