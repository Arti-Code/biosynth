#![allow(unused)]

use macroquad::{experimental::collections::storage, prelude::Vec2};
use serde::{Deserialize, Serialize};


pub const SCREEN_WIDTH: f32 = 1800.0;
pub const SCREEN_HEIGHT: f32 = 900.0;
pub const WORLD_W: f32 = 7000.0;
pub const WORLD_H: f32 = 6000.0;
pub const ZOOM_RATE: f32 = 1.0 / 800.0;
pub const SCREEN_RATIO: f32 = SCREEN_WIDTH / SCREEN_HEIGHT;

pub fn set_global_settings(settings: Settings) {
    storage::store(settings);
}

pub fn get_settings() -> Settings {
    return storage::get::<Settings>().clone();
}



pub fn set_global_signals(signals: Signals) {
    storage::store(signals);
}

pub fn get_signals() -> Signals {
    let signals = storage::get::<Signals>();
    return signals.clone();
}

pub fn set_mutations(stats: MutationStats) {
    storage::store(stats);
}

pub fn get_mutations() -> MutationStats {
    let stats = storage::get::<MutationStats>();
    return stats.clone();
}

fn default_size_cost() -> f32 {
    return 1.0;
}

fn default_base_hp() -> i32 {
    return 100;
}

fn default_size_to_hp() -> f32 {
    return 10.0;
}

fn default_res_prob() -> f32 {
    return 0.1;
}

fn default_growth() -> f32 {
    return 5.0;
}

fn default_water_lvl() -> u8 {
    return 4;
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
    pub show_cells: bool,
    pub show_res_rad: bool,
    pub mutations: f32,
    pub neurolink_rate: f32,
    pub damage: f32,
    pub base_energy_cost: f32,
    pub move_energy_cost: f32,
    pub attack_energy_cost: f32,
    #[serde(default = "default_size_cost")]
    pub size_cost: f32,
    #[serde(default = "default_base_hp")]
    pub base_hp: i32,
    #[serde(default = "default_size_to_hp")]
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
    #[serde(default = "default_res_prob")]
    pub resource_probability: f32,
    #[serde(default = "default_growth")]
    pub growth: f32,
    #[serde(default = "default_water_lvl")]
    pub water_lvl: u8,
}

impl Default for Settings {

    fn default() -> Self {
        Self {
            world_w: WORLD_W as i32,
            world_h: WORLD_H as i32,
            agent_eng_bar: true,
            agent_init_num: 70,
            res_init_num: 50,
            res_balance: 2,
            res_detection_radius: 200.0,
            agent_min_num: 12,
            res_min_num: 5,
            agent_rotate: 40.0,
            agent_speed: 40.0,
            agent_size_min: 2,
            agent_size_max: 10,
            agent_vision_range: 450.0,
            show_network: true,
            show_specie: true,
            show_cells: false,
            show_res_rad: false,
            mutations: 0.5,
            neurolink_rate: 0.2,
            damage: 50.0,
            base_energy_cost: 0.2,
            move_energy_cost: 0.5,
            attack_energy_cost: 0.2,
            size_cost: 1.5,
            base_hp: 100,
            size_to_hp: 75.0,
            res_num: 70.0,
            hidden_nodes_num: 0,
            neuro_duration: 0.15,
            atk_to_eng: 0.7,
            eat_to_eng: 6.0,
            ranking_size: 20,
            repro_points: 300.0,
            repro_time: 75.0,
            new_one_probability: 0.04,
            grid_size: 50,
            follow_mode: false,
            resource_probability: 0.5,
            growth: 5.0,
            water_lvl: 0,
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
    pub load_sim: bool,
    pub load_sim_name: Option<String>,
    pub del_sim_name: Option<String>,
    pub save_agent: bool,
    pub load_agent: bool,
    pub load_agent_name: Option<String>,
    pub del_agent_name: Option<String>,
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
            load_sim: false,
            del_sim_name: None,
            load_sim_name: None,
            load_agent: false,
            del_agent_name: None,
            load_agent_name: None,
            save_agent: false,
        }
    }

}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MutationStats {
    pub interval: f32,
    //temp_mutation_rates: Vec<f32>,
    pub mutation_rate: f32,
    pub nodes_added: i32,
    pub nodes_deleted: i32,
    pub links_added: i32,
    pub links_deleted: i32,
    pub biases_changed: i32,
    pub weights_changed: i32,
}

impl MutationStats {

    pub fn new(interval: f32, mutation_rate: f32) -> Self {
        Self { interval, mutation_rate, nodes_added: 0, nodes_deleted: 0, links_added: 0, links_deleted: 0, biases_changed: 0, weights_changed: 0 }
    }

    pub fn add_values(&mut self, nodes_added: i32, nodes_deleted: i32, links_added: i32, links_deleted: i32, biases_changed: i32, weights_changed: i32) {
        self.nodes_added += nodes_added;
        self.nodes_deleted += nodes_deleted;
        self.links_added += links_added;
        self.links_deleted += links_deleted;
        self.biases_changed += biases_changed;
        self.weights_changed += weights_changed;
    }

    pub fn print_data(&self) {
        println!("-------------------");
        println!("Mutation stats");
        //println!("Interval: {}", self.interval);
        //println!("Mutation rate: {:?}", self.mutation_rate);
        println!("nodes added:      {}", self.nodes_added);
        println!("nodes deleted:    {}", self.nodes_deleted);
        println!("links added:      {}", self.links_added);
        println!("links deleted:    {}", self.links_deleted);
        println!("biases changed:   {}", self.biases_changed);
        println!("weights changed:  {}", self.weights_changed);
        println!("-------------------");
    }

}