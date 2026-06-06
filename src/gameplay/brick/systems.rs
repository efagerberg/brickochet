use bevy::asset;
use bevy::prelude::*;

use crate::audio;
use crate::gameplay::brick::key_frames;
use crate::gameplay::paddle;
use crate::gameplay::player;
use crate::gameplay::{brick, playfield};
use crate::{asset_loading, health, physics, states};

pub fn spawn_brick_wall(
    mut commands: Commands,
    goal_query: Query<(
        &playfield::components::Goal,
        &Transform,
        &physics::components::BoundingCuboid,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    brick_folder: Res<asset_loading::resources::LoadedBrickFolder>,
    brick_assets: ResMut<Assets<brick::assets::BrickAsset>>,
    loaded_folders: Res<Assets<asset::LoadedFolder>>,
    playfield: Res<playfield::resources::Playfield>,
) {
    let (_, enemy_goal_transform, enemy_goal_bounds) = goal_query
        .iter()
        .find(|(goal, _, __)| **goal == playfield::components::Goal::Enemy)
        .expect("Missing enemy goal, cannot spawn brick wall");

    // Dimensions of the wall
    let wall_width = enemy_goal_bounds.half_extents.x * 2.0;
    let wall_height = enemy_goal_bounds.half_extents.y * 2.0;
    let wall_depth = enemy_goal_bounds.half_extents.z * 2.0;

    // Brick size (uniform)
    let brick_size = playfield.brick_size;

    // How many bricks fit
    let bricks_x = (wall_width / brick_size.x).floor() as i32;
    let bricks_y = (wall_height / brick_size.y).floor() as i32;
    let total_bricks = bricks_x * bricks_y;

    // Total grid size
    let total_width = bricks_x as f32 * brick_size.x;
    let total_height = bricks_y as f32 * brick_size.y;

    let brick_asset_folder = loaded_folders.get(&brick_folder.0).unwrap();
    let brick_handles: Vec<Handle<brick::assets::BrickAsset>> = brick_asset_folder
        .handles
        .iter()
        .map(|x: &UntypedHandle| x.clone().typed())
        .collect();

    let mut asset_index = 0;

    for index in 0..total_bricks {
        let x = index % bricks_x;
        let y = index / bricks_x;

        let pos = Vec3::new(
            enemy_goal_transform.translation.x - total_width * 0.5 + x as f32 * brick_size.x,
            enemy_goal_transform.translation.y - total_height * 0.5 + y as f32 * brick_size.y,
            enemy_goal_transform.translation.z + wall_depth + brick_size.z,
        );

        if let Some(brick_asset) = brick_assets.get(brick_handles[asset_index].id()) {
            spawn_brick(
                &mut commands,
                &mut meshes,
                &mut materials,
                &asset_server,
                pos,
                brick_size,
                brick_asset.clone(),
            );
        }
        asset_index = ((index as usize) + 1) % brick_assets.len();
    }
}

fn spawn_brick(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
    position: Vec3,
    size: Vec3,
    brick_asset: brick::assets::BrickAsset,
) {
    // Outer black border (slightly larger)
    let border_padding = 0.25;

    let border = commands
        .spawn((
            Name::new("Brick Border"),
            Mesh3d(meshes.add(Cuboid::new(
                size.x + border_padding,
                size.y + border_padding,
                size.z * 0.05, // thin
            ))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::BLACK,
                unlit: true,
                ..default()
            })),
        ))
        .id();

    let healthy_color = LinearRgba::rgb(0.0, 1.0, 0.0);
    let critical_color = LinearRgba::rgb(1.0, 0.0, 0.0);

    let sfx_optional_handle = brick_asset
        .collision_sfx
        .and_then(|path| asset_server.get_handle::<AudioSource>(path));

    // Main colored brick
    let main = commands
        .spawn((
            Name::new(brick_asset.name),
            brick::components::Brick,
            physics::components::BoundingCuboid {
                half_extents: size * 0.5,
            },
            Transform::from_translation(Vec3::new(
                position.x + size.x * 0.5,
                position.y + size.y * 0.5,
                position.z,
            )),
            GlobalTransform::default(),
            Mesh3d(meshes.add(Cuboid::new(
                size.x - border_padding,
                size.y - border_padding,
                size.z,
            ))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::from(healthy_color),
                ..default()
            })),
            health::components::Health {
                max: brick_asset.health,
                current: brick_asset.health,
            },
            health::components::HealthColors {
                max: healthy_color,
                min: critical_color,
            },
            health::components::ChangeOnCollision {
                delta: -1,
                affected: health::components::Affects::SelfOnly,
            },
            DespawnOnExit(states::GameState::Gameplay),
        ))
        .id();

    if let Some(sfx) = sfx_optional_handle.clone() {
        commands
            .entity(main)
            .insert(audio::components::CollisionSFX(sfx));
    }
    if let Some(definition) = brick_asset.ricochet_effect {
        commands
            .entity(main)
            .insert(brick::components::RicochetEffectConfig { definition });
    }
    commands.entity(main).add_child(border);
}

pub fn initialize_ricochet_effect(
    mut non_brick_query: Query<&Transform, Without<brick::components::Brick>>,
    brick_query: Query<&brick::components::RicochetEffectConfig, With<brick::components::Brick>>,
    paddle_query: Query<
        &Transform,
        (
            With<paddle::components::Paddle>,
            With<player::components::Player>,
        ),
    >,
    mut collision_messages: MessageReader<physics::messages::CollisionMessage>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for message in collision_messages.read() {
        let non_brick_result = non_brick_query.get_mut(message.a);
        let brick_result = brick_query.get(message.b);
        let combined =
            non_brick_result.and_then(|non_brick| brick_result.map(|brick| (non_brick, brick)));

        if let Ok((non_brick_transform, ricochet_effect)) = combined {
            let (start, end) = match ricochet_effect.definition.driver {
                brick::assets::EffectDriver::Time { duration_seconds } => {
                    let start = time.elapsed_secs();
                    let end = time.elapsed_secs() + duration_seconds;
                    (start, end)
                }
                brick::assets::EffectDriver::DistanceToPlayer => {
                    let start = non_brick_transform.translation.z;
                    let paddle_transform = paddle_query
                        .iter()
                        .next()
                        .expect("No player, cannot use DistanceToPlayer driver");
                    let end = paddle_transform.translation.z;
                    (start, end)
                }
            };
            let definition = ricochet_effect.definition.clone();
            match definition.attribute {
                brick::assets::RicochetEffectAttribute::Speed(scalar_curve) => {
                    let effect_state = brick::components::RicochetSpeedEffectState {
                        driver: definition.driver,
                        start,
                        end,
                        key_frames: scalar_curve.key_frames,
                    };
                    commands.entity(message.a).insert(effect_state);
                }
                brick::assets::RicochetEffectAttribute::Curve(vec2_curve) => {
                    let effect_state = brick::components::RicochetCurveEffectState {
                        driver: definition.driver,
                        start,
                        end,
                        key_frames: vec2_curve.key_frames,
                    };
                    commands.entity(message.a).insert(effect_state);
                }
                brick::assets::RicochetEffectAttribute::Size(scalar_curve) => {
                    let effect_state = brick::components::RicochetSizeEffectState {
                        driver: definition.driver,
                        start,
                        end,
                        key_frames: scalar_curve.key_frames,
                    };
                    commands.entity(message.a).insert(effect_state);
                }
            }
        }
    }
}

pub fn update_speed_effect(
    query: Query<(
        Entity,
        &Transform,
        &mut physics::components::Velocity,
        &mut brick::components::RicochetSpeedEffectState,
    )>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, transform, mut velocity, mut effect_state) in query {
        let current = match effect_state.driver {
            crate::gameplay::brick::assets::EffectDriver::Time {
                duration_seconds: _,
            } => time.elapsed_secs(),
            crate::gameplay::brick::assets::EffectDriver::DistanceToPlayer => {
                transform.translation.z
            }
        };
        let t = (current - effect_state.start) / (effect_state.end - effect_state.start);
        let sampled = match key_frames::sample_key_frames::<f32>(&effect_state.key_frames, t) {
            Err(key_frames::NextKeyFrameError::NotStarted) => continue,
            Ok(v) => v,
            Err(key_frames::NextKeyFrameError::Finished) => {
                commands
                    .entity(entity)
                    .remove::<brick::components::RicochetSpeedEffectState>();
                continue;
            }
        };

        // preserve direction, adjust magnitude
        if velocity.0.length_squared() > 0.0 {
            velocity.0 = velocity.0.normalize() * sampled;
        } else {
            // if zero velocity, set in z direction (or skip) — adjust behavior as desired
            velocity.0 = Vec3::new(0.0, 0.0, sampled);
        }
    }
}

pub fn update_curve_effect(
    query: Query<(
        Entity,
        &Transform,
        &mut physics::components::Curve,
        &mut brick::components::RicochetCurveEffectState,
    )>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, transform, mut curve, mut effect_state) in query {
        let current = match effect_state.driver {
            crate::gameplay::brick::assets::EffectDriver::Time {
                duration_seconds: _,
            } => time.elapsed_secs(),
            crate::gameplay::brick::assets::EffectDriver::DistanceToPlayer => {
                transform.translation.z
            }
        };
        let t = (current - effect_state.start) / (effect_state.end - effect_state.start);
        let sampled =
            match key_frames::sample_key_frames::<bevy::prelude::Vec2>(&effect_state.key_frames, t)
            {
                Err(key_frames::NextKeyFrameError::NotStarted) => continue,
                Ok(v) => v,
                Err(key_frames::NextKeyFrameError::Finished) => {
                    commands
                        .entity(entity)
                        .remove::<brick::components::RicochetCurveEffectState>();
                    continue;
                }
            };

        curve.0 = sampled;
    }
}

pub fn update_size_effect(
    query: Query<(
        Entity,
        &mut Transform,
        &mut physics::components::BoundingSphere,
        &brick::components::RicochetSizeEffectState,
    )>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut transform, mut bounding_sphere, effect_state) in query {
        let current = match effect_state.driver {
            crate::gameplay::brick::assets::EffectDriver::Time {
                duration_seconds: _,
            } => time.elapsed_secs(),
            crate::gameplay::brick::assets::EffectDriver::DistanceToPlayer => {
                transform.translation.z
            }
        };
        let t = (current - effect_state.start) / (effect_state.end - effect_state.start);

        let sampled = match key_frames::sample_key_frames(&effect_state.key_frames, t) {
            Err(key_frames::NextKeyFrameError::NotStarted) => continue,
            Ok(v) => v,
            Err(key_frames::NextKeyFrameError::Finished) => {
                commands
                    .entity(entity)
                    .remove::<brick::components::RicochetSizeEffectState>();
                continue;
            }
        };

        transform.scale = Vec3::ONE * sampled;
        bounding_sphere.radius = sampled / 2.0;
    }
}
