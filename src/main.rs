use bevy::prelude::*;
use common::*;
use player::*;
use world::*;

mod common;
mod modules;
mod player;
mod world;

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
    app.add_systems(Startup, setup);
}
fn update(app: &mut App) {
    app.add_systems(Update, execute_animations);
    app.add_systems(Update, player_sprite_controller);
    app.add_systems(Update, player_movement);
}
