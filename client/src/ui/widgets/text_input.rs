use crate::ui::{config::{dimension::ObjectDimension, parent::ParentState, position::ObjectPosition}, widgets::{container::Container, rectangle::RectangleConfig, text::TextConfig}};

pub struct TextInputConfig {
    text_config: TextConfig,
    bg_config: RectangleConfig,
}

pub struct TextInput {
    value: String,
    position: ObjectPosition,
    dimension: ObjectDimension,
    parent: ParentState,
    container: Container,
    config: TextInputConfig
}
