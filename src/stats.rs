//#![allow(unused)]


use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use macroquad::experimental::collections::storage;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    limit: usize,
    data: HashMap<String, VecDeque<[f64; 2]>>,
}

impl Stats {

    pub fn new(limit: usize) -> Stats {
        Stats {
            limit,
            data: HashMap::new()
        }
    }
    
    pub fn add_data_type(&mut self, type_name: &str) {
        self.data.insert(type_name.to_string(), VecDeque::new());
    }
    
    pub fn add_data(&mut self, type_name: &str, data_row: (i32, f64)) {
        let data = self.data.get_mut(type_name).unwrap();
        data.push_back([data_row.0 as f64, data_row.1 as f64]);
        if data.len() > self.limit {
            data.pop_front().unwrap();
        }
    }
    
    pub fn get_data_as_slice(&self, type_name: &str) -> Vec<[f64; 2]> {
        match self.data.get(type_name) {
            Some(data) => {
                let d = data.to_owned();
                let v: Vec<[f64; 2]> = d.iter().map(|&x| x).collect();
                return  v;
            },
            None => {
                eprintln!("No such data type {}", type_name.to_uppercase());
                return vec![];
            },
        }
    }

    pub fn _list_types(&self) -> Vec<&str> {
        let keys = self.data.keys().map(|s| s.as_ref()).collect();
        return keys;
    }

}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ancestor {
    name: String,
    gen: i32,
    time: i32,
}

impl Ancestor {

    pub fn new(name: &str, gen: i32, time: i32) -> Self {
        Self {
            name: name.to_string(),
            gen,
            time
        }
    }

    pub fn get_name_gen_time(&self) -> (&str, i32, i32) {
        return (&self.name, self.gen, self.time);
    }

}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ancestors {
    list: Vec<Ancestor>
}

impl Ancestors {

    pub fn new() -> Self {
        Self {
            list: vec![],
        }
    }

    pub fn add_ancestor(&mut self, ancestor: Ancestor) {
        self.list.push(ancestor);
    }

    pub fn get_ancestors(&self) -> Vec<Ancestor> {
        return self.list.clone();
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

pub fn set_mutations(stats: MutationStats) {
    storage::store(stats);
}

pub fn get_mutations() -> MutationStats {
    let stats = storage::get::<MutationStats>();
    return stats.clone();
}