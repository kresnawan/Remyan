use std::sync::Arc;

use macroquad::{
    color::{BLACK, BLANK, Color, WHITE},
    window::{screen_height, screen_width},
};
use remyan_core::Player;

use crate::ui::{
    config::dimension::DynamicDimension, widgets::{
        button::Button, container::Flex, text::{HEADING_5, TextConfig},
    },
};

use crate::{
    page::{Page, Pages},
    state::State,
    ui::{
        config::{
            dimension::{ObjectDimension},
            font::Nunito,
            gradient::Gradient,
            parent::ParentState,
            position::{DynamicPosition, ObjectPosition},
        },
        traits::object::Object,
        widgets::{
            button::regular_button::RegularButton,
            container::Container,
            dialogue_box::{DialogueBox, DialogueBoxState},
            player_slot::PlayerSlot,
            rectangle::{Rectangle, RectangleConfig},
            text::{HEADING_2, Text},
        },
    },
    wrapper::draw::draw_rectangle_extended,
};

pub struct Room {
    players: Vec<Player>,
    objects: Vec<Box<dyn Object>>,
}

impl Room {
    pub fn new() -> Self {
        let wrapper = load_room_objects();
        Self {
            players: Vec::new(),
            objects: vec![wrapper],
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

fn load_room_objects() -> Box<dyn Object> {
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

    let mut room_code = Text::new("Kode Room: 2919391379919339557");

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
        TextConfig::default(),
        RectangleConfig::new(5.0, Gradient::primary(), 0.0, BLANK),
        6.0,
    )
    .on_click(|| return Some(State::MovePage(Pages::MainMenu)))
    .set_padding(100.0, 50.0);

    let room_config_btn = RegularButton::new(
        ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::Start),
        None,
        "Konfigurasi",
        TextConfig::default(),
        RectangleConfig::new(5.0, Gradient::primary(), 0.0, BLANK),
        6.0,
    )
    .on_click(|| return Some(State::MovePage(Pages::MainMenu)))
    .set_padding(75.0, 25.0);

    let left_room_btn = RegularButton::new(
        ObjectPosition::dynamic(DynamicPosition::End, DynamicPosition::Start),
        None,
        "Keluar",
        TextConfig::default(),
        RectangleConfig::new(5.0, Gradient::primary(), 0.0, BLANK),
        6.0,
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
        ObjectPosition::dynamic(DynamicPosition::Flex, DynamicPosition::Center),
        ObjectDimension::dynamic(DynamicDimension::Flex, DynamicDimension::Full),
    )
    .set_player(format!("Kresnawan"));

    let player_slot_2 = PlayerSlot::new(
        ObjectPosition::dynamic(DynamicPosition::Flex, DynamicPosition::Center),
        ObjectDimension::dynamic(DynamicDimension::Flex, DynamicDimension::Full),
    );

    let player_slot_3 = PlayerSlot::new(
        ObjectPosition::dynamic(DynamicPosition::Flex, DynamicPosition::Center),
        ObjectDimension::dynamic(DynamicDimension::Flex, DynamicDimension::Full),
    );

    let player_slot_4 = PlayerSlot::new(
        ObjectPosition::dynamic(DynamicPosition::Flex, DynamicPosition::Center),
        ObjectDimension::dynamic(DynamicDimension::Flex, DynamicDimension::Full),
    );

    let quit_room_dialog_heading = Text::new("Keluar Dari Room?")
        .set_position(ObjectPosition::dynamic(
            DynamicPosition::Center,
            DynamicPosition::Start,
        ))
        .set_font_size(HEADING_2);

    let quit_room_dialog_p = Text::new(
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vestibulum et tincidunt arcu. Curabitur libero sapien, tristique nec elementum sed, rhoncus sed sapien. Donec ut urna at sem aliquet tempor ac et tortor.",
        )
            .wrap_text()
            .set_font_size(HEADING_5)
            .set_position(ObjectPosition::new(0.0, 60.0, Some(DynamicPosition::Center), None));

    let y_btn = RegularButton::new(
        ObjectPosition::dynamic(DynamicPosition::Flex, DynamicPosition::Center),
        None,
        "Ya",
        TextConfig::default(),
        RectangleConfig::new(5.0, Gradient::primary(), 0.0, BLANK),
        6.0,
    )
    .set_dimensions(ObjectDimension::new(0.0, 0.0, Some(DynamicDimension::Flex), Some(DynamicDimension::Full)))
    .on_click(|| return Some(State::MovePage(Pages::MainMenu)))
    .set_is_on_dialogue();
    // .set_padding(75.0, 25.0);

    let n_btn = RegularButton::new(
        ObjectPosition::dynamic(DynamicPosition::Flex, DynamicPosition::Center),
        None,
        "Tidak",
        TextConfig::default(),
        RectangleConfig::new(5.0, Gradient::primary(), 0.0, BLANK),
        6.0,
    )
    .set_dimensions(ObjectDimension::new(0.0, 0.0, Some(DynamicDimension::Flex), Some(DynamicDimension::Full)))
    .on_click(|| {
        return Some(State::CloseDialogueBox(1));
    })
    .set_is_on_dialogue();
    // .set_padding(75.0, 25.0);

    let mut quit_room_dialog_btn_wrapper = Container::new(
        ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::End),
        ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Percent(20.0)),
        ParentState::new(),
        None,
    );

    quit_room_dialog_btn_wrapper.add_child_ref(Box::new(y_btn));
    quit_room_dialog_btn_wrapper.add_child_ref(Box::new(n_btn));
    quit_room_dialog_btn_wrapper.set_is_flex_ref(Flex::X,20.0);

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
    wrapper_3_top_bottom_marginer.set_is_flex_ref(Flex::X,25.0);

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
