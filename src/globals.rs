#![allow(unused)]

use crate::consts::*;
use macroquad::{experimental::collections::storage, prelude::Vec2};


pub fn init_global_settings(settings: Settings) {
    storage::store(settings);
}

pub fn get_settings() -> Settings {
    return *storage::get::<Settings>();
}

pub fn mod_settings() -> Settings {
    return *storage::get_mut::<Settings>();
}


pub fn init_global_signals(signals: Signals) {
    storage::store(signals);
}

pub fn get_signals() -> Signals {
    let signals = storage::get::<Signals>();
    return signals.clone();
}

pub fn mod_signals() -> Signals {
    let signals = storage::get_mut::<Signals>();
    return signals.clone();
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
    pub damage: f32,
    pub base_energy_cost: f32,
    pub move_energy_cost: f32,
    pub attack_energy_cost: f32,
    pub res_num: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            world_w: WORLD_W as i32,
            world_h: WORLD_H as i32,
            agent_eng_bar: true,
            agent_init_num: 60,
            agent_min_num: 27,
            agent_rotate: 1.7,
            agent_speed: 60.0,
            agent_size_min: 3,
            agent_size_max: 10,
            agent_vision_range: 300.0,
            show_network: true,
            show_specie: true,
            mutations: 0.2,
            neurolink_rate: 0.2,
            damage: 80.0,
            base_energy_cost: 0.3,
            move_energy_cost: 0.1,
            attack_energy_cost: 0.1,
            res_num: 0.2,

       }
    }
}

#[derive(Clone)]
pub struct Signals {
    pub world: Vec2,
    pub spawn_agent: bool,
    pub spawn_plant: bool,
    pub spawn_asteroid: bool,
    pub spawn_jet: bool,
    pub spawn_particles: bool,
    pub new_sim: bool,
    pub new_sim_name: String,
    pub new_settings: bool,
    pub save_selected: bool,
    pub save_sim: bool,
    pub ranking: bool,
}

impl Signals {
    
    pub fn new() -> Self {
        Self {
            world: Vec2::NAN,
            spawn_agent: false,
            spawn_plant: false,
            spawn_asteroid: false,
            spawn_jet: false,
            spawn_particles: false,
            new_sim: false,
            new_sim_name: String::new(),
            new_settings: false,
            save_selected: false,
            save_sim: false,
            ranking: false,
        }
    }
}