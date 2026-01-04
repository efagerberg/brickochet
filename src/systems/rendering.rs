use crate::components;
use bevy::prelude::*;

pub fn update_material_color(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<
        (
            &MeshMaterial3d<StandardMaterial>,
            &components::rendering::MaterialColorsUpdate,
        ),
        Changed<components::rendering::MaterialColorsUpdate>,
    >,
) {
    for (mesh, material_color_update) in &mut query {
        let Some(material) = materials.get_mut(&mesh.0) else {
            continue;
        };

        if let Some(base_color) = material_color_update.base_color {
            material.base_color = base_color;
        }

        if let Some(emissive) = material_color_update.emissive {
            material.emissive = emissive;
        }
    }
}
