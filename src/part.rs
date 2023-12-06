#![allow(unused)]

use std::f32::consts::PI;

use macroquad::prelude::*;
use rapier2d::prelude::*;
use crate::physics::*;



pub trait AgentPart {

    //fn create_part(offset: Vec2);
    fn update_part(&mut self);
    fn draw_part(&self, position: Vec2, rotation: f32);
}

impl Clone for Box<dyn AgentPart> {
    fn clone(&self) -> Box<dyn AgentPart> {
        let agent_part = self.to_owned();
        let s = agent_part;
        s
        //Box::new(s)
    }

    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

#[derive(Clone)]
pub struct Tail {
    phase: f32,
    color: Color,
    pub pos: Vec2,
    rot: f32,
    pub length: f32,
    pub run: bool,
    turn: f32,
}

impl Tail {
    pub fn new(pos: Vec2, length: f32, color: Color) -> Self {
        Self { phase: 0.0, color, pos, rot: 0.0, length: 25.0, run: true, turn: 1.0 }
    }

    pub fn draw(&self, position: Vec2, rotation: f32) {
        self.draw_part(position, rotation);
    }

    pub fn update(&mut self, physics: &mut Physics) {
        self.update_part()
    }

}

impl AgentPart for Tail {
    
    //fn create_part(offset: Vec2) {
        //Self { phase: 0.0, color, pos, rot: 0.0, length: 25.0, run: true }
    //}

    fn draw_part(&self, position: Vec2, rotation: f32) {
        let rot = (rotation + PI + self.phase)%PI;
        let pos0 = position + self.pos;
        let dir = Vec2::from_angle(rot);
        let l = self.length;
        let pos1 = pos0 + l*dir;
        draw_line(pos0.x, pos0.y, pos1.x, pos1.y, 2.0, self.color);
    }

    fn update_part(&mut self) {
        self.phase += get_frame_time() * self.turn;
        if self.phase >= PI/2.0 { self.turn = -1.0; }
        else if self.phase <= -PI/2.0 { self.turn = 1.0; }
        //self.phase = self.phase%(2.0*PI);
    }

}