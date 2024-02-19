#![allow(unused)]

use macroquad::{experimental::collections::storage, prelude::Vec2};
use serde::{Deserialize, Serialize};

use crate::globals::*;
pub fn set_settings(settings: Settings) {
    storage::store(settings);
}

pub fn settings() -> Settings {
    return storage::get::<Settings>().clone();
}

pub fn sim_speed() -> f32 {
    return settings().sim_speed;
}
fn default_stats_limit() -> usize {
    return 100;
}


fn default_eng_bias() -> f32 {
    return 0.3;
}

fn default_pause() -> bool {
    return false;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub world_w: i32,
    pub world_h: i32,
    pub agent_min_num: usize,
    pub plant_min_num: usize,
    pub agent_init_num: usize,
    pub plant_init_num: usize,
    pub plant_balance: usize,
    pub plant_detection_radius: f32,
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
    pub plant_num: f32,
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
    pub plant_probability: f32,
    pub plant_lifetime: f32,
    pub growth: f32,
    pub water_lvl: u8,
    pub mut_add_link: f32,
    pub mut_del_link: f32,
    pub mut_add_node: f32,
    pub mut_del_node: f32,
    pub mut_change_val: f32,
    pub rare_specie_mod: i32,
    pub born_eng: f32,
    pub sim_speed: f32,
    #[serde(default = "default_stats_limit")]
    pub stats_limit: usize,
    #[serde(default = "default_pause")]
    pub pause: bool,
    #[serde(default = "default_eng_bias")]
    pub eng_bias: f32,
}

impl Default for Settings {

    fn default() -> Self {
        Self {
            world_w: 3000,
            world_h: 2000,
            agent_eng_bar: true,
            agent_init_num: 100,
            plant_init_num: 100,
            plant_balance: 3,
            plant_detection_radius: 200.0,
            agent_min_num: 20,
            plant_min_num: 10,
            agent_rotate: 50.0,
            agent_speed: 30.0,
            agent_size_min: 2,
            agent_size_max: 10,
            agent_vision_range: 350.0,
            show_network: true,
            show_specie: true,
            show_generation: true,
            show_cells: false,
            show_plant_rad: false,
            mutations: 0.1,
            neurolink_rate: 0.1,
            damage: 60.0,
            base_energy_cost: 0.3,
            move_energy_cost: 0.4,
            attack_energy_cost: 0.1,
            size_cost: 1.8,
            base_hp: 150,
            size_to_hp: 55.0,
            plant_num: 75.0,
            hidden_nodes_num: 5,
            neuro_duration: 0.25,
            atk_to_eng: 1.3,
            eat_to_eng: 5.0,
            ranking_size: 30,
            repro_points: 30.0,
            repro_time: 100.0,
            new_one_probability: 0.2,
            grid_size: 50,
            follow_mode: false,
            plant_probability: 0.6,
            plant_lifetime: 300.0,
            growth: 6.0,
            water_lvl: 0,
            mut_add_link: 0.02,
            mut_del_link: 0.02,
            mut_add_node: 0.01,
            mut_change_val: 0.04,
            mut_del_node: 0.01,
            rare_specie_mod: 1500,
            born_eng: 0.8,
            sim_speed: 1.0,
            stats_limit: 25,
            pause: false,
            eng_bias: 0.3,
       }
    }

}