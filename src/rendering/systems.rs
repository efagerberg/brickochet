use bevy::prelude::*;

use crate::rendering;

pub fn update_material_color(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<&MeshMaterial3d<StandardMaterial>>,
    mut messages: MessageReader<rendering::messages::MaterialColorsChangedMessage>,
) {
    for message in messages.read() {
        if let Ok(material_handle) = query.get_mut(message.entity) {
            let material = materials.get_mut(material_handle.id()).unwrap();
            if let Some(base_color) = message.base_color {
                material.base_color = base_color;
            }

            if let Some(emissive) = message.emissive {
                material.emissive = emissive;
            }
        }
    }
}
