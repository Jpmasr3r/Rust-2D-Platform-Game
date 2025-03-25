use bevy::{ecs::system::Query, prelude::*};

use crate::{common::check_colision, modules::*};

pub fn enemy_movement(mut enemys_querry: Query<(&Enemy, &mut Movable), With<Enemy>>) {
    for mut enemy in enemys_querry.iter_mut() {
        match enemy.0.state {
            EnemyState::Walking => {
                if enemy.1.vel_x.abs() < enemy.1.max_vel_x {
                    enemy.1.vel_x -= enemy.0.speed;
                }
            }
            _ => {
                enemy.1.vel_x = 0.;
                continue;
            }
        }
    }
}

pub fn enemy_state_controller(
    mut enemys_querry: Query<(&mut Enemy, &Transform, &Collider), With<Enemy>>,
    mut players_querry: Query<(&Player, &Transform, &Collider, &mut Movable), With<Player>>,
) {
    for mut enemy in enemys_querry.iter_mut() {
        for mut player in players_querry.iter_mut() {
            if player.1.translation.y - 16. > enemy.1.translation.y {
                if check_colision((player.2, player.1), (enemy.2, enemy.1)) {
                    enemy.0.state = EnemyState::Death;
                    player.3.vel_y = f32::MAX;
                }
            }
        }
    }
}

pub fn enemy_sprite_controller(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut enemys_querry: Query<(&mut Sprite, &Enemy, &mut AnimationConfig), With<Enemy>>,
) {
    for mut enemy in enemys_querry.iter_mut() {
        let sprite: &str;
        let (width, height): (u32, u32);
        let (first, last): (usize, usize);
        match enemy.1.enemy_type {
            EnemyType::RedTurtle => match enemy.1.state {
                EnemyState::Walking => {
                    sprite = "enemys/red turtle/red_turtle_walk.png";
                    first = 0;
                    last = 1;
                    width = 16;
                    height = 27;
                }
                EnemyState::Death => {
                    sprite = "enemys/red turtle/red_turtle_death.png";
                    first = 0;
                    last = 0;
                    width = 16;
                    height = 16;
                }
            },
        }
        enemy.2.first_frame = first;
        enemy.2.last_frame = last;

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

        enemy.0.image = texture.clone();
        enemy.0.texture_atlas = Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: enemy.2.current_frame,
        });
    }
}

pub fn kill_player(
    mut players_querry: Query<(&mut Player, &Collider, &Transform, &mut Movable), With<Player>>,
    enemys_querry: Query<(&Enemy, &Collider, &Transform), With<Enemy>>,
) {
    for mut player in players_querry.iter_mut() {
        for enemy in enemys_querry.iter() {
            match enemy.0.state {
                EnemyState::Death => {
                    continue;
                }
                _ => {}
            }
            if player.2.translation.y - 16. <= enemy.2.translation.y {
                if check_colision((player.1, player.2), (enemy.1, enemy.2)) {
                    player.0.state = PlayerState::Death;
                    player.3.vel_y = f32::MAX;
                }
            }
        }
    }
}
