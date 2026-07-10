use std::sync::Arc;

use macroquad::input::{
    KeyCode, MouseButton, get_char_pressed, is_key_pressed, is_mouse_button_released,
};

use crate::{
    state::State,
    ui::{
        config::{
            dimension::{
                DynamicDimension::{Custom, Full},
                ObjectDimension,
            },
            parent::ParentState,
            position::{
                DynamicPosition::{Center, Start},
                ObjectPosition,
            },
        },
        traits::{click::Clickable, hover::Hoverable, object::Object, press::Pressable},
        widgets::{
            container::Container,
            rectangle::{Rectangle, RectangleConfig},
            text::{Text, TextConfig},
        },
    },
};

pub struct TextInputComponents {
    container: Container,
    marginer: Container,
    text: Text,
    background: Rectangle,
}

pub struct TextInputState {
    is_hovered: bool,
    is_pressed: bool,
    is_clicked: bool,
    is_focused: bool,
}

impl TextInputState {
    pub fn new() -> Self {
        TextInputState {
            is_hovered: false,
            is_pressed: false,
            is_clicked: false,
            is_focused: false,
        }
    }
}

pub struct TextInput {
    parent: ParentState,
    components: TextInputComponents,
    pub state: TextInputState,
}

impl TextInput {
    pub fn new(
        position: ObjectPosition,
        dimension: ObjectDimension,
        text_config: TextConfig,
        background_config: RectangleConfig,
    ) -> Self {
        let container = Container::new(position, dimension, ParentState::new(), None);

        let marginer = Container::new(
            ObjectPosition::dynamic(Center, Center),
            ObjectDimension::dynamic(
                Custom(Arc::new(|_, _, pw, _| pw - 20.0)),
                Custom(Arc::new(|_, _, _, ph| ph - 20.0)),
            ),
            ParentState::new(),
            None,
        );

        let background = Rectangle::new(
            ObjectPosition::dynamic(Center, Center),
            ObjectDimension::dynamic(Full, Full),
            ParentState::new(),
            background_config,
        );

        let text = Text::new("")
            .set_config(text_config)
            .set_position(ObjectPosition::dynamic(Start, Center))
            .wrap_text();

        Self {
            parent: ParentState::new(),
            state: TextInputState::new(),
            components: TextInputComponents {
                container,
                marginer,
                text,
                background,
            },
        }
    }
}

impl Object for TextInput {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn draw(&self) {
        self.components.background.draw();
        self.components.text.draw();
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

        self.update_hover();
        self.update_is_clicked();
        self.update_is_pressed();

        if self.state.is_focused {
            self.components.background.config.outline = 2.0;
        } else {
            self.components.background.config.outline = 0.0;
        }

        let is_clicking = is_mouse_button_released(MouseButton::Left);
        if is_clicking && !self.state.is_hovered {
            self.state.is_focused = false;
        }

        if self.state.is_clicked {
            self.state.is_focused = true
        }

        if let Some(c) = get_char_pressed() {
            if self.state.is_focused {
                if c.is_ascii_graphic() || c == ' ' {
                    self.components.text.value.push(c);
                }
            }
        }

        if is_key_pressed(KeyCode::Backspace) {
            self.components.text.value.pop();
        }

        self.components
            .container
            .update(parent_x, parent_y, parent_w, parent_h, state);

        self.components.background.update(
            Some(self.components.container.position.x + self.components.container.parent.x),
            Some(self.components.container.position.y + self.components.container.parent.y),
            Some(self.components.container.dimension.width),
            Some(self.components.container.dimension.height),
            state,
        );

        self.components.marginer.update(
            Some(self.components.container.position.x + self.components.container.parent.x),
            Some(self.components.container.position.y + self.components.container.parent.y),
            Some(self.components.container.dimension.width),
            Some(self.components.container.dimension.height),
            state,
        );

        self.components.text.update(
            Some(self.components.marginer.position.x + self.components.marginer.parent.x),
            Some(self.components.marginer.position.y + self.components.marginer.parent.y),
            Some(self.components.marginer.dimension.width),
            Some(self.components.marginer.dimension.height),
            state,
        );

        return None;
    }

    fn get_dimension(&self) -> ObjectDimension {
        self.components.container.dimension.clone()
    }

    fn get_parent_state(&self) -> ParentState {
        self.parent.clone()
    }

    fn get_position(&self) -> ObjectPosition {
        self.components.container.position.clone()
    }

    fn set_dimension_ref(&mut self, value: ObjectDimension) {
        self.components.container.dimension = value;
    }

    fn set_parent_state_ref(&mut self, value: ParentState) {
        self.parent = value;
    }

    fn set_position_ref(&mut self, value: ObjectPosition) {
        self.components.container.position = value;
    }
}

impl Hoverable for TextInput {
    fn get_is_hovered(&self) -> bool {
        self.state.is_hovered
    }
    fn set_is_hovered(&mut self, value: bool) {
        self.state.is_hovered = value;
    }
}

impl Clickable for TextInput {
    fn set_is_clicked(&mut self, value: bool) {
        self.state.is_clicked = value;
    }
}

impl Pressable for TextInput {
    fn set_is_pressed(&mut self, value: bool) {
        self.state.is_pressed = value;
    }
}
