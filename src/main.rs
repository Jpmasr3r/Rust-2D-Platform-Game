use bevy::prelude::*;
use common::*;
use enemys::*;
use player::*;
use stage_1::*;

mod bundles;
mod common;
mod enemys;
mod modules;
mod player;
mod stage_1;

const GRAVITY: f32 = 9.8 * 50.;

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
    //Others
    app.add_systems(Update, execute_animations);
    app.add_systems(Update, camera_controller);
    app.add_systems(Update, acceleration);
    app.add_systems(Update, gravity);

    //Player Updates
    app.add_systems(Update, player_movement);
    app.add_systems(Update, player_jump);
    app.add_systems(Update, player_sprite_controller);
    app.add_systems(Update, player_state_controller);

    //Enemy Updates
    app.add_systems(Update, enemy_movement);
    app.add_systems(Update, enemy_state_controller);
    app.add_systems(Update, enemy_sprite_controller);
    app.add_systems(Update, kill_player);
}
