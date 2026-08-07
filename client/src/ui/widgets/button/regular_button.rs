use std::sync::Arc;

use macroquad::prelude::*;
use remyan_core::protocol::event::ServerEventPlayer;

use crate::{
    state::State,
    ui::{
        config::{
            dimension::ObjectDimension,
            font::Nunito,
            gradient::Gradient,
            parent::ParentState,
            position::{DynamicPosition::Center, ObjectPosition},
        },
        traits::{click::Clickable, hover::Hoverable, object::Object, press::Pressable},
        widgets::{
            button::{Button, ButtonId, ButtonState},
            rectangle::{Rectangle, RectangleConfig},
            text::{Text, TextConfig},
        },
    },
};

pub struct RegularButtonComponents {
    background: Rectangle,
    shadow: Rectangle,
    text: Text,
}

pub struct RegularButton {
    pub id: Option<ButtonId>,
    parent: ParentState,
    components: RegularButtonComponents,
    state: ButtonState,
    is_on_dialogue: bool,
    dialogue_id: Option<u8>,
    shadow_offset: f32,
    is_active: bool,
    is_shown: bool,
    is_disabled: bool,
}

unsafe impl Sync for RegularButton {}
unsafe impl Send for RegularButton {}

impl RegularButton {
    pub fn set_padding(mut self, x: f32, y: f32) -> Self {
        self.components.background.dimension.width += x * 2.0;
        self.components.shadow.dimension.width += x * 2.0;

        self.components.background.dimension.height += y * 2.0;
        self.components.shadow.dimension.width += x * 2.0;
        return self;
    }

    pub fn set_dimensions(mut self, value: ObjectDimension) -> Self {
        self.components.background.dimension = value.clone();
        self.components.shadow.dimension = value;
        self
    }

    pub fn set_position(mut self, value: ObjectPosition) -> Self {
        self.components.background.position = value.clone();
        self.components.shadow.position = value;
        self
    }

    pub fn set_is_on_dialogue(mut self, id: u8) -> Self {
        self.is_on_dialogue = true;
        self.dialogue_id = Some(id);
        self
    }

    pub fn set_id(mut self, id: ButtonId) -> Self {
        self.id = Some(id);
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
    fn update(&mut self, parent_state: ParentState, state: &Option<State>) -> Option<State> {
        self.update_parent_state(parent_state.clone());
        self.update_dimension();
        self.update_alignment();

        self.components
            .background
            .update(parent_state.clone(), state);
        self.components.shadow.update(parent_state.clone(), state);

        self.components.shadow.position.x =
            self.components.background.position.x + self.shadow_offset;
        self.components.shadow.position.y =
            self.components.background.position.y + self.shadow_offset;

        if self.state.is_pressed {
            self.components.background.position.x = self.components.shadow.position.x;
            self.components.background.position.y = self.components.shadow.position.y;
        }

        self.components
            .text
            .update(self.components.background.as_parent_state(), state);

        self.update_hover();

        // this is just naive workaround to disable buttons which
        // not contained in currently opened dialogue
        // box so that it's not clicked unintentionally
        //
        // it's created this way because we didn't find a way to make sure that hovered element is only
        // one on top
        //
        //
        if let Some(state) = state {
            match state {
                State::RoomPlayers {
                    players,
                    host_id,
                    self_id,
                } => {
                    if let Some(id) = &self.id {
                        match id {
                            ButtonId::ApplyRoomConfig => {
                                self.is_shown = host_id == self_id;
                            }

                            ButtonId::StartGame => {
                                let filtered: Vec<&Option<ServerEventPlayer>> =
                                    players.iter().filter(|item| item.is_some()).collect();

                                self.is_disabled = !(filtered.len() > 2 && host_id == self_id);
                                return Some(State::Reset);
                            }
                        }
                    };
                }
                State::OpenDialogueBox(id) => {
                    if !self.is_on_dialogue {
                        self.is_active = false;
                    } else {
                        if let Some(n) = self.dialogue_id {
                            if *id != n {
                                self.is_active = false;
                            }
                        }
                    }
                }

                State::CloseDialogueBox(id) => {
                    if !self.is_on_dialogue {
                        self.is_active = true;
                    } else {
                        if let Some(n) = self.dialogue_id {
                            if *id != n {
                                self.is_active = true;
                            }
                        }
                    }
                }

                _ => {}
            }
        }

        self.update_is_clicked();
        self.update_is_pressed();

        if self.state.is_hovered {
            self.components.background.config.color = Gradient::gray();
        } else {
            self.components.background.config.color = Gradient::primary();
        }

        if self.is_disabled {
            self.components.background.config.color.set_opacity_ref(0.3);
            self.components.text.config.color.a = 0.3;
            self.components.background.position.x = self.components.shadow.position.x;
            self.components.background.position.y = self.components.shadow.position.y;
        } else {
            self.components.background.config.color.set_opacity_ref(1.0);
            self.components.text.config.color.a = 1.;
        }

        if let Some(event) = &mut self.state.on_click_event {
            if self.state.is_clicked {
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
        if self.is_shown {
            self.components.shadow.draw();
            self.components.background.draw();
            self.components.text.draw();
        }
    }

    fn get_dimension(&self) -> ObjectDimension {
        return self.components.background.dimension.clone();
    }

    fn get_parent_state(&self) -> ParentState {
        return self.parent.clone();
    }

    fn get_position(&self) -> ObjectPosition {
        return self.components.background.position.clone();
    }

    fn set_dimension_ref(&mut self, value: ObjectDimension) {
        self.components.background.dimension = value.clone();
        self.components.shadow.dimension = value;
    }

    fn set_position_ref(&mut self, value: ObjectPosition) {
        self.components.background.position = value.clone();
        self.components.shadow.position = value;
    }

    fn set_parent_state_ref(&mut self, value: ParentState) {
        self.parent = value;
    }
}

impl Button for RegularButton {
    fn new(
        position: ObjectPosition,
        dimension: Option<ObjectDimension>,
        text: &str,
        text_config: TextConfig,
        background_config: RectangleConfig,
        shadow_offset: f32,
        font: Arc<Nunito>,
    ) -> Self {
        let dimension = if let Some(value) = dimension {
            value
        } else {
            let text_dimension = measure_text(
                text,
                Some(&text_config.font),
                text_config.font_size as u16,
                1.0,
            );
            ObjectDimension {
                width: text_dimension.width,
                height: text_dimension.height,
                width_dyn: None,
                height_dyn: None,
            }
        };

        let background = Rectangle::new(
            position.clone(),
            dimension.clone(),
            ParentState::new(),
            background_config.clone(),
        );
        let text = Text::new(text, font).set_position(ObjectPosition::dynamic(Center, Center));
        let shadow = Rectangle::new(
            position,
            dimension,
            ParentState::new(),
            RectangleConfig::new(
                background_config.corner_radius,
                Gradient::new(0.0, vec![Color::from_rgba(0, 0, 0, 127)]),
                0.0,
                BLANK,
            ),
        );

        RegularButton {
            parent: ParentState::new(),
            id: None,
            components: RegularButtonComponents {
                background,
                text,
                shadow,
            },
            state: ButtonState::new(),
            is_on_dialogue: false,
            shadow_offset,
            dialogue_id: None,
            is_active: true,
            is_shown: true,
            is_disabled: false,
        }
    }

    fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn() -> Option<State> + 'static,
    {
        self.state.on_click_event = Some(Box::new(callback));
        return self;
    }
}

impl Hoverable for RegularButton {
    fn set_is_hovered(&mut self, value: bool) {
        self.state.is_hovered = value && self.is_shown && self.is_active && !self.is_disabled;
    }

    fn get_is_hovered(&self) -> bool {
        return self.state.is_hovered;
    }
}

impl Clickable for RegularButton {
    fn set_is_clicked(&mut self, value: bool) {
        self.state.is_clicked = value;
    }
}

impl Pressable for RegularButton {
    fn set_is_pressed(&mut self, value: bool) {
        self.state.is_pressed = value;
    }
}
