use std::sync::Arc;

use macroquad::color::{BLACK, BLANK, Color, GREEN, WHITE};

use crate::{
    state::State,
    ui::{
        config::{
            dimension::{DynamicDimension, ObjectDimension},
            font::Nunito,
            gradient::Gradient,
            parent::ParentState,
            position::{DynamicPosition, ObjectPosition},
        },
        traits::object::Object,
        widgets::{
            button::{Button, ButtonId, regular_button::RegularButton},
            container::{Container, Direction},
            dialogue_box::{DialogueBoxComponents, DialogueBoxState},
            rectangle::{Rectangle, RectangleConfig},
            switch_button::SwitchButton,
            text::{HEADING_5, Text, TextConfig},
        },
    },
};

pub struct RoomConfigDialogueBox {
    id: u8,
    parent: ParentState,
    state: DialogueBoxState,
    components: DialogueBoxComponents,
    room_config_components: RoomConfigDialogueBoxComponents,
}

pub struct RoomConfigDialogueBoxComponents {
    allow_court_stacking_switch: SwitchButton,
    free_hit_switch: SwitchButton,
    allow_railing_switch: SwitchButton,
    hitter_scoring_switch: SwitchButton,
    allow_closing_switch: SwitchButton,

    top_container: Container,
    left_container: Container,
    right_container: Container,
    btn_container: Container,

    apply_config_btn: RegularButton,
    cancel_btn: RegularButton,

    allow_court_stacking_container: Container,
    free_hit_container: Container,
    allow_railing_container: Container,
    hitter_scoring_container: Container,
    allow_closing_container: Container,
}

impl RoomConfigDialogueBox {
    pub fn new(
        position: ObjectPosition,
        dimension: ObjectDimension,
        bg_config: RectangleConfig,
        font: Arc<Nunito>,
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
            ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Full),
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

        let header = Text::new("Konfigurasi", font.clone()).set_position(ObjectPosition::dynamic(
            DynamicPosition::Center,
            DynamicPosition::Start,
        ));

        let top_container = Container::new(
            ObjectPosition::dynamic(DynamicPosition::Center, DynamicPosition::Start),
            ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Percent(15.0)),
            ParentState::new(),
            None,
        )
        .add_child(Box::new(header));

        let allow_railing = load_config_option_switch("Boleh nge-rail", font.clone());
        let allow_court_stacking = load_config_option_switch("Boleh tumpuk londo", font.clone());
        let free_hit = load_config_option_switch("Pukulan bebas", font.clone());
        let hitter_scoring = load_config_option_switch("Skor pemukul", font.clone());

        let left_container = Container::new(
            ObjectPosition::dynamic(
                DynamicPosition::Start,
                DynamicPosition::Custom(Arc::new(|_, _, _, ph| ph * 0.15)),
            ),
            ObjectDimension::dynamic(
                DynamicDimension::Percent(50.0),
                DynamicDimension::Percent(75.0),
            ),
            ParentState::new(),
            None,
        )
        .set_is_flex(Direction::Y, 25.0)
        .set_padding_all(0.0, 20.0, 0.0, 0.0);

        let allow_closing = load_config_option_switch("Boleh nutup", font.clone());

        let right_container = Container::new(
            ObjectPosition::dynamic(
                DynamicPosition::End,
                DynamicPosition::Custom(Arc::new(|_, _, _, ph| ph * 0.15)),
            ),
            ObjectDimension::dynamic(
                DynamicDimension::Percent(50.0),
                DynamicDimension::Percent(75.0),
            ),
            ParentState::new(),
            None,
        )
        .set_is_flex(Direction::Y, 25.0)
        .set_is_flex(Direction::Y, 25.0)
        .set_padding_all(0.0, 0.0, 0.0, 20.0);

        let apply_btn = RegularButton::new(
            ObjectPosition::dynamic(DynamicPosition::Grid, DynamicPosition::Center),
            Some(ObjectDimension::dynamic(
                DynamicDimension::Grid,
                DynamicDimension::Full,
            )),
            "Terapkan",
            TextConfig::default(font.clone()),
            RectangleConfig::new(5.0, Gradient::primary(), 0.0, BLANK),
            6.0,
            font.clone(),
        )
        .set_is_on_dialogue(3)
        .on_click(|| return Some(State::ApplyConfig))
        .set_id(ButtonId::ApplyRoomConfig);

        let cancel_btn = RegularButton::new(
            ObjectPosition::dynamic(DynamicPosition::Grid, DynamicPosition::Center),
            Some(ObjectDimension::dynamic(
                DynamicDimension::Grid,
                DynamicDimension::Full,
            )),
            "Batal",
            TextConfig::default(font.clone()),
            RectangleConfig::new(5.0, Gradient::gray(), 0.0, BLANK),
            6.0,
            font.clone(),
        )
        .set_is_on_dialogue(3)
        .on_click(|| return Some(State::CloseDialogueBox(3)));

        let btn_container = Container::new(
            ObjectPosition::dynamic(DynamicPosition::End, DynamicPosition::End),
            ObjectDimension::dynamic(DynamicDimension::Full, DynamicDimension::Percent(15.0)),
            ParentState::new(),
            None,
        )
        .set_is_grid(Direction::X, 15.0);

        Self {
            parent: ParentState::new(),
            state: DialogueBoxState::new(),
            id,
            components: DialogueBoxComponents {
                container,
                marginer,
                dim,
                background,
            },
            room_config_components: RoomConfigDialogueBoxComponents {
                allow_court_stacking_switch: allow_court_stacking.0,
                free_hit_switch: free_hit.0,
                allow_railing_switch: allow_railing.0,
                hitter_scoring_switch: hitter_scoring.0,
                allow_closing_switch: allow_closing.0,

                top_container,
                left_container,
                right_container,
                btn_container,

                apply_config_btn: apply_btn,
                cancel_btn,

                allow_closing_container: allow_closing.1,
                allow_court_stacking_container: allow_court_stacking.1,
                free_hit_container: free_hit.1,
                allow_railing_container: allow_railing.1,
                hitter_scoring_container: hitter_scoring.1,
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

impl Object for RoomConfigDialogueBox {
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

    fn update(&mut self, parent_state: ParentState, state: &Option<State>) -> Option<State> {
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

        self.components.container.update(parent_state, state);
        self.components.dim.update(ParentState::root(), state);
        self.components
            .background
            .update(self.components.container.as_parent_state(), state);

        self.room_config_components
            .allow_court_stacking_switch
            .update(
                self.room_config_components
                    .allow_court_stacking_container
                    .as_parent_state(),
                state,
            );

        self.room_config_components.allow_closing_switch.update(
            self.room_config_components
                .allow_closing_container
                .as_parent_state(),
            state,
        );

        self.room_config_components.free_hit_switch.update(
            self.room_config_components
                .free_hit_container
                .as_parent_state(),
            state,
        );

        self.room_config_components.allow_railing_switch.update(
            self.room_config_components
                .allow_railing_container
                .as_parent_state(),
            state,
        );

        self.room_config_components.hitter_scoring_switch.update(
            self.room_config_components
                .hitter_scoring_container
                .as_parent_state(),
            state,
        );

        self.room_config_components
            .allow_court_stacking_container
            .update(
                self.room_config_components.left_container.as_parent_state(),
                state,
            );

        self.room_config_components.allow_closing_container.update(
            self.room_config_components.left_container.as_parent_state(),
            state,
        );

        self.room_config_components.free_hit_container.update(
            self.room_config_components.left_container.as_parent_state(),
            state,
        );

        self.room_config_components.allow_railing_container.update(
            self.room_config_components.left_container.as_parent_state(),
            state,
        );

        self.room_config_components.hitter_scoring_container.update(
            self.room_config_components
                .right_container
                .as_parent_state(),
            state,
        );

        self.room_config_components
            .left_container
            .update(self.components.marginer.as_parent_state(), state);

        self.room_config_components
            .right_container
            .update(self.components.marginer.as_parent_state(), state);

        self.room_config_components
            .top_container
            .update(self.components.marginer.as_parent_state(), state);

        self.room_config_components.apply_config_btn.update(
            self.room_config_components.btn_container.as_parent_state(),
            state,
        );

        self.room_config_components.cancel_btn.update(
            self.room_config_components.btn_container.as_parent_state(),
            state,
        );

        self.room_config_components
            .btn_container
            .update(self.components.marginer.as_parent_state(), state);

        if let Some(value) = self
            .components
            .marginer
            .update(self.components.container.as_parent_state(), state)
        {
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

fn load_config_option_switch(text: &str, font: Arc<Nunito>) -> (SwitchButton, Container) {
    let switch = SwitchButton::new(
        ObjectPosition::new(
            0.0,
            0.0,
            Some(DynamicPosition::End),
            Some(DynamicPosition::Center),
        ),
        50.0,
    );
    let desc = Text::new(text, font.clone())
        .set_config(TextConfig::new(font.regular.clone(), WHITE, HEADING_5))
        .set_position(ObjectPosition::dynamic(
            DynamicPosition::Start,
            DynamicPosition::Center,
        ));

    let container = Container::new(
        ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::Grid),
        ObjectDimension::new(0.0, 50.0, Some(DynamicDimension::Full), None),
        ParentState::new(),
        None,
    )
    .add_child(Box::new(desc));

    (switch, container)
}
