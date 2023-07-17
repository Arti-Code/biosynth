#![allow(unused)]
use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::f32::consts::PI;

use crate::consts::*;
use crate::kinetic::make_isometry;
use crate::neuro::*;
use crate::timer::*;
use crate::util::*;
use crate::world::*;
use ::rand::{thread_rng, Rng};
use macroquad::{color, prelude::*};
use rapier2d::geometry::*;
use rapier2d::prelude::{RigidBody, RigidBodyHandle};

pub struct Agent {
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
    pub pulse: f32,
    pub shape: Ball,
    motor: bool,
    motor_phase: f32,
    motor_phase2: f32,
    motor_side: bool,
    analize_timer: Timer,
    analizer: DummyNetwork,
    pub alife: bool,
    //enemy: Detection,
    pub detected: Option<Detected>,
    pub enemy: Option<RigidBodyHandle>,
    pub enemy_position: Option<Vec2>,
    pub enemy_dir: Option<f32>,
    pub physics_handle: Option<RigidBodyHandle>,
}

impl Agent {
    pub fn new() -> Self {
        let s = rand::gen_range(AGENT_SIZE_MIN, AGENT_SIZE_MAX) as f32;
        let motor = thread_rng().gen_bool(1.0);
        let p = thread_rng().gen_range(0.2..0.8);

        Self {
            key: thread_rng().gen::<u64>(),
            pos: random_position(WORLD_W, WORLD_H),
            rot: random_rotation(),
            mass: 0.0,
            vel: 0.0,
            ang_vel: 0.0,
            size: s,
            vision_range: (rand::gen_range(0.5, 1.5) * AGENT_VISION_RANGE).round(),
            max_eng: s.powi(2) * 10.0,
            eng: s.powi(2) * 10.0,
            color: random_color(),
            pulse: rand::gen_range(0.0, 1.0),
            shape: Ball { radius: s },
            motor: motor,
            motor_phase: p,
            motor_phase2: p,
            motor_side: true,
            analize_timer: Timer::new(0.3, true, true, true),
            analizer: DummyNetwork::new(2),
            alife: true,
            detected: None,
            enemy: None,
            enemy_position: None,
            enemy_dir: None,
            //enemy: Detection::new_empty(),
            physics_handle: None,
        }
    }

    pub fn draw(&self, field_of_view: bool, font: Font) {
        //let dir = Vec2::from_angle(self.rot);
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        //let x1 = x0 + dir.x * self.size * 1.0;
        //let y1 = y0 + dir.y * self.size * 1.0;
        //let x2 = x0 + dir.x * self.size * 2.0;
        //let y2 = y0 + dir.y * self.size * 2.0;
        if self.motor {
            let tail = Vec2::from_angle(self.rot + (self.motor_phase * 0.5));
            //let tail2 = Vec2::from_angle(self.rot + (self.motor_phase2 * 0.5));
            let x3 = x0 - tail.x * self.size * 1.4;
            let y3 = y0 - tail.y * self.size * 1.4;
            //let x4 = x0 - tail2.x * self.size * 2.4;
            //let y4 = y0 - tail2.y * self.size * 2.4;
            draw_circle(x3, y3, self.size / 2.0, self.color);
            //draw_circle(x4, y4, self.size / 3.0, self.color);
            //draw_line(x4, y4, x3, y3, self.size / 2.0, self.color)
        }
        let pulse = (self.pulse * 2.0) - 1.0;
        self.draw_target();
        draw_circle_lines(x0, y0, self.size, 2.0, self.color);
        draw_circle(x0, y0, (self.size / 2.0) * pulse.abs(), self.color);
        self.draw_front();
        //draw_line(x1, y1, x2, y2, 1.0, self.color);
        //draw_text(&self.key.to_string(), x0-80.0, y0-self.size*2.0, 20.0, WHITE);
        if field_of_view {
            draw_circle_lines(x0, y0, self.vision_range, 0.75, GRAY);
        }
        self.draw_info(font);
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
                draw_line(x0l, y0l, x1, y1, 0.75, self.color);
                draw_line(x0r, y0r, x1, y1, 0.75, self.color);
            }
        }
    }

    fn draw_info(&self, font: Font) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let text_cfg = TextParams {
            font: font,
            font_size: 13,
            color: WHITE,
            ..Default::default()
        };
        let rot = self.rot;
        let mass = self.mass;
        let info = format!("rot: {}", (rot * 10.0).round() / 10.0);
        let info_mass = format!("mass: {}", mass.round());
        let txt_center = get_text_center(&info, Some(font), 13, 1.0, 0.0);
        let txt_center2 = get_text_center(&info_mass, Some(font), 13, 1.0, 0.0);
        draw_text_ex(
            &info,
            x0 - txt_center.x,
            y0 - txt_center.y + self.size * 2.0,
            text_cfg,
        );
        draw_text_ex(
            &info_mass,
            x0 - txt_center.x,
            y0 - txt_center.y + self.size * 2.0 + 15.0,
            text_cfg,
        );
    }

    pub fn update2(&mut self, physics: &mut World) {
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
                        let rot = self.ang_vel * AGENT_TORQUE * self.size.powi(2);
                        let v = dir * self.vel * AGENT_IMPULSE * self.size.powi(2);
                        //body.set_linvel([v.x, v.y].into(), true);
                        //body.set_angvel(self.ang_vel, true);
                        body.apply_impulse([v.x, v.y].into(), true);
                        body.apply_torque_impulse(rot, true);
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

    fn update_enemy_position(&mut self, physics: &World) {
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

    pub fn update(&mut self, dt: f32, physics: &World) -> bool {
        if self.analize_timer.update(dt) {
            match self.physics_handle {
                Some(handle) => {
                    if let Some(tg) = physics.get_closesd_agent(handle) {
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
            let outputs = self.analizer.analize();
            if outputs[0] >= 0.0 {
                self.vel = outputs[0];
            } else {
                self.vel = 0.0;
            }
            self.ang_vel = outputs[1];
        }
        self.pulse = (self.pulse + dt * 0.25) % 1.0;
        if self.motor {
            if self.motor_side {
                self.motor_phase = self.motor_phase + dt * (self.vel) * 30.0;
                if self.motor_phase >= 1.0 {
                    self.motor_side = false;
                }
            } else {
                self.motor_phase = self.motor_phase - dt * (self.vel) * 30.0;
                if self.motor_phase <= -1.0 {
                    self.motor_side = true;
                }
            }
            if self.motor_side {
                self.motor_phase2 = self.motor_phase2 + dt * (0.75 + self.vel);
            } else {
                self.motor_phase2 = self.motor_phase2 - dt * (0.75 + self.vel);
            }
        }
        //self.calc_energy(dt);
        return self.alife;
    }

    fn calc_energy(&mut self, dt: f32) {
        if self.eng > 0.0 {
            self.eng -= self.size * 1.0 * dt;
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

pub struct AgentsBox {
    pub agents: HashMap<u64, Agent>,
}

impl AgentsBox {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub fn add_many_agents(&mut self, agents_num: usize, physics_world: &mut World) {
        for _ in 0..agents_num {
            let agent = Agent::new();
            _ = self.add_agent(agent, physics_world);
        }
    }

    pub fn add_agent(&mut self, mut agent: Agent, physics_world: &mut World) -> u64 {
        let key = agent.key;
        let handle = physics_world.add_dynamic_agent(
            key,
            &agent.pos,
            agent.size,
            agent.rot,
            Some(agent.vision_range),
        );
        agent.physics_handle = Some(handle);
        self.agents.insert(key, agent);
        return key;
    }

    pub fn get(&self, id: u64) -> Option<&Agent> {
        return self.agents.get(&id);
    }

    pub fn remove(&mut self, id: u64) {
        self.agents.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, Agent> {
        return self.agents.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<u64, Agent> {
        return self.agents.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.agents.len();
    }
}

pub struct Detected {
    pub target_handle: RigidBodyHandle,
    pub dist: f32,
}
