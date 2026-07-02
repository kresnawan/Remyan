use macroquad::prelude::*;

pub struct RegularButton {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    shadow_offset: f32,
    text: String,
    on_click_event: Option<Box<dyn Fn() + 'static>>,
    is_pressed: bool,
    is_hovered: bool,
    is_clicked: bool
}

impl Object for RegularButton {
    fn new<T, U>(position: T, dimension: U) -> Self
    where
        T: PositionConfig,
        U: DimensionConfig,
        Self: Sized,
    {
        RegularButton {
            x: position.get_x(),
            y: position.get_y(),
            width: dimension.get_width(),
            height: dimension.get_height(),
            shadow_offset: 6.0,
            text: String::from("Click me"),
            on_click_event: None,
            is_clicked: false,
            is_hovered: false,
            is_pressed: false
        }
    }

    fn update(&mut self) {
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
        let is_clicked = is_hovered && is_mouse_button_pressed(MouseButton::Left);

        self.is_hovered = is_hovered;
        self.is_pressed = is_pressed;
        self.is_clicked = is_clicked;
    }

    fn draw(&self) {
        let shadow_offset = self.shadow_offset;

        if let Some(event) = &self.on_click_event {
            if self.is_clicked {
                event();
            }
        }

        let (draw_x, draw_y, current_shadow) = if self.is_pressed {
            (self.x + shadow_offset, self.y + shadow_offset, 0.0)
        } else {
            (self.x, self.y, shadow_offset)
        };

        if current_shadow > 0.0 {
            draw_rectangle(
                self.x + shadow_offset,
                self.y + shadow_offset,
                self.width,
                self.height,
                Color::new(0.1, 0.1, 0.1, 0.25),
            );
        }

        let btn_color = if self.is_hovered {
            Color::new(0.12, 0.53, 0.90, 1.0)
        } else {
            Color::new(0.07, 0.45, 0.80, 1.0)
        };
        draw_rectangle(draw_x, draw_y, self.width, self.height, btn_color);

        let text = &self.text;
        let font_size = 24.0;
        let text_dimensions = measure_text(text, None, font_size as u16, 1.0);

        let text_x = draw_x + (self.width - text_dimensions.width) / 2.0;
        let text_y = draw_y + (self.height + text_dimensions.height) / 2.0 - 2.0;

        draw_text(text, text_x, text_y, font_size, WHITE);
    }
}

impl Button for RegularButton {
    fn on_click<F>(&mut self, callback: F)
    where
        F: Fn() -> () + 'static,
    {
        self.on_click_event = Some(Box::new(callback));
    }
}

pub trait Button {
    fn on_click<F>(&mut self, callback: F)
    where
        F: Fn() -> () + 'static;
}

pub trait Object {
    fn new<T, U>(position: T, dimension: U) -> Self
    where
        T: PositionConfig,
        U: DimensionConfig,
        Self: Sized;

    fn update(&mut self) {}
    fn draw(&self) {}
}

pub trait PositionConfig {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
}
pub trait DimensionConfig {
    fn get_width(&self) -> f32;
    fn get_height(&self) -> f32;
}

pub struct Position {
    x: f32,
    y: f32,
}

pub struct Dimension {
    width: f32,
    height: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Position {
        Position { x, y }
    }
}

impl Dimension {
    pub fn new(width: f32, height: f32) -> Dimension {
        Dimension { width, height }
    }
}

impl DimensionConfig for Dimension {
    fn get_width(&self) -> f32 {
        self.width
    }

    fn get_height(&self) -> f32 {
        self.height
    }
}

impl PositionConfig for Position {
    fn get_x(&self) -> f32 {
        return self.x;
    }

    fn get_y(&self) -> f32 {
        return self.y;
    }
}
