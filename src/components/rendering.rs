use bevy::prelude::*;

#[derive(Component, Default)]
pub struct MaterialColorsUpdate {
    pub base_color: Option<bevy::color::Color>,
    pub emissive: Option<LinearRgba>,
}
