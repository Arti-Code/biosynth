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
use rapier2d::prelude::RigidBodyHandle;

pub trait DynamicElement {
    //fn create() -> Self;
    fn draw(&self, font: Font);
    fn update(&mut self, dt: f32, physics: &mut World);
}

pub struct Asteroid {
    pub key: u64,
    pub pos: Vec2,
    pub rot: f32,
    pub vel: f32,
    pub ang_vel: f32,
    pub size: f32,
    pub color: color::Color,
    pub points: Vec<Vec2>,
    pub points2: Vec<Point2<f32>>,
    pub shape: ConvexPolygon,
    pub physics_handle: Option<RigidBodyHandle>,
    pub kin_eng: f32,
}

impl Asteroid {
    pub fn new() -> Self {
        let size = rand::gen_range(ASTER_SIZE_MIN, ASTER_SIZE_MAX);
        //let n = size / 4;
        let n = rand::gen_range(4, 4);
        let points = map_polygon(n as usize, size as f32, 0.2);
        let points2 = vec2_to_point2_collection(&points);
        Self {
            key: thread_rng().gen::<u64>(),
            pos: random_position(WORLD_W, WORLD_H),
            rot: random_rotation(),
            vel: rand::gen_range(0.0, 1.0) * ASTER_SPEED,
            ang_vel: rand::gen_range(-1.0, 1.0),
            size: (size as f32),
            color: random_color(),
            points,
            points2: points2.clone(),
            shape: ConvexPolygon::from_convex_polyline(points2).unwrap(),
            physics_handle: None,
            kin_eng: 0.0,
        }
    }
}

impl DynamicElement for Asteroid {
    fn draw(&self, font: Font) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let l = self.points.len();
        for i in 1..=l {
            let mut v1: &Vec2;
            let mut v2: &Vec2;
            if i == l {
                v1 = self.points.get(i - 1).unwrap();
                v2 = self.points.get(0).unwrap();
            } else {
                v1 = self.points.get(i - 1).unwrap();
                v2 = self.points.get(i).unwrap();
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
        let text_cfg = TextParams {
            font,
            font_size: 14,
            color: WHITE,
            ..Default::default()
        };
        let kin_eng_info = String::from(&(self.kin_eng / 10000.0).round().to_string());
        let txt_center = get_text_center(&kin_eng_info, Some(font), 14, 1.0, 0.0);
        draw_text_ex(
            &kin_eng_info,
            x0 - txt_center.x,
            y0 - txt_center.y,
            text_cfg,
        );
        //draw_text(kin_eng_info, x0-18.0, y0, 16.0, WHITE);
    }
    fn update(&mut self, dt: f32, physics: &mut World) {
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
                            let v2: Vec2 =
                                Vec2::new(body.linvel().data.0[0][0], body.linvel().data.0[0][1])
                                    * -1.0;
                            //body.set_position(make_isometry(raw_pos.x, raw_pos.y, self.rot), true);
                            body.set_linvel([v2.x, v2.y].into(), true);
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }
    }
}

pub struct DynamicCollector {
    pub elements: HashMap<u64, Asteroid>,
}

impl DynamicCollector {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    pub fn add_many_elements(&mut self, elements_num: usize, physics_world: &mut World) {
        for _ in 0..elements_num {
            let element = Asteroid::new();
            _ = self.add_element(element, physics_world);
        }
    }

    pub fn add_element(&mut self, mut element: Asteroid, physics_world: &mut World) -> u64 {
        let key = element.key;
        //let handle = physics_world.add_poly_body(key,&element.pos, element.points2.clone());
        let handle = physics_world.add_poly_body(key, &element.pos, element.points2.clone());
        element.physics_handle = Some(handle);
        self.elements.insert(key, element);
        return key;
    }

    pub fn get(&self, id: u64) -> Option<&Asteroid> {
        return self.elements.get(&id);
    }

    pub fn remove(&mut self, id: u64) {
        self.elements.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, Asteroid> {
        return self.elements.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<u64, Asteroid> {
        return self.elements.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.elements.len();
    }
}

pub struct StaticBox {
    pub key: u64,
    pub pos: Vec2,
    pub rot: f32,
    pub size: f32,
    pub color: color::Color,
    pub shape: Cuboid,
    pub physics_handle: Option<RigidBodyHandle>,
}

impl StaticBox {
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        //let size = rand::gen_range(ASTER_SIZE_MIN, ASTER_SIZE_MAX);
        Self {
            key: thread_rng().gen::<u64>(),
            pos: Vec2::new(x, y),
            rot: 0.0,
            size,
            color: random_color(),
            shape: Cuboid::new([size / 2., size / 2.].into()),
            physics_handle: None,
        }
    }
    pub fn draw(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let size = self.size;
        //draw_circle_lines(x0, y0, self.size, 4.0, WHITE);
        draw_rectangle_lines(x0 - size, y0 - 10.0, size * 2.0, 10.0, 2., self.color);
    }

    pub fn update(&mut self, dt: f32, physics: &mut World) {
        match self.physics_handle {
            Some(handle) => {
                let physics_data = physics.get_physics_data(handle);
                self.pos = physics_data.position;
                self.rot = physics_data.rotation;
            }
            None => {}
        }
    }
}

pub struct StaticCollector {
    pub elements: HashMap<u64, StaticBox>,
}

impl StaticCollector {
    pub fn new() -> Self {
        Self {
            elements: HashMap::new(),
        }
    }

    pub fn add_element(&mut self, mut element: StaticBox, physics_world: &mut World) -> u64 {
        let key = element.key;
        //let handle = physics_world.add_poly_body(key,&element.pos, element.points2.clone());
        let handle = physics_world.add_static_box(key, &element.pos, element.size);
        element.physics_handle = Some(handle);
        self.elements.insert(key, element);
        return key;
    }

    pub fn get(&self, id: u64) -> Option<&StaticBox> {
        return self.elements.get(&id);
    }

    pub fn remove(&mut self, id: u64) {
        self.elements.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, StaticBox> {
        return self.elements.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<u64, StaticBox> {
        return self.elements.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.elements.len();
    }
}
