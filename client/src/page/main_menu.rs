use std::sync::Arc;

use macroquad::prelude::*;

use crate::ui::config::font::Nunito;
use crate::ui::config::gradient::Gradient;
use crate::ui::config::position::DynamicPosition::{Center, Custom, End, Start};
use crate::ui::widgets::button::Button;

use crate::ui::widgets::container::Direction;
use crate::ui::widgets::dialogue_box::DialogueBox;
use crate::ui::widgets::rectangle::RectangleConfig;
use crate::ui::widgets::text::{HEADING_1, HEADING_2, Text, TextConfig};
use crate::ui::widgets::text_input::TextInput;
use crate::{
    page::{Page, Pages},
    state::State,
    ui::{
        config::{
            dimension::{DynamicDimension, ObjectDimension},
            parent::ParentState,
            position::{DynamicPosition, ObjectPosition},
        },
        traits::object::Object,
        widgets::{button::regular_button::RegularButton, container::Container},
    },
    wrapper::draw::draw_rectangle_extended,
};

pub struct MainMenu {
    player_name: String,
    objects: Vec<Box<dyn Object + Send>>,
    room_id_input: String,
}

impl MainMenu {
    pub fn new(player_name: &str, font: Arc<Nunito>) -> MainMenu {
        let top_container = Container::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Start),
            ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Percent(50.0)),
            ParentState::new(),
            None,
        );

        let create_room_btn = RegularButton::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Start),
            Some(ObjectDimension::dynamic(
                DynamicDimension::Full,
                DynamicDimension::Grid,
            )),
            "Buat Room",
            TextConfig::default(font.clone()),
            RectangleConfig::new(5.0, Gradient::primary(), 0.0, BLANK),
            6.0,
            font.clone(),
        )
        .on_click(|| return Some(State::CreateRoom))
        .set_padding(0.0, 50.0)
        .set_alignment(Some(DynamicPosition::Center), None);

        let join_room_btn = RegularButton::new(
            ObjectPosition::absolute(0.0, 200.0),
            Some(ObjectDimension::dynamic(
                DynamicDimension::Full,
                DynamicDimension::Grid,
            )),
            "Gabung Room",
            TextConfig::default(font.clone()),
            RectangleConfig::new(5.0, Gradient::primary(), 0.0, BLANK),
            6.0,
            font.clone(),
        )
        .on_click(|| {
            return Some(State::OpenDialogueBox(2));
        })
        .set_padding(0.0, 50.0)
        .set_alignment(Some(DynamicPosition::Center), None);

        let settings_btn = RegularButton::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Grid),
            Some(ObjectDimension::dynamic(
                DynamicDimension::Full,
                DynamicDimension::Grid,
            )),
            "Pengaturan",
            TextConfig::default(font.clone()),
            RectangleConfig::new(5.0, Gradient::primary(), 0.0, BLANK),
            6.0,
            font.clone(),
        )
        .on_click(|| {
            return None;
        })
        .set_padding(0.0, 50.0)
        .set_alignment(Some(DynamicPosition::Center), None);

        let bottom_container = Container::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::End),
            ObjectDimension::dynamic(
                DynamicDimension::Percent(50.0),
                DynamicDimension::Percent(50.0),
            ),
            ParentState::new(),
            None,
        )
        .add_child(Box::new(create_room_btn))
        .add_child(Box::new(join_room_btn))
        .add_child(Box::new(settings_btn))
        .set_is_flex(Direction::Y, 30.0);

        let container = Container::new(
            ObjectPosition::absolute(0.0, 0.0),
            ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Full),
            ParentState::new(),
            None,
        )
        .add_child(Box::new(top_container))
        .add_child(Box::new(bottom_container))
        .set_padding(90.0, 90.0);

        let join_room_text = Text::new("Masuk Room", font.clone())
            .set_config(TextConfig::new(font.bold.clone(), WHITE, HEADING_1))
            .set_position(ObjectPosition::dynamic(Center, Start));

        let join_room_dialogue_top_wrapper = Container::new(
            ObjectPosition::dynamic(Center, Start),
            ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Percent(20.0)),
            ParentState::new(),
            None,
        )
        .add_child(Box::new(join_room_text));

        let room_code_input = TextInput::new(
            ObjectPosition::dynamic(Center, Center),
            ObjectDimension::absolute(300.0, 100.0),
            TextConfig::new(font.regular.clone(), WHITE, HEADING_2),
            RectangleConfig::new(5.0, Gradient::new(0.0, vec![BLACK]), 2.0, WHITE),
            font.clone(),
        )
        .set_on_change_event(Box::new(|value| return Some(State::InputRoomId(value))));

        let join_room_dialogue_middle_wrapper = Container::new(
            ObjectPosition::dynamic(Center, Custom(Arc::new(|_, _, _, ph| ph * 0.2))),
            ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Percent(60.0)),
            ParentState::new(),
            None,
        )
        .add_child(Box::new(room_code_input));

        let cancel_btn = RegularButton::new(
            ObjectPosition::dynamic(DynamicPosition::Grid, DynamicPosition::Center),
            None,
            "Batal",
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
        .on_click(|| return Some(State::CloseDialogueBox(2)))
        .set_is_on_dialogue(2);

        let join_btn = RegularButton::new(
            ObjectPosition::dynamic(DynamicPosition::Grid, DynamicPosition::Center),
            None,
            "Masuk",
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
        .set_is_on_dialogue(2)
        .on_click(|| return Some(State::JoinRoom(String::new())));

        let join_room_dialogue_bottom_wrapper = Container::new(
            ObjectPosition::dynamic(Center, End),
            ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Percent(20.0)),
            ParentState::new(),
            None,
        )
        .add_child(Box::new(cancel_btn))
        .add_child(Box::new(join_btn))
        .set_is_grid(Direction::X, 15.0);

        let mut join_room_dialogue = DialogueBox::new(
            ObjectPosition::dynamic(Center, Center),
            ObjectDimension::absolute(800.0, 500.0),
            RectangleConfig::new(
                5.0,
                Gradient::new(0.0, vec![Color::from_hex(0x181d30)]),
                2.0,
                Color::from_hex(0x242b45),
            ),
            2,
        );

        join_room_dialogue.add_object_ref(Box::new(join_room_dialogue_top_wrapper));
        join_room_dialogue.add_object_ref(Box::new(join_room_dialogue_middle_wrapper));
        join_room_dialogue.add_object_ref(Box::new(join_room_dialogue_bottom_wrapper));

        return MainMenu {
            player_name: String::from(player_name),
            objects: vec![Box::new(container), Box::new(join_room_dialogue)],
            room_id_input: String::new(),
        };
    }
}

impl Page for MainMenu {
    fn update(&mut self, state: &Option<State>) -> Option<State> {
        for item in &mut self.objects {
            if let Some(n) = item.update(None, None, None, None, state) {
                match n {
                    State::InputRoomId(value) => {
                        self.room_id_input = value;

                        println!("{}", self.room_id_input);
                        return None;
                    }

                    State::JoinRoom(_) => return Some(State::JoinRoom(self.room_id_input.clone())),

                    _ => {
                        return Some(n);
                    }
                }
            }
        }

        return None;
    }
    fn draw(&self) {
        clear_background(BLACK);
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
        for item in &self.objects {
            item.draw();
        }
    }
}
