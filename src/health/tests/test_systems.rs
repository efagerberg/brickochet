use bevy::prelude::*;
use test_case::test_case;

use crate::health::{components, messages, systems};
use crate::{physics, rendering, test_utils};

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

enum Target {
    SelfOnly,
    Others,
    SelfAndOthers,
}

struct HandleCollisionCase {
    target: Target,
    delta: i16,
}

#[test_case(
    HandleCollisionCase {
        target: Target::SelfOnly,
        delta: 1
    }; "self only")]
#[test_case(
    HandleCollisionCase {
        target: Target::Others,
        delta: -1
    }; "others only")]
#[test_case(
    HandleCollisionCase {
        target: Target::SelfAndOthers,
        delta: -1
    }; "self and others")]
fn test_handle_collision(case: HandleCollisionCase) {
    let mut app = App::new();
    app.add_message::<messages::HealChangedMessage>();
    app.add_message::<physics::messages::CollisionMessage>();
    app.add_systems(Update, systems::handle_collision);

    let collision_a_entity = app.world_mut().spawn_empty().id();
    let collision_b_entity = app
        .world_mut()
        .spawn(components::Health { current: 1, max: 1 })
        .id();

    let potential_target_entity = app.world_mut().spawn(components::Health { current: 1, max: 1 }).id();
    let affects = match case.target {
        Target::SelfOnly => components::Affects::SelfOnly,
        Target::Others => components::Affects::Others(vec![potential_target_entity]),
        Target::SelfAndOthers => components::Affects::SelfAndOthers(vec![potential_target_entity]),
    };

    app.world_mut()
        .entity_mut(collision_b_entity)
        .insert(components::ChangeOnCollision {
            delta: case.delta,
            targets: affects.clone(),
        });

    let affected_targets = match affects {
        components::Affects::SelfOnly => vec![collision_b_entity],
        components::Affects::Others(ref v) => v.clone(),
        components::Affects::SelfAndOthers(ref v) => {
            let mut vec = vec![collision_b_entity];
            vec.extend(v.iter().copied());
            vec
        }
    };

    let mut writer = app
        .world_mut()
        .resource_mut::<Messages<physics::messages::CollisionMessage>>();
    writer.write(physics::messages::CollisionMessage {
        a: collision_a_entity,
        b: collision_b_entity,
        normal: Vec3::ZERO,
        contact_point: Vec3::ZERO,
        penetration: 0.0,
    });

    app.update();

    let expected: Vec<_> = affected_targets
        .into_iter()
        .map(|e| messages::HealChangedMessage {
            entity: e,
            delta: case.delta,
        })
        .collect();

    test_utils::assertions::assert_messages(&app, &expected);
}
