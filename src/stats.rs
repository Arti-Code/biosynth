#![allow(unused)]
use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Stats {
    data: HashMap<String, Vec<[f64; 2]>>,
}

impl Stats {

    pub fn new() -> Stats {
        Stats {data: HashMap::new()}
    }
    
    pub fn add_data_type(&mut self, type_name: &str) {
        self.data.insert(type_name.to_string(), vec![]);
    }
    
    pub fn add_data(&mut self, type_name: &str, data_row: (i32, f64)) {
        let mut data = self.data.get_mut(type_name).unwrap();
        data.push([data_row.0 as f64, data_row.1 as f64]);
    }
    
    pub fn get_data_as_slice(&self, type_name: &str) -> Vec<[f64; 2]> {
        let data = self.data.get(type_name).unwrap();
        let d = data.to_owned();
        return d;
    }

    pub fn list_types(&self) -> Vec<&str> {
        let keys = self.data.keys().map(|s| s.as_ref()).collect();
        return keys;
    }

}