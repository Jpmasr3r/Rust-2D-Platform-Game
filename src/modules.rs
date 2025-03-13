use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
}

pub enum PlayerState {
    Idle,
    Walking,
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
