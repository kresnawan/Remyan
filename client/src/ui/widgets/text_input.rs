use std::sync::Arc;

use macroquad::{
    color::{BLACK, BLUE, Color, GREEN, PURPLE, WHITE},
    input::{
        KeyCode, MouseButton, get_char_pressed, is_key_down, is_key_pressed,
        is_mouse_button_released,
    },
};

use crate::{
    state::State,
    ui::{
        config::{
            dimension::{
                DynamicDimension::{Custom, Full},
                ObjectDimension,
            },
            font::Nunito,
            gradient::Gradient,
            parent::ParentState,
            position::{
                self,
                DynamicPosition::{Center, End, Start},
                ObjectPosition,
            },
        },
        traits::{click::Clickable, hover::Hoverable, object::Object, press::Pressable},
        widgets::{
            container::Container,
            rectangle::{Rectangle, RectangleConfig},
            text::{HEADING_1, HEADING_3, HEADING_4, HEADING_5, Text, TextConfig},
        },
    },
};

pub struct TextInputConfig {
    text_config: TextConfig,
    bg_config: RectangleConfig,
    placeholder: String,
    is_hovered: bool,
    is_pressed: bool,
    is_clicked: bool,
    is_focused: bool,
}

impl TextInputConfig {
    pub fn default() -> Self {
        Self {
            text_config: TextConfig {
                font: Nunito::regular(),
                font_size: HEADING_5,
                color: WHITE,
                is_shown: true,
            },
            bg_config: RectangleConfig {
                corner_radius: 5.0,
                color: Gradient::new(0.0, vec![BLACK]),
                outline: 2.0,
                outline_color: WHITE,
            },
            placeholder: String::new(),
            is_clicked: false,
            is_hovered: false,
            is_pressed: false,
            is_focused: false,
        }
    }
}

pub struct TextInput {
    value: String,
    position: ObjectPosition,
    dimension: ObjectDimension,
    parent: ParentState,
    container: Container,
    pub config: TextInputConfig,
}

impl TextInput {
    pub fn new(
        position: ObjectPosition,
        dimension: ObjectDimension,
        text_config: TextConfig,
    ) -> Self {
        let config = TextInputConfig::default();

        let bg = Rectangle::new(
            ObjectPosition::dynamic(Center, Center),
            ObjectDimension::dynamic(Full, Full),
            ParentState::new(),
            config.bg_config.clone(),
        );

        let text = Text::new("")
            .set_config(text_config)
            .set_position(ObjectPosition::dynamic(Start, Center))
            .wrap_text();

        let marginer = Container::new(
            ObjectPosition::dynamic(Center, Center),
            ObjectDimension::dynamic(
                Custom(Arc::new(|_, _, pw, _| pw - 20.0)),
                Custom(Arc::new(|_, _, _, ph| ph - 20.0)),
            ),
            ParentState::new(),
            None,
        )
        .add_child(Box::new(text));
        // .add_child(Box::new(placeholder_obj));

        let outer_container = Container::new(
            position.clone(),
            dimension.clone(),
            ParentState::new(),
            None,
        )
        .add_child(Box::new(bg))
        .add_child(Box::new(marginer));

        Self {
            value: String::new(),
            position,
            dimension,
            parent: ParentState::new(),
            container: outer_container,
            config: config,
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
        self.container.draw();
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

        

        {
            let bg_any = self.container.objects[0].as_any_mut();
            let bg_casted: &mut Rectangle = bg_any.downcast_mut().unwrap();

            if self.config.is_focused {
                bg_casted.config.outline = 2.0;
            } else {
                bg_casted.config.outline = 0.0;
            }
        }

        let marginer_any = self.container.objects[1].as_any_mut();
        let marginer_casted: &mut Container = marginer_any.downcast_mut().unwrap();

        let text_any = marginer_casted.objects[0].as_any_mut();
        let text_casted: &mut Text = text_any.downcast_mut().unwrap();

        let is_clicking = is_mouse_button_released(MouseButton::Left);
        if is_clicking && !self.config.is_hovered {
            self.config.is_focused = false;
        }

        if self.config.is_clicked {
            self.config.is_focused = true
        }

        if let Some(c) = get_char_pressed() {
            if self.config.is_focused {
                if c.is_ascii_graphic() || c == ' ' {
                    self.value.push(c);

                    text_casted.value.push(c);
                }
            }
        }

        if is_key_pressed(KeyCode::Backspace) {
            self.value.pop();

            text_casted.value.pop();
        }

        self.container.update(
            Some(self.position.x + self.parent.x),
            Some(self.position.y + self.parent.y),
            Some(self.dimension.width),
            Some(self.dimension.height),
            state,
        );

        return None;
    }

    fn get_dimension(&self) -> ObjectDimension {
        self.dimension.clone()
    }

    fn get_parent_state(&self) -> ParentState {
        self.parent.clone()
    }

    fn get_position(&self) -> ObjectPosition {
        self.position.clone()
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

impl Hoverable for TextInput {
    fn get_is_hovered(&self) -> bool {
        self.config.is_hovered
    }
    fn set_is_hovered(&mut self, value: bool) {
        self.config.is_hovered = value;
    }
}

impl Clickable for TextInput {
    fn set_is_clicked(&mut self, value: bool) {
        self.config.is_clicked = value;
    }
}

impl Pressable for TextInput {
    fn set_is_pressed(&mut self, value: bool) {
        self.config.is_pressed = value;
    }
}
