use bevy::prelude::*;

use crate::rendering;

#[test]
fn test_updates_color_when_components_has_values() {
    let (mut app, material_handle) = setup();
    let expected_color = Color::linear_rgb(1.0, 0.0, 0.0);
    app.world_mut().spawn((
        MeshMaterial3d(material_handle.clone()),
        rendering::components::MaterialColorsUpdate {
            base_color: Some(expected_color),
            emissive: Some(expected_color.to_linear()),
        },
    ));

    app.update();
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    let material = materials.get(&material_handle).unwrap();

    assert_eq!(material.base_color, expected_color);
    assert_eq!(material.emissive, expected_color.to_linear());
}

#[test]
fn test_does_not_updates_color_when_component_has_nones() {
    let (mut app, material_handle) = setup();
    app.world_mut().spawn((
        MeshMaterial3d(material_handle.clone()),
        rendering::components::MaterialColorsUpdate {
            base_color: None,
            emissive: None,
        },
    ));

    app.update();
    let materials = app.world().resource::<Assets<StandardMaterial>>();
    let material = materials.get(&material_handle).unwrap();

    assert_eq!(material.base_color, Color::WHITE);
    assert_eq!(material.emissive, Color::BLACK.to_linear());
}

fn setup() -> (App, Handle<StandardMaterial>) {
    let mut app = App::new();
    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<Assets<Mesh>>();
    app.add_systems(Update, rendering::systems::update_material_color);
    let material_handle = {
        let mut materials = app.world_mut().resource_mut::<Assets<StandardMaterial>>();
        materials.add(StandardMaterial::default())
    };
    (app, material_handle)
}
