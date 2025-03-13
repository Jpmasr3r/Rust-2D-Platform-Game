use bevy::prelude::*;

use crate::modules::AnimationConfig;

pub fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in query.iter_mut() {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if config.current_frame >= config.last_frame {
                    config.current_frame = config.first_frame;
                } else {
                    config.current_frame += 1;
                }
                atlas.index = config.current_frame;
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}
