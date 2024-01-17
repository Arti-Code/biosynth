#![allow(unused)]

use macroquad::{experimental::collections::storage, prelude::Vec2};
use serde::{Deserialize, Serialize};


pub const SCREEN_WIDTH: f32 = 1800.0;
pub const SCREEN_HEIGHT: f32 = 900.0;
pub const WORLD_W: f32 = 1800.0;
pub const WORLD_H: f32 = 900.0;
pub const ZOOM_RATE: f32 = 1.0 / 800.0;
pub const SCREEN_RATIO: f32 = SCREEN_WIDTH / SCREEN_HEIGHT;


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
    pub resize_world: Option<Vec2>,
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
            resize_world: None,
        }
    }

}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MutationStats {
    pub interval: f32,
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

}


#[derive(Clone)]
pub struct AncestorsHistory {
    pub data: Vec<(String, String, i32)>
}

impl Default for AncestorsHistory {
    
    fn default() -> Self {
        AncestorsHistory{data: vec![]}
    }

}

pub fn set_ancestors(ancestors: AncestorsHistory) {
    storage::store(ancestors);
}

pub fn get_ancestors() -> AncestorsHistory {
    return storage::get::<AncestorsHistory>().clone();
}