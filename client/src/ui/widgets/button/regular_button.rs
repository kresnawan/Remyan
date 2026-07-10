use macroquad::prelude::*;

use crate::{state::State, ui::{config::{dimension::ObjectDimension, parent::ParentState, position::ObjectPosition}, traits::{click::Clickable, hover::Hoverable, object::Object, press::Pressable}, widgets::button::{Button, ButtonAttribute, ButtonConfig}}, wrapper::draw::draw_rectangle_extended};

pub struct RegularButton {
    position: ObjectPosition,
    dimension: ObjectDimension,
    parent: ParentState,
    text: String,
    attribute: ButtonAttribute,
    is_on_dialogue: bool
}

impl RegularButton {
    pub fn set_padding(mut self, x: f32, y: f32) -> Self {
        self.dimension.width += x * 2.0;
        self.dimension.height += y * 2.0;
        return self;
    }

    pub fn set_dimensions(mut self, value: ObjectDimension) -> Self {
        self.dimension = value;
        self
    }

    pub fn set_is_on_dialogue(mut self) -> Self {
        self.is_on_dialogue = true;
        self
    }
}

impl Object for RegularButton {
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
        state: &Option<State>
    ) -> Option<State> {
        self.update_parent_state(parent_x, parent_y, parent_w, parent_h);
        self.update_dimension();
        self.update_alignment();

        self.update_hover();
        
        // this is just naive workaround to make buttons which not contained in currently opened dialogue
        // box to disabled so that it's not accidentaly clicked unintentionally
        //
        // it's created this way because we didn't find a way to make sure that hovered element is only
        // one that visible on top
        //
        //
        if let Some(state) = state {
            match state {
                State::OpenDialogueBox(_) => {
                    if !self.is_on_dialogue {
                        self.attribute.is_hovered = false;
                    }
                }

                _ => {}
            }
        }

        self.update_is_clicked();
        self.update_is_pressed();

        if let Some(event) = &mut self.attribute.on_click_event {
            if self.attribute.is_clicked {
                if let Some(n) = event() {
                    return Some(n);
                } else {
                    return None;
                }
            }
        }

        return None;
    }

    fn draw(&self) {
        let (draw_x, draw_y, current_shadow) = if self.attribute.is_pressed {
            (
                (self.position.x + self.parent.x) + self.attribute.shadow_offset,
                (self.position.y + self.parent.y) + self.attribute.shadow_offset,
                0.0,
            )
        } else {
            (
                self.position.x + self.parent.x,
                self.position.y + self.parent.y,
                self.attribute.shadow_offset,
            )
        };

        if current_shadow > 0.0 {
            draw_rectangle_extended(
                (self.position.x + self.parent.x) + self.attribute.shadow_offset,
                (self.position.y + self.parent.y) + self.attribute.shadow_offset,
                self.dimension.width,
                self.dimension.height,
                self.attribute.corner_radius,
                Color::new(0.1, 0.1, 0.1, 0.25),
                Color::new(0.1, 0.1, 0.1, 0.25),
                self.attribute.background_color.angle,
                0.0,
                BLACK,
            );
        }

        //     if self.is_hovered {
        //     Color::new(0.12, 0.53, 0.90, 1.0)
        // } else {
        //     Color::new(0.07, 0.45, 0.80, 1.0)
        // };

        let text = &self.text;
        let font_size = self.attribute.text_size;
        let text_dimensions = measure_text(text, Some(&self.attribute.font), font_size as u16, 1.0);

        let net_width: f32 = self.dimension.width;
        let net_height: f32 = self.dimension.height;

        draw_rectangle_extended(
            draw_x,
            draw_y,
            net_width,
            net_height,
            self.attribute.corner_radius,
            self.attribute.background_color.colors[0],
            self.attribute.background_color.colors[1],
            self.attribute.background_color.angle,
            self.attribute.outline_thickness,
            self.attribute.outline_color,
        );

        let text_x = draw_x + (net_width / 2.0) - (text_dimensions.width / 2.0);
        let text_y =
            (draw_y + text_dimensions.height) + (net_height / 2.0) - (text_dimensions.height / 2.0);

        draw_text_ex(
            text,
            text_x,
            text_y,
            TextParams {
                font: Some(&self.attribute.font),
                font_size: self.attribute.text_size as u16,
                color: self.attribute.text_color,
                ..Default::default()
            },
        );
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

    fn set_position_ref(&mut self, value: ObjectPosition) {
        self.position = value;
    }

    fn set_parent_state_ref(&mut self, value: ParentState) {
        self.parent = value;
    }
}

impl Button for RegularButton {
    fn new(position: ObjectPosition, config: ButtonConfig) -> Self
    where
        Self: Sized + Object,
    {
        let text_dimensions = measure_text(&config.text, None, config.text_size as u16, 1.0);

        RegularButton {
            position: position,
            dimension: ObjectDimension {
                width: text_dimensions.width,
                height: text_dimensions.height,
                width_dyn: None,
                height_dyn: None
            },
            parent: ParentState {
                x: 0.0,
                y: 0.0,
                height: 0.0,
                width: 0.0,
            },
            attribute: ButtonAttribute {
                outline_thickness: config.outline,
                outline_color: config.outline_color,
                background_color: config.background_color,
                corner_radius: config.radius,
                text_color: config.text_color,
                on_click_event: None,
                is_clicked: false,
                is_hovered: false,
                is_pressed: false,
                shadow_offset: 12.0,
                text_size: config.text_size,
                font: config.font,
            },
            text: config.text.to_uppercase(),
            is_on_dialogue: false
        }
    }

    

    fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() -> Option<State> + 'static,
    {
        self.attribute.on_click_event = Some(Box::new(callback));
        return self;
    }
}

impl Hoverable for RegularButton {
    fn set_is_hovered(&mut self, value: bool) {
        self.attribute.is_hovered = value;
    }

    fn get_is_hovered(&self) -> bool {
        return self.attribute.is_hovered
    }
}

impl Clickable for RegularButton {
    fn set_is_clicked(&mut self, value: bool) {
        self.attribute.is_clicked = value;
    }
}

impl Pressable for RegularButton {
    fn set_is_pressed(&mut self, value: bool) {
        self.attribute.is_pressed = value;
    }
}