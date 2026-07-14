use macroquad::{color::Color, shapes::draw_rectangle};

use crate::{
    state::State,
    ui::{
        config::{dimension::ObjectDimension, parent::ParentState, position::ObjectPosition},
        traits::object::Object,
        widgets::container::Display::{Flex, Grid},
    },
};

pub enum Display {
    Flex(Direction),
    Grid(Direction),
}

pub enum Direction {
    X,
    Y,
}

pub struct Container {
    pub position: ObjectPosition,
    pub dimension: ObjectDimension,
    pub parent: ParentState,
    display: Option<Display>,
    gap: f32,
    padding_t: f32,
    padding_r: f32,
    padding_b: f32,
    padding_l: f32,
    pub objects: Vec<Box<dyn Object + Sync + Send>>,
    background_color: Option<Color>,
}

impl Container {
    pub fn new(
        position: ObjectPosition,
        dimension: ObjectDimension,
        parent: ParentState,
        color: Option<Color>,
    ) -> Container {
        return Container {
            position,
            dimension,
            objects: Vec::new(),
            parent,
            padding_t: 0.0,
            padding_r: 0.0,
            padding_b: 0.0,
            padding_l: 0.0,
            display: None,
            gap: 0.0,
            background_color: color,
        };
    }

    pub fn add_child(mut self, object: Box<dyn Object + Sync + Send>) -> Container {
        self.objects.push(object);
        self
    }

    pub fn add_child_ref(&mut self, object: Box<dyn Object + Sync + Send>) {
        self.objects.push(object);
    }

    pub fn set_is_flex(mut self, direction: Direction, gap: f32) -> Container {
        self.display = Some(Flex(direction));
        self.gap = gap;

        return self;
    }

    pub fn set_is_grid(mut self, direction: Direction, gap: f32) -> Container {
        self.display = Some(Grid(direction));
        self.gap = gap;

        return self;
    }

    pub fn set_padding(mut self, padding_x: f32, padding_y: f32) -> Self {
        self.padding_t = padding_y;
        self.padding_r = padding_x;
        self.padding_b = padding_y;
        self.padding_l = padding_x;

        self
    }

    pub fn set_padding_all(mut self, padding_t: f32, padding_r: f32, padding_b: f32,padding_l: f32) -> Self {
        self.padding_b = padding_b;
        self.padding_l = padding_l;
        self.padding_r = padding_r;
        self.padding_t = padding_t;

        self
    }

    pub fn set_is_flex_ref(&mut self, direction: Direction, gap: f32) {
        self.display = Some(Flex(direction));
        self.gap = gap;
    }

    pub fn set_is_grid_ref(&mut self, direction: Direction, gap: f32) {
        self.display = Some(Grid(direction));
        self.gap = gap;
    }
}

impl Object for Container {
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
        state: &Option<State>,
    ) -> Option<State> {
        self.update_parent_state(parent_x, parent_y, parent_w, parent_h);
        self.update_dimension();
        self.update_alignment();

        if let Some(display) = &self.display {
            let child_number = self.objects.len();
            match display {
                Flex(dir) => match dir {
                    Direction::X => {
                        let mut before_width: f32 = 0.0;

                        for i in &mut self.objects {
                            let child_dimension = i.get_dimension();
                            let child_position = i.get_position();

                            i.set_position_ref(ObjectPosition {
                                x: before_width,
                                ..child_position
                            });

                            before_width += child_dimension.width + self.gap;
                        }
                    }
                    Direction::Y => {
                        let mut before_height: f32 = 0.0;

                        for i in &mut self.objects {
                            let child_dimension = i.get_dimension();
                            let child_position = i.get_position();

                            i.set_position_ref(ObjectPosition {
                                y: before_height,
                                ..child_position
                            });

                            before_height += child_dimension.height + self.gap;
                        }
                    }
                },
                Grid(dir) => match dir {
                    Direction::X => {
                        let net_width =
                            self.dimension.width - (self.gap * (child_number - 1) as f32);
                        let child_net_width = 1.0 / child_number as f32 * net_width;
                        let mut counter: f32 = 0.0;

                        for i in &mut self.objects {
                            let child_dimension = i.get_dimension();
                            let child_position = i.get_position();

                            i.set_dimension_ref(ObjectDimension {
                                width: child_net_width,
                                ..child_dimension
                            });
                            i.set_position_ref(ObjectPosition {
                                x: counter * child_net_width + (self.gap * counter),
                                ..child_position
                            });

                            counter += 1.0;
                        }
                    }
                    Direction::Y => {
                        let net_height =
                            self.dimension.height - (self.gap * (child_number - 1) as f32);
                        let child_net_height = 1.0 / child_number as f32 * net_height;
                        let mut counter: f32 = 0.0;

                        for i in &mut self.objects {
                            let child_dimension = i.get_dimension();
                            let child_position = i.get_position();

                            i.set_dimension_ref(ObjectDimension {
                                height: child_net_height,
                                ..child_dimension
                            });
                            i.set_position_ref(ObjectPosition {
                                y: counter * child_net_height + (self.gap * counter),
                                ..child_position
                            });

                            counter += 1.0;
                        }
                    }
                },
            }
        }

        for i in &mut self.objects {
            if let Some(n) = i.update(
                Some(self.position.x + self.parent.x + self.padding_l),
                Some(self.position.y + self.parent.y + self.padding_t),
                Some(self.dimension.width - (self.padding_r + self.padding_l)),
                Some(self.dimension.height - (self.padding_b + self.padding_t)),
                state,
            ) {
                return Some(n);
            }
        }

        return None;
    }

    fn draw(&self) {
        if let Some(color) = self.background_color {
            draw_rectangle(
                self.position.x + self.parent.x,
                self.position.y + self.parent.y,
                self.dimension.width,
                self.dimension.height,
                color,
            );
        }
        for i in &self.objects {
            i.draw();
        }
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
