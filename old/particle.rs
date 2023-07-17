#![allow(unused)]
use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
//use std::f32::consts::PI;

use crate::consts::*;
use crate::kinetic::make_isometry;
use crate::util::*;
use crate::world::*;
use ::rand::{thread_rng, Rng};
use macroquad::{color, prelude::*};
use nalgebra::{Point2, Vector2};
use rapier2d::geometry::*;
use rapier2d::parry::shape::Cuboid;
use rapier2d::prelude::{RigidBody, RigidBodyHandle};

pub struct ParticleType {
    pub id: u8,
    pub color: Color,
    pub actions: [f32; 5],
}

impl ParticleType {
    pub fn new_random(id: u8, p_color: Color) -> Self {
        Self {
            id: id,
            color: p_color,
            actions: [
                random_unit(),
                random_unit(),
                random_unit(),
                random_unit(),
                random_unit(),
            ],
        }
    }
}

pub struct ParticleTable {
    pub colors: [Color; 5],
    pub particle_types: Vec<ParticleType>,
}

impl ParticleTable {
    pub fn new_random() -> Self {
        let c = [RED, BLUE, GREEN, YELLOW, WHITE];
        let mut tab: Vec<ParticleType> = vec![];
        for i in 0..5 {
            let pt = ParticleType::new_random(i as u8, c[i]);
            tab.push(pt);
        }
        Self {
            colors: [RED, BLUE, GREEN, YELLOW, WHITE],
            particle_types: tab.into(),
        }
    }
    pub fn get_type(&self, id: u8) -> &ParticleType {
        return self.particle_types.get(id as usize).unwrap();
    }
    pub fn get_action(&self, particle_type: u8, other_particle_type: u8) -> f32 {
        let pt = self.particle_types.get(particle_type as usize).unwrap();
        return pt.actions[other_particle_type as usize];
    }
    pub fn get_color(&self, particle_type: u8) -> Color {
        let pt = self.particle_types.get(particle_type as usize).unwrap();
        return pt.color;
    }
}

pub struct Particle {
    pub key: u64,
    pub pos: Vec2,
    particle_type: u8,
    pub rot: f32,
    pub vel: f32,
    pub ang_vel: f32,
    pub size: f32,
    pub color: color::Color,
    pub shape: Ball,
    pub physics_handle: Option<RigidBodyHandle>,
    pub kin_eng: f32,
}

impl Particle {
    pub fn new(p_type: u8, p_color: Color) -> Self {
        let size = PARTICLE_SIZE;

        Self {
            key: thread_rng().gen::<u64>(),
            pos: random_position(WORLD_W, WORLD_H),
            particle_type: p_type,
            rot: random_rotation(),
            vel: rand::gen_range(0.0, 1.0) * PARTICLE_SPEED,
            ang_vel: 0.0,
            size: (size as f32),
            color: p_color,
            shape: Ball::new(size),
            physics_handle: None,
            kin_eng: 0.0,
        }
    }
    pub fn draw(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        draw_circle(x0, y0, self.size, self.color); //draw_text(kin_eng_info, x0-18.0, y0, 16.0, WHITE);
    }
    pub fn update(&mut self, dt: f32, physics: &mut World) {
        match self.physics_handle {
            Some(handle) => {
                let physics_data = physics.get_physics_data(handle);
                self.pos = physics_data.position;
                self.rot = physics_data.rotation;
                self.kin_eng = physics_data.kin_eng.unwrap();
                match physics.rigid_bodies.get_mut(handle) {
                    Some(body) => {
                        let dir = Vec2::from_angle(self.rot);
                        let v = dir * self.vel;
                        //body.set_linvel([v.x, v.y].into(), true);
                        //body.set_angvel(self.ang_vel, true);
                        self.edges_check(body);
                    }
                    None => {}
                }
            }
            None => {}
        }
    }

    pub fn attract_repel(&mut self, physics: &mut World) {
        match self.physics_handle {
            Some(handle) => {
                physics.get_around(handle);
            }
            None => {}
        }
    }

    fn edges_check(&mut self, body: &mut RigidBody) {
        let mut raw_pos = matric_to_vec2(body.position().translation);
        let mut out_of_edge = false;
        if raw_pos.x < 0.0 {
            raw_pos.x = WORLD_W - 5.0;
            out_of_edge = true;
        } else if raw_pos.x > WORLD_W {
            raw_pos.x = 5.0;
            out_of_edge = true;
        }
        if raw_pos.y < 0.0 {
            raw_pos.y = WORLD_H - 5.0;
            out_of_edge = true;
        } else if raw_pos.y > WORLD_H {
            raw_pos.y = 5.0;
            out_of_edge = true;
        }
        if out_of_edge {
            //let v2: Vec2 = Vec2::new(body.linvel().data.0[0][0], body.linvel().data.0[0][1])*-1.0;
            body.set_position(make_isometry(raw_pos.x, raw_pos.y, self.rot), true);
            //body.set_linvel([v2.x, v2.y].into(), true);
        }
    }
}

pub struct ParticleCollector {
    particle_types: ParticleTable,
    pub elements: HashMap<u64, Particle>,
}

impl ParticleCollector {
    pub fn new() -> Self {
        Self {
            particle_types: ParticleTable::new_random(),
            elements: HashMap::new(),
        }
    }

    pub fn add_many_elements(&mut self, elements_num: usize, physics_world: &mut World) {
        for _ in 0..elements_num {
            let t = rand::gen_range(0, 5);
            let p_type = self.get_type(t as u8);
            let element = Particle::new(t, p_type.color);
            _ = self.add_element(element, physics_world);
        }
    }

    pub fn add_element(&mut self, mut element: Particle, physics_world: &mut World) -> u64 {
        let p_type = element.particle_type;
        let key = element.key;
        //let handle = physics_world.add_poly_body(key,&element.pos, element.points2.clone());
        let handle = physics_world.add_particle(p_type, &element.pos, element.size);
        element.physics_handle = Some(handle);
        self.elements.insert(key, element);
        return key;
    }

    pub fn get_type(&self, p_type: u8) -> &ParticleType {
        let particle_type = self.particle_types.get_type(p_type);
        return particle_type;
    }

    pub fn get(&self, id: u64) -> Option<&Particle> {
        return self.elements.get(&id);
    }

    pub fn remove(&mut self, id: u64) {
        self.elements.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, Particle> {
        return self.elements.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<u64, Particle> {
        return self.elements.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.elements.len();
    }
}
