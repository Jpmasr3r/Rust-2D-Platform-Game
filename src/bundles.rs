use bevy::prelude::*;

use crate::{GRAVITY, modules::*};

pub fn bundle_red_turtle(
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    block: &(f32, f32, Block),
) -> (
    Sprite,
    Transform,
    AnimationConfig,
    Collider,
    Movable,
    Enemies,
) {
    let texture: Handle<Image> = asset_server.load("enemies/red turtle/red_turtle_walk.png");
    let layout: TextureAtlasLayout = TextureAtlasLayout::from_grid(
        UVec2 { x: 16, y: 27 },
        2,
        1,
        Some(UVec2 { x: 1, y: 0 }),
        None,
    );
    let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);

    let bundle: (
        Sprite,
        Transform,
        AnimationConfig,
        Collider,
        Movable,
        Enemies,
    ) = (
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
                x: block.0 * 16.,
                y: block.1 * 16.,
                z: 0.,
            },
            ..Default::default()
        },
        AnimationConfig::new(5, 0, 1),
        Collider {
            height: 16.,
            width: 27.,
        },
        Movable {
            max_vel_x: 50.,
            max_vel_y: (2. * GRAVITY * 32.).sqrt(),
            vel_x: 0.,
            vel_y: 0.,
        },
        Enemies {
            state: EnemiesState::Walking,
            speed: 50.,
            enemies_type: EnemiesType::RedTurtle,
        },
    );

    bundle
}

pub fn bundle_ground(
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    block: &(f32, f32, Block),
) -> (Sprite, Transform, Ground, Collider) {
    let texture: Handle<Image> = asset_server.load("stages/ground_sheet.png");
    let size: f32 = 16.;
    let layout: TextureAtlasLayout = TextureAtlasLayout::from_grid(
        UVec2 {
            x: size as u32,
            y: size as u32,
        },
        3,
        3,
        None,
        None,
    );
    let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);

    let bundle: (Sprite, Transform, Ground, Collider) = (
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
    );

    bundle
}

pub fn bundle_fake_ground(
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    block: &(f32, f32, Block),
) -> (Sprite, Transform) {
    let texture: Handle<Image> = asset_server.load("stages/ground_sheet.png");
    let size: f32 = 16.;
    let layout: TextureAtlasLayout = TextureAtlasLayout::from_grid(
        UVec2 {
            x: size as u32,
            y: size as u32,
        },
        3,
        3,
        None,
        None,
    );
    let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);

    let bundle: (Sprite, Transform) = (
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
    );

    bundle
}

pub fn bundle_power_block(
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    block: &(f32, f32, Block),
) -> (Sprite, Transform, PowerBlock, Collider, Ground) {
    let texture: Handle<Image> = asset_server.load("stages/mario-3-power-block.png");
    let size: f32 = 16.;
    let layout: TextureAtlasLayout = TextureAtlasLayout::from_grid(
        UVec2 {
            x: size as u32,
            y: size as u32,
        },
        1,
        1,
        None,
        None,
    );
    let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);

    let bundle: (Sprite, Transform, PowerBlock, Collider, Ground) = (
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
        PowerBlock {
            power_up: PowerUps::Mushroom,
        },
        Collider {
            height: 16.,
            width: 16.,
        },
        Ground,
    );

    bundle
}
