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
        Self { phase: 0.0, color, pos, rot: PI/2.0, length: 25.0, run: true, turn: 1.0 }
    }

    pub fn draw(&self, position: Vec2, rotation: f32) {
        self.draw_part(position, rotation);
    }

    pub fn update(&mut self, physics: &mut Physics) {
        self.update_part()
    }

}

impl AgentPart for Tail {

    fn draw_part(&self, position: Vec2, rotation: f32) {
        let mut phase_vec = Vec2::from_angle(self.phase);
        phase_vec.x = -phase_vec.x.abs();
        let mut dir2 = Vec2::from_angle(self.rot + rotation).rotate(phase_vec);
        //let rot = (self.rot + rotation + PI/2.0 + self.phase);
        let pos0 = position + self.pos;
        //let mut dir = Vec2::from_angle(rot);
        //dir.x = -dir.x.abs();
        let l = self.length;
        let pos1 = pos0 + l*dir2;
        draw_line(pos0.x, pos0.y, pos1.x, pos1.y, 2.0, self.color);
    }

    fn update_part(&mut self) {
        self.phase += get_frame_time() * self.turn;
        if self.phase >= PI { self.turn = -1.0; }
        else if self.phase <= 0.0 { self.turn = 1.0; }
        //self.phase = self.phase%(2.0*PI);
    }

}