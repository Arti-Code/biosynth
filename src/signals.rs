//#![allow(unused)]

use macroquad::{experimental::collections::storage, prelude::Vec2};

pub fn set_global_signals(signals: Signals) {
    storage::store(signals);
}

pub fn get_signals() -> Signals {
    let signals = storage::get::<Signals>();
    return signals.clone();
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
