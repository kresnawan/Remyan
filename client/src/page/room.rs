use std::sync::Arc;

use macroquad::{
    color::{BLACK, BLUE, Color, GREEN, PURPLE, WHITE},
    window::{screen_height, screen_width},
};
use remyan_core::Player;

use crate::{
    PageIndex, page::Page, ui::{
        HEADING_2, HEADING_3, Object, button::{Button, ButtonConfig, regular_button::RegularButton}, config::{
            dimension::{
                DynamicDimension::{self, Custom, Full, Percent}, ObjectDimension,
            }, position::{DynamicPosition, ObjectPosition, Position},
        }, container::Container, draw::draw_rectangle_extended, font::Nunito, gradient::Gradient, parent::ParentState, player_slot::PlayerSlot, plus::{Plus, PlusAttribute}, rectangle::{Rectangle, RectangleConfig},
    },
};

pub struct Room {
    players: Vec<Player>,
    objects: Vec<Box<dyn Object>>,
}

impl Room {
    pub fn new() -> Self {
        let start_game_btn = RegularButton::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Start),
            ButtonConfig::default("Mulai Game"),
        )
        .on_click(|| return Some(PageIndex::MainMenu as usize))
        .set_padding(100.0, 50.0);

        let room_config_btn = RegularButton::new(
            ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::Start),
            ButtonConfig::new(
                "Konfigurasi",
                HEADING_3,
                Gradient::secondary(),
                10.0,
                2.0,
                WHITE,
                WHITE,
                Nunito::black(),
            ),
        )
        .on_click(|| return Some(PageIndex::MainMenu as usize))
        .set_padding(75.0, 25.0);

        let left_room_btn = RegularButton::new(
            ObjectPosition::dynamic(DynamicPosition::End, DynamicPosition::Start),
            ButtonConfig::new(
                "Keluar",
                HEADING_3,
                Gradient::danger(),
                10.0,
                2.0,
                WHITE,
                WHITE,
                Nunito::black(),
            ),
        )
        .on_click(|| return Some(PageIndex::MainMenu as usize))
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
            ObjectDimension::dynamic(DynamicDimension::Percent(25.0), DynamicDimension::Percent(80.0)),
        );

        let player_slot_2 = PlayerSlot::new(
            ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::End),
            ObjectDimension::dynamic(DynamicDimension::Percent(25.0), DynamicDimension::Percent(80.0)),
        );

        let player_slot_3 = PlayerSlot::new(
            ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::End),
            ObjectDimension::dynamic(DynamicDimension::Percent(25.0), DynamicDimension::Percent(80.0)),
        );

        let player_slot_4 = PlayerSlot::new(
            ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::End),
            ObjectDimension::dynamic(DynamicDimension::Percent(25.0), DynamicDimension::Percent(80.0)),
        );

        let wrapper_3_top = Container::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Start),
            ObjectDimension::dynamic(Full, Percent(80.0)),
            ParentState::new(),
            None,
        )
        .add_child(Box::new(rectang));

        let wrapper_3_bottom = Container::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::End),
            ObjectDimension::dynamic(Full, Percent(15.0)),
            ParentState::new(),
            None,
        )
        .add_child(Box::new(start_game_btn))
        .add_child(Box::new(room_config_btn))
        .add_child(Box::new(left_room_btn));

        let wrapper_2 = Container::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Center),
            ObjectDimension::dynamic(
                Custom(Arc::new(|_, _, p_width, _| p_width - 70.5 * 2.0)),
                Custom(Arc::new(|_, _, _, p_height| p_height - 70.5 * 2.0)),
            ),
            ParentState::new(),
            None,
        )
        .add_child(Box::new(wrapper_3_top))
        .add_child(Box::new(wrapper_3_bottom));

        let wrapper = Container::new(
            ObjectPosition::absolute(0.0, 0.0),
            ObjectDimension::dynamic(Full, Full),
            ParentState::new(),
            None,
        )
        .add_child(Box::new(wrapper_2));

        Self {
            players: Vec::new(),
            objects: vec![Box::new(wrapper)],
        }
    }
}

impl Page for Room {
    fn update(&mut self) -> Option<usize> {
        for i in &mut self.objects {
            if let Some(n) = i.update(None, None, None, None) {
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
