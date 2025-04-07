use bevy::prelude::*;

use crate::{bundles::*, common::load_image_to_blocks, modules::*, player::player_setup};

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
                commands.spawn(bundle_ground(
                    &asset_server,
                    &mut texture_atlas_layouts,
                    &block,
                ));
            }
            Block::RedTurtle => {
                commands.spawn(bundle_red_turtle(
                    &asset_server,
                    &mut texture_atlas_layouts,
                    &block,
                ));
            }
            Block::FakeGround => {
                commands.spawn(bundle_fake_ground(
                    &asset_server,
                    &mut texture_atlas_layouts,
                    &block,
                ));
            }
            Block::PowerBlock => {
                commands.spawn(bundle_power_block(
                    &asset_server,
                    &mut texture_atlas_layouts,
                    &block,
                ));
            }
            Block::None => {}
        }
    }
}
