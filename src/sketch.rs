//#![allow(unused)]

use crate::neuro::*;
use crate::terrain::*;
use crate::sim::Simulation;
use crate::util::random_position;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use crate::settings::*;
use crate::statistics::*;
use crate::misc::*;

fn random_location() -> [f32; 2] {
    let w = get_settings().world_w as f32;
    let h = get_settings().world_h as f32;
    let pos =  random_position(w, h);
    return [pos.x, pos.y];
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentSketch {
    pub specie: String,
    pub generation: u32,
    pub size: f32,
    pub shape: MyShapeType,
    pub color: [f32; 4],
    pub color_second: [f32; 4],
    pub network: NetworkSketch,
    #[serde(default = "random_location")]
    pub pos: [f32; 2],
    pub points: f32,
    pub neuro_map: NeuroMap,
    pub power: i32,
    pub speed: i32,
    pub shell: i32,
    pub mutations: i32,
    pub eyes: i32,
    pub ancestors: Ancestors,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationSketch {
    pub simulation_name: String,
    pub world_size: MyPos2,
    pub sim_time: f64,
    pub last_autosave: f64,
    pub agents: Vec<AgentSketch>,
    pub ranking: Vec<AgentSketch>,
    pub school: Vec<AgentSketch>,
    pub settings: Settings,
    pub terrain: SerializedTerrain,
}

impl SimulationSketch {
    
    pub fn from_sim(sim: &Simulation) -> Self {
        let mut agents: Vec<AgentSketch> = vec![];
        let mut ranking: Vec<AgentSketch> = vec![];
        let mut school: Vec<AgentSketch> = vec![];
        for (_, agent) in sim.agents.get_iter() {
            let sketch = agent.get_sketch();
            agents.push(sketch);
        }
        for sketch in sim.ranking.get_general_rank().iter() {
            let sketch2 = sketch.to_owned();
            ranking.push(sketch2);
        }
        for sketch in sim.ranking.get_school_rank().iter() {
            let sketch2 = sketch.to_owned();
            school.push(sketch2);
        }
        let settings = get_settings();
        Self { 
            simulation_name: sim.simulation_name.to_owned(), 
            world_size: MyPos2::from_vec(&sim.world_size), 
            sim_time: sim.sim_state.sim_time.round(), 
            agents: agents.to_owned(), 
            ranking: ranking.to_owned(),
            school: school.to_owned(),
            last_autosave: sim.sim_state.sim_time.round(),
            settings: settings.to_owned(),
            terrain: SerializedTerrain::new(&sim.terrain),

        }
    }

}


fn default_memory_type() -> bool {
    return false;
}

fn default_memory() -> Option<MemStore> {
    return None;
}

fn default_active_rate() -> f32 {
    return 1.0;
}

fn default_lazy_num() -> u32 {
    return 0;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeSketch {
    pub id: u64,
    pub pos: MyPos2,
    pub bias: f32,
    pub node_type: NeuronTypes,
    pub label: String,
    #[serde(default = "default_memory")]
    pub memory: Option<MemStore>,
    #[serde(default = "default_memory_type")]
    pub memory_type: bool,
    #[serde(default = "default_lazy_num")]
    pub lazy_num: u32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct LinkSketch {
    pub id: u64,
    pub w: f32,
    pub node_from: u64,
    pub node_to: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkSketch {
    pub nodes: HashMap<u64, NodeSketch>,
    pub links: HashMap<u64, LinkSketch>,
    //pub margins: NeuroMargins,
}

impl NetworkSketch {
    
    pub fn from_sketch(&self) -> Network {
        let mut nodes: HashMap<u64, Node> = HashMap::new();
        let mut links: HashMap<u64, Link> = HashMap::new();
        for (key, sketch_node) in self.nodes.iter() {
            let mut node = Node::from_sketch(sketch_node.to_owned());
            if sketch_node.memory_type && sketch_node.memory.is_none() {
                node.memory = Some(MemStore::new_random());
            }
            nodes.insert(*key, node);
        }

        for (key, sketch_link) in self.links.iter() {
            let link = Link::from_sketch(sketch_link.to_owned());
            links.insert(*key, link);
        }

        let mut net = Network { 
            nodes: nodes.to_owned(), 
            links: links.to_owned(), 
            //margins: self.get_margins(), 
            input_keys: vec![], 
            output_keys: vec![], 
        };

        let (mut i, _, mut o) = net.get_node_keys_by_type();
        net.input_keys.append(&mut i);
        net.output_keys.append(&mut o);
        return net;
    }

    /* fn get_margins(&self) -> NeuroMargins {
        return self.margins.to_owned();
    } */

}