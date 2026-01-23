use bevy::prelude::*;
use bevy::window;
use bevy_inspector_egui::{bevy_egui, quick};

mod ball;
mod brick;
mod health;
mod input;
mod paddle;
mod physics;
mod playfield;
mod rendering;
mod scene;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // https://github.com/bevyengine/bevy/issues/3317
                present_mode: window::PresentMode::Immediate, // 🚫 VSync OFF
                ..default()
            }),
            ..default()
        }))
        // .add_plugins((
        //     diagnostic::FrameTimeDiagnosticsPlugin::default(),
        //     diagnostic::LogDiagnosticsPlugin::default(),
        // ))
        .add_plugins(bevy_egui::EguiPlugin::default())
        .add_plugins(quick::WorldInspectorPlugin::default())
        .add_plugins(health::HealthPlugin)
        .add_message::<physics::messages::CollisionMessage>()
        .add_systems(Startup, (scene::setup, brick::systems::spawn_brick_wall).chain())
        .add_systems(
            Update,
            (
                input::systems::grab_mouse,
                (
                    paddle::systems::paddle_mouse_control,
                    paddle::systems::initialize_paddle_motion,
                    paddle::systems::finalize_paddle_motion,
                )
                    .chain(),
                playfield::systems::highlight_depth_lines,
                brick::systems::update_health_color,
                rendering::systems::update_material_color,
            ),
        )
        .add_systems(
            FixedUpdate,
            (
                (
                    physics::systems::apply_curve,
                    physics::systems::apply_velocity,
                    physics::systems::detect_collisions,
                    physics::systems::resolve_sphere_aabb_collision,
                    brick::systems::handle_collision,
                )
                    .chain(),
                (
                    paddle::systems::apply_paddle_impact_modifiers,
                    playfield::systems::wall_collision_handler,
                    paddle::systems::apply_curve_from_motion_record,
                )
                    .chain(),
            ),
        )
        .run();
}
