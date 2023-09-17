#![allow(unused)]


use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use crate::util::*;
use crate::physics::*;
use crate::unit::*;
use crate::globals::*;
use macroquad::prelude::*;
use rapier2d::prelude::RigidBodyHandle;

pub trait PhysicsObject {
    fn new() -> Self;
    fn draw(&self, selected: bool, font: &Font);
    fn update(&mut self, dt: f32, physics: &mut PhysicsWorld) -> bool;
    fn update_physics(&mut self, physics: &mut PhysicsWorld);
    fn link_physics_handle(&mut self, handle: RigidBodyHandle);
}

pub struct UnitsBox {
    pub agents: HashMap<u64, Unit>,
}

impl UnitsBox {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub fn add_many_agents(&mut self, agents_num: usize, physics_world: &mut PhysicsWorld) {
        for _ in 0..agents_num {
            let agent = Unit::new(physics_world);
            _ = self.add_agent(agent);
        }
    }

    pub fn populate(&mut self, physics: &mut PhysicsWorld) {
        let settings = get_settings();
        let mut newborns: Vec<Unit> = vec![];
        for (_, agent) in self.get_iter_mut() {
            if agent.lifetime >= (100 + 100 * agent.childs) as f32 && (agent.eng/agent.max_eng) >= 0.75 {
                let newone = agent.replicate(physics);
                newborns.push(newone);
                agent.childs += 1;
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

    pub fn add_agent(&mut self, agent: Unit) -> u64 {
        let key = agent.key;
        self.agents.insert(key, agent);
        return key;
    }

    pub fn get(&self, id: u64) -> Option<&Unit> {
        return self.agents.get(&id);
    }

    pub fn _remove(&mut self, id: u64) {
        self.agents.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, Unit> {
        return self.agents.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<u64, Unit> {
        return self.agents.iter_mut();
    }

    pub fn _count(&self) -> usize {
        return self.agents.len();
    }

}

pub struct ElementsBox {

}