use macroquad::color::{BLANK, Color};

use crate::ui::{
    config::{
        dimension::ObjectDimension,
        gradient::Gradient,
        parent::ParentState,
        position::{DynamicPosition, ObjectPosition},
    }, traits::{click::Clickable, hover::Hoverable, object::Object}, widgets::rectangle::{Rectangle, RectangleConfig},
};

struct SwitchButtonState {
    is_on: bool,
    is_hovered: bool,
    is_pressed: bool,
    is_clicked: bool
}

impl SwitchButtonState {
    pub fn new() -> Self {
        SwitchButtonState { is_on: false, is_hovered: false, is_pressed: false, is_clicked: false }
    }
}

struct SwitchButtonComponents {
    background: Rectangle,
    switch: Rectangle,
}

pub struct SwitchButton {
    components: SwitchButtonComponents,
    state: SwitchButtonState
}

impl SwitchButton {
    pub fn new(position: ObjectPosition, width: f32) -> Self {
        let background = Rectangle::new(
            position,
            ObjectDimension::absolute(width, width / 2.0),
            ParentState::new(),
            RectangleConfig::new(
                width,
                Gradient::new(0.0, vec![Color::from_hex(0x252730)]),
                5.0,
                Color::from_hex(0x2f313d),
            ),
        );

        let switch = Rectangle::new(
            ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::Center),
            ObjectDimension::absolute(width / 2.0, width / 2.0),
            ParentState::new(),
            RectangleConfig::new(width, Gradient::gray(), 0.0, BLANK),
        );
        SwitchButton {
            components: SwitchButtonComponents { background, switch },
            state: SwitchButtonState::new()
        }
    }
}

unsafe impl Sync for SwitchButton {}
unsafe impl Send for SwitchButton {}

impl Object for SwitchButton {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn update(
        &mut self,
        parent_x: Option<f32>,
        parent_y: Option<f32>,
        parent_w: Option<f32>,
        parent_h: Option<f32>,
        state: &Option<crate::state::State>
    ) -> Option<crate::state::State>
    {
        self.update_parent_state(parent_x, parent_y, parent_w, parent_h);
        self.update_dimension();
        self.update_alignment();
        self.update_hover();
        self.update_is_clicked();

        if self.state.is_clicked {
            if self.state.is_on {
                self.state.is_on = false;
                self.components.switch.position.x_dyn = Some(DynamicPosition::Start);
                self.components.switch.config.color = Gradient::gray();
            } else {
                self.state.is_on = true;
                self.components.switch.position.x_dyn = Some(DynamicPosition::End);
                self.components.switch.config.color = Gradient::primary();
            }
        }

        self.components.switch.update(
            Some(self.components.background.position.x + self.components.background.parent.x), 
            Some(self.components.background.position.y + self.components.background.parent.y), 
            Some(self.components.background.dimension.width), 
            Some(self.components.background.dimension.height), 
            state
        );

        return None;
    }

    fn draw(&self) {
        self.components.background.draw();
        self.components.switch.draw();
    }
    

    fn get_dimension(&self) -> ObjectDimension {
        self.components.background.dimension.clone()
    }

    fn get_parent_state(&self) -> ParentState {
        self.components.background.parent.clone()
    }

    fn get_position(&self) -> ObjectPosition {
        self.components.background.position.clone()
    }

    fn set_dimension_ref(&mut self, value: ObjectDimension) {
        self.components.background.dimension = value;
    }

    fn set_parent_state_ref(&mut self, value: ParentState) {
        self.components.background.parent = value;
    }

    fn set_position_ref(&mut self, value: ObjectPosition) {
        self.components.background.position = value;
    }
}

impl Hoverable for SwitchButton {
    fn get_is_hovered(&self) -> bool {
        self.state.is_hovered
    }

    fn set_is_hovered(&mut self, value: bool) {
        self.state.is_hovered = value;
    }
}

impl Clickable for SwitchButton {
    fn set_is_clicked(&mut self, value: bool) {
        self.state.is_clicked = value;
    }
}