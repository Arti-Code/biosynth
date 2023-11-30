#![allow(unused)]

use macroquad::prelude::*;
use rapier2d::prelude::*;
use crate::physics::*;


trait Ability {

    fn new(offset: Vec2) -> Self;
    fn update(&mut self, physics: &mut Physics);
    fn draw(&self, position: Vec2);
    fn get_input_num(&self) -> usize;
    fn set_input_nodes(&mut self, node_keys: Vec<u64>);
    fn get_output_num(&self) -> usize;
    fn set_output_nodes(&mut self, node_keys: Vec<u64>);
    fn situation(&self) -> Vec<(u64, f32)>;
    fn reaction(&mut self, values: Vec<(u64, f32)>);
    fn get_eng_cost(&self) -> f32;
}


struct Movent {
    offset: Vec2,
    velocity: f32,
    rotation: f32,
    eng_cost: f32,
    inputs: Vec<(u64, f32)>,
    outputs: Vec<(u64, f32)>,
}


impl Ability for Movent {
    
    fn new(offset: Vec2) -> Self {
        Self { offset: offset, velocity: 0.0, rotation: 0.0, eng_cost: 0.0, inputs: vec![], outputs: vec![] }
    }

    fn draw(&self, position: Vec2) {
        todo!()
    }

    fn update(&mut self, physics: &mut Physics) {
        todo!()
    }

    fn get_eng_cost(&self) -> f32 {
        todo!()
    }

    fn get_input_num(&self) -> usize {
        todo!()
    }

    fn get_output_num(&self) -> usize {
        todo!()
    }

    fn reaction(&mut self, values: Vec<(u64, f32)>) {
        todo!()
    }

    fn set_input_nodes(&mut self, node_keys: Vec<u64>) {
        todo!()
    }

    fn set_output_nodes(&mut self, node_keys: Vec<u64>) {
        todo!()
    }

    fn situation(&self) -> Vec<(u64, f32)> {
        todo!()
    }

    

}


/* pub enum PartShapeType {
    Circle,
    Triangle,
    Hexagon,
    Box,
}

pub struct Part {
    rigid_handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
    pub shape: SharedShape,
    pub size: f32,
    pub pos: Vec2,
}

impl Part {
    pub fn new_circle(position: Vec2, radius: f32, rigid_handle: RigidBodyHandle, physics_world: &mut Physics, properties: PhysicsProperities) -> Self {
        let shape = SharedShape::ball(radius);
        let collider_handle = physics_world.add_ball_collider(rigid_handle, radius, properties.density, properties.restitution, properties.friction);
        Self {
            rigid_handle,
            collider_handle,
            shape,
            size: radius,
            pos: position,
        }
    }

    pub fn draw(&self, base_pos: Vec2) {
        todo!();
    }

} */