use macroquad::color::{BLANK, Color};

use crate::{
    state::State::{self, ConfigInput},
    ui::{
        config::{
            dimension::ObjectDimension,
            gradient::Gradient,
            parent::ParentState,
            position::{DynamicPosition, ObjectPosition},
        },
        traits::{click::Clickable, hover::Hoverable, object::Object},
        widgets::rectangle::{Rectangle, RectangleConfig},
    },
};

#[derive(Clone)]
pub enum SwitchButtonId {
    RoomConfig(RoomConfigSwitchId),
}

#[derive(Clone, Debug)]
pub enum RoomConfigSwitchId {
    AllowCourtStacking(bool),
    FreeHit(bool),
    AllowRailing(bool),
    WithJoker(bool),
    HitterScoring(bool),
    AllowClosing(bool),
}

struct SwitchButtonState {
    is_on: bool,
    is_disabled: bool,
    is_hovered: bool,
    is_pressed: bool,
    is_clicked: bool,
}

impl SwitchButtonState {
    pub fn new() -> Self {
        SwitchButtonState {
            is_on: false,
            is_disabled: false,
            is_hovered: false,
            is_pressed: false,
            is_clicked: false,
        }
    }
}

struct SwitchButtonComponents {
    background: Rectangle,
    switch: Rectangle,
}

pub struct SwitchButton {
    id: Option<SwitchButtonId>,
    components: SwitchButtonComponents,
    state: SwitchButtonState,
}

impl SwitchButton {
    pub fn new(position: ObjectPosition, width: f32) -> Self {
        let background = Rectangle::new(
            position,
            ObjectDimension::absolute(width, width / 2.0),
            ParentState::new(),
            RectangleConfig::new(
                width,
                Gradient::new(0.0, vec![Color::from_hex(0x252730)]),
                5.0,
                Color::from_hex(0x2f313d),
            ),
        );

        let switch = Rectangle::new(
            ObjectPosition::dynamic(DynamicPosition::Start, DynamicPosition::Center),
            ObjectDimension::absolute(width / 2.0, width / 2.0),
            ParentState::new(),
            RectangleConfig::new(width, Gradient::gray(), 0.0, BLANK),
        );
        SwitchButton {
            id: None,
            components: SwitchButtonComponents { background, switch },
            state: SwitchButtonState::new(),
        }
    }

    pub fn set_id(mut self, id: SwitchButtonId) -> Self {
        self.id = Some(id);
        self
    }
}

unsafe impl Sync for SwitchButton {}
unsafe impl Send for SwitchButton {}

impl Object for SwitchButton {
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
        state: &Option<crate::state::State>,
    ) -> Option<crate::state::State> {
        self.update_parent_state(parent_x, parent_y, parent_w, parent_h);
        self.update_dimension();
        self.update_alignment();

        if !self.state.is_disabled {
            self.update_hover();
        }

        self.update_is_clicked();

        if self.state.is_on {
            self.components.switch.position.x_dyn = Some(DynamicPosition::End);
            if self.state.is_disabled {
                self.components.switch.config.color = Gradient::primary_with_opacity(0.5);
            } else {
                self.components.switch.config.color = Gradient::primary();
            }
        } else {
            self.components.switch.position.x_dyn = Some(DynamicPosition::Start);
            if self.state.is_disabled {
                self.components.switch.config.color = Gradient::gray_with_opacity(0.5);
            } else {
                self.components.switch.config.color = Gradient::gray();
            }
        }

        if self.state.is_clicked {
            self.state.is_on = !self.state.is_on;

            match self.id.as_ref().unwrap() {
                SwitchButtonId::RoomConfig(config) => match config {
                    RoomConfigSwitchId::AllowClosing(_) => {
                        return Some(ConfigInput(RoomConfigSwitchId::AllowClosing(
                            self.state.is_on,
                        )));
                    }

                    RoomConfigSwitchId::AllowCourtStacking(_) => {
                        return Some(ConfigInput(RoomConfigSwitchId::AllowCourtStacking(
                            self.state.is_on,
                        )));
                    }

                    RoomConfigSwitchId::AllowRailing(_) => {
                        return Some(ConfigInput(RoomConfigSwitchId::AllowRailing(
                            self.state.is_on,
                        )));
                    }

                    RoomConfigSwitchId::FreeHit(_) => {
                        return Some(ConfigInput(RoomConfigSwitchId::FreeHit(self.state.is_on)));
                    }

                    RoomConfigSwitchId::HitterScoring(_) => {
                        return Some(ConfigInput(RoomConfigSwitchId::HitterScoring(
                            self.state.is_on,
                        )));
                    }

                    RoomConfigSwitchId::WithJoker(_) => {
                        return Some(ConfigInput(RoomConfigSwitchId::WithJoker(self.state.is_on)));
                    }
                },
            }
        }

        self.components.switch.update(
            Some(self.components.background.position.x + self.components.background.parent.x),
            Some(self.components.background.position.y + self.components.background.parent.y),
            Some(self.components.background.dimension.width),
            Some(self.components.background.dimension.height),
            state,
        );

        let Some(state) = state else {
            return None;
        };

        match state {
            State::ConfigUpdate(new_config) => {
                match self.id.as_ref().unwrap() {
                    SwitchButtonId::RoomConfig(config_type) => match config_type {
                        RoomConfigSwitchId::AllowClosing(_) => {
                            self.state.is_on = new_config.allow_closing;
                        }
                        RoomConfigSwitchId::AllowCourtStacking(_) => {
                            self.state.is_on = new_config.allow_court_stacking;
                        }
                        RoomConfigSwitchId::AllowRailing(_) => {
                            self.state.is_on = new_config.allow_railing;
                        }
                        RoomConfigSwitchId::HitterScoring(_) => {
                            self.state.is_on = new_config.hitter_scoring;
                        }
                        RoomConfigSwitchId::FreeHit(_) => {
                            self.state.is_on = new_config.free_hit;
                        }
                        RoomConfigSwitchId::WithJoker(_) => {
                            self.state.is_on = new_config.with_joker;
                        }
                    },
                }

                if let Some(SwitchButtonId::RoomConfig(RoomConfigSwitchId::WithJoker(_))) = self.id
                {
                    return Some(State::Reset);
                }
            }

            State::RoomPlayers {
                players: _,
                is_host,
            } => {
                if !is_host {
                    self.state.is_disabled = true;

                    if let Some(SwitchButtonId::RoomConfig(RoomConfigSwitchId::WithJoker(_))) =
                        self.id
                    {
                        return Some(State::Reset);
                    }
                }
            }

            _ => {}
        }

        return None;
    }

    fn draw(&self) {
        self.components.background.draw();
        self.components.switch.draw();
    }

    fn get_dimension(&self) -> ObjectDimension {
        self.components.background.dimension.clone()
    }

    fn get_parent_state(&self) -> ParentState {
        self.components.background.parent.clone()
    }

    fn get_position(&self) -> ObjectPosition {
        self.components.background.position.clone()
    }

    fn set_dimension_ref(&mut self, value: ObjectDimension) {
        self.components.background.dimension = value;
    }

    fn set_parent_state_ref(&mut self, value: ParentState) {
        self.components.background.parent = value;
    }

    fn set_position_ref(&mut self, value: ObjectPosition) {
        self.components.background.position = value;
    }
}

impl Hoverable for SwitchButton {
    fn get_is_hovered(&self) -> bool {
        self.state.is_hovered
    }

    fn set_is_hovered(&mut self, value: bool) {
        self.state.is_hovered = value;
    }
}

impl Clickable for SwitchButton {
    fn set_is_clicked(&mut self, value: bool) {
        self.state.is_clicked = value;
    }
}
