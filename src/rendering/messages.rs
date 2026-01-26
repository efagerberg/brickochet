use bevy::prelude::*;

#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct MaterialColorsChangedMessage {
    pub entity: Entity,
    pub base_color: Option<bevy::color::Color>,
    pub emissive: Option<LinearRgba>,
}
