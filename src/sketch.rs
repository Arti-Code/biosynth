#![allow(unused)]

use crate::neuro::*;
use crate::terrain::*;
use crate::sim::Simulation;
use macroquad::prelude::*;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use crate::settings::*;
use crate::statistics::*;
use crate::misc::*;



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentSketch {
    pub specie: String,
    pub generation: u32,
    pub size: f32,
    pub shape: MyShapeType,
    pub color: [f32; 4],
    pub color_second: [f32; 4],
    pub network: NetworkSketch,
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
        let settings = settings();
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