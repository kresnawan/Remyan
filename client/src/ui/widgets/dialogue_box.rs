use std::sync::Arc;

use macroquad::{color::{BLACK, Color, GREEN}, window::{screen_height, screen_width}};

use crate::{state::State, ui::{config::{dimension::{DynamicDimension, ObjectDimension}, gradient::Gradient, parent::ParentState, position::{DynamicPosition, ObjectPosition}}, traits::object::Object, widgets::{container::Container, rectangle::{Rectangle, RectangleConfig}}}};

pub struct DialogueBoxState {
    is_shown: bool,
    background_color: Gradient,
    outline_thickness: f32,
    outline_color: Color,
}

impl DialogueBoxState {
    pub fn new(bg_color: Gradient, outline_thickness: f32, outline_color: Color) -> Self {
        DialogueBoxState {
            is_shown: false,
            background_color: bg_color,
            outline_color,
            outline_thickness,
        }
    }
}

pub struct DialogueBox {
    id: u8,
    position: ObjectPosition,
    dimension: ObjectDimension,
    parent: ParentState,
    state: DialogueBoxState,
    container: Container,
}

impl DialogueBox {
    pub fn new(
        position: ObjectPosition,
        dimension: ObjectDimension,
        state: DialogueBoxState,
        id: u8
    ) -> Self {
        let mut cont = Container::new(position, dimension.clone(), ParentState::new(), Some(GREEN));
        let marginer = Container::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Center),
            ObjectDimension::dynamic(
                DynamicDimension::Custom(Arc::new(|_, _, pw, _| pw - 80.0)),
                DynamicDimension::Custom(Arc::new(|_, _, _, ph| ph - 80.0)),
            ),
            ParentState::new(),
            None,
        );

        let bg_dim = Rectangle::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Center),
            ObjectDimension::dynamic(
                DynamicDimension::Custom(Arc::new(|_, _, _, _| screen_width())),
                DynamicDimension::Custom(Arc::new(|_, _, _, _| screen_height())),
            ),
            ParentState::new(),
            RectangleConfig::new(
                0.0,
                Gradient::new(0.0, vec![Color::from_rgba(0, 0, 0, 127)]),
                0.0,
                BLACK,
            ),
        );

        let d_bg = Rectangle::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Center),
            ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Full),
            ParentState::new(),
            RectangleConfig::new(
                5.0,
                state.background_color.clone(),
                state.outline_thickness,
                state.outline_color,
            ),
        );

        cont.add_child_ref(Box::new(bg_dim));
        cont.add_child_ref(Box::new(d_bg));
        cont.add_child_ref(Box::new(marginer));

        DialogueBox {
            position: ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Center),
            dimension,
            parent: ParentState::new(),
            state: state,
            container: cont,
            id
        }
    }

    pub fn show(&mut self) {
        self.state.is_shown = true;
    }

    pub fn add_object_ref(&mut self, object: Box<dyn Object>) {
        let a = self.container.objects.get_mut(2);
        if let Some(container) = a {
            let casted: &mut Container = container.as_any_mut().downcast_mut().unwrap();
            casted.add_child_ref(object);
        }
    }
}

impl Object for DialogueBox {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn draw(&self) {
        if self.state.is_shown {
            self.container.draw();
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

        if let Some(n) = state {
            if let &State::OpenDialogueBox(id) = n {
                if self.id == id {
                    self.state.is_shown = true;
                }
            }

            if let &State::CloseDialogueBox(id) = n {
                if self.id == id {
                    self.state.is_shown = false;
                }
            }
        }

        if let Some(value) = self
            .container
            .update(parent_x, parent_y, parent_w, parent_h, state)
        {
            return Some(value);
        }

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
