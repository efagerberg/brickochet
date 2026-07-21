use bevy::prelude::*;

use crate::audio;
use crate::gameplay::brick::key_frames;
use crate::gameplay::paddle;
use crate::gameplay::player;
use crate::gameplay::{brick, playfield};
use crate::{health, physics, states};
use rand::seq::SliceRandom;

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
    brick_assets: ResMut<Assets<brick::assets::BrickAsset>>,
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

    let brick_handles: Vec<&brick::assets::BrickAsset> = brick_assets.iter().map(|x| x.1).collect();

    let mut rng = rand::rng();
    let mut brick_order: Vec<usize> = (0..total_bricks as usize)
        .map(|i| i % brick_handles.len())
        .collect();
    brick_order.shuffle(&mut rng);

    for (index, asset_index) in (0..total_bricks).zip(brick_order.iter()) {
        let x = index % bricks_x;
        let y = index / bricks_x;

        let pos = Vec3::new(
            enemy_goal_transform.translation.x - total_width * 0.5 + x as f32 * brick_size.x,
            enemy_goal_transform.translation.y - total_height * 0.5 + y as f32 * brick_size.y,
            enemy_goal_transform.translation.z + wall_depth + brick_size.z,
        );

        let brick_asset = brick_handles[*asset_index];
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
                ..default()
            })),
        ))
        .id();

    let healthy_color = LinearRgba::rgb(0.0, 1.0, 0.0);
    let critical_color = LinearRgba::rgb(1.0, 0.0, 0.0);

    let sfx_with_handle = brick_asset.collision_sfx.and_then(|sfx| {
        asset_server
            .get_handle::<AudioSource>(&sfx.path)
            .map(|h| (h, sfx.volume))
    });

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
            children![(
                Mesh3d(meshes.add(Rectangle::new(
                    size.x - border_padding,
                    size.y - border_padding
                ))),
                Transform::from_xyz(0.0, 0.0, size.z / 2.0 + 0.01),
                GlobalTransform::default()
            )],
            health::components::Health {
                max: brick_asset.health,
                current: brick_asset.health,
            },
            health::components::HealthColors {
                max: healthy_color,
                min: critical_color,
                color_type: health::components::HealthColorType::BaseColor,
            },
            health::components::ChangeOnCollision {
                delta: -1,
                affected: health::components::Affects::SelfOnly,
            },
            DespawnOnExit(states::GameState::Gameplay),
        ))
        .id();

    if let Some((handle, volume)) = sfx_with_handle.clone() {
        commands
            .entity(main)
            .insert(audio::components::CollisionSFX { handle, volume });
    }
    if let Some(definition) = brick_asset.ricochet_effect {
        commands
            .entity(main)
            .insert(brick::components::RicochetEffectConfig { definition });
    }
    if let Some(border_effect) = &brick_asset.light_fx {
        commands
            .entity(main)
            .insert(brick::components::LightFX(border_effect.clone()));
    }

    commands.entity(main).add_child(border);
}

pub fn initialize_ricochet_effect(
    mut ball_query: Query<
        (&Transform, &physics::components::BoundingSphere),
        Without<brick::components::Brick>,
    >,
    brick_query: Query<&brick::components::RicochetEffectConfig, With<brick::components::Brick>>,
    paddle_query: Query<
        (&Transform, &physics::components::BoundingCuboid),
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
        let ball_result = ball_query.get_mut(message.a);
        let brick_result = brick_query.get(message.b);
        let combined =
            ball_result.and_then(|non_brick| brick_result.map(|brick| (non_brick, brick)));

        if let Ok(((ball_transform, bounding_sphere), ricochet_effect)) = combined {
            let (start, end) = match ricochet_effect.definition.driver {
                brick::assets::EffectDriver::Time { duration_seconds } => {
                    let start = time.elapsed_secs();
                    let end = time.elapsed_secs() + duration_seconds;
                    (start, end)
                }
                brick::assets::EffectDriver::DistanceToPlayer => {
                    let start = ball_transform.translation.z;
                    let (paddle_transform, bounding_cuboid) = paddle_query
                        .iter()
                        .next()
                        .expect("No player, cannot use DistanceToPlayer driver");
                    let ball_radius = bounding_sphere.radius;
                    let contact_offset = bounding_cuboid.half_extents.z + ball_radius;
                    // ball travels from start toward the paddle's center; it will
                    // actually stop at the paddle's surface, short by contact_offset
                    let raw_end = paddle_transform.translation.z;
                    let direction = (raw_end - start).signum();
                    let end = raw_end - direction * contact_offset;
                    (start, end)
                }
            };
            let definition = ricochet_effect.definition.clone();
            match definition.attribute {
                brick::assets::RicochetEffectAttribute::Speed(scalar_curve) => {
                    let effect =
                        brick::components::RicochetSpeedEffect(brick::components::RicochetEffect {
                            driver: definition.driver,
                            start,
                            end,
                            key_frames: scalar_curve.key_frames,
                        });
                    commands.entity(message.a).insert(effect);
                }
                brick::assets::RicochetEffectAttribute::Curve(vec2_curve) => {
                    let effect =
                        brick::components::RicochetCurveEffect(brick::components::RicochetEffect {
                            driver: definition.driver,
                            start,
                            end,
                            key_frames: vec2_curve.key_frames,
                        });
                    commands.entity(message.a).insert(effect);
                }
                brick::assets::RicochetEffectAttribute::Size(scalar_curve) => {
                    let effect =
                        brick::components::RicochetSizeEffect(brick::components::RicochetEffect {
                            driver: definition.driver,
                            start,
                            end,
                            key_frames: scalar_curve.key_frames,
                        });
                    commands.entity(message.a).insert(effect);
                }
            }
        }
    }
}

fn driver_current_value(
    driver: &brick::assets::EffectDriver,
    transform: &Transform,
    time: &Time,
) -> f32 {
    match driver {
        brick::assets::EffectDriver::Time { .. } => time.elapsed_secs(),
        brick::assets::EffectDriver::DistanceToPlayer => transform.translation.z,
    }
}

/// Samples a ricochet effect's current value, handling lifecycle (removal on
/// completion or missing keyframes) via `commands`. Returns `None` when there's
/// nothing to apply this frame (not started yet, or just removed).
fn sample_ricochet_effect<T, V>(
    commands: &mut Commands,
    entity: Entity,
    transform: &Transform,
    time: &Time,
    effect: &brick::components::RicochetEffect<V>,
) -> Option<V>
where
    T: Component,
    V: key_frames::Lerp + Copy,
{
    let current = driver_current_value(&effect.driver, transform, time);
    let t = (current - effect.start) / (effect.end - effect.start);

    match key_frames::sample_key_frames::<V>(&effect.key_frames, t) {
        Err(key_frames::SampleKeyFramesError::NotStarted) => None,
        Err(key_frames::SampleKeyFramesError::NoKeyFrames) => {
            warn!("No keyframes given, skipping and removing effect");
            commands.entity(entity).remove::<T>();
            None
        }
        Ok(key_frames::SampleKeyFrameValue::InProgress(v)) => Some(v),
        Ok(key_frames::SampleKeyFrameValue::Complete(v)) => {
            commands.entity(entity).remove::<T>();
            Some(v)
        }
    }
}

pub fn update_speed_effect(
    query: Query<(
        Entity,
        &Transform,
        &mut physics::components::Velocity,
        &brick::components::RicochetSpeedEffect,
    )>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, transform, mut velocity, effect) in query {
        let Some(sampled) = sample_ricochet_effect::<brick::components::RicochetSpeedEffect, f32>(
            &mut commands,
            entity,
            transform,
            &time,
            &effect.0,
        ) else {
            continue;
        };

        if velocity.0.length_squared() > 0.0 {
            velocity.0 = velocity.0.normalize() * sampled;
        } else {
            velocity.0 = Vec3::new(0.0, 0.0, sampled);
        }
    }
}

pub fn update_curve_effect(
    query: Query<(
        Entity,
        &Transform,
        &mut physics::components::Curve,
        &brick::components::RicochetCurveEffect,
    )>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, transform, mut curve, effect) in query {
        let Some(sampled) = sample_ricochet_effect::<brick::components::RicochetCurveEffect, Vec2>(
            &mut commands,
            entity,
            transform,
            &time,
            &effect.0,
        ) else {
            continue;
        };

        curve.0 = sampled;
    }
}

pub fn update_size_effect(
    query: Query<(
        Entity,
        &mut Transform,
        &mut physics::components::BoundingSphere,
        &brick::components::RicochetSizeEffect,
    )>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut transform, mut bounding_sphere, effect) in query {
        let Some(sampled) = sample_ricochet_effect::<brick::components::RicochetSizeEffect, f32>(
            &mut commands,
            entity,
            &transform,
            &time,
            &effect.0,
        ) else {
            continue;
        };

        transform.scale = Vec3::ONE * sampled;
        bounding_sphere.radius = sampled / 2.0;
    }
}

pub fn animate_light_fx(
    query: Query<(
        &brick::components::LightFX,
        &MeshMaterial3d<StandardMaterial>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (border_effect, material_handle) in &query {
        let t = time.elapsed_secs();

        let (pulse, [r, g, b]) = match &border_effect.0.effect_type {
            brick::assets::LightFXType::Pulse { speed, color } => {
                ((t * speed).sin() * 0.5 + 0.5, *color)
            }
            brick::assets::LightFXType::Wave { speed, color } => {
                ((t * speed).sin() * 0.5 + 0.5, *color)
            }
            brick::assets::LightFXType::Strobe { speed, color } => {
                (((t * speed).sin() * 0.5 + 0.5).powf(8.0), *color)
            }
            brick::assets::LightFXType::Interference {
                freq1,
                freq2,
                color,
            } => (((t * freq1).sin() * (t * freq2).sin()) * 0.5 + 0.5, *color),
        };

        if let Some(mat) = materials.get_mut(material_handle) {
            mat.emissive = LinearRgba::rgb(
                r * pulse * border_effect.0.intensity,
                g * pulse * border_effect.0.intensity,
                b * pulse * border_effect.0.intensity,
            );
        }
    }
}
