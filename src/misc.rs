//#![allow(unused)]

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::neuro::Network;


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MyShapeType {
    Ball,
    Cuboid,
    Segment,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NeuroMap {
    pub sensors: HashMap<String, u64>,
    pub effectors: HashMap<String, u64>,
    signals: HashMap<u64, f32>,
    actions: HashMap<String, f32>,
}

impl NeuroMap {

    pub fn new() -> Self {
        Self { 
            sensors: HashMap::new(), 
            effectors: HashMap::new(),
            signals: HashMap::new(),
            actions: HashMap::new(), 
        }
    }

    pub fn add_sensor(&mut self, name: &str, node_key: u64) {
        self.sensors.insert(name.to_string(), node_key);
    }

    pub fn add_sensors(&mut self, pairs: Vec<(u64, String)>) {
        for (k, s) in pairs.iter() {
            self.add_sensor(s, *k);
        }
    }

    pub fn add_effector(&mut self, name: &str, node_key: u64) {
        self.effectors.insert(name.to_string(), node_key);
    }

    pub fn add_effectors(&mut self, pairs: Vec<(u64, String)>) {
        for (k, s) in pairs.iter() {
            self.add_effector(s, *k);
        }
    }

    pub fn send_signals(&self, network: &mut Network) {
        let mut input_values: Vec<(u64, f32)> = vec![];
        for (k, v) in self.signals.iter() {
            input_values.push((*k, *v));
        }
        network.input(input_values);
    }

    pub fn recv_actions(&mut self, network: &Network) {
        self.actions = HashMap::new();
        for (k, v) in self.effectors.iter() {
            self.actions.insert(k.to_owned(), network.get_node_value(v).unwrap());
        }
    }

    pub fn set_signal(&mut self, name: &str, value: f32) {
        let node_key = self.sensors.get(name).unwrap();
        self.signals.insert(*node_key, value);
    }

    pub fn get_action(&self, name: &str) -> f32 {
        return *self.actions.get(name).unwrap();
    }

    pub fn get_signal_list(&self) -> Vec<(String, f32)> {
        let mut signal_list: Vec<(String, f32)> = vec![];
        for (s, k) in self.sensors.iter() {
            let v = self.signals.get(k).unwrap();
            signal_list.push((s.to_owned(), *v));
        }
        return signal_list;
    }

    pub fn get_action_list(&self) -> Vec<(String, f32)> {
        let mut action_list: Vec<(String, f32)> = vec![];
        for (s, v) in self.actions.iter() {
            action_list.push((s.to_owned(), *v));
        }
        return action_list;
    }

}


#[derive(Clone)]
pub struct EnergyCost {
    pub basic: f32,
    pub movement: f32,
    pub attack: f32,
}

impl Default for EnergyCost {
    fn default() -> Self {
        EnergyCost{basic: 0., movement: 0., attack: 0.}
    }
}
