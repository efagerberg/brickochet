use bevy::prelude::*;

use crate::rendering;

pub fn update_material_color(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<
        (
            &MeshMaterial3d<StandardMaterial>,
            &rendering::components::MaterialColorsUpdate,
        ),
        Changed<rendering::components::MaterialColorsUpdate>,
    >,
) {
    for (mesh, material_color_update) in &mut query {
        // Safe because all entities in query will have a material
        let material = materials.get_mut(&mesh.0).unwrap();

        if let Some(base_color) = material_color_update.base_color {
            material.base_color = base_color;
        }

        if let Some(emissive) = material_color_update.emissive {
            material.emissive = emissive;
        }
    }
}
