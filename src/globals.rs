use crate::consts::*;
use macroquad::experimental::collections::storage;


pub fn init_global_settings(settings: Settings) {
    storage::store(settings);
}

pub fn get_settings() -> Settings {
    return *storage::get::<Settings>();
}

pub fn mod_settings() -> Settings {
    return *storage::get_mut::<Settings>();
}


#[derive(Clone, Copy)]
pub struct Settings {
    pub world_w: i32,
    pub world_h: i32,
    pub agent_min_num: usize,
    pub agent_init_num: usize,
    pub agent_speed: f32,
    pub agent_rotate: f32,
    pub agent_eng_bar: bool,
    pub agent_vision_range: f32,
    pub agent_size_min: i32,
    pub agent_size_max: i32,
    pub show_network: bool,
    pub show_specie: bool,
    pub mutations: f32,
    pub neurolink_rate: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            world_w: WORLD_W as i32,
            world_h: WORLD_H as i32,
            agent_eng_bar: true,
            agent_init_num: 75,
            agent_min_num: 16,
            agent_rotate: 2.0,
            agent_speed: 100.0,
            agent_size_min: 5,
            agent_size_max: 12,
            agent_vision_range: 300.0,
            show_network: true,
            show_specie: true,
            mutations: 0.2,
            neurolink_rate: 0.2,
        }
    }
}