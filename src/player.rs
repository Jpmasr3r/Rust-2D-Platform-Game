use bevy::prelude::*;

use crate::{GRAVITY, modules::*};

pub fn player_setup(commands: &mut Commands, x: f32, y: f32) {
    commands.spawn((
        Sprite {
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(x, y, 0.),
            ..Default::default()
        },
        Player {
            state: PlayerState::Idle,
        },
        AnimationConfig::new(10, 0, 0),
        Gravity {
            max_vel_x: 200.,
            max_vel_y: GRAVITY,
            vel_x: 0.,
            vel_y: 0.,
        },
    ));
}

pub fn player_sprite_controller(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut players: Query<(&mut Sprite, &Player, &mut AnimationConfig), With<Player>>,
) {
    for mut player in players.iter_mut() {
        let sprite: &str;
        let (width, height): (u32, u32);
        let (first, last): (usize, usize);

        match player.1.state {
            PlayerState::Idle => {
                sprite = "player/player_iddle.png";
                first = 0;
                last = 0;
                width = 14;
                height = 27;
            }
            PlayerState::Walking => {
                sprite = "player/player_walk.png";
                first = 0;
                last = 3;
                width = 16;
                height = 27;
            }
        }

        player.2.first_frame = first;
        player.2.last_frame = last;

        let texture: Handle<Image> = asset_server.load(sprite);
        let layout = TextureAtlasLayout::from_grid(
            UVec2 {
                x: width,
                y: height,
            },
            last as u32 + 1,
            1,
            Some(UVec2 { x: 1, y: 0 }),
            None,
        );
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        player.0.image = texture.clone();
        player.0.texture_atlas = Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player.2.current_frame,
        });
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<&mut Player, With<Player>>,
) {
    for mut player in players.iter_mut() {
        let new_movment = Vec3::new(
            ((keyboard_input.pressed(KeyCode::KeyA) as i8)
                - (keyboard_input.pressed(KeyCode::KeyD) as i8)) as f32,
            0.,
            0.,
        );

        if new_movment.x != 0. {
            player.state = PlayerState::Walking;
        } else {
            player.state = PlayerState::Idle;
        }
    }
}
