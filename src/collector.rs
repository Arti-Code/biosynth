#![allow(unused)]


use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use crate::util::*;
use crate::phyx::physics::Physics;
use crate::agent::*;
use crate::plant::*;
use macroquad::prelude::*;
use rapier2d::prelude::RigidBodyHandle;
use crate::settings::*;

pub trait PhysicsObject {
    fn new() -> Self;
    fn draw(&self, selected: bool, font: &Font);
    fn update(&mut self, dt: f32, physics: &mut Physics) -> bool;
    fn update_physics(&mut self, physics: &mut Physics);
    fn link_physics_handle(&mut self, handle: RigidBodyHandle);
}

pub struct AgentBox {
    pub agents: HashMap<RigidBodyHandle, Agent>,
}

impl AgentBox {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub fn add_many_agents(&mut self, agents_num: usize, physics_world: &mut Physics) -> (i32, i32) {
        let mut n = 0; let mut l = 0;
        for _ in 0..agents_num {
            let agent = Agent::new(physics_world);
            let (n0, l0) = self.add_agent(agent);
            n += n0; l += l0;
        }
        return (n, l);
    }

    pub fn populate(&mut self, physics: &mut Physics, time: f64) -> (i32, i32, i32) {
        let mut counter: i32 = 0; let mut n = 0; let mut l = 0;
        let settings = get_settings();
        let mut newborns: Vec<Agent> = vec![];
        for (_, agent) in self.get_iter_mut() {
            if agent.lifetime >= (settings.repro_time + settings.repro_time * agent.childs as f32) && (agent.eng/agent.max_eng) >= settings.born_eng_min {
                let newbie = agent.replicate(physics, time).to_owned();
                newborns.push(newbie);
                agent.childs += 1;
                agent.points += settings.repro_points;
                agent.eng -= settings.born_eng_cost*agent.max_eng;
            }
        }
        loop {
            match newborns.pop() {
                Some(newbie) => {
                    counter += 1;
                    let (n0, l0) = self.add_agent(newbie);
                    n += n0; l += l0;
                },
                None => {
                    break;
                }
            }
        }
        return (counter, n, l);
    }

    pub fn add_agent(&mut self, mut agent: Agent) -> (i32, i32) {
        let settings = get_settings();
        while agent.pos.x >= settings.world_w as f32 || agent.pos.y >= settings.world_h as f32 || agent.pos.x <= 0.0 || agent.pos.y <= 0.0 {
            agent.pos = random_position(settings.world_w as f32, settings.world_h as f32);
        }     
        let nl_num = agent.get_nodes_links_num();       
        self.agents.insert(agent.physics_handle, agent); 
        return nl_num;
    }

    pub fn get(&self, id: RigidBodyHandle) -> Option<&Agent> {
        return self.agents.get(&id);
    }

    pub fn remove(&mut self, id: RigidBodyHandle) {
        self.agents.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<RigidBodyHandle, Agent> {
        return self.agents.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<RigidBodyHandle, Agent> {
        return self.agents.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.agents.len();
    }

}



pub struct PlantBox {
    pub plants: HashMap<RigidBodyHandle, Plant>,
}

impl PlantBox {
    pub fn new() -> Self {
        Self {
            plants: HashMap::new(),
        }
    }

    pub fn add_many_plants(&mut self, plants_num: usize, physics_world: &mut Physics) {
        for _ in 0..plants_num {
            let num = self.count() as i32;
            let plant = Plant::new(physics_world);
            _ = self.add_plant(plant);
        }
    }

    pub fn add_plant(&mut self, plant: Plant) {
        self.plants.insert(plant.get_body_handle(), plant);
    }

    pub fn get(&self, id: RigidBodyHandle) -> Option<&Plant> {
        return self.plants.get(&id);
    }

    pub fn remove(&mut self, id: RigidBodyHandle) {
        self.plants.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<RigidBodyHandle, Plant> {
        return self.plants.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<RigidBodyHandle, Plant> {
        return self.plants.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.plants.len();
    }

}

/* pub struct PlantsList<'a> {
    pub plants: &'a Vec<&'a impl PlantType>,
} */