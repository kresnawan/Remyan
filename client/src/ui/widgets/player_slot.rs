use std::sync::Arc;

use macroquad::color::{BLANK, Color};

use crate::{
    state::State, ui::{
        config::{
            dimension::{DynamicDimension::Full, ObjectDimension}, font::Nunito, gradient::Gradient, parent::ParentState, position::{
                DynamicPosition::{self, Center},
                ObjectPosition,
            },
        }, traits::object::Object, widgets::{
            plus::{Plus, PlusAttribute},
            rectangle::{Rectangle, RectangleConfig},
            text::Text,
        },
    },
};

pub struct PlayerSlotState {
    pub player: Option<String>,
    pub is_hovered: bool,
    pub is_pressed: bool,
    pub is_clicked: bool,
}

impl PlayerSlotState {
    pub fn new() -> Self {
        PlayerSlotState {
            player: None,
            is_hovered: false,
            is_pressed: false,
            is_clicked: false,
        }
    }
}

pub struct PlayerSlot {
    position: ObjectPosition,
    dimension: ObjectDimension,
    parent: ParentState,
    state: PlayerSlotState,
    plus: Plus,
    rec_outline: Rectangle,
    rec_fill: Rectangle,
    player_name: Text,
}

impl PlayerSlot {
    pub fn new(position: ObjectPosition, dimension: ObjectDimension, font: Arc<Nunito>) -> PlayerSlot {
        let plus = Plus::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Center),
            PlusAttribute::default(),
        );
        let rec_fill = Rectangle::new(
            position.clone(),
            ObjectDimension::dynamic(Full, Full),
            ParentState::new(),
            RectangleConfig::new(
                10.0,
                Gradient::new(0.0, vec![Color::from_rgba(255, 255, 255, 25)]),
                0.0,
                BLANK,
            ),
        );

        let rec_outline = Rectangle::new(
            position.clone(),
            ObjectDimension::dynamic(Full, Full),
            ParentState::new(),
            RectangleConfig::new(
                10.0,
                Gradient::new(0.0, vec![BLANK]),
                2.0,
                Color::from_hex(0x4f4f4f),
            ),
        );

        let player_name = Text::new("", font)
            .set_position(ObjectPosition::dynamic(Center, Center))
            .set_dimension(ObjectDimension::new(0.0, 0.0, None, Some(Full)));

        PlayerSlot {
            position,
            dimension,
            parent: ParentState::new(),
            state: PlayerSlotState::new(),
            plus: plus,
            rec_outline: rec_outline,
            rec_fill: rec_fill,
            player_name,
        }
    }

    pub fn set_player(mut self, name: String) -> Self {
        self.state.player = Some(name);
        self
    }
}

impl Object for PlayerSlot {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn draw(&self) {
        self.rec_fill.draw();
        self.rec_outline.draw();

        if let Some(_) = &self.state.player {
            self.player_name.draw();
        } else {
            self.plus.draw();
        }
    }

    fn update(
        &mut self,
        parent_x: Option<f32>,
        parent_y: Option<f32>,
        parent_w: Option<f32>,
        parent_h: Option<f32>,
        state: &Option<State>,
    ) -> Option<State> {
        self.update_parent_state(parent_x, parent_y, parent_w, parent_h);
        self.update_dimension();
        self.update_alignment();

        if let Some(name) = &self.state.player {
            self.rec_fill.config.color = Gradient::new(
                0.0,
                vec![Color::from_hex(0x363636), Color::from_hex(0x363636)],
            );
            self.rec_outline.config.outline_color = Color::from_hex(0x454545);

            self.player_name.value = name.clone();
        } else {
            self.rec_fill.config.color = Gradient::new(
                0.0,
                vec![
                    Color::from_rgba(255, 255, 255, 25),
                    Color::from_rgba(255, 255, 255, 25),
                ],
            );
            self.rec_outline.config.outline_color = Color::from_rgba(255, 255, 255, 50);
        }

        self.plus.update(
            Some(self.position.x + self.parent.x),
            Some(self.position.y + self.parent.y),
            Some(self.dimension.width),
            Some(self.dimension.height),
            state,
        );

        self.player_name.update(
            Some(self.position.x + self.parent.x),
            Some(self.position.y + self.parent.y),
            Some(self.dimension.width),
            Some(self.dimension.height),
            state,
        );

        self.rec_fill.update(
            Some(self.position.x + self.parent.x),
            Some(self.position.y + self.parent.y),
            Some(self.dimension.width),
            Some(self.dimension.height),
            state,
        );

        self.rec_outline.update(
            Some(self.position.x + self.parent.x),
            Some(self.position.y + self.parent.y),
            Some(self.dimension.width),
            Some(self.dimension.height),
            state,
        );

        return None;
    }

    fn get_dimension(&self) -> ObjectDimension {
        return self.dimension.clone();
    }

    fn get_parent_state(&self) -> ParentState {
        return self.parent.clone();
    }

    fn get_position(&self) -> ObjectPosition {
        return self.position.clone();
    }

    fn set_dimension_ref(&mut self, value: ObjectDimension) {
        self.dimension = value;
    }

    fn set_parent_state_ref(&mut self, value: ParentState) {
        self.parent = value;
    }

    fn set_position_ref(&mut self, value: ObjectPosition) {
        self.position = value;
    }
}
