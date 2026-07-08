use macroquad::color::{BLANK, Color};

use crate::ui::{
    Object, State, config::{dimension::ObjectDimension, position::{DynamicPosition, ObjectPosition}}, gradient::Gradient, parent::ParentState, plus::{Plus, PlusAttribute}, rectangle::{Rectangle, RectangleConfig},
};

pub struct PlayerSlotState {
    pub player: Option<String>,
    pub is_hovered: bool,
    pub is_pressed: bool,
    pub is_clicked: bool,
    plus: Plus,
}

impl PlayerSlotState {
    pub fn new() -> Self {
        let plus = Plus::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Center),
            PlusAttribute::default(),
        );
        PlayerSlotState {
            player: None,
            is_hovered: false,
            is_pressed: false,
            is_clicked: false,
            plus,
        }
    }
}

pub struct PlayerSlot {
    position: ObjectPosition,
    dimension: ObjectDimension,
    parent: ParentState,
    state: PlayerSlotState,
}

impl PlayerSlot {
    pub fn new(position: ObjectPosition, dimension: ObjectDimension) -> PlayerSlot {
        PlayerSlot {
            position,
            dimension,
            parent: ParentState::new(),
            state: PlayerSlotState::new(),
        }
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
        Rectangle::new(
            self.position.clone(),
            self.dimension.clone(),
            self.parent.clone(),
            RectangleConfig::new(
                10.0,
                Gradient::new(0.0, vec![Color::from_rgba(255, 255, 255, 25)]),
                0.0,
                BLANK,
            ),
        )
        .draw();
        Rectangle::new(
            self.position.clone(),
            self.dimension.clone(),
            self.parent.clone(),
            RectangleConfig::new(
                10.0,
                Gradient::new(0.0, vec![Color::from_rgba(255, 255, 255, 0)]),
                2.0,
                Color::from_rgba(255, 255, 255, 50),
            ),
        )
        .draw();

        self.state.plus.draw();
    }

    fn update(
        &mut self,
        parent_x: Option<f32>,
        parent_y: Option<f32>,
        parent_w: Option<f32>,
        parent_h: Option<f32>,
        state: &Option<State>
    ) -> Option<State> {
        self.update_parent_state(parent_x, parent_y, parent_w, parent_h);
        self.update_dimension();
        self.update_alignment();

        self.state.plus.update(
            Some(self.position.x + self.parent.x),
            Some(self.position.y + self.parent.y),
            Some(self.dimension.width),
            Some(self.dimension.height),
            state
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
        self.state.plus.update(
            Some(self.position.x + self.parent.x),
            Some(self.position.y + self.parent.y),
            Some(self.dimension.width),
            Some(self.dimension.height),
            &None
        );
    }

    fn set_parent_state_ref(&mut self, value: ParentState) {
        self.parent = value;
    }

    fn set_position_ref(&mut self, value: ObjectPosition) {
        self.position = value;
        self.state.plus.update(
            Some(self.position.x + self.parent.x),
            Some(self.position.y + self.parent.y),
            Some(self.dimension.width),
            Some(self.dimension.height),
            &None
        );
    }
}
