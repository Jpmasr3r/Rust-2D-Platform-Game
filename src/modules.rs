use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Player {
    pub state: PlayerState,
    pub jump_force: f32,
}

#[derive(Debug)]
pub enum PlayerState {
    Idle,
    Walking,
    Jumping,
    Death,
}

#[derive(Component)]
pub struct AnimationConfig {
    pub fps: u8,
    pub first_frame: usize,
    pub last_frame: usize,
    pub frame_timer: Timer,
    pub current_frame: usize,
}

impl AnimationConfig {
    pub fn new(fps: u8, first_frame: usize, last_frame: usize) -> Self {
        Self {
            fps,
            first_frame,
            last_frame,
            frame_timer: Self::timer_from_fps(fps),
            current_frame: first_frame,
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::from_seconds(1. / (fps as f32), TimerMode::Once)
    }
}

#[derive(Component)]
pub struct Movable {
    pub vel_y: f32,
    pub vel_x: f32,
    pub max_vel_y: f32,
    pub max_vel_x: f32,
    pub in_grav: bool,
}

pub enum Block {
    Player,
    Ground,
    FakeGround,
    RedTurtle,
    PowerBlock,
    None,
}

#[derive(Component)]
pub struct CameraPlayer;

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Collider {
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
pub struct Enemies {
    pub state: EnemiesState,
    pub speed: f32,
    pub enemies_type: EnemiesType,
}

pub enum EnemiesType {
    RedTurtle,
}

pub enum EnemiesState {
    Walking,
    Death,
}

#[derive(Component)]
pub struct PowerBlock {
    pub power_up: PowerUps,
}

pub enum PowerUps {
    Mushroom,
    FireFlower,
    TanukiSuit,
}
