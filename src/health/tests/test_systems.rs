use bevy::prelude::*;
use test_case::test_case;

use crate::health::{components, messages, systems};
use crate::{rendering, test_utils};

fn create_health_change_app() -> App {
    let mut app = App::new();
    app.add_message::<messages::HealChangedMessage>()
        .add_message::<messages::DeathMessage>()
        .add_systems(Update, systems::handle_health_changed);
    app
}

pub struct HealthChangedCase {
    starting_health: components::Health,
    delta: i16,
    expected_current: u8,
}

#[test_case(
    HealthChangedCase {
        starting_health: components::Health { max: 10, current: 10 },
        delta: -3,
        expected_current: 7,
    }; "health reduced by 3")]
#[test_case(
    HealthChangedCase {
        starting_health: components::Health { max: 10, current: 7 },
        delta: 3,
        expected_current: 10
    }; "health increased by 3")]
#[test_case(
    HealthChangedCase {
        starting_health: components::Health { max: 10, current: 10 },
        delta: 3,
        expected_current: 10
    }; "health clamped to max"
)]
#[test_case(
    HealthChangedCase {
        starting_health: components::Health { max: 10, current: 0 },
        delta: -10,
        expected_current: 0
    }; "health clamped to zero"
)]
fn test_health_change(case: HealthChangedCase) {
    let mut app = create_health_change_app();
    let entity = app.world_mut().spawn(case.starting_health).id();

    app.world_mut().write_message(messages::HealChangedMessage {
        entity,
        delta: case.delta,
    });
    app.update();

    let health = app.world().get::<components::Health>(entity).unwrap();
    assert_eq!(health.current, case.expected_current);

    let expected = if case.expected_current == 0 {
        vec![messages::DeathMessage { entity }]
    } else {
        vec![]
    };
    test_utils::assertions::assert_messages(&app, &expected);
}

#[test]
fn death_message_removes_entity() {
    let mut app = create_death_app();
    let entity = app.world_mut().spawn_empty().id();

    app.world_mut()
        .write_message(messages::DeathMessage { entity });
    app.update();

    assert!(app.world().get_entity(entity).is_err());
}

#[test]
fn no_death_message_keeps_entity_alive() {
    let mut app = create_death_app();
    let entity = app.world_mut().spawn_empty().id();

    app.update();

    assert!(app.world().get_entity(entity).is_ok());
}

fn create_death_app() -> App {
    let mut app = App::new();
    app.add_message::<messages::DeathMessage>()
        .add_systems(Update, systems::handle_death);
    app
}

struct UpdateHealthColorCase {
    health: components::Health,
    expected_color: Option<Color>,
}

#[test_case(
    UpdateHealthColorCase {
        health: components::Health {
            current: 10,
            max: 10,
        },
        expected_color: Some(Color::linear_rgb(0.0, 1.0, 0.0))
    }; "green when health is still full"
)]
#[test_case(
    UpdateHealthColorCase {
        health: components::Health {
            current: 1,
            max: 3,
        },
        expected_color: Some(Color::linear_rgb(1.0, 0.0, 0.0))
    }; "red when 1 hp left"
)]
#[test_case(
    UpdateHealthColorCase {
        health: components::Health {
            current: 0,
            max: 3,
        },
        expected_color: None
    }; "no message when 0 hp"
)]
fn test_update_health_color(case: UpdateHealthColorCase) {
    let mut app = App::new();
    app.add_message::<messages::HealChangedMessage>();
    app.add_message::<rendering::messages::MaterialColorsChangedMessage>();
    app.add_systems(Update, systems::update_health_color);
    let entity = app
        .world_mut()
        .spawn((
            case.health,
            components::HealthColors {
                max: LinearRgba::rgb(0.0, 1.0, 0.0),
                min: LinearRgba::rgb(1.0, 0.0, 0.0),
            },
        ))
        .id();
    let mut writer = app
        .world_mut()
        .resource_mut::<Messages<messages::HealChangedMessage>>();
    writer.write(messages::HealChangedMessage { entity, delta: 0 });
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
