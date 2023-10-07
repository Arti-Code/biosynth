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

pub trait PhysicsObject {
    fn new() -> Self;
    fn draw(&self, selected: bool, font: &Font);
    fn update(&mut self, dt: f32, physics: &mut PhysicsWorld) -> bool;
    fn update_physics(&mut self, physics: &mut PhysicsWorld);
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

    pub fn add_many_agents(&mut self, agents_num: usize, physics_world: &mut PhysicsWorld) {
        for _ in 0..agents_num {
            let agent = Agent::new(physics_world);
            _ = self.add_agent(agent);
        }
    }

    pub fn populate(&mut self, physics: &mut PhysicsWorld) {
        let settings = get_settings();
        let mut newborns: Vec<Agent> = vec![];
        for (_, agent) in self.get_iter_mut() {
            if agent.lifetime >= (100 + 100 * agent.childs) as f32 && (agent.eng/agent.max_eng) >= 0.75 {
                let newone = agent.replicate(physics);
                newborns.push(newone);
                agent.childs += 1;
                agent.points += 100.0;
                agent.eng -= 0.40*agent.max_eng;
            }
        }
        loop {
            match newborns.pop() {
                Some(mut newone) => {
                    newone.network.mutate(settings.mutations);
                    self.add_agent(newone);
                    //self.agents.insert(newone.key, newone);
                },
                None => {
                    break;
                }
            }
        }
    }

    pub fn add_agent(&mut self, agent: Agent) {
        //let key = agent.key;
        self.agents.insert(agent.physics_handle, agent);
        //return key;
    }

    pub fn get(&self, id: RigidBodyHandle) -> Option<&Agent> {
        return self.agents.get(&id);
    }

    pub fn _remove(&mut self, id: RigidBodyHandle) {
        self.agents.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<RigidBodyHandle, Agent> {
        return self.agents.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<RigidBodyHandle, Agent> {
        return self.agents.iter_mut();
    }

    pub fn _count(&self) -> usize {
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

    pub fn add_many_resources(&mut self, resources_num: usize, physics_world: &mut PhysicsWorld) {
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

    pub fn _remove(&mut self, id: RigidBodyHandle) {
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
