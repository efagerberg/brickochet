use crate::rendering;
use bevy::prelude::*;
use test_case::test_case;

struct UpdateMaterialColorCase {
    messages: Vec<rendering::messages::MaterialColorsChangedMessage>,
    expected_base: Color,
    expected_emissive: LinearRgba,
    has_material: bool, // whether the entity actually has a MeshMaterial3d
}

#[test_case(
    UpdateMaterialColorCase {
        messages: vec![
            rendering::messages::MaterialColorsChangedMessage {
                entity: Entity::PLACEHOLDER,
                base_color: Some(Color::WHITE),
                emissive: Some(Color::BLACK.to_linear()),
            }
        ],
        expected_base: Color::WHITE,
        expected_emissive: Color::BLACK.to_linear(),
        has_material: true,
    };
    "single message updates both fields"
)]
#[test_case(
    UpdateMaterialColorCase {
        messages: vec![
            rendering::messages::MaterialColorsChangedMessage {
                entity: Entity::PLACEHOLDER,
                base_color: None,
                emissive: None,
            }
        ],
        expected_base: Color::WHITE,
        expected_emissive: Color::BLACK.to_linear(),
        has_material: true,
    };
    "single message with None does not update"
)]
#[test_case(
    UpdateMaterialColorCase {
        messages: vec![
            rendering::messages::MaterialColorsChangedMessage {
                entity: Entity::PLACEHOLDER,
                base_color: Some(Color::WHITE),
                emissive: None,
            },
            rendering::messages::MaterialColorsChangedMessage {
                entity: Entity::PLACEHOLDER,
                base_color: Some(Color::WHITE),
                emissive: None,
            },
        ],
        expected_base: Color::WHITE,
        expected_emissive: Color::BLACK.to_linear(),
        has_material: true,
    };
    "last message wins for same field"
)]
#[test_case(
    UpdateMaterialColorCase {
        messages: vec![
            rendering::messages::MaterialColorsChangedMessage {
                entity: Entity::PLACEHOLDER,
                base_color: Some(Color::WHITE),
                emissive: None,
            },
            rendering::messages::MaterialColorsChangedMessage {
                entity: Entity::PLACEHOLDER,
                base_color: None,
                emissive: Some(Color::BLACK.to_linear()),
            },
        ],
        expected_base: Color::WHITE,
        expected_emissive: Color::BLACK.to_linear(),
        has_material: true,
    };
    "multiple messages can update different fields"
)]
#[test_case(
    UpdateMaterialColorCase {
        messages: vec![
            rendering::messages::MaterialColorsChangedMessage {
                entity: Entity::PLACEHOLDER,
                base_color: Some(Color::WHITE),
                emissive: Some(Color::BLACK.to_linear()),
            }
        ],
        expected_base: Color::WHITE,
        expected_emissive: Color::BLACK.to_linear(),
        has_material: false,
    };
    "messages ignored for entity without material"
)]
fn test_update_material_color_system(case: UpdateMaterialColorCase) {
    let (mut app, material_handle, entity) = setup_update_material_color(case.has_material);

    let mut messages_res = app
        .world_mut()
        .resource_mut::<Messages<rendering::messages::MaterialColorsChangedMessage>>();

    for mut msg in case.messages {
        msg.entity = entity;
        messages_res.write(msg);
    }

    app.update();

    let materials = app.world().resource::<Assets<StandardMaterial>>();
    let material = materials.get(&material_handle).unwrap();

    assert_eq!(material.base_color, case.expected_base);
    assert_eq!(material.emissive, case.expected_emissive);
}

fn setup_update_material_color(has_material: bool) -> (App, Handle<StandardMaterial>, Entity) {
    let mut app = App::new();

    app.init_resource::<Assets<StandardMaterial>>();
    app.init_resource::<Assets<Mesh>>();
    app.add_message::<rendering::messages::MaterialColorsChangedMessage>();
    app.add_systems(Update, rendering::systems::update_material_color);

    let material_handle = {
        let mut materials = app.world_mut().resource_mut::<Assets<StandardMaterial>>();
        materials.add(StandardMaterial::default())
    };

    let entity = if has_material {
        app.world_mut()
            .spawn(MeshMaterial3d(material_handle.clone()))
            .id()
    } else {
        app.world_mut().spawn_empty().id()
    };

    (app, material_handle, entity)
}
