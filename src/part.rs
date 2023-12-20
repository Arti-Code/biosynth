#![allow(unused)]

use std::f32::consts::PI;

use macroquad::prelude::*;
use rapier2d::prelude::*;
use crate::physics::*;



pub trait AgentPart {

    //fn create_part(offset: Vec2);
    fn update_part(&mut self, angle: f32);
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
    phase1: f32,
    phase2: f32,
    color: Color,
    pub pos: Vec2,
    rot: f32,
    pub length: f32,
    pub run: bool,
    turn: f32,
    turn1: f32,
    turn2: f32,
}

impl Tail {
    pub fn new(pos: Vec2, length: f32, rotation: f32, color: Color) -> Self {
        Self { phase: 0.0, color, pos, rot: rotation, length, run: true, turn: 1.0, turn1: 1.0, turn2: 1.0, phase1: -0.2, phase2: -0.4 }
    }

    pub fn draw(&self, position: Vec2, rotation: f32) {
        self.draw_part(position, rotation);
    }

    pub fn update(&mut self, angle: f32, physics: &mut Physics) {
        self.update_part(angle);
    }

}

impl AgentPart for Tail {

    fn draw_part(&self, position: Vec2, rotation: f32) {
        let l = self.length;
        let mut dir = Vec2::from_angle(self.rot + rotation+self.phase);
        let mut dir1 = Vec2::from_angle(self.rot + rotation+self.phase+self.phase1);
        let mut dir2 = Vec2::from_angle(self.rot + rotation+self.phase+self.phase1+self.phase2);
        let pos0 = position + self.pos;
        let pos = pos0 + l*dir;
        let pos1 = pos + l/1.25*dir1;
        let pos2 = pos1 + l/1.75*dir2;
        draw_line(pos0.x, pos0.y, pos.x, pos.y, 3.0, self.color);
        draw_line(pos.x, pos.y, pos1.x, pos1.y, 3.0, self.color);
        draw_line(pos1.x, pos1.y, pos2.x, pos2.y, 3.0, self.color);
    }

    fn update_part(&mut self, angle: f32) {
        self.phase += get_frame_time() * self.turn;
        self.phase1 += get_frame_time() * self.turn1;
        self.phase2 += get_frame_time() * self.turn2;
        if self.phase >= angle/2.0 { self.turn = -1.0; }
        else if self.phase <= -angle/2.0 { self.turn = 1.0; }
        if self.phase1 >= angle/2.0 { self.turn1 = -1.0; }
        else if self.phase1 <= -angle/2.0 { self.turn1 = 1.0; }
        if self.phase2 >= angle/2.0 { self.turn2 = -1.0; }
        else if self.phase2 <= -angle/2.0 { self.turn2 = 1.0; }
        //self.phase = self.phase%(2.0*PI);
    }

}