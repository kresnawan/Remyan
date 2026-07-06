use macroquad::prelude::*;

use crate::ui::{
    Object, XAlignment, YAlignment,
    button::{Button, ButtonConfig},
    config::position::PositionConfig,
    draw::draw_rectangle_extended,
    gradient::Gradient,
};

pub struct RegularButton {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    shadow_offset: f32,
    text: String,
    text_size: f32,
    font: Font,
    background_color: Gradient,
    radius: f32,
    text_color: Color,
    parent_x: f32,
    parent_y: f32,
    x_alignment: Option<XAlignment>,
    y_alignment: Option<YAlignment>,
    on_click_event: Option<Box<dyn Fn() -> Option<usize> + 'static>>,
    is_pressed: bool,
    is_hovered: bool,
    is_clicked: bool,
}

impl RegularButton {
    pub fn set_padding(mut self, x: f32, y: f32) -> Self {
        self.width += x * 2.0;
        self.height += y * 2.0;
        return self;
    }

    pub fn set_alignment(mut self, x: Option<XAlignment>, y: Option<YAlignment>) -> Self {
        self.x_alignment = x;
        self.y_alignment = y;

        return self;
    }

    pub fn set_dimensions(mut self, width: f32, height: f32) -> Self {
        if width > 0.0 {
            self.width = width;
        }
        if height > 0.0 {
            self.height = height;
        }

        return self;
    }
}

impl Object for RegularButton {
    fn update(
        &mut self,
        parent_x: Option<f32>,
        parent_y: Option<f32>,
        parent_w: Option<f32>,
        parent_h: Option<f32>,
    ) -> Option<usize> {
        if let Some(value) = parent_x {
            self.parent_x = value;
        }

        if let Some(value) = parent_y {
            self.parent_y = value;
        }

        let (mouse_x, mouse_y) = mouse_position();
        let btn_x = self.x + self.parent_x;
        let btn_y = self.y + self.parent_y;
        let btn_w = self.width;
        let btn_h = self.height;
        let is_hovered = mouse_x >= btn_x
            && mouse_x <= btn_x + btn_w
            && mouse_y >= btn_y
            && mouse_y <= btn_y + btn_h;

        let is_pressed = is_hovered && is_mouse_button_down(MouseButton::Left);
        let is_clicked = is_hovered && is_mouse_button_released(MouseButton::Left);

        self.is_hovered = is_hovered;
        self.is_pressed = is_pressed;
        self.is_clicked = is_clicked;

        if let Some(value) = &self.x_alignment {
            match value {
                XAlignment::Left => {
                    let parent_x_a = if let Some(value) = parent_x {
                        value
                    } else {
                        0.0
                    };

                    self.x = parent_x_a;
                }
                XAlignment::Center => {
                    let parent_w_a = if let Some(value) = parent_w {
                        value
                    } else {
                        screen_width()
                    };
                    self.x = parent_w_a / 2.0 - self.width / 2.0;
                }
                XAlignment::Right => {
                    let parent_w_a = if let Some(value) = parent_w {
                        value
                    } else {
                        screen_width()
                    };

                    self.x = parent_w_a - self.width;
                }
            }
        }

        if let Some(value) = &self.y_alignment {
            match value {
                YAlignment::Top => {
                    let parent_y_a = if let Some(value) = parent_x {
                        value
                    } else {
                        0.0
                    };

                    self.y = parent_y_a;
                }
                YAlignment::Center => {
                    let parent_h_a = if let Some(value) = parent_h {
                        value
                    } else {
                        screen_height()
                    };
                    self.y = parent_h_a / 2.0 - self.height / 2.0;
                }
                YAlignment::Bottom => {
                    let parent_h_a = if let Some(value) = parent_h {
                        value
                    } else {
                        screen_height()
                    };

                    self.x = parent_h_a - self.height;
                }
            }
        }

        if let Some(event) = &self.on_click_event {
            if self.is_clicked {
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
        let (draw_x, draw_y, current_shadow) = if self.is_pressed {
            (
                (self.x + self.parent_x) + self.shadow_offset,
                (self.y + self.parent_y) + self.shadow_offset,
                0.0,
            )
        } else {
            (
                self.x + self.parent_x,
                self.y + self.parent_y,
                self.shadow_offset,
            )
        };

        if current_shadow > 0.0 {
            draw_rectangle(
                (self.x + self.parent_x) + self.shadow_offset,
                (self.y + self.parent_y) + self.shadow_offset,
                self.width,
                self.height,
                Color::new(0.1, 0.1, 0.1, 0.25),
            );
        }

        //     if self.is_hovered {
        //     Color::new(0.12, 0.53, 0.90, 1.0)
        // } else {
        //     Color::new(0.07, 0.45, 0.80, 1.0)
        // };

        let text = &self.text;
        let font_size = self.text_size;
        let text_dimensions = measure_text(text, Some(&self.font), font_size as u16, 1.0);

        let net_width: f32 = self.width;
        let net_height: f32 = self.height;

        draw_rectangle_extended(
            draw_x,
            draw_y,
            net_width,
            net_height,
            self.radius,
            self.background_color.colors[0],
            self.background_color.colors[1],
            self.background_color.angle,
        );

        let text_x = draw_x + (net_width / 2.0) - (text_dimensions.width / 2.0);
        let text_y =
            (draw_y + text_dimensions.height) + (net_height / 2.0) - (text_dimensions.height / 2.0);

        draw_text_ex(
            text,
            text_x,
            text_y,
            TextParams {
                font: Some(&self.font),
                font_size: self.text_size as u16,
                color: self.text_color,
                ..Default::default()
            },
        );
    }
}

impl Button for RegularButton {
    fn new<T>(position: T, config: ButtonConfig) -> Self
    where
        T: PositionConfig,
        Self: Sized,
    {
        let text_dimensions = measure_text(&config.text, None, config.text_size as u16, 1.0);

        RegularButton {
            x: position.get_x(),
            y: position.get_y(),
            width: text_dimensions.width,
            height: text_dimensions.height,
            shadow_offset: 6.0,
            text: config.text,
            text_size: config.text_size,
            font: config.font,
            x_alignment: None,
            y_alignment: None,
            parent_x: 0.0,
            parent_y: 0.0,
            background_color: config.background_color,
            radius: config.radius,
            text_color: config.text_color,
            on_click_event: None,
            is_clicked: false,
            is_hovered: false,
            is_pressed: false,
        }
    }

    fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() -> Option<usize> + 'static,
    {
        self.on_click_event = Some(Box::new(callback));
        return self;
    }
}
