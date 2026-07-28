use bevy::prelude::*;

use crate::states;

pub mod ball;
pub mod brick;
pub mod paddle;
pub mod player;
pub mod playfield;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameplaySet {
    Initialize,
}

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(states::GameState::Gameplay),
        brick::systems::spawn_brick_wall
            .in_set(GameplaySet::Initialize)
            .run_if(in_state(states::GameState::Gameplay)),
    )
    .add_systems(
        Update,
        (
            brick::systems::animate_light_fx,
            paddle::systems::paddle_mouse_control,
            (
                paddle::systems::initialize_paddle_motion,
                paddle::systems::finalize_paddle_motion,
            )
                .chain(),
        )
            .run_if(in_state(states::GameState::Gameplay)),
    )
    .add_systems(
        PostUpdate,
        player::systems::restart_on_player_death.run_if(in_state(states::GameState::Gameplay)),
    )
    .add_systems(
        FixedUpdate,
        (
            paddle::systems::apply_curve_from_motion_record
                .before(crate::physics::PhysicsSet::ComputeForces),
            (
                paddle::systems::apply_paddle_impact_modifiers,
                playfield::systems::handle_wall_collision,
                (
                    brick::systems::initialize_ricochet_effect,
                    (
                        brick::systems::update_curve_effect,
                        brick::systems::update_speed_effect,
                        brick::systems::update_size_effect,
                    ),
                )
                    .chain(),
            )
                .after(crate::physics::PhysicsSet::ResolveCollisions)
                .run_if(in_state(states::GameState::Gameplay)),
        ),
    )
    .add_systems(
        PostUpdate,
        (
            playfield::systems::track_ball_with_depth_line,
            ball::systems::ball_to_paddle_distance_glow,
        )
            .before(crate::rendering::RenderingSet::Integrate)
            .run_if(in_state(states::GameState::Gameplay)),
    );
}
