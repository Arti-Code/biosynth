#![allow(unused)]
use crate::consts::*;
use crate::kinetic::make_isometry;
use crate::timer::*;
use crate::util::*;
use crate::world::*;
use ::rand::{thread_rng, Rng};
use macroquad::{color, prelude::*};
use nalgebra::{Point2, Vector2};
use rapier2d::geometry::*;
use rapier2d::prelude::{RigidBody, RigidBodyHandle};
use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::f32::consts::PI;

pub struct Jet {
    pub key: u64,
    pub pos: Vec2,
    pub rot: f32,
    pub throttle: f32,
    pub turn: f32,
    pub vertices: Vec<Vec2>,
    pub points: Vec<Point2<f32>>,
    pub size: f32,
    pub color: color::Color,
    pub physics_handle: Option<RigidBodyHandle>,
    controller: Vec2,
    eng_k: f32,
    mass: f32,
}

impl Jet {
    pub fn new() -> Self {
        let s = 16.0;
        let verts = map_polygon(6, s, 0.0);
        let pts = vec2_to_point2_collection(&verts);
        Self {
            key: thread_rng().gen::<u64>(),
            pos: random_position(WORLD_W, WORLD_H),
            rot: random_rotation(),
            throttle: 0.0,
            turn: 0.0,
            vertices: verts,
            points: pts,
            size: s,
            color: random_color(),
            physics_handle: None,
            controller: Vec2::new(0.0, 0.0),
            eng_k: 0.0,
            mass: 0.0,
        }
    }

    pub fn draw(&self, font: Font) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let l = self.vertices.len();
        for i in 1..=l {
            let v1: &Vec2;
            let v2: &Vec2;
            if i == l {
                v1 = self.vertices.get(i - 1).unwrap();
                v2 = self.vertices.get(0).unwrap();
            } else {
                v1 = self.vertices.get(i - 1).unwrap();
                v2 = self.vertices.get(i).unwrap();
            }
            //let v1n = v1.normalize_or_zero();
            let v1r = v1.rotate(Vec2::from_angle(self.rot));
            let v2r = v2.rotate(Vec2::from_angle(self.rot));
            draw_line(
                v1r.x + x0,
                v1r.y + y0,
                v2r.x + x0,
                v2r.y + y0,
                4.0,
                self.color,
            );
        }
        let dir = Vec2::from_angle(self.rot) * self.size;
        draw_line(x0, y0, x0 + dir.x, y0 + dir.y, 4.0, self.color);
        self.draw_info(font);
    }

    fn draw_info(&self, font: Font) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let text_cfg = TextParams {
            font: font,
            font_size: 14,
            color: WHITE,
            ..Default::default()
        };
        let mut k_eng = self.eng_k;
        k_eng = (k_eng / 1000.0).round();
        let rot = self.rot;
        let info = format!(
            "mass: {} | E(k): {} | rot: {}",
            self.mass.round(),
            k_eng,
            (rot * 10.0).round() / 10.0
        );
        let txt_center = get_text_center(&info, Some(font), 14, 1.0, 0.0);
        draw_text_ex(
            &info,
            x0 - txt_center.x,
            y0 - txt_center.y - self.size * 2.0,
            text_cfg,
        );
    }

    fn controll_system(&mut self) {
        self.controller = Vec2::ZERO;
        if is_key_down(KeyCode::W) {
            self.controller.x = 1.0;
        }
        if is_key_down(KeyCode::A) {
            self.controller.y = -1.0;
        }
        if is_key_down(KeyCode::D) {
            self.controller.y = 1.0;
        }
    }

    fn check_edges(&mut self, object: &mut RigidBody) {
        let mut raw_pos = matric_to_vec2(object.position().translation);
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
            object.set_position(make_isometry(raw_pos.x, raw_pos.y, self.rot), true);
        }
    }

    pub fn update(&mut self, physics: &mut World) {
        self.controll_system();
        match self.physics_handle {
            Some(handle) => {
                let physics_data = physics.get_physics_data(handle);
                self.eng_k = physics_data.kin_eng.unwrap_or(0.0);
                self.pos = physics_data.position;
                self.rot = physics_data.rotation;
                self.mass = physics_data.mass;
                match physics.rigid_bodies.get_mut(handle) {
                    Some(body) => {
                        let dir = Vec2::from_angle(self.rot);
                        let v = dir * self.controller.x * JET_IMPULSE;
                        let impulse: Vector2<f32> = Vector2::new(v.x, v.y);
                        let turning = self.controller.y * JET_TORQUE;
                        body.apply_impulse(impulse, true);
                        body.apply_torque_impulse(turning, true);
                        self.check_edges(body);
                    }
                    None => {}
                }
            }
            None => {}
        }
    }
}

pub struct JetCollector {
    pub jets: HashMap<u64, Jet>,
}

impl JetCollector {
    pub fn new() -> Self {
        Self {
            jets: HashMap::new(),
        }
    }

    pub fn add_many_jets(&mut self, jets_num: usize, physics_world: &mut World) {
        for _ in 0..jets_num {
            let jet = Jet::new();
            _ = self.add_jet(jet, physics_world);
        }
    }

    pub fn add_jet(&mut self, mut jet: Jet, physics_world: &mut World) -> u64 {
        let key = jet.key;
        //let handle = physics_world.add_poly_body(key,&jet.pos, jet.points2.clone());
        let handle = physics_world.add_jet_hull(key, &jet.pos, jet.points.clone());
        jet.physics_handle = Some(handle);
        self.jets.insert(key, jet);
        return key;
    }

    pub fn get(&self, id: u64) -> Option<&Jet> {
        return self.jets.get(&id);
    }

    pub fn remove(&mut self, id: u64) {
        self.jets.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, Jet> {
        return self.jets.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<u64, Jet> {
        return self.jets.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.jets.len();
    }
}
