use bevy::prelude::*;

use crate::state;

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
        OnEnter(state::GameState::Gameplay),
        brick::systems::spawn_brick_wall
            .in_set(GameplaySet::Initialize)
            .run_if(in_state(state::GameState::Gameplay)),
    )
    .add_systems(
        Update,
        (
            paddle::systems::paddle_mouse_control,
            (
                paddle::systems::initialize_paddle_motion,
                paddle::systems::finalize_paddle_motion,
            )
                .chain(),
        )
            .run_if(in_state(state::GameState::Gameplay)),
    )
    .add_systems(
        Update,
        player::systems::restart_on_player_death.run_if(in_state(state::GameState::Gameplay)),
    )
    .add_systems(
        FixedUpdate,
        (
            paddle::systems::apply_curve_from_motion_record
                .before(crate::physics::PhysicsSet::ApplyForces),
            (
                paddle::systems::apply_paddle_impact_modifiers,
                playfield::systems::handle_wall_collision,
            )
                .after(crate::physics::PhysicsSet::ResolveCollisions)
                .run_if(in_state(state::GameState::Gameplay)),
        ),
    )
    .add_systems(
        PostUpdate,
        (playfield::systems::highlight_depth_lines,)
            .before(crate::rendering::RenderingSet::Integrate)
            .run_if(in_state(state::GameState::Gameplay)),
    );
}
