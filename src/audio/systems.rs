use bevy::{audio::Volume, prelude::*};

use crate::{audio, physics};

pub fn play_collide_sfx(
    mut commands: Commands,
    sfx_query: Query<&audio::components::CollisionSFX>,
    mut collision_messages: MessageReader<physics::messages::CollisionMessage>,
) {
    for message in collision_messages.read() {
        for &entity in [message.a, message.b].iter() {
            if let Ok(sfx) = sfx_query.get(entity) {
                let settings = PlaybackSettings::DESPAWN.with_volume(Volume::Linear(sfx.volume));
                commands
                    .spawn_empty()
                    .insert((AudioPlayer::new(sfx.handle.clone()), settings));
            } else {
                debug!(
                    "Entity from collision {:?} exists but has no sound effect.",
                    entity
                );
            }
        }
    }
}
