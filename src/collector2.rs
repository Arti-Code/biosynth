//#![allow(unused)]


use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::f32::consts::PI;
use crate::collector::PhysicsObject;
use crate::consts::{WORLD_H, WORLD_W};
use crate::neuro::*;
//use crate::sim::*;
use crate::timer::*;
use crate::util::*;
use crate::physics::*;
use crate::unit::*;
use crate::collector2::*;
use macroquad::{color, prelude::*};
use macroquad::rand::*;
use rapier2d::geometry::*;
use rapier2d::na::Vector2;
use rapier2d::prelude::{RigidBody, RigidBodyHandle};



pub struct Collector<T: PhysicsObject> {
    pub elements: HashMap<u64, T>,
}

impl<T> Collector<T: PhysicsObject> {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    pub fn reload_settings<T>(&mut self, settings: &Settings) {
        for (_, element) in self.get_iter_mut() {
            element.update_settings(&settings.clone());
        }
    }

    pub fn add_many_agents(&mut self, agents_num: usize, physics_world: &mut PhysicsWorld, settings: &Settings) {
        for _ in 0..agents_num {
            let agent = T::new(settings);
            _ = self.add_agent(agent, physics_world);
        }
    }

    pub fn add_agent<T: PhysicsObject>(&mut self, mut element: T, physics_world: &mut PhysicsWorld) -> u64 {
        let key = agent.key;
        let handle = physics_world.add_dynamic(key, &agent.pos, agent.rot, agent.shape.clone(), PhysicsProperities::default());
        element.link_physics_handle(handle);
        self.elements.insert(key, element);
        return key;
    }

    pub fn get(&self, id: u64) -> Option<&T> {
        return self.elements.get(&id);
    }

    pub fn _remove(&mut self, id: u64) {
        self.elements.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, T> {
        return self.elements.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<u64, T> {
        return self.elements.iter_mut();
    }

    pub fn _count(&self) -> usize {
        return self.elements.len();
    }
}