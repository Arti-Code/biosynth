#![allow(unused)]
use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::f32::consts::PI;
use crate::consts::*;
use crate::neuro::*;
use crate::timer::*;
use crate::util::*;
use crate::world::*;
use crate::being::*;
use macroquad::{color, prelude::*};
use macroquad::rand::*;
use rapier2d::geometry::*;
use rapier2d::prelude::{RigidBody, RigidBodyHandle};

pub struct Plant {
    pub key: u64,
    pub pos: Vec2,
    pub rot: f32,
    pub mass: f32,
    pub size: i32,
    pub max_size: i32,
    pub max_eng: f32,
    pub eng: f32,
    pub color: color::Color,
    pub shape: Vec<Vec2>,
    pub alife: bool,
    pub physics_handle: Option<RigidBodyHandle>,
}

impl Being for Plant {

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

    fn update(&mut self, dt: f32, physics: &mut World) -> bool {
        self.update_physics(physics);
        self.calc_energy(dt);
        return self.alife;
    }

}

impl Plant {
    
    pub fn new() -> Self {
        let s = 2;

        Self {
            key: gen_range(u64::MIN, u64::MAX),
            pos: random_position(WORLD_W, WORLD_H),
            rot: random_rotation(),
            mass: 0.0,
            size: s,
            max_size: PLANT_MAX_SIZE,
            max_eng: s as f32 * 20.0,
            eng: s as f32 * 10.0,
            color: LIME,
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

    fn update_physics(&mut self, physics: &mut World) {
        match self.physics_handle {
            Some(handle) => {
                let physics_data = physics.get_physics_data(handle);
                self.pos = physics_data.position;
                self.rot = physics_data.rotation;
                self.mass = physics_data.mass;
                match physics.rigid_bodies.get_mut(handle) {
                    Some(body) => {
                        if self.eng >= self.max_eng && (self.size as i32) < PLANT_MAX_SIZE {
                            self.size += 1;
                            self.max_eng = self.size as f32 * 20.0;
                            //self.shape = Ball {radius: self.size as f32};
                            for collider_handle in body.colliders().iter() {
                                match physics.colliders.get_mut(*collider_handle) {
                                    Some(collider) => {
                                        let shape = collider.shape_mut();
                                        match shape.as_ball_mut() {
                                            Some(ball_shape) => {
                                                collider.set_shape(SharedShape::ball(self.size as f32));
                                            },
                                            None => {},
                                        }
                                    },
                                    None =>  {},
                                }
                            }
                        }
                        let dir = Vec2::from_angle(self.rot);
                        self.check_edges(body);
                    }
                    None => {}
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

pub struct PlantsBox {
    pub plants: HashMap<u64, Plant>,
}

impl PlantsBox {
    pub fn new() -> Self {
        Self {
            plants: HashMap::new(),
        }
    }

    pub fn add_many_plants(&mut self, plants_num: usize, physics_world: &mut World) {
        for _ in 0..plants_num {
            let plant = Plant::new();
            _ = self.add_plant(plant, physics_world);
        }
    }

    pub fn add_plant(&mut self, mut plant: Plant, physics_world: &mut World) -> u64 {
        let key = plant.key;
        let handle = physics_world.add_complex_agent(key, &plant.pos, plant.shape.clone(), plant.rot, None);
        plant.physics_handle = Some(handle);
        self.plants.insert(key, plant);
        return key;
    }

    pub fn get(&self, id: u64) -> Option<&Plant> {
        return self.plants.get(&id);
    }

    pub fn remove(&mut self, id: u64) {
        self.plants.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, Plant> {
        return self.plants.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<u64, Plant> {
        return self.plants.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.plants.len();
    }
}