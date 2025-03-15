use bevy::prelude::*;

use crate::modules::*;
use image::GenericImageView;

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

pub fn camera_controller(
    mut camera: Single<&mut Transform, (With<CameraPlayer>, Without<Player>)>,
    player: Single<&Transform, With<Player>>,
) {
    camera.translation = player.translation;
}

pub fn acelleration(
    mut entity_querry: Query<(&mut Transform, &mut Gravity), With<Gravity>>,
    time: Res<Time>,
) {
    for mut entity in entity_querry.iter_mut() {
        entity.0.translation.y += entity.1.vel_y * time.delta_secs();
        entity.0.translation.x += entity.1.vel_x * time.delta_secs();

        if entity.1.vel_y > entity.1.max_vel_y {
            entity.1.vel_y = entity.1.max_vel_y;
        }
        if entity.1.vel_x > entity.1.max_vel_x {
            entity.1.vel_x = entity.1.max_vel_x;
        }
    }
}

pub fn gravity(
    mut entity_querry: Query<
        (&mut Gravity, &Collider, &mut Transform),
        (With<Gravity>, Without<Ground>),
    >,
    ground_querry: Query<(&Collider, &Transform), With<Ground>>,
) {
    for mut entity in entity_querry.iter_mut() {
        if entity.0.vel_y >= entity.0.max_vel_y {
            continue;
        }

        let mut in_grav: bool = true;

        let mut transform_a = entity.2.clone();
        transform_a.translation.y -= 1.;

        for ground in ground_querry.iter() {
            if check_colision((entity.1, &transform_a), ground) {
                in_grav = false;
                entity.0.vel_y = 0.;
                break;
            }
        }

        if in_grav {
            entity.0.vel_y -= 5.;
        } else {
            entity.0.vel_y = 0.;
        }
    }
}

pub fn check_colision(
    entity_a: (&Collider, &Transform),
    entity_b: (&Collider, &Transform),
) -> bool {
    let (collider_a, transform_a) = entity_a;
    let (collider_b, transform_b) = entity_b;

    let collision_x = (transform_a.translation.x - transform_b.translation.x).abs()
        < (collider_a.width + collider_b.width) / 2.0;
    let collision_y = (transform_a.translation.y - transform_b.translation.y).abs()
        < (collider_a.height + collider_b.height) / 2.0;

    collision_x && collision_y
}

pub fn load_image_to_blocks(image_path: &str) -> Vec<(f32, f32, Block)> {
    let img = image::open(image_path).expect("Failed to open image");
    let mut blocks = Vec::new();

    for (x, y, pixel) in img.pixels() {
        let rgba = pixel.0;
        let mut block = match (rgba[0], rgba[1], rgba[2]) {
            (128, 0, 128) => Block::Player,
            (255, 255, 0) => Block::Ground,
            (0, 0, 0) => Block::FakeGround,
            _ => Block::None,
        };
        if rgba[3] != 255 {
            block = Block::None;
        }
        blocks.push((x as f32, y as f32 * -1., block));
    }

    blocks
}
