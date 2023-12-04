#![allow(unused)]

use std::f32::consts::PI;

use macroquad::prelude::*;
use rapier2d::prelude::*;
use crate::physics::*;


trait Ability {

    fn create(offset: Vec2);
    fn _update(&mut self, physics: &mut PhysicsWorld);
    fn _draw(&self, position: Vec2);
    //fn get_input_num(&self) -> usize;
    //fn set_input_nodes(&mut self, node_keys: Vec<u64>);
    //fn get_output_num(&self) -> usize;
    //fn set_output_nodes(&mut self, node_keys: Vec<u64>);
    //fn situation(&self) -> Vec<(u64, f32)>;
    //fn reaction(&mut self, values: Vec<(u64, f32)>);
    //fn get_eng_cost(&self) -> f32;
}


pub struct Tail {
    phase: f32,
    color: Color,
    pub pos: Vec2,
    rot: f32,
    pub length: f32,
    pub run: bool,
}

impl Tail {
    pub fn new(pos: Vec2, length: f32, color: Color) -> Self {
        Self { phase: 0.0, color, pos, rot: 0.0, length: 25.0, run: true }
    }

    pub fn draw(&self, position: Vec2) {
        self._draw(position);
    }

    pub fn update(&mut self, physics: &mut PhysicsWorld) {
        self._update(physics)
    }

}

impl Ability for Tail {
    
    fn create(offset: Vec2) {
        //Self { phase: 0.0, color, pos, rot: 0.0, length: 25.0, run: true }
    }

    fn _draw(&self, position: Vec2) {
        let pos0 = position + self.pos;
        let dir = Vec2::from_angle(self.phase);
        let l = self.length;
        let pos1 = pos0 + l*dir;
        draw_line(pos0.x, pos0.y, pos1.x, pos1.y, 2.0, self.color);
    }

    fn _update(&mut self, physics: &mut PhysicsWorld) {
        self.phase += get_frame_time();
        self.phase = self.phase%(2.0*PI);
    }

}


/* struct Movent {
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

    fn update(&mut self, physics: &mut PhysicsWorld) {
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

    

} */


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
    pub fn new_circle(position: Vec2, radius: f32, rigid_handle: RigidBodyHandle, physics_world: &mut PhysicsWorld, properties: PhysicsProperities) -> Self {
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