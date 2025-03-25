use bevy::prelude::*;

use crate::{GRAVITY, common::check_colision, modules::*};

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
            speed: 50.,
            jump_force: (2.0 * GRAVITY * 64.).sqrt(),
        },
        AnimationConfig::new(10, 0, 0),
        Movable {
            max_vel_x: 200.,
            max_vel_y: (2.0 * GRAVITY * 32.).sqrt(),
            vel_x: 0.,
            vel_y: 0.,
        },
        Collider {
            width: 16.,
            height: 27.,
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
            PlayerState::Jumping => {
                sprite = "player/player_jump.png";
                first = 0;
                last = 0;
                width = 16;
                height = 26;
            }
            PlayerState::Death => {
                sprite = "player/player_death.png";
                first = 0;
                last = 0;
                width = 16;
                height = 16;
            }
        }

        player.2.first_frame = first;
        player.2.last_frame = last;

        let texture: Handle<Image> = asset_server.load(sprite);
        let layout: TextureAtlasLayout = TextureAtlasLayout::from_grid(
            UVec2 {
                x: width,
                y: height,
            },
            last as u32 + 1,
            1,
            Some(UVec2 { x: 1, y: 0 }),
            None,
        );
        let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);

        player.0.image = texture.clone();
        player.0.texture_atlas = Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: player.2.current_frame,
        });
    }
}

pub fn player_state_controller(
    mut players: Query<(&mut Player, &mut Movable, &mut Sprite, &mut Collider), With<Player>>,
) {
    for mut player in players.iter_mut() {
        match player.1.vel_x.signum() {
            1. => {
                player.2.flip_x = false;
            }
            -1. => {
                player.2.flip_x = true;
            }
            _ => {
                player.2.flip_x = player.2.flip_x;
            }
        }

        match player.0.state {
            PlayerState::Death => {
                player.1.vel_x = 0.;
                player.3.width = -1.;
                player.3.height = -1.;
            }
            _ => {
                if player.1.vel_y != 0. {
                    player.0.state = PlayerState::Jumping;
                } else if player.1.vel_x != 0. {
                    player.0.state = PlayerState::Walking;
                } else {
                    player.0.state = PlayerState::Idle;
                }
            }
        }
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<(&mut Player, &mut Movable, &mut Sprite), With<Player>>,
) {
    let new_movment = ((keyboard_input.pressed(KeyCode::KeyD) as i8)
        - (keyboard_input.pressed(KeyCode::KeyA) as i8)) as f32;
    for mut player in players.iter_mut() {
        match player.0.state {
            PlayerState::Death => {
                continue;
            }
            _ => {}
        }
        if player.1.vel_x.abs() >= player.1.max_vel_x {
            player.1.vel_x = player.1.max_vel_x * player.1.vel_x.signum();
        }

        if new_movment != 0. {
            player.1.vel_x += new_movment * player.0.speed;
        } else {
            player.1.vel_x = 0.;
        }
    }
}

pub fn player_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut players: Query<
        (&mut Player, &mut Movable, &Collider, &Transform),
        (With<Player>, Without<Ground>),
    >,
    ground_querry: Query<(&Collider, &Transform), With<Ground>>,
) {
    let new_movment: f32 = (keyboard_input.pressed(KeyCode::Space) as i8) as f32;
    for mut player in players.iter_mut() {
        match player.0.state {
            PlayerState::Death => {
                continue;
            }
            _ => {}
        }
        let mut can_jump: bool = false;
        let mut entity_transform: Transform = player.3.clone();
        entity_transform.translation.y -= 5.;

        for ground in ground_querry.iter() {
            if check_colision((player.2, &entity_transform), ground) {
                can_jump = true;
            }
        }
        if new_movment != 0. && can_jump {
            player.1.vel_y += new_movment * player.0.jump_force;
        }
    }
}
