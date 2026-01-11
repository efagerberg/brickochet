use crate::{ball, playfield, rendering};
use bevy::prelude::*;

pub fn highlight_depth_lines(
    ball_query: Single<(&Transform, &ball::components::BallModifiers)>,
    lines: Query<
        (&Transform, &mut rendering::components::MaterialColorsUpdate),
        With<playfield::components::DepthLines>,
    >,
    playfield: Res<playfield::resources::Playfield>,
) {
    let (ball_transform, ball_modifiers) = ball_query.into_inner();

    let ball_z = ball_transform.translation.z;
    // 2 ball diameters distance away, increase for smoothing animation, decrease
    // to make animation more choppy
    let max_distance = 2.0 * ball_modifiers.radius * 2.0;
    let base_color = &playfield.wall_line_default_color;
    let highlight_color = &playfield.wall_line_highlight_color;

    for (line_transform, mut material_color) in lines {
        let distance = (line_transform.translation.z - ball_z).abs();
        let t = (max_distance - distance).clamp(0.0, 1.0); // 0 if far, 1 if very closet);
        let new_color = LinearRgba::mix(base_color, highlight_color, t);

        if material_color
            .emissive
            .is_some_and(|current| current == new_color)
        {
            continue;
        }
        material_color.emissive.replace(new_color);
    }
}
