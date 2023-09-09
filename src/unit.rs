#![allow(unused)]


use std::f32::consts::PI;
use crate::neuro::*;
use crate::timer::*;
use crate::util::*;
use crate::physics::*;
use macroquad::{color, prelude::*};
use macroquad::rand::*;
use rapier2d::geometry::*;
use rapier2d::na::Vector2;
use rapier2d::prelude::{RigidBody, RigidBodyHandle};


pub struct BodyPart {
    pub rel_pos: Vec2,
    pub color: Color,
    pub shape: SharedShape,
    handle: Option<ColliderHandle>,
}

impl BodyPart {
    pub fn add_new(relative_position: Vec2, size: f32, color: Color) -> Self {
        Self {
            color,
            rel_pos: relative_position,
            shape: SharedShape::ball(size),
            handle: None,
        }
    }

    pub fn draw_circle(&self, position: &Vec2, rot: f32) {
        let mut pos = Vec2::from_angle(rot).rotate(self.rel_pos);
        pos += position.clone();
        let size = self.shape.as_ball().unwrap().radius;
        draw_circle(pos.x, pos.y, size, self.color); 
    }

    pub fn get_rel_position(&self) -> Vec2 {
        return self.rel_pos
    }

    pub fn get_color(&self) -> Color {
        return self.color;
    }

    pub fn get_shape(&self) -> SharedShape {
        return self.shape.clone();
    }

    pub fn set_collider_handle(&mut self, collider_handle: ColliderHandle) {
        self.handle = Some(collider_handle);
    }

    pub fn get_collider_handler(&self) -> Option<ColliderHandle> {
        return self.handle;
    }
}


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
    analize_timer: Timer,
    analizer: DummyNetwork,
    network: Network,
    pub alife: bool,
    pub contacts: Vec<(RigidBodyHandle, f32)>,
    pub detected: Option<Detected>,
    pub enemy: Option<RigidBodyHandle>,
    pub enemy_position: Option<Vec2>,
    pub enemy_dir: Option<f32>,
    pub physics_handle: RigidBodyHandle,
    pub body_parts: Vec<BodyPart>,
    pub settings: Settings
}

impl Unit {
    
    pub fn new(settings: &Settings, physics: &mut PhysicsWorld) -> Self {
        let key = gen_range(u64::MIN, u64::MAX);
        let size = rand::gen_range(settings.agent_size_min, settings.agent_size_max) as f32;
        let pos = random_position(settings.world_w as f32, settings.world_h as f32);
        let shape = SharedShape::ball(size);
        let rbh = physics.add_dynamic(key, &pos, 0.0, shape.clone(), PhysicsProperities::default());
        let color = random_color();
        let mut network = Network::new();
        network.build(3, 0, 3, 0.25);
        let mut parts: Vec<BodyPart> = Self::create_body_parts(0, size*0.66, color, rbh, physics);
        Self {
            key: gen_range(u64::MIN, u64::MAX),
            pos,
            //rot: random_rotation(),
            rot: 0.0,
            mass: 0.0,
            vel: 0.0,
            ang_vel: 0.0,
            size,
            vision_range: (rand::gen_range(0.5, 1.5) * settings.agent_vision_range).round(),
            max_eng: size.powi(2) * 10.0,
            eng: size.powi(2) * 10.0,
            color,
            shape,
            analize_timer: Timer::new(0.3, true, true, true),
            analizer: DummyNetwork::new(2),
            network,
            alife: true,
            detected: None,
            enemy: None,
            enemy_position: None,
            enemy_dir: None,
            contacts: Vec::new(),
            physics_handle: rbh,
            body_parts: parts,
            settings: settings.clone()
        }
    }

    fn create_body_parts(num: usize, size: f32, color: Color, rigid_handle: RigidBodyHandle, physics: &mut PhysicsWorld) -> Vec<BodyPart> {
        let mut parts: Vec<BodyPart> = vec![];
        let step = 2.0*PI/num as f32;
        let size2 = 3.0_f32.sqrt() * size;
        for i in 0..num {
            let rel_pos = Vec2::from_angle(i as f32 * step) * size2;
            let mut part = BodyPart::add_new(rel_pos, size, color);
            let coll_handle = physics.add_collider(rigid_handle, &rel_pos, 0.0, part.get_shape(), PhysicsProperities::free());
            part.set_collider_handle(coll_handle);
            parts.push(part);
        }
        return parts;
    }

    pub fn draw(&self, selected: bool, font: &Font) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        if self.settings.agent_eng_bar {
            let e = self.eng/self.max_eng;
            self.draw_status_bar(e, SKYBLUE, ORANGE, Vec2::new(0.0, self.size*1.5+4.0));
        }
        for part in self.body_parts.iter() {
            part.draw_circle(&self.pos, self.rot);
        }
        draw_circle(x0, y0, self.size, self.color);
        self.draw_front();
        if selected {
            self.draw_target();
            //draw_circle_lines(x0, y0, self.vision_range, 2.0, GRAY);
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

    fn analize(&mut self) {
        let enemy_dist = match self.enemy_position {
            None => 0.0,
            Some(pos2) => {
                let dist = pos2.distance(self.pos);
                1.0-(dist/self.vision_range)
            },
        };
        let enemy_ang = match self.enemy_dir {
            None => 0.0,
            Some(dir) => {
                dir
            },
        };
        let (inp_keys, _, _) = self.network.get_node_keys_by_type();
        let outputs = self.analizer.analize();
        if outputs[0] >= 0.0 {
            self.vel = outputs[0];
        } else {
            self.vel = 0.0;
        }
        self.ang_vel = outputs[1];
    }

    fn draw_front(&self) {
        let dir = Vec2::from_angle(self.rot);
        let v0 = dir * self.size;
        let x0 = self.pos.x + v0.x;
        let y0 = self.pos.y + v0.y;
        let x1 = self.pos.x + dir.x * self.size * 1.6;
        let y1 = self.pos.y + dir.y * self.size * 1.6;
        draw_line(x0, y0, x1, y1, 3.0, self.color);
    }

    fn draw_circle(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        draw_circle_lines(x0, y0, self.size, 4.0, self.color);
        //self.draw_front();
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
        //let mass = self.mass;
        let info = format!("rot: {}", (rot * 10.0).round() / 10.0);
        //let info_mass = format!("mass: {}", mass.round());
        let txt_center = get_text_center(&info, Some(*font), 13, 1.0, 0.0);
        draw_text_ex(&info, x0 - txt_center.x, y0 - txt_center.y + self.size * 2.0 + 30.0, text_cfg.clone());
        //draw_text_ex(&info_mass, x0 - txt_center.x, y0 - txt_center.y + self.size * 2.0 + 43.0, text_cfg.clone());
    }

    fn draw_status_bar(&self, percent: f32, color1: Color, color2: Color, offset: Vec2) {
        let xc = self.pos.x + offset.x; let yc = self.pos.y + offset.y;
        let x0 = xc-20.0; let y0 = yc -1.5;
        let w = 40.0*percent;
        draw_rectangle(x0, y0, 40.0, 3.0, color2);
        draw_rectangle(x0, y0, w, 3.0, color1);
    }

    fn update_physics(&mut self, physics: &mut PhysicsWorld) {
        self.update_enemy_position(physics);
        let physics_data = physics.get_physics_data(self.physics_handle);
        self.pos = physics_data.position;
        self.rot = physics_data.rotation;
        self.mass = physics_data.mass;
        match physics.rigid_bodies.get_mut(self.physics_handle) {
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
        self.contacts.clear();
        let contacts = physics.get_contacts_set(self.physics_handle, self.size);
        for contact in contacts.iter() {
            if let Some(pos2) = physics.get_object_position(*contact) {
                let mut rel_pos = pos2 - self.pos;
                rel_pos = rel_pos.normalize_or_zero();
                let target_angle = rel_pos.angle_between(Vec2::from_angle(self.rot));
                self.contacts.push((*contact, target_angle));
            }

        }
    }

    fn watch(&mut self, physics: &PhysicsWorld) {
        if let Some(tg) = physics.get_closesd_agent(self.physics_handle, self.vision_range) {
            self.enemy = Some(tg);
            self.update_enemy_position(physics);
        } else {
            self.enemy = None;
            self.enemy_position = None;
            self.enemy_dir = None;
        }
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

pub struct Detected {
    pub target_handle: RigidBodyHandle,
    pub dist: f32,
}
