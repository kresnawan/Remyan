use std::sync::Arc;

use macroquad::{
    color::{BLACK, Color, GREEN}, window::{screen_height, screen_width},
};

use crate::{
    state::State,
    ui::{
        config::{
            dimension::{DynamicDimension, ObjectDimension},
            gradient::Gradient,
            parent::ParentState,
            position::{DynamicPosition, ObjectPosition},
        },
        traits::object::Object,
        widgets::{
            container::Container,
            rectangle::{Rectangle, RectangleConfig},
        },
    },
};

pub struct DialogueBoxState {
    pub is_shown: bool,
}

impl DialogueBoxState {
    pub fn new() -> Self {
        DialogueBoxState { is_shown: false }
    }
}

pub struct DialogueBoxComponents {
    pub container: Container,
    pub marginer: Container,
    pub dim: Rectangle,
    pub background: Rectangle,
}

pub struct DialogueBox {
    id: u8,
    parent: ParentState,
    state: DialogueBoxState,
    components: DialogueBoxComponents,
}

impl DialogueBox {
    pub fn new(
        position: ObjectPosition,
        dimension: ObjectDimension,
        bg_config: RectangleConfig,
        id: u8,
    ) -> Self {
        let container = Container::new(position, dimension, ParentState::new(), Some(GREEN));
        let marginer = Container::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Center),
            ObjectDimension::dynamic(
                DynamicDimension::Custom(Arc::new(|_, _, pw, _| pw - 80.0)),
                DynamicDimension::Custom(Arc::new(|_, _, _, ph| ph - 80.0)),
            ),
            ParentState::new(),
            None,
        );

        let dim = Rectangle::new(
            ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::Start),
            ObjectDimension::dynamic(
                DynamicDimension::Full,
                DynamicDimension::Full,
            ),
            ParentState::new(),
            RectangleConfig::new(
                0.0,
                Gradient::new(0.0, vec![Color::from_rgba(0, 0, 0, 127)]),
                0.0,
                BLACK,
            ),
        );

        let background = Rectangle::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Center),
            ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Full),
            ParentState::new(),
            bg_config,
        );

        DialogueBox {
            parent: ParentState::new(),
            state: DialogueBoxState::new(),
            id,
            components: DialogueBoxComponents {
                container,
                marginer,
                dim,
                background,
            },
        }
    }

    pub fn show(&mut self) {
        self.state.is_shown = true;
    }

    pub fn add_object_ref(&mut self, object: Box<dyn Object + Sync + Send>) {
        self.components.marginer.add_child_ref(object);
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
            self.components.container.draw();
            self.components.dim.draw();
            self.components.background.draw();
            self.components.marginer.draw();
        }
    }

    fn update(
        &mut self,
        parent_state: ParentState,
        state: &Option<State>,
    ) -> Option<State> {
        self.update_parent_state(parent_state.clone());
        self.update_dimension();
        self.update_alignment();

        if let Some(n) = state {
            if let &State::OpenDialogueBox(id) = n {
                if self.id == id {
                    self.state.is_shown = true;
                    return Some(State::Reset);
                }
            }

            if let &State::CloseDialogueBox(id) = n {
                if self.id == id {
                    self.state.is_shown = false;
                    return Some(State::Reset);
                }
            }
        }

        self.components
            .container
            .update(parent_state, state);
        self.components.dim.update(
            ParentState::root(),
            state,
        );
        self.components.background.update(
            self.components.container.as_parent_state(),
            state,
        );

        if let Some(value) = self.components.marginer.update(
            self.components.container.as_parent_state(),
            state,
        ) {
            return Some(value);
        }

        return None;
    }

    fn get_dimension(&self) -> ObjectDimension {
        return self.components.container.dimension.clone();
    }

    fn get_parent_state(&self) -> ParentState {
        return self.parent.clone();
    }

    fn get_position(&self) -> ObjectPosition {
        return self.components.container.position.clone();
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
