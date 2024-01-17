#![allow(unused)]


use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use crate::util::*;
use crate::physics::*;
use crate::agent::*;
use crate::globals::*;
use crate::resource::*;
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

    pub fn add_many_agents(&mut self, agents_num: usize, physics_world: &mut Physics) {
        for _ in 0..agents_num {
            let agent = Agent::new(physics_world);
            _ = self.add_agent(agent);
        }
    }

    pub fn populate(&mut self, physics: &mut Physics) -> i32 {
        let mut counter: i32 = 0;
        let settings = get_settings();
        let mut newborns: Vec<Agent> = vec![];
        for (_, agent) in self.get_iter_mut() {
            if agent.lifetime >= (settings.repro_time + settings.repro_time * (agent.childs*2) as f32) && (agent.eng/agent.max_eng) >= 0.75 {
                let mut newbie = agent.replicate(physics).to_owned();
                newborns.push(newbie);
                agent.childs += 1;
                agent.points += settings.repro_points;
                agent.eng -= 0.4*agent.max_eng;
            }
        }
        loop {
            match newborns.pop() {
                Some(mut newbie) => {
                    //newbie.network.mutate(settings.mutations);
                    counter += 1;
                    self.add_agent(newbie);
                },
                None => {
                    break;
                }
            }
        }
        return counter;
    }

    pub fn add_agent(&mut self, mut agent: Agent) {
        let settings = get_settings();
        while agent.pos.x >= settings.world_w as f32 || agent.pos.y >= settings.world_h as f32 || agent.pos.x <= 0.0 || agent.pos.y <= 0.0 {
            agent.pos = random_position(settings.world_w as f32, settings.world_h as f32);
        }            
        self.agents.insert(agent.physics_handle, agent);
        
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
    pub resources: HashMap<RigidBodyHandle, Resource>,
}

impl ResBox {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn add_many_resources(&mut self, resources_num: usize, physics_world: &mut Physics) {
        for _ in 0..resources_num {
            let resource = Resource::new(physics_world);
            _ = self.add_resource(resource);
        }
    }

    pub fn add_resource(&mut self, resource: Resource) {
        //let key = resource.key;
        self.resources.insert(resource.physics_handle, resource);
        //return key;
    }

    pub fn get(&self, id: RigidBodyHandle) -> Option<&Resource> {
        return self.resources.get(&id);
    }

    pub fn remove(&mut self, id: RigidBodyHandle) {
        self.resources.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<RigidBodyHandle, Resource> {
        return self.resources.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<RigidBodyHandle, Resource> {
        return self.resources.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.resources.len();
    }

}
