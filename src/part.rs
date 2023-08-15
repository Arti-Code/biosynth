#![allow(unused)]

use macroquad::prelude::*;
use rapier2d::prelude::*;
use crate::physics::*;


pub enum PartShapeType {
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

}