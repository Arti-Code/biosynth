//#![allow(unused)]

use macroquad::{experimental::collections::storage, prelude::Vec2};

pub fn set_signals(signals: Signals) {
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
    pub new_sim: bool,
    pub rename: bool,
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
    pub export_settings: bool,
    pub import_settings: bool,
    pub update_terrain: bool,
}

impl Signals {
    
    pub fn new() -> Self {
        Self {
            world: Vec2::NAN,
            spawn_agent: false,
            new_sim: false,
            rename: false,
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
            export_settings: false,
            import_settings: false,
            update_terrain: false,
        }
    }

}


#[derive(Clone)]
pub enum UserAction {
    Idle,
    Info,
    WaterAdd,
    WaterRemove,
    TerrainAdd,
}

impl UserAction {

    pub fn new() -> Self {
        Self::Idle
    }

}