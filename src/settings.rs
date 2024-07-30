#![allow(unused)]

use macroquad::{experimental::collections::storage, prelude::Vec2};
use serde::{Deserialize, Serialize};

//use crate::globals::*;


pub const SCREEN_WIDTH: f32 = 1800.0;
pub const SCREEN_HEIGHT: f32 = 950.0;
pub const WORLD_W: f32 = 3000.0;
pub const WORLD_H: f32 = 3000.0;
pub const ZOOM_RATE: f32 = 1.0 / 800.0;
pub const SCREEN_RATIO: f32 = SCREEN_WIDTH / SCREEN_HEIGHT;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SelectMode {
    POINTS,
    LIFETIME,
    RANDOM,
    KILLS,
    CHILDS,
}


pub fn set_settings(settings: Settings) {
    storage::store(settings);
}

pub fn get_settings() -> Settings {
    return storage::get::<Settings>().clone();
}

pub fn sim_speed() -> f32 {
    return get_settings().sim_speed;
}

fn default_rank_decay() -> f32 { 0.1 }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub world_w: i32,
    pub world_h: i32,
    pub agent_min_num: usize,
    pub agent_init_num: usize,
    pub plant_min_num: usize,
    pub plant_init_num: usize,
    pub plant_balance: usize,
    pub plant_clone_size: i32,
    pub agent_speed: f32,
    pub agent_rotate: f32,
    pub agent_eng_bar: bool,
    pub agent_vision_range: f32,
    pub agent_size_min: i32,
    pub agent_size_max: i32,
    pub show_network: bool,
    pub show_specie: bool,
    pub show_generation: bool,
    pub show_cells: bool,
    pub show_plant_rad: bool,
    pub mutations: f32,
    pub neurolink_rate: f32,
    pub damage: f32,
    pub base_energy_cost: f32,
    pub move_energy_cost: f32,
    pub attack_energy_cost: f32,
    pub size_cost: f32,
    pub base_hp: i32,
    pub size_to_hp: f32,
    pub neuro_duration: f32,
    pub hidden_nodes_num: usize,
    pub hidden_layers_num: usize,
    pub atk_to_eng: f32,
    pub eat_to_eng: f32,
    pub ranking_size: usize,
    pub repro_points: f32,
    pub repro_time: f32,
    pub new_one_probability: f32,
    pub grid_size: u32,
    pub follow_mode: bool,
    pub plant_lifetime: f32,
    pub growth: f32,
    pub mut_add_link: f32,
    pub mut_del_link: f32,
    pub mut_add_node: f32,
    pub mut_del_node: f32,
    pub mut_change_val: f32,
    pub rare_specie_mod: i32,
    pub born_eng: f32,
    pub born_eng_min: f32,
    pub born_eng_cost: f32,
    pub sim_speed: f32,
    pub stats_limit: usize,
    pub pause: bool,
    pub eng_bias: f32,
    pub dmg_to_hp: f32,
    pub peripheral_vision: f32,
    pub debug: bool,
    pub select_mode: SelectMode,
    pub terrain_edit: bool,
    pub brush_size: usize,
    #[serde(default = "default_rank_decay")]
    pub rank_decay: f32,
}

impl Default for Settings {

    fn default() -> Self {
        Self {
            world_w: 2000,
            world_h: 2000,
            
            agent_eng_bar: true,
            agent_init_num: 100,
            agent_min_num: 25,
            agent_rotate: 50.0,
            agent_speed: 40.0,
            agent_size_min: 3,
            agent_size_max: 10,
            agent_vision_range: 400.0,

            mutations: 0.3,
            damage: 100.0,
            base_energy_cost: 0.2,
            move_energy_cost: 0.2,
            attack_energy_cost: 0.1,
            size_cost: 3.0,
            base_hp: 250,
            size_to_hp: 50.0,
            eng_bias: 0.1,
            dmg_to_hp: 0.1,
            peripheral_vision: 0.15,
            
            plant_init_num: 500,
            plant_balance: 40,
            plant_lifetime: 300.0,
            growth: 5.0,
            plant_min_num: 40,
            plant_clone_size: 6,
            
            neurolink_rate: 0.15,
            hidden_nodes_num: 5,
            hidden_layers_num: 4,
            neuro_duration: 0.2,
            mut_add_link: 0.025,
            mut_del_link: 0.010,
            mut_add_node: 0.020,
            mut_del_node: 0.008,
            mut_change_val: 0.035,

            atk_to_eng: 0.8,
            eat_to_eng: 2.5,
            
            ranking_size: 30,
            repro_points: 30.0,
            rare_specie_mod: 8,
            born_eng: 0.5,
            born_eng_min: 0.9,
            born_eng_cost: 0.5,
            repro_time: 100.0,
            new_one_probability: 0.2,
            
            grid_size: 20,
            follow_mode: false,
            show_network: true,
            show_specie: true,
            show_generation: true,
            show_cells: false,
            show_plant_rad: false,

            sim_speed: 1.0,
            stats_limit: 500,
            pause: false,
            debug: false,
            select_mode: SelectMode::RANDOM,
            terrain_edit: false,
            brush_size: 1,
            rank_decay: 0.1,
       }
    }

}