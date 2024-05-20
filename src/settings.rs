#![allow(unused)]

use macroquad::{experimental::collections::storage, prelude::Vec2};
use serde::{Deserialize, Serialize};

use crate::globals::*;
pub fn set_settings(settings: Settings) {
    storage::store(settings);
}

pub fn get_settings() -> Settings {
    return storage::get::<Settings>().clone();
}

pub fn sim_speed() -> f32 {
    return get_settings().sim_speed;
}

fn born_eng_cost() -> f32 {
    return 0.5;
}

fn peripheral_vision() -> f32 {
    return 0.25;
}

fn default_debug() -> bool {
    return false;
}

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
    pub water_lvl: i32,
    pub mut_add_link: f32,
    pub mut_del_link: f32,
    pub mut_add_node: f32,
    pub mut_del_node: f32,
    pub mut_change_val: f32,
    pub rare_specie_mod: i32,
    pub born_eng: f32,
    pub born_eng_min: f32,
    #[serde(default = "born_eng_cost")]
    pub born_eng_cost: f32,
    pub sim_speed: f32,
    pub stats_limit: usize,
    pub pause: bool,
    pub eng_bias: f32,
    pub dmg_to_hp: f32,
    #[serde(default = "peripheral_vision")]
    pub peripheral_vision: f32,
    #[serde(default = "default_debug")]
    pub debug: bool,
}

impl Default for Settings {

    fn default() -> Self {
        Self {
            world_w: 3000,
            world_h: 3000,
            
            agent_eng_bar: true,
            agent_init_num: 100,
            agent_min_num: 10,
            agent_rotate: 50.0,
            agent_speed: 50.0,
            agent_size_min: 4,
            agent_size_max: 10,
            agent_vision_range: 400.0,

            mutations: 0.25,
            damage: 100.0,
            base_energy_cost: 0.25,
            move_energy_cost: 0.3,
            attack_energy_cost: 0.1,
            size_cost: 2.5,
            base_hp: 250,
            size_to_hp: 50.0,
            eng_bias: 0.15,
            dmg_to_hp: 0.2,
            peripheral_vision: 0.25,
            
            plant_init_num: 500,
            plant_balance: 10,
            plant_lifetime: 350.0,
            growth: 6.0,
            plant_min_num: 10,
            plant_clone_size: 8,
            
            neurolink_rate: 0.05,
            hidden_nodes_num: 0,
            hidden_layers_num: 0,
            neuro_duration: 0.1,
            mut_add_link: 0.003,
            mut_del_link: 0.003,
            mut_add_node: 0.002,
            mut_change_val: 0.008,
            mut_del_node: 0.002,

            atk_to_eng: 0.8,
            eat_to_eng: 2.5,
            
            ranking_size: 40,
            repro_points: 30.0,
            rare_specie_mod: 2500,
            born_eng: 0.5,
            born_eng_min: 0.9,
            born_eng_cost: 0.5,
            repro_time: 100.0,
            new_one_probability: 0.2,
            
            grid_size: 40,
            water_lvl: 0,
            follow_mode: false,
            show_network: true,
            show_specie: true,
            show_generation: true,
            show_cells: false,
            show_plant_rad: false,

            sim_speed: 1.0,
            stats_limit: 20,
            pause: false,
            debug: false,
       }
    }

}