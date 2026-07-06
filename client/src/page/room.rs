use macroquad::{
    color::{BLACK, Color},
    window::{screen_height, screen_width},
};
use remyan_core::Player;

use crate::{
    PageIndex,
    page::Page,
    ui::{
        Object, XAlignment, YAlignment,
        button::{Button, ButtonConfig, regular_button::RegularButton},
        config::{
            dimension::{
                DynamicLength::{Custom, Full},
                ObjectDimension,
            },
            position::{ObjectPosition, Position},
        },
        container::Container,
        draw::draw_rectangle_extended,
        gradient::Gradient,
        parent::ParentState,
        rectangle::{Rectangle, RectangleConfig},
    },
};

pub struct Room {
    players: Vec<Player>,
    objects: Vec<Box<dyn Object>>,
}

impl Room {
    pub fn new() -> Self {
        let btn = Box::new(
            RegularButton::new(
                Position::new(0.0, 0.0),
                ButtonConfig::default("Kembali ke Menu Utama"),
            )
            .on_click(|| return Some(PageIndex::MainMenu as usize))
            .set_padding(100.0, 50.0),
        );

        let rectang = Rectangle::new(
            ObjectPosition {
                x: 0.0,
                y: 0.0,
                x_alignment: Some(XAlignment::Center),
                y_alignment: Some(YAlignment::Center),
            },
            ObjectDimension::dynamic(
                Custom(|_, _, p_width, _| p_width - 70.5 * 2.0),
                Custom(|_, _, _, p_height| p_height - 70.5 * 2.0),
            ),
            ParentState::new(),
            RectangleConfig {
                corner_radius: 10.0,
                color: Gradient {
                    colors: vec![Color::from_rgba(0, 0, 0, 75)],
                    angle: 0.0,
                },
                outline: 0.0,
                outline_color: BLACK,
            },
        );

        let wrapper = Container::new(
            ObjectPosition::absolute(0.0, 0.0),
            ObjectDimension::dynamic(Full, Full),
            ParentState::new(),
        )
        .add_child(Box::new(rectang));

        Self {
            players: Vec::new(),
            objects: vec![Box::new(wrapper), btn],
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
