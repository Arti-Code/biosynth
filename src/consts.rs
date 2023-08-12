#![allow(unused)]

pub const SCREEN_WIDTH: f32 = 1400.0;
pub const SCREEN_HEIGHT: f32 = 900.0;
pub const WORLD_W: f32 = 1400.0;
pub const WORLD_H: f32 = 900.0;
//pub const WORLD_W: f32              = 3200.0;
//pub const WORLD_H: f32              = 1800.0;

pub const AGENTS_NUM: usize = 10;
pub const AGENTS_NUM_MIN: usize = 4;
pub const AGENT_SIZE_MIN: i32 = 4;
pub const AGENT_SIZE_MAX: i32 = 12;
pub const AGENT_SPEED: f32 = 500.0;
pub const AGENT_VISION_RANGE: f32 = 240.0;
pub const AGENT_ROTATION: f32 = 120.0;
pub const AGENT_IMPULSE: f32 = 3.0;
pub const AGENT_TORQUE: f32 = 2.0;

pub const FIX_DT: f32 = 1.0 / 30.0;
pub const ZOOM_RATE: f32 = 1.0 / 800.0;
pub const SCREEN_RATIO: f32 = SCREEN_WIDTH / SCREEN_HEIGHT;

pub const PLANTS_NUM: usize = 0;
pub const PLANTS_MIN_NUM: usize = 0;
pub const PLANT_MAX_SIZE: i32 = 10;

pub const GRAV: f32 = 50.0;

pub const LIFE_SIZE_MIN: i32 = 4;
pub const LIFE_SIZE_MAX: i32 = 10;
pub const LIFE_NUM_MIN: i32 = 0;
