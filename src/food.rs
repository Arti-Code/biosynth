#![allow(unused)]

use std::collections::HashMap;
use std::collections::hash_map::{Iter, IterMut};
use rapier2d::geometry::*;
use macroquad::{prelude::*, color};
use crate::util::*;
use crate::timer::*;
use crate::globals::*;
use crate::settings::*;

pub struct Food {
    pub pos: Vec2,
    pub rot: f32,
    pub size: f32,
    pub max_eng: f32,
    pub eng: f32,
    pub color: color::Color,
    pub shape: Ball,
    pub alife: bool,
}

impl Food {
    pub fn new_random() -> Self {
        let settings = get_settings();
        let size = rand::gen_range(5, 10) as f32;
        let position = random_position(settings.world_w as f32, settings.world_h as f32);
        let rotation = random_rotation();
        let color = random_color();
        Self::new(position, rotation, size, color)
    }

    pub fn new(position: Vec2, rotation: f32, size: f32, color: Color) -> Self {
        Self {
            pos: position,
            rot: rotation,
            size,
            max_eng: size.powi(2)*10.0,
            eng: size.powi(2)*10.0,
            color,
            shape: Ball { radius: size },
            alife: true,
        }
    }

    pub fn draw(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        draw_circle(x0, y0, self.size, self.color);
    }
    pub fn update(&mut self, _dt: f32){
        self.pos = wrap_around(&self.pos);
        if self.eng <= 0.0 {
            self.eng = 0.0;
            self.alife = false;
        }
    }

    pub fn drain_eng(&mut self, eng_loss: f32) {
        self.eng -= eng_loss;
    }

    pub fn update_collision(&mut self, collision_normal: &Vec2, penetration: f32, dt: f32) {
        self.pos -= *collision_normal * penetration.abs() * dt * 0.3;
    }
}



pub struct SourcesBox {
    pub sources: HashMap<u64, Food>
}

impl SourcesBox {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
        }
    }

    pub fn add_many(&mut self, source_num: usize) {
        for _ in 0..source_num {
            let source = Food::new_random();
            _ = self.add_source(source);
        }
    }

    pub fn add_source(&mut self, source: Food) -> u64 {
        let key: u64 = rand::gen_range(u64::MIN, u64::MAX);
        self.sources.insert(key, source);
        return key;
    }

    pub fn get(&self, id: u64) -> Option<&Food> {
        return self.sources.get(&id);
    }

    pub fn remove(&mut self, id: u64) {
        self.sources.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, Food> {
        return self.sources.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<u64, Food> {
        return self.sources.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.sources.len();
    }
}
