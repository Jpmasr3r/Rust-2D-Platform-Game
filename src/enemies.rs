use bevy::{ecs::system::Query, prelude::*};

use crate::{common::check_colision, modules::*};

pub fn enemies_movement(mut enemies_querry: Query<(&Enemies, &mut Movable), With<Enemies>>) {
    for mut enemies in enemies_querry.iter_mut() {
        match enemies.0.state {
            EnemiesState::Walking => {
                enemies.1.vel_x -= enemies.0.speed;
            }
            EnemiesState::Death => {}
        }
    }
}

pub fn enemies_state_controller(
    mut enemies_querry: Query<
        (&mut Enemies, &Transform, &Collider, &mut Movable),
        (With<Enemies>, Without<Player>),
    >,
    mut players_querry: Query<(&Player, &Transform, &Collider, &mut Movable), With<Player>>,
) {
    let mut enemies: Vec<_> = enemies_querry.iter_mut().collect();

    for i in 0..enemies.len() {
        for mut player in players_querry.iter_mut() {
            match enemies[i].0.state {
                EnemiesState::Walking => {
                    enemies[i].3.max_vel_x = 50.;
                    if player.1.translation.y - 16. > enemies[i].1.translation.y {
                        if check_colision((player.2, player.1), (enemies[i].2, enemies[i].1)) {
                            enemies[i].0.state = EnemiesState::Death;
                            enemies[i].3.max_vel_x = 0.;
                            player.3.vel_y = 65504.;
                        }
                    }
                }
                EnemiesState::Death => {
                    let hull_speed: f32 = 300.;
                    if check_colision((player.2, player.1), (enemies[i].2, enemies[i].1))
                        && enemies[i].3.vel_x == 0.
                    {
                        enemies[i].3.max_vel_x = hull_speed;
                        enemies[i].3.vel_x =
                            65504. * (player.3.vel_x - enemies[i].3.vel_x).signum();
                    }

                    for j in 0..enemies.len() {
                        if i == j {
                            continue;
                        }
                        if check_colision(
                            (enemies[j].2, enemies[j].1),
                            (enemies[i].2, enemies[i].1),
                        ) {
                            enemies[j].0.state = EnemiesState::Death;
                            enemies[j].3.max_vel_x = hull_speed;
                            enemies[j].3.vel_x = 65504. * enemies[i].3.vel_x.signum();
                            enemies[i].3.vel_x *= -1.;
                        }
                    }
                }
            }
        }
    }
}

pub fn enemies_sprite_controller(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut enemies_querry: Query<(&mut Sprite, &Enemies, &mut AnimationConfig), With<Enemies>>,
) {
    for mut enemies in enemies_querry.iter_mut() {
        let sprite: &str;
        let (width, height): (u32, u32);
        let (first, last): (usize, usize);
        match enemies.1.enemies_type {
            EnemiesType::RedTurtle => match enemies.1.state {
                EnemiesState::Walking => {
                    sprite = "enemies/red turtle/red_turtle_walk.png";
                    first = 0;
                    last = 1;
                    width = 16;
                    height = 27;
                }
                EnemiesState::Death => {
                    sprite = "enemies/red turtle/red_turtle_death.png";
                    first = 0;
                    last = 0;
                    width = 16;
                    height = 16;
                }
            },
        }
        enemies.2.first_frame = first;
        enemies.2.last_frame = last;

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

        enemies.0.image = texture.clone();
        enemies.0.texture_atlas = Some(TextureAtlas {
            layout: texture_atlas_layout.clone(),
            index: enemies.2.current_frame,
        });
    }
}

pub fn kill_player(
    mut players_querry: Query<
        (&mut Player, &Collider, &Transform, &mut Movable),
        (With<Player>, Without<Enemies>),
    >,
    mut enemies_querry: Query<
        (&Enemies, &Collider, &Transform, &mut Movable),
        (With<Enemies>, Without<Player>),
    >,
) {
    for mut player in players_querry.iter_mut() {
        for enemie in enemies_querry.iter_mut() {
            match enemie.0.state {
                EnemiesState::Death => {
                    if enemie.3.vel_x == 0. {
                        continue;
                    }
                    let mut new_transform: Transform = player.2.clone();
                    new_transform.translation.x += 5. * new_transform.translation.x.signum();
                    new_transform.translation.y += 30.;

                    if player.2.translation.y - 16. <= enemie.2.translation.y {
                        if check_colision((player.1, &new_transform), (enemie.1, enemie.2)) {
                            player.0.state = PlayerState::Death;
                            player.3.vel_y = 65504.;
                        }
                    }
                }
                _ => {
                    if player.2.translation.y - 16. <= enemie.2.translation.y {
                        if check_colision((player.1, player.2), (enemie.1, enemie.2)) {
                            player.0.state = PlayerState::Death;
                            player.3.vel_y = 65504.;
                        }
                    }
                }
            }
        }
    }
}
