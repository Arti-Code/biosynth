//#![allow(unused)]


use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::f32::consts::PI;
use crate::consts::{WORLD_H, WORLD_W};
use crate::neuro::*;
//use crate::sim::*;
use crate::timer::*;
use crate::util::*;
use crate::physics::*;
use macroquad::{color, prelude::*};
use macroquad::rand::*;
use rapier2d::geometry::*;
use rapier2d::na::Vector2;
use rapier2d::prelude::{RigidBody, RigidBodyHandle};

pub struct Unit {
    pub key: u64,
    pub pos: Vec2,
    pub rot: f32,
    pub mass: f32,
    pub vel: f32,
    pub ang_vel: f32,
    pub size: f32,
    pub vision_range: f32,
    pub max_eng: f32,
    pub eng: f32,
    pub color: color::Color,
    pub shape: SharedShape,
    pub vertices: Vec<Vec2>,
    analize_timer: Timer,
    analizer: DummyNetwork,
    pub alife: bool,
    pub contacts: Vec<(RigidBodyHandle, f32)>,
    pub detected: Option<Detected>,
    pub enemy: Option<RigidBodyHandle>,
    pub enemy_position: Option<Vec2>,
    pub enemy_dir: Option<f32>,
    pub physics_handle: Option<RigidBodyHandle>,
    settings: Settings
}

impl Unit {
    
    pub fn new(settings: &Settings) -> Self {
        let s = rand::gen_range(settings.agent_size_min, settings.agent_size_max) as f32;
        let v0 = Vec2::from_angle(0.0)*s;
        let v1 = Vec2::from_angle(-2.0*PI/3.0)*s;
        let v2 = Vec2::from_angle(2.0*PI/3.0)*s;
        let vertices = vec![v0, v2, v1];
        //let vecs = vec2_to_point2_collection(&vertices).to_owned();
        //let points = vecs.as_slice();
        Self {
            key: gen_range(u64::MIN, u64::MAX),
            pos: random_position(settings.world_w as f32, settings.world_h as f32),
            //rot: random_rotation(),
            rot: 0.0,
            mass: 0.0,
            vel: 0.0,
            ang_vel: 0.0,
            size: s,
            vision_range: (rand::gen_range(0.5, 1.5) * settings.agent_vision_range).round(),
            max_eng: s.powi(2) * 10.0,
            eng: s.powi(2) * 10.0,
            color: random_color(),
            vertices: vertices,
            shape: SharedShape::ball(s),
            analize_timer: Timer::new(0.3, true, true, true),
            analizer: DummyNetwork::new(2),
            alife: true,
            detected: None,
            enemy: None,
            enemy_position: None,
            enemy_dir: None,
            contacts: Vec::new(),
            physics_handle: None,
            settings: settings.clone()
        }
    }

    pub fn draw(&self, selected: bool, font: &Font) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        //self.draw_tri();
        if self.settings.agent_eng_bar {
            let e = self.eng/self.max_eng;
            self.draw_status_bar(e, SKYBLUE, ORANGE, Vec2::new(0.0, self.size*1.5+4.0));
        }
        self.draw_circle();
        if selected {
            self.draw_target();
            draw_circle_lines(x0, y0, self.vision_range, 2.0, GRAY);
            self.draw_info(&font);
        }
    }    

    pub fn update(&mut self, dt: f32, physics: &mut PhysicsWorld) -> bool {
        if self.analize_timer.update(dt) {
            self.watch(physics);
            self.update_contacts(physics);
            self.analize();
        }
        for (_contact, ang) in self.contacts.iter() {
            if *ang <= PI/4.0 && *ang >= -PI/4.0 {
                //self.add_energy(dt);
                self.eng += dt*10.0*self.size;
                if self.eng > self.max_eng {
                    self.eng = self.max_eng;
                }
            }
        }
        self.update_physics(physics);
        self.calc_timers(dt);
        self.calc_energy(dt);
        return self.alife;
    }

    fn draw_front(&self) {
        let dir = Vec2::from_angle(self.rot);
        let v0l = Vec2::from_angle(self.rot - PI / 2.0) * self.size;
        let v0r = Vec2::from_angle(self.rot + PI / 2.0) * self.size;
        let x0l = self.pos.x + v0l.x;
        let y0l = self.pos.y + v0l.y;
        let x0r = self.pos.x + v0r.x;
        let y0r = self.pos.y + v0r.y;
        let x2 = self.pos.x + dir.x * self.size * 2.0;
        let y2 = self.pos.y + dir.y * self.size * 2.0;
        draw_line(x0l, y0l, x2, y2, 2.0, self.color);
        draw_line(x0r, y0r, x2, y2, 2.0, self.color);
    }

    fn _draw_tri(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let dir = Vec2::from_angle(self.rot);
        let mut v0 = self.vertices[0];
        v0 = dir.rotate(v0);
        let mut v1 = self.vertices[1];
        v1 = dir.rotate(v1);
        let mut v2 = self.vertices[2];
        v2 = dir.rotate(v2);
        draw_triangle_lines(self.pos+v0, self.pos+v1, self.pos+v2, 2.0, self.color);
        draw_line(x0, y0, x0+dir.x*self.size, y0+dir.y*self.size, 3.0, GREEN)
        //draw_poly_lines(x0, y0, 3, self.size, 0.0, 2.0, RED);
    }

    fn draw_circle(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        draw_circle_lines(x0, y0, self.size, 4.0, self.color);
        self.draw_front();
    }

    fn draw_target(&self) {
        //if !self.enemy.is_none() {
        if let Some(_rb) = self.enemy {
            if let Some(enemy_position) = self.enemy_position {
                let v0l = Vec2::from_angle(self.rot - PI / 2.0) * self.size;
                let v0r = Vec2::from_angle(self.rot + PI / 2.0) * self.size;
                let x0l = self.pos.x + v0l.x;
                let y0l = self.pos.y + v0l.y;
                let x0r = self.pos.x + v0r.x;
                let y0r = self.pos.y + v0r.y;
                let x1 = enemy_position.x;
                let y1 = enemy_position.y;
                draw_line(x0l, y0l, x1, y1, 2.0, self.color);
                draw_line(x0r, y0r, x1, y1, 2.0, self.color);
            }
        }
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
        let info = format!("rot: {}", (rot * 10.0).round() / 10.0);
        let info_mass = format!("mass: {}", mass.round());
        let txt_center = get_text_center(&info, Some(*font), 13, 1.0, 0.0);
        draw_text_ex(&info, x0 - txt_center.x, y0 - txt_center.y + self.size * 2.0 + 30.0, text_cfg.clone());
        draw_text_ex(&info_mass, x0 - txt_center.x, y0 - txt_center.y + self.size * 2.0 + 43.0, text_cfg.clone());
    }

    fn draw_status_bar(&self, percent: f32, color1: Color, color2: Color, offset: Vec2) {
        let xc = self.pos.x + offset.x; let yc = self.pos.y + offset.y;
        let x0 = xc-20.0; let y0 = yc -1.5;
        let w = 40.0*percent;
        draw_rectangle(x0, y0, 40.0, 3.0, color2);
        draw_rectangle(x0, y0, w, 3.0, color1);
    }

    fn update_physics(&mut self, physics: &mut PhysicsWorld) {
        match self.physics_handle {
            Some(handle) => {
                self.update_enemy_position(physics);
                let physics_data = physics.get_physics_data(handle);
                self.pos = physics_data.position;
                self.rot = physics_data.rotation;
                self.mass = physics_data.mass;
                match physics.rigid_bodies.get_mut(handle) {
                    Some(body) => {
                        let dir = Vec2::from_angle(self.rot);
                        let v = dir * self.vel * self.settings.agent_speed;
                        let rot = self.ang_vel * self.settings.agent_rotate;
                        body.set_linvel(Vector2::new(v.x, v.y), true);
                        body.set_angvel(rot, true);
                        self.check_edges(body);
                    }
                    None => {}
                }
            }
            None => {}
        }
    }

    fn check_edges(&mut self, body: &mut RigidBody) {
        let mut raw_pos = matrix_to_vec2(body.position().translation);
        let mut out_of_edge = false;
        if raw_pos.x < 0.0 {
            raw_pos.x = 0.0;
            out_of_edge = true;
        } else if raw_pos.x > self.settings.world_w as f32 {
            raw_pos.x = self.settings.world_w as f32;
            out_of_edge = true;
        }
        if raw_pos.y < 0.0 {
            raw_pos.y = 0.0;
            out_of_edge = true;
        } else if raw_pos.y > self.settings.world_h as f32 {
            raw_pos.y = self.settings.world_h as f32;
            out_of_edge = true;
        }
        if out_of_edge {
            body.set_position(make_isometry(raw_pos.x, raw_pos.y, self.rot), true);
            //body.set_linvel([0.0, 0.0].into(), true);
            //self.vel = 0.0;
        }
    }

    fn update_enemy_position(&mut self, physics: &PhysicsWorld) {
        if let Some(rb) = self.enemy {
            if let Some(enemy_position) = physics.get_object_position(rb) {
                self.enemy_position = Some(enemy_position);
                let rel_pos = enemy_position - self.pos;
                let enemy_dir = rel_pos.angle_between(Vec2::from_angle(self.rot));
                self.enemy_dir = Some(enemy_dir);
            } else {
                self.enemy = None;
                self.enemy_position = None;
                self.enemy_dir = None;
            }
        } else if self.enemy_position.is_some() {
            self.enemy_position = None;
            self.enemy_dir = None;
        }
    }

    fn update_contacts(&mut self, physics: &mut PhysicsWorld) {
        match self.physics_handle {
            Some(rbh) => {
                self.contacts.clear();
                let contacts = physics.get_contacts_set(rbh, self.size);
                for contact in contacts.iter() {
                    if let Some(pos2) = physics.get_object_position(*contact) {
                        let mut rel_pos = pos2 - self.pos;
                        rel_pos = rel_pos.normalize_or_zero();
                        let target_angle = rel_pos.angle_between(Vec2::from_angle(self.rot));
                        self.contacts.push((*contact, target_angle));
                    }

                }
            },
            None => {},
        }
    }

    fn watch(&mut self, physics: &PhysicsWorld) {
        match self.physics_handle {
            Some(handle) => {
                if let Some(tg) = physics.get_closesd_agent(handle, self.vision_range) {
                    self.enemy = Some(tg);
                    self.update_enemy_position(physics);
                } else {
                    self.enemy = None;
                    self.enemy_position = None;
                    self.enemy_dir = None;
                }
            }
            None => {}
        }
    }

    fn analize(&mut self) {
        let outputs = self.analizer.analize();
        if outputs[0] >= 0.0 {
            self.vel = outputs[0];
        } else {
            self.vel = 0.0;
        }
        self.ang_vel = outputs[1];
    }

    fn calc_timers(&mut self, _dt: f32) {

    }

    fn calc_energy(&mut self, dt: f32) {
        let basic_loss = self.size * dt * 0.5;
        let move_loss = self.vel * self.size * dt * 2.0;
        let loss = basic_loss + move_loss;
        if self.eng > 0.0 {
            self.eng -= loss;
        } else {
            self.eng = 0.0;
            self.alife = false;
        }
    }

    pub fn _add_energy(&mut self, e: f32) {
        self.eng += e;
        if self.eng > self.max_eng {
            self.eng = self.max_eng;
        }
    }
}

pub struct UnitsBox {
    pub agents: HashMap<u64, Unit>,
}

impl UnitsBox {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub fn reload_settings(&mut self, settings: &Settings) {
        for (_, agent) in self.get_iter_mut() {
            agent.settings = settings.clone();
        }
    }

    pub fn add_many_agents(&mut self, agents_num: usize, physics_world: &mut PhysicsWorld, settings: &Settings) {
        for _ in 0..agents_num {
            let agent = Unit::new(settings);
            _ = self.add_agent(agent, physics_world);
        }
    }

    pub fn add_agent(&mut self, mut agent: Unit, physics_world: &mut PhysicsWorld) -> u64 {
        let key = agent.key;
        let handle = physics_world.add_dynamic(key, &agent.pos, agent.rot, agent.shape.clone(), PhysicsProperities::default());
        agent.physics_handle = Some(handle);
        self.agents.insert(key, agent);
        return key;
    }

    pub fn get(&self, id: u64) -> Option<&Unit> {
        return self.agents.get(&id);
    }

    pub fn _remove(&mut self, id: u64) {
        self.agents.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, Unit> {
        return self.agents.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<u64, Unit> {
        return self.agents.iter_mut();
    }

    pub fn _count(&self) -> usize {
        return self.agents.len();
    }
}

pub struct Detected {
    pub target_handle: RigidBodyHandle,
    pub dist: f32,
}
