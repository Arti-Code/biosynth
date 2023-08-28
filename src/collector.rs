//#![allow(unused)]


use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use crate::util::*;
use crate::physics::*;
use crate::unit::*;
use macroquad::prelude::*;
use rapier2d::prelude::RigidBodyHandle;

pub trait PhysicsObject {
    fn new(settings: &Settings) -> Self;
    fn draw(&self, selected: bool, font: &Font);
    fn update(&mut self, dt: f32, physics: &mut PhysicsWorld) -> bool;
    fn update_physics(&mut self, physics: &mut PhysicsWorld);
    fn link_physics_handle(&mut self, handle: RigidBodyHandle);
    fn update_settings(&mut self, settings: &Settings);
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

    pub fn reload_settings(&mut self, settings: &Settings) {
        for (_, agent) in self.get_iter_mut() {
            agent.settings = settings.clone();
        }
    }

    pub fn add_many_agents(&mut self, agents_num: usize, physics_world: &mut PhysicsWorld, settings: &Settings) {
        for _ in 0..agents_num {
            let agent = Unit::new_regular(settings);
            _ = self.add_agent(agent, physics_world);
        }
    }

    pub fn add_agent(&mut self, mut agent: Unit, physics_world: &mut PhysicsWorld) -> u64 {
        let key = agent.key;
        let handle = physics_world.add_dynamic(key, &agent.pos, agent.rot, agent.shape.clone(), PhysicsProperities::default());
        agent.physics_handle = Some(handle);
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