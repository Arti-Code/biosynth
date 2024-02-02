#![allow(unused)]

use crate::util::*;
use macroquad::prelude::*;
use rapier2d::na::*;
use rapier2d::prelude::*;
use std::collections::hash_set::{Iter};
use std::collections::{HashMap, HashSet};
use std::io;
use crate::settings::*;


pub struct PhysicState {
    pub position: Vec2,
    pub rotation: f32,
    pub mass: f32,
    pub kin_eng: Option<f32>,
    pub force: Option<Vec2>,
}


pub struct PhysicsMaterial {
    pub friction: f32,
    pub restitution: f32,
    pub density: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
}

impl Default for PhysicsMaterial {
    
    fn default() -> Self {
        Self { friction: 0.5, restitution: 0.5, density: 0.5, linear_damping: 0.1, angular_damping: 0.9 }
    }
}

impl PhysicsMaterial {
    
    pub fn new(friction: f32, restitution: f32, density: f32, linear_damping: f32, angular_damping: f32) -> Self {
        Self { friction, restitution, density, linear_damping, angular_damping }
    }

    pub fn high_inert() -> Self {
        Self { friction: 4.0, restitution: -0.6, density: 20.0, linear_damping: 1.0, angular_damping: 1.0 }
    }

}

