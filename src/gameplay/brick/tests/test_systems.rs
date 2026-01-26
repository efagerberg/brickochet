use bevy::prelude::*;

use crate::gameplay::brick;
use crate::health;
use crate::physics;
use crate::rendering;
use crate::test_utils;

use test_case::test_case;

struct UpdateHealthColorCase {
    health: health::components::Health,
    expected_color: Option<Color>,
}

#[test_case(
    UpdateHealthColorCase {
        health: health::components::Health {
            current: 10,
            max: 10,
        },
        expected_color: Some(Color::linear_rgb(0.0, 1.0, 0.0))
    }; "green when health is still full"
)]
#[test_case(
    UpdateHealthColorCase {
        health: health::components::Health {
            current: 1,
            max: 3,
        },
        expected_color: Some(Color::linear_rgb(1.0, 0.0, 0.0))
    }; "red when 1 hp left"
)]
#[test_case(
    UpdateHealthColorCase {
        health: health::components::Health {
            current: 0,
            max: 3,
        },
        expected_color: None
    }; "no message when 0 hp"
)]
fn test_update_health_color(case: UpdateHealthColorCase) {
    let mut app = App::new();
    app.add_message::<health::messages::HealChangedMessage>();
    app.add_message::<rendering::messages::MaterialColorsChangedMessage>();
    app.add_systems(Update, brick::systems::update_health_color);
    let entity = app
        .world_mut()
        .spawn((case.health, brick::components::Brick))
        .id();
    let mut writer = app
        .world_mut()
        .resource_mut::<Messages<health::messages::HealChangedMessage>>();
    writer.write(health::messages::HealChangedMessage { entity, delta: 0 });
    app.update();

    let mut expected: Vec<rendering::messages::MaterialColorsChangedMessage> = vec![];
    if case.expected_color.is_some() {
        expected.push(rendering::messages::MaterialColorsChangedMessage {
            entity,
            base_color: case.expected_color,
            emissive: None,
        })
    };
    test_utils::assertions::assert_messages(&app, &expected);
}

#[test]
fn test_handle_collision_decreases_health() {
    let mut app = App::new();
    app.add_message::<health::messages::HealChangedMessage>();
    app.add_message::<physics::messages::CollisionMessage>();
    app.add_systems(Update, brick::systems::handle_collision);
    let entity = app
        .world_mut()
        .spawn((
            health::components::Health { current: 1, max: 1 },
            brick::components::Brick,
        ))
        .id();
    let collided_with = app.world_mut().spawn_empty().id();

    let mut writer = app
        .world_mut()
        .resource_mut::<Messages<physics::messages::CollisionMessage>>();
    writer.write(physics::messages::CollisionMessage {
        a: collided_with,
        b: entity,
        normal: Vec3::ZERO,
        contact_point: Vec3::ZERO,
        penetration: 0.0,
    });
    app.update();

    let expected = health::messages::HealChangedMessage {
        entity,
        delta: -1
    };
    test_utils::assertions::assert_messages(&app, &vec![expected]);
}
