#![allow(unused)]

use macroquad::{experimental::collections::storage, prelude::Vec2};
use serde::{Deserialize, Serialize};


pub const SCREEN_WIDTH: f32 = 1900.0;
pub const SCREEN_HEIGHT: f32 = 980.0;
pub const WORLD_W: f32 = 3000.0;
pub const WORLD_H: f32 = 2000.0;
pub const ZOOM_RATE: f32 = 1.0 / 800.0;
pub const SCREEN_RATIO: f32 = SCREEN_WIDTH / SCREEN_HEIGHT;
