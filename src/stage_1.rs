use bevy::prelude::*;

use crate::{common::load_image_to_blocks, modules::*, player::player_setup};

pub fn setup_stage_1(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Camera2d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        CameraPlayer,
    ));

    for block in load_image_to_blocks("assets/stages/stage-1.png") {
        match block.2 {
            Block::Player => {
                player_setup(&mut commands, block.0, block.1);
            }
            Block::Ground => {
                let texture: Handle<Image> = asset_server.load("stages/ground_sheet.png");
                let size = 16.;
                let layout = TextureAtlasLayout::from_grid(
                    UVec2 {
                        x: size as u32,
                        y: size as u32,
                    },
                    3,
                    3,
                    None,
                    None,
                );
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                commands.spawn((
                    Sprite {
                        image: texture,
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: 1,
                        }),
                        ..Default::default()
                    },
                    Transform {
                        translation: Vec3 {
                            x: block.0 * size,
                            y: block.1 * size,
                            z: 0.,
                        },
                        ..Default::default()
                    },
                    Ground,
                    Collider {
                        height: size,
                        width: size,
                    },
                ));
            }
            Block::FakeGround => {
                let texture: Handle<Image> = asset_server.load("stages/ground_sheet.png");
                let size = 16.;
                let layout = TextureAtlasLayout::from_grid(
                    UVec2 {
                        x: size as u32,
                        y: size as u32,
                    },
                    3,
                    3,
                    None,
                    None,
                );
                let texture_atlas_layout = texture_atlas_layouts.add(layout);

                commands.spawn((
                    Sprite {
                        image: texture,
                        texture_atlas: Some(TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: 4,
                        }),
                        ..Default::default()
                    },
                    Transform {
                        translation: Vec3 {
                            x: block.0 * size,
                            y: block.1 * size,
                            z: 0.,
                        },
                        ..Default::default()
                    },
                ));
            }
            Block::None => {}
        }
    }
}
