use macroquad::prelude::*;

use crate::ui::{
    Object,
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
    is_center_x: bool,
    is_center_y: bool,
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

    pub fn to_center(mut self) -> Self {
        self.is_center_x = true;
        self.is_center_y = true;

        return self;
    }

    pub fn to_center_x(mut self) -> Self {
        self.is_center_x = true;

        return self;
    }

    pub fn to_center_y(mut self) -> Self {
        self.is_center_y = true;

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
    fn update(&mut self) -> Option<usize> {
        let (mouse_x, mouse_y) = mouse_position();
        let btn_x = self.x;
        let btn_y = self.y;
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

        if self.is_center_x {
            self.x = screen_width() / 2.0 - self.width / 2.0;
        }

        if self.is_center_y {
            self.y = screen_height() / 2.0 - self.height / 2.0;
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
                self.x + self.shadow_offset,
                self.y + self.shadow_offset,
                0.0,
            )
        } else {
            (self.x, self.y, self.shadow_offset)
        };

        if current_shadow > 0.0 {
            draw_rectangle(
                self.x + self.shadow_offset,
                self.y + self.shadow_offset,
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
            is_center_x: false,
            is_center_y: false,
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
