use bevy::prelude::*;

use crate::{audio, physics};

pub fn play_collide_sfx(
    mut commands: Commands,
    sfx_query: Query<&audio::components::CollisionSFX>,
    mut collision_messages: MessageReader<physics::messages::CollisionMessage>,
) {
    for message in collision_messages.read() {
        for &entity in [message.a, message.b].iter() {
            if let Ok(sfx) = sfx_query.get(entity) {
                commands
                    .spawn_empty()
                    .insert((AudioPlayer::new(sfx.0.clone()), PlaybackSettings::DESPAWN));
            } else {
                debug!(
                    "Entity from collision {:?} exists but has no sound effect.",
                    entity
                );
            }
        }
    }
}
