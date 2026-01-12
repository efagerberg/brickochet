use bevy::prelude::*;

use crate::{ball, physics, playfield, rendering};

const PLAYFIELD_RES: playfield::resources::Playfield = playfield::resources::Playfield {
    bounds: physics::components::BoundingCuboid {
        half_extents: Vec3::new(1.0, 2.0, 3.0),
    },
    wall_line_default_color: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
    wall_line_highlight_color: LinearRgba::new(1.0, 0.0, 0.0, 1.0),
};

#[test]
fn test_updates_emissive_based_on_distance() {
    let (ball_transform, lines_transform, material_color) = setup(
        2.0,
        0.0,
        &PLAYFIELD_RES,
        PLAYFIELD_RES.wall_line_default_color,
    );
    let expected_color = LinearRgba::mix(
        &PLAYFIELD_RES.wall_line_default_color,
        &PLAYFIELD_RES.wall_line_highlight_color,
        (lines_transform.translation.z - ball_transform.translation.z)
            .abs()
            .clamp(0.0, 1.0),
    );

    assert_eq!(material_color.emissive.unwrap(), expected_color);
}

#[test]
fn test_emissive_does_not_change_if_already_highlighted_and_z_position_equal_ball_z() {
    let (_, __, material_color) = setup(
        0.0,
        0.0,
        &PLAYFIELD_RES,
        PLAYFIELD_RES.wall_line_highlight_color,
    );

    assert_eq!(
        material_color.emissive.unwrap(),
        PLAYFIELD_RES.wall_line_highlight_color
    );
}

fn make_transform(z: f32) -> Transform {
    Transform::from_translation(Vec3::new(0.0, 0.0, z))
}

fn setup(
    ball_z: f32,
    lines_z: f32,
    playfield_res: &playfield::resources::Playfield,
    initial_emissive: LinearRgba,
) -> (
    Transform,
    Transform,
    rendering::components::MaterialColorsUpdate,
) {
    let mut app = App::new();
    app.insert_resource(playfield_res.clone());
    let ball_transform = make_transform(ball_z);
    let ball_modifiers = ball::components::BallModifiers::starting();
    app.world_mut().spawn((
        ball_modifiers.clone(),
        ball_transform,
        physics::components::BoundingSphere {
            radius: ball_modifiers.base_radius,
        },
    ));
    let lines_transform = make_transform(lines_z);
    let lines_entity = app
        .world_mut()
        .spawn((
            playfield::components::DepthLines,
            lines_transform,
            rendering::components::MaterialColorsUpdate {
                base_color: None,
                emissive: Some(initial_emissive),
            },
        ))
        .id();
    app.add_systems(Update, playfield::systems::highlight_depth_lines);
    app.update();
    let lines_material_color = app
        .world()
        .get::<rendering::components::MaterialColorsUpdate>(lines_entity)
        .unwrap()
        .clone();
    (ball_transform, lines_transform, lines_material_color)
}
