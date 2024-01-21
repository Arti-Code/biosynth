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

fn default_plant_life() -> f32 {
    return 128.0;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub world_w: i32,
    pub world_h: i32,
    pub agent_min_num: usize,
    pub res_min_num: usize,
    pub agent_init_num: usize,
    pub res_init_num: usize,
    pub res_balance: usize,
    pub res_detection_radius: f32,
    pub agent_speed: f32,
    pub agent_rotate: f32,
    pub agent_eng_bar: bool,
    pub agent_vision_range: f32,
    pub agent_size_min: i32,
    pub agent_size_max: i32,
    pub show_network: bool,
    pub show_specie: bool,
    //#[serde(default = "default_false")]
    pub show_generation: bool,
    pub show_cells: bool,
    pub show_res_rad: bool,
    pub mutations: f32,
    pub neurolink_rate: f32,
    pub damage: f32,
    pub base_energy_cost: f32,
    pub move_energy_cost: f32,
    pub attack_energy_cost: f32,
    //#[serde(default = "default_size_cost")]
    pub size_cost: f32,
    //#[serde(default = "default_base_hp")]
    pub base_hp: i32,
    //#[serde(default = "default_size_to_hp")]
    pub size_to_hp: f32,
    pub res_num: f32,
    pub neuro_duration: f32,
    pub hidden_nodes_num: usize,
    pub atk_to_eng: f32,
    pub eat_to_eng: f32,
    pub ranking_size: usize,
    pub repro_points: f32,
    pub repro_time: f32,
    pub new_one_probability: f32,
    pub grid_size: u32,
    pub follow_mode: bool,
    //#[serde(default = "default_res_prob")]
    pub plant_probability: f32,
    //#[serde(default = "default_plant_life")]
    pub plant_lifetime: f32,
    //#[serde(default = "default_growth")]
    pub growth: f32,
    //#[serde(default = "default_water_lvl")]
    pub water_lvl: u8,
    //#[serde(default = "default_mutations")]
    pub mut_add_link: f32,
    //#[serde(default = "default_mutations")]
    pub mut_del_link: f32,
    //#[serde(default = "default_mutations")]
    pub mut_add_node: f32,
    //#[serde(default = "default_mutations")]
    pub mut_del_node: f32,
    //#[serde(default = "default_mutations")]
    pub mut_change_val: f32,
    //#[serde(default = "default_specie_mod")]
    pub rare_specie_mod: i32,
    //#[serde(default = "default_born_eng")]
    pub born_eng: f32,
}

impl Default for Settings {

    fn default() -> Self {
        Self {
            world_w: 3000,
            world_h: 2000,
            agent_eng_bar: true,
            agent_init_num: 100,
            res_init_num: 100,
            res_balance: 2,
            res_detection_radius: 200.0,
            agent_min_num: 20,
            res_min_num: 15,
            agent_rotate: 50.0,
            agent_speed: 40.0,
            agent_size_min: 2,
            agent_size_max: 10,
            agent_vision_range: 450.0,
            show_network: true,
            show_specie: true,
            show_generation: false,
            show_cells: false,
            show_res_rad: false,
            mutations: 0.1,
            neurolink_rate: 0.2,
            damage: 50.0,
            base_energy_cost: 0.2,
            move_energy_cost: 0.25,
            attack_energy_cost: 0.15,
            size_cost: 1.8,
            base_hp: 300,
            size_to_hp: 55.0,
            res_num: 70.0,
            hidden_nodes_num: 5,
            neuro_duration: 0.25,
            atk_to_eng: 1.4,
            eat_to_eng: 10.0,
            ranking_size: 20,
            repro_points: 300.0,
            repro_time: 150.0,
            new_one_probability: 0.1,
            grid_size: 50,
            follow_mode: false,
            plant_probability: 0.5,
            plant_lifetime: 128.0,
            growth: 5.0,
            water_lvl: 0,
            mut_add_link: 0.02,
            mut_del_link: 0.02,
            mut_add_node: 0.01,
            mut_change_val: 0.05,
            mut_del_node: 0.01,
            rare_specie_mod: 750,
            born_eng: 1.0,
       }
    }

}