use std::sync::Arc;

use macroquad::{
    color::{BLACK, BLUE, Color, GREEN, PURPLE, RED, WHITE, YELLOW},
    window::{screen_height, screen_width},
};
use remyan_core::Player;

use crate::{
    Pages,
    page::Page,
    ui::{
        HEADING_2, HEADING_3, Object,
        State::{self, MovePage},
        button::{Button, ButtonConfig, regular_button::RegularButton},
        config::{
            dimension::{
                DynamicDimension::{self, Custom, Full, Percent},
                ObjectDimension,
            },
            position::{
                DynamicPosition::{self, Center, End, Start},
                ObjectPosition, Position,
            },
        },
        container::Container,
        dialogue_box::{DialogueBox, DialogueBoxState},
        draw::draw_rectangle_extended,
        font::Nunito,
        gradient::Gradient,
        parent::ParentState,
        player_slot::PlayerSlot,
        plus::{Plus, PlusAttribute},
        rectangle::{Rectangle, RectangleConfig},
        text::Text,
    },
};

pub struct Room {
    players: Vec<Player>,
    objects: Vec<Box<dyn Object>>,
}

impl Room {
    pub fn new() -> Self {
        let mut quit_room_dialog = DialogueBox::new(
            ObjectPosition::dynamic(Center, Center),
            ObjectDimension::absolute(800.0, 400.0),
            DialogueBoxState::new(
                Gradient::new(0.0, vec![Color::from_hex(0x2e2e2e)]),
                2.0,
                Color::from_hex(0x5e5e5e),
            ),
            1,
        );

        let mut room_code = Text::new("Kode Room: 2919391379919339557");

        let mut wrapper_3_top_top = Container::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Start),
            ObjectDimension::dynamic(DynamicDimension::Full, Percent(10.0)),
            ParentState::new(),
            None,
        );

        let mut wrapper_3_top_bottom_marginer = Container::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Center),
            ObjectDimension::dynamic(DynamicDimension::Percent(70.0), Percent(90.0)),
            ParentState::new(),
            None,
        );

        let mut wrapper_3_top_bottom = Container::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::End),
            ObjectDimension::dynamic(Full, Percent(90.0)),
            ParentState::new(),
            None,
        );

        let mut wrapper_3_top = Container::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Start),
            ObjectDimension::dynamic(Full, Percent(80.0)),
            ParentState::new(),
            None,
        );

        let mut wrapper_3_bottom = Container::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::End),
            ObjectDimension::dynamic(Full, Percent(15.0)),
            ParentState::new(),
            None,
        );

        let mut wrapper_2 = Container::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Center),
            ObjectDimension::dynamic(
                Custom(Arc::new(|_, _, p_width, _| p_width - 70.5 * 2.0)),
                Custom(Arc::new(|_, _, _, p_height| p_height - 70.5 * 2.0)),
            ),
            ParentState::new(),
            None,
        );

        let mut wrapper = Container::new(
            ObjectPosition::absolute(0.0, 0.0),
            ObjectDimension::dynamic(Full, Full),
            ParentState::new(),
            None,
        );

        let start_game_btn = RegularButton::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Start),
            ButtonConfig::default("Mulai Game"),
        )
        .on_click(|| return Some(MovePage(Pages::MainMenu)))
        .set_padding(100.0, 50.0);

        let room_config_btn = RegularButton::new(
            ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::Start),
            ButtonConfig::new(
                "Konfigurasi",
                HEADING_3,
                Gradient::gray(),
                10.0,
                0.0,
                WHITE,
                BLACK,
                Nunito::black(),
            ),
        )
        .on_click(|| return Some(MovePage(Pages::MainMenu)))
        .set_padding(75.0, 25.0);

        let left_room_btn = RegularButton::new(
            ObjectPosition::dynamic(DynamicPosition::End, DynamicPosition::Start),
            ButtonConfig::new(
                "Keluar",
                HEADING_3,
                Gradient::danger(),
                10.0,
                0.0,
                WHITE,
                BLACK,
                Nunito::black(),
            ),
        )
        .on_click(|| return Some(State::OpenDialogueBox(1)))
        .set_padding(75.0, 25.0);

        let rectang = Rectangle::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Start),
            ObjectDimension::dynamic(Full, Full),
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
            ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::End),
            ObjectDimension::dynamic(DynamicDimension::Percent(25.0), DynamicDimension::Full),
        );

        let player_slot_2 = PlayerSlot::new(
            ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::End),
            ObjectDimension::dynamic(DynamicDimension::Percent(25.0), DynamicDimension::Full),
        );

        let player_slot_3 = PlayerSlot::new(
            ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::End),
            ObjectDimension::dynamic(DynamicDimension::Percent(25.0), DynamicDimension::Full),
        );

        let player_slot_4 = PlayerSlot::new(
            ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::End),
            ObjectDimension::dynamic(DynamicDimension::Percent(25.0), DynamicDimension::Full),
        );

        let quit_room_dialog_heading = Text::new("Keluar Dari Room?")
            .set_position(ObjectPosition::dynamic(Center, Start))
            .set_font_size(HEADING_2);

        let quit_room_dialog_p = Text::new(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum et tincidunt arcu. Curabitur libero sapien, tristique nec elementum sed, rhoncus sed sapien. Donec ut urna at sem aliquet tempor ac et tortor.",
        )
            .wrap_text()
            .set_font_size(HEADING_3)
            .set_position(ObjectPosition::new(0.0, 60.0, Some(Center), None));

        let y_btn = RegularButton::new(
            ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::Start),
            ButtonConfig::new(
                "Ya",
                HEADING_3,
                Gradient::gray(),
                10.0,
                0.0,
                WHITE,
                BLACK,
                Nunito::black(),
            ),
        )
        .on_click(|| return Some(State::MovePage(Pages::MainMenu)))
        .set_padding(75.0, 25.0);

        let n_btn = RegularButton::new(
            ObjectPosition::dynamic(DynamicPosition::End, DynamicPosition::Start),
            ButtonConfig::new(
                "Tidak",
                HEADING_3,
                Gradient::danger(),
                10.0,
                0.0,
                WHITE,
                BLACK,
                Nunito::black(),
            ),
        )
        .on_click(|| {
            return Some(State::CloseDialogueBox(1));
        })
        .set_padding(75.0, 25.0);

        let mut quit_room_dialog_btn_wrapper = Container::new(
            ObjectPosition::dynamic(Center, End),
            ObjectDimension::dynamic(Full, Percent(20.0)),
            ParentState::new(),
            None,
        );

        quit_room_dialog_btn_wrapper.add_child_ref(Box::new(y_btn));
        quit_room_dialog_btn_wrapper.add_child_ref(Box::new(n_btn));
        quit_room_dialog_btn_wrapper.set_is_flex_ref(20.0);

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
        wrapper_3_top_bottom_marginer.set_is_flex_ref(25.0);

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

        Self {
            players: Vec::new(),
            objects: vec![Box::new(wrapper)],
        }
    }
}

impl Page for Room {
    fn update(&mut self, state: &Option<State>) -> Option<State> {
        for i in &mut self.objects {
            if let Some(n) = i.update(None, None, None, None, state) {
                return Some(n);
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
