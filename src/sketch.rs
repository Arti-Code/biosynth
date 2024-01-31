#![allow(unused)]


use std::collections::HashMap;
use std::collections::HashSet;
use std::f32::consts::PI;
use crate::neuro::*;
use crate::timer::*;
use crate::util::*;
use crate::physics::*;
use crate::globals::*;
use crate::terrain::*;
use crate::sim::Simulation;
use macroquad::{color, prelude::*};
use macroquad::rand::*;
use rapier2d::geometry::*;
use rapier2d::na::Vector2;
use rapier2d::prelude::{RigidBody, RigidBodyHandle};
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use crate::settings::*;
use crate::stats::*;
use crate::misc::*;



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentSketch {
    pub specie: String,
    pub generation: u32,
    pub size: f32,
    pub shape: MyShapeType,
    pub color: [f32; 4],
    //#[serde(default = "default_color")]
    pub color_second: [f32; 4],
    pub network: NetworkSketch,
    pub points: f32,
    pub neuro_map: NeuroMap,
    pub power: i32,
    pub speed: i32,
    pub shell: i32,
    //#[serde(default = "default_mutation")]
    pub mutations: i32,
    //#[serde(default = "default_mutation")]
    pub eyes: i32,
    //#[serde(default="default_ancestors")]
    pub ancestors: Ancestors,
}

/* pub fn default_mutation() -> i32 {
    return gen_range(0, 10);
}

pub fn default_color() -> [f32; 4] {
    let c = random_color();
    return [c.r, c.g, c.b, c.a];
}

fn default_ancestors() -> Ancestors {
    let mut ancestors = Ancestors::new();
    ancestors.add_ancestor(Ancestor::new("--forgotten--", 0, 0));
    return ancestors;
}  */


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationSketch {
    pub simulation_name: String,
    pub world_size: MyPos2,
    pub sim_time: f64,
    pub last_autosave: f64,
    pub agents: Vec<AgentSketch>,
    pub ranking: Vec<AgentSketch>,
    pub settings: Settings,
    pub terrain: SerializedTerrain,
}

impl SimulationSketch {
    
    pub fn from_sim(sim: &Simulation) -> Self {
        let mut agents: Vec<AgentSketch> = vec![];
        let mut ranking: Vec<AgentSketch> = vec![];
        for (_, agent) in sim.agents.get_iter() {
            let sketch = agent.get_sketch();
            agents.push(sketch);
        }
        for sketch in sim.ranking.iter() {
            let sketch2 = sketch.to_owned();
            ranking.push(sketch2);
        }
        let settings = get_settings();
        Self { 
            simulation_name: sim.simulation_name.to_owned(), 
            world_size: MyPos2::from_vec(&sim.world_size), 
            sim_time: sim.sim_state.sim_time.round(), 
            agents: agents.to_owned(), 
            ranking: ranking.to_owned(),
            last_autosave: sim.sim_state.sim_time.round(),
            settings: settings.to_owned(),
            terrain: SerializedTerrain::new(&sim.terrain),

        }
    }

}