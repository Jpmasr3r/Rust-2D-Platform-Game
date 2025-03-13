use bevy::prelude::*;

use crate::player::player_setup;

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
    ));
    player_setup(&mut commands, 0.0, 0.0);
}
