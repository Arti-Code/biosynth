#![allow(unused)]

use macroquad::{experimental::collections::storage, prelude::Vec2};
use serde::{Deserialize, Serialize};


pub const SCREEN_WIDTH: f32 = 1200.0;
pub const SCREEN_HEIGHT: f32 = 800.0;
pub const WORLD_W: f32 = 3000.0;
pub const WORLD_H: f32 = 2000.0;
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub neuro_duration: f32,
    pub hidden_nodes_num: usize,
    pub atk_to_eng: f32,
    pub eat_to_eng: f32,
    pub ranking_size: usize,
    pub repro_points: f32,
    pub repro_time: f32,
    pub new_one_probability: f32,
    pub grid_size: u32,
}

impl Default for Settings {

    fn default() -> Self {
        Self {
            world_w: WORLD_W as i32,
            world_h: WORLD_H as i32,
            agent_eng_bar: true,
            agent_init_num: 70,
            agent_min_num: 12,
            agent_rotate: 1.0,
            agent_speed: 40.0,
            agent_size_min: 2,
            agent_size_max: 10,
            agent_vision_range: 300.0,
            show_network: true,
            show_specie: true,
            mutations: 0.4,
            neurolink_rate: 0.2,
            damage: 40.0,
            base_energy_cost: 0.4,
            move_energy_cost: 0.3,
            attack_energy_cost: 0.3,
            res_num: 60.0,
            hidden_nodes_num: 0,
            neuro_duration: 0.3,
            atk_to_eng: 0.8,
            eat_to_eng: 9.0,
            ranking_size: 20,
            repro_points: 250.0,
            repro_time: 75.0,
            new_one_probability: 0.03,
            grid_size: 50,
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