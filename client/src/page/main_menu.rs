use std::sync::Arc;

use macroquad::prelude::*;

use crate::ui::config::dimension::DynamicDimension::{Full, Percent};
use crate::ui::config::font::Nunito;
use crate::ui::config::gradient::Gradient;
use crate::ui::config::position::DynamicPosition::{Center, Custom, End, Start};
use crate::ui::widgets::button::Button;

use crate::ui::widgets::dialogue_box::{DialogueBox, DialogueBoxState};
use crate::ui::widgets::text::{HEADING_1, HEADING_2, HEADING_3, HEADING_4, Text, TextConfig};
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
        widgets::{
            button::{ButtonConfig, regular_button::RegularButton},
            container::Container,
        },
    },
    wrapper::draw::draw_rectangle_extended,
};

pub struct MainMenu {
    player_name: String,
    objects: Vec<Box<dyn Object>>,
}

impl MainMenu {
    pub fn new(player_name: &str) -> MainMenu {
        let create_room_btn = RegularButton::new(
            ObjectPosition::absolute(0.0, 0.0),
            ButtonConfig::default("Buat Room"),
        )
        .on_click(|| return Some(State::MovePage(Pages::Room)))
        .set_padding(0.0, 50.0)
        .set_alignment(Some(DynamicPosition::Center), None);

        let join_room_btn = RegularButton::new(
            ObjectPosition::absolute(0.0, 150.0),
            ButtonConfig::default("Masuk Room"),
        )
        .on_click(|| {
            return Some(State::OpenDialogueBox(2));
        })
        .set_padding(0.0, 50.0)
        .set_alignment(Some(DynamicPosition::Center), None);

        let settings_btn = RegularButton::new(
            ObjectPosition::absolute(0.0, 150.0 * 2.0),
            ButtonConfig::default("Pengaturan"),
        )
        .on_click(|| {
            return None;
        })
        .set_padding(0.0, 50.0)
        .set_alignment(Some(DynamicPosition::Center), None);

        let div = Container::new(
            ObjectPosition::absolute(0.0, 500.0),
            ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Full),
            ParentState::new(),
            None,
        )
        .add_child(Box::new(create_room_btn))
        .add_child(Box::new(join_room_btn))
        .add_child(Box::new(settings_btn));

        let join_room_text = Text::new("Masuk Room")
            .set_config(TextConfig::new(Nunito::bold(), WHITE, HEADING_1))
            .set_position(ObjectPosition::dynamic(Center, Start));

        let join_room_dialogue_top_wrapper = Container::new(
            ObjectPosition::dynamic(Center, Start),
            ObjectDimension::dynamic(Full, Percent(20.0)),
            ParentState::new(),
            None,
        )
        .add_child(Box::new(join_room_text));

        let room_code_input = TextInput::new(
            ObjectPosition::dynamic(Center, Center),
            ObjectDimension::absolute(300.0, 100.0),
            TextConfig::new(Nunito::regular(), WHITE, HEADING_2),
        );

        let join_room_dialogue_middle_wrapper = Container::new(
            ObjectPosition::dynamic(Center, Custom(Arc::new(|_, _, _, ph| ph * 0.2))),
            ObjectDimension::dynamic(Full, Percent(60.0)),
            ParentState::new(),
            None,
        ).add_child(Box::new(room_code_input));

        let cancel_btn = RegularButton::new(
            ObjectPosition::absolute(0.0, 0.0),
            ButtonConfig::default("Batal"),
        )
        .set_dimensions(ObjectDimension::new(0.0, 0.0, None, Some(Full)))
        .on_click(|| return Some(State::CloseDialogueBox(2))).set_is_on_dialogue();

        let join_btn = RegularButton::new(
            ObjectPosition::absolute(0.0, 0.0),
            ButtonConfig::default("Masuk"),
        )
        .set_dimensions(ObjectDimension::new(0.0, 0.0, None, Some(Full))).set_is_on_dialogue();

        let join_room_dialogue_bottom_wrapper = Container::new(
            ObjectPosition::dynamic(Center, End),
            ObjectDimension::dynamic(Full, Percent(20.0)),
            ParentState::new(),
            None,
        )
        .add_child(Box::new(cancel_btn))
        .add_child(Box::new(join_btn))
        .set_is_flex(15.0);

        let mut join_room_dialogue = DialogueBox::new(
            ObjectPosition::dynamic(Center, Center),
            ObjectDimension::absolute(800.0, 500.0),
            DialogueBoxState::new(
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
            objects: vec![Box::new(div), Box::new(join_room_dialogue)],
        };
    }
}

impl Page for MainMenu {
    fn update(&mut self, state: &Option<State>) -> Option<State> {
        for item in &mut self.objects {
            if let Some(n) = item.update(None, None, None, None, state) {
                return Some(n);
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
