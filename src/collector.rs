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

    pub fn populate(&mut self, physics: &mut Physics) -> (i32, i32, i32) {
        let mut counter: i32 = 0; let mut n = 0; let mut l = 0;
        let settings = get_settings();
        let mut newborns: Vec<Agent> = vec![];
        for (_, agent) in self.get_iter_mut() {
            if agent.lifetime >= (settings.repro_time + settings.repro_time * agent.childs as f32) && (agent.eng/agent.max_eng) >= 0.75 {
                let newbie = agent.replicate(physics).to_owned();
                newborns.push(newbie);
                agent.childs += 1;
                agent.points += settings.repro_points;
                agent.eng -= 0.4*agent.max_eng;
            }
        }
        loop {
            match newborns.pop() {
                Some(newbie) => {
                    //newbie.network.mutate(settings.mutations);
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



pub struct ResBox {
    pub resources: HashMap<RigidBodyHandle, Plant>,
}

impl ResBox {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn add_many_resources(&mut self, resources_num: usize, physics_world: &mut Physics) {
        for _ in 0..resources_num {
            let resource = Plant::new(physics_world);
            _ = self.add_resource(resource);
        }
    }

    pub fn add_resource(&mut self, resource: Plant) {
        //let key = resource.key;
        self.resources.insert(resource.physics_handle, resource);
        //return key;
    }

    pub fn get(&self, id: RigidBodyHandle) -> Option<&Plant> {
        return self.resources.get(&id);
    }

    pub fn remove(&mut self, id: RigidBodyHandle) {
        self.resources.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<RigidBodyHandle, Plant> {
        return self.resources.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<RigidBodyHandle, Plant> {
        return self.resources.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.resources.len();
    }

}
