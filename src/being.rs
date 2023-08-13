#![allow(unused)]

use std::collections::HashMap;
use std::collections::hash_map::{Iter, IterMut};

use macroquad::prelude::*;
use macroquad::color::*;
use rapier2d::prelude::*;
use macroquad::rand::*;
use crate::physics::*;
use crate::util::*;
use crate::consts::*;

pub trait Being {
    //fn new() -> Self;
    fn draw(&self, selected: bool, font: &Font);
    fn update(&mut self, dt: f32, physics: &mut PhysicsWorld) -> bool;
}

trait Collector<T> {
    fn new() -> Self;
    fn add_many(&mut self, number: usize, physics: &mut PhysicsWorld);
    fn add(&mut self, being: T, physics: &mut PhysicsWorld);
    fn get(&self, id: u64) -> Option<T>;
    fn remove(&mut self, id: u64);
    fn get_iter(&self) -> Iter<u64, T>;
    fn get_iter_mut(&mut self) -> IterMut<u64, T>;
    fn count(&self) -> usize;
}

pub struct Life {
    pub key: u64,
    pub pos: Vec2,
    pub rot: f32,
    pub mass: f32,
    pub size: i32,
    pub max_eng: f32,
    pub eng: f32,
    pub color: Color,
    pub shape: Vec<Vec2>,
    pub alife: bool,
    pub physics_handle: Option<RigidBodyHandle>,
}

impl Being for Life {

    fn draw(&self, selected: bool, font: &Font) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        self.draw_shape();
        //draw_circle_lines(x0, y0, self.size as f32, 4.0, self.color);
        //self.draw_info(font);
        if selected {
            //self.draw_info(&font);
        }
    }    

    fn update(&mut self, dt: f32, physics: &mut PhysicsWorld) -> bool {
        self.update_physics(physics);
        //self.calc_energy(dt);
        return self.alife;
    }

}

impl Life {
    
    pub fn new() -> Self {
        let s = gen_range(LIFE_SIZE_MIN, LIFE_SIZE_MAX);

        Self {
            key: gen_range(u64::MIN, u64::MAX),
            pos: random_position(WORLD_W, WORLD_H),
            rot: random_rotation(),
            mass: 0.0,
            size: s,
            max_eng: s as f32 * 20.0,
            eng: s as f32 * 10.0,
            color: random_color5(),
            shape: map_polygon(6, 16., 0.0),
            alife: true,
            physics_handle: None,
        }
    }

    fn draw_shape(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let pos = self.pos;
        for i in 0..self.shape.len() {
            if i > 0 {
                let p0 = self.shape[i-1];
                let p1 = self.shape[i];
                draw_line(x0+p0.x, y0+p0.y, x0+p1.x, y0+p1.y, 2.0, self.color);
            }
        }
        let last = self.shape.len();
        let p0 = self.shape[last-1];
        let p1 = self.shape[0];
        draw_line(x0+p0.x, y0+p0.y, x0+p1.x, y0+p1.y, 2.0, self.color);
    }

    fn draw_info(&self, font: &Font) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let text_cfg = TextParams {
            font: *font,
            font_size: 13,
            color: WHITE,
            ..Default::default()
        };
        let rot = self.rot;
        let mass = self.mass;
        let eng = self.eng.round();
        let info = format!("eng: {}", eng);
        let max_eng = self.max_eng.round();
        let info2 = format!("max_eng: {}", max_eng);
        let size = self.size;
        //let info_mass = format!("mass: {}", mass.round());
        let new_size = (self.eng.sqrt()/2.0).floor();
        let info3 = format!("size {} -> {}", size as i32, new_size as i32);
        let txt_center = get_text_center(&info, Some(*font), 13, 1.0, 0.0);
        let txt_center2 = get_text_center(&info2, Some(*font), 13, 1.0, 0.0);
        let txt_center3 = get_text_center(&info3, Some(*font), 13, 1.0, 0.0);
        draw_text_ex(&info, x0 - txt_center.x, y0 - txt_center.y + self.size as f32 * 2.0+8.0, text_cfg.clone());
        draw_text_ex(&info2, x0 - txt_center.x, y0 - txt_center.y + self.size as f32 * 2.0 + 23.0, text_cfg.clone());
        draw_text_ex(&info3, x0 - txt_center.x, y0 - txt_center.y + self.size as f32 * 2.0 + 38.0, text_cfg.clone());
    }

    fn update_physics(&mut self, physics: &mut PhysicsWorld) {
        match self.physics_handle {
            Some(handle) => {
                let physics_data = physics.get_physics_data(handle);
                self.pos = physics_data.position;
                self.rot = physics_data.rotation;
                self.mass = physics_data.mass;
                let dir = Vec2::from_angle(self.rot);
                match physics.rigid_bodies.get_mut(handle) {
                    Some(body) => {
                        self.check_edges(body);
                    },
                    None => {},
                }
            }
            None => {}
        }
    }

    fn check_edges(&mut self, body: &mut RigidBody) {
        let mut raw_pos = matric_to_vec2(body.position().translation);
        let mut out_of_edge = false;
        if raw_pos.x < -5.0 {
            raw_pos.x = 0.0;
            out_of_edge = true;
        } else if raw_pos.x > WORLD_W + 5.0 {
            raw_pos.x = WORLD_W;
            out_of_edge = true;
        }
        if raw_pos.y < -5.0 {
            raw_pos.y = 0.0;
            out_of_edge = true;
        } else if raw_pos.y > WORLD_H + 5.0 {
            raw_pos.y = WORLD_H;
            out_of_edge = true;
        }
        if out_of_edge {
            body.set_position(make_isometry(raw_pos.x, raw_pos.y, -self.rot), true);
            body.set_linvel([0.0, 0.0].into(), true);
        }
    }

    fn calc_energy(&mut self, dt: f32) {
        let growth = 5.0 * dt;
        let loss = (self.size as f32) * dt * 0.2;
        //let loss = (self.size-6.0).powi(2) * dt *0.1;
        if self.eng > 0.0 {
            if self.eng < self.max_eng {
                self.eng += (growth-loss);
            }
        } else {
            self.eng = 0.0;
            self.alife = false;
        }
    }

    pub fn add_energy(&mut self, e: f32) {
        self.eng += e;
        if self.eng > self.max_eng {
            self.eng = self.max_eng;
        }
    }
}



pub struct LifesBox {
    pub elements: HashMap<u64, Life>,
}

impl LifesBox {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    pub fn add_many_plants(&mut self, plants_num: usize, physics_world: &mut PhysicsWorld) {
        for _ in 0..plants_num {
            let plant = Life::new();
            _ = self.add_plant(plant, physics_world);
        }
    }

    pub fn add_plant(&mut self, mut plant: Life, physics_world: &mut PhysicsWorld) -> u64 {
        let key = plant.key;
        let props = PhysicsProperities::new(0.5, 0.5, 1.0, 0.5, 0.5);
        let handle = physics_world.add_dynamic_ball(key, plant.size as f32, &plant.pos, plant.rot, props);
        //let handle2 = physics_world.add_complex_agent(key, &plant.pos, plant.shape.to_vec().to_owned(), plant.rot, None);
        plant.physics_handle = Some(handle);
        self.elements.insert(key, plant);
        return key;
    }

    pub fn get(&self, id: u64) -> Option<&Life> {
        return self.elements.get(&id);
    }

    pub fn remove(&mut self, id: u64) {
        self.elements.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, Life> {
        return self.elements.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<u64, Life> {
        return self.elements.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.elements.len();
    }
}