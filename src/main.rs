use bevy::prelude::*;
use common::*;
use player::*;
use stage_1::*;

mod common;
mod modules;
mod player;
mod stage_1;

const GRAVITY: f32 = 9.8 * 5.;

fn main() {
    let mut app = App::new();
    plugin(&mut app);
    startup(&mut app);
    update(&mut app);
    app.run();
}

fn plugin(app: &mut App) {
    app.add_plugins(DefaultPlugins);
}
fn startup(app: &mut App) {
    app.add_systems(Startup, setup_stage_1);
}
fn update(app: &mut App) {
    app.add_systems(Update, execute_animations);
    app.add_systems(Update, player_sprite_controller);
    app.add_systems(Update, player_movement);
    app.add_systems(Update, acceleration);
    app.add_systems(Update, gravity);
    app.add_systems(Update, camera_controller);
}
