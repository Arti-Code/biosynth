#![allow(unused)]


use std::collections::HashMap;
use std::f32::consts::PI;
use crate::neuro::*;
use crate::timer::*;
use crate::util::*;
use crate::physics::*;
use crate::globals::*;
use macroquad::{color, prelude::*};
use macroquad::rand::*;
use rapier2d::geometry::*;
use rapier2d::na::Vector2;
use rapier2d::prelude::{RigidBody, RigidBodyHandle};
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use serde_json::{self, *};
use std::fs;


pub struct NeuroTable {
    pub inputs: Vec<(u64, String, Option<f32>)>,
    pub outputs: Vec<(u64, String, f32)>,
}


struct NeuroValue {
    pub label: String,
    pub val: f32,
}
pub struct NeuroMap {
    pub sensors: HashMap<String, u64>,
    pub effectors: HashMap<String, u64>,
    signals: HashMap<u64, f32>,
    actions: HashMap<String, f32>,
}

impl NeuroMap {

    pub fn new() -> Self {
        Self { 
            sensors: HashMap::new(), 
            effectors: HashMap::new(),
            signals: HashMap::new(),
            actions: HashMap::new(), 
        }
    }

    pub fn add_sensor(&mut self, name: &str, node_key: u64) {
        self.sensors.insert(name.to_string(), node_key);
    }

    pub fn add_sensors(&mut self, pairs: Vec<(u64, String)>) {
        for (k, s) in pairs.iter() {
            self.add_sensor(s, *k);
        }
    }

    pub fn add_effector(&mut self, name: &str, node_key: u64) {
        self.effectors.insert(name.to_string(), node_key);
    }


    pub fn add_effectors(&mut self, pairs: Vec<(u64, String)>) {
        for (k, s) in pairs.iter() {
            self.add_effector(s, *k);
        }
    }

    pub fn send_signals(&self, network: &mut Network) {
        //self.signals = HashMap::new();
        let mut input_values: Vec<(u64, f32)> = vec![];
        for (k, v) in self.signals.iter() {
            input_values.push((*k, *v));
        }
        network.input(input_values);
    }

    pub fn recv_actions(&mut self, network: &Network) {
        self.actions = HashMap::new();
        for (k, v) in self.effectors.iter() {
            self.actions.insert(k.to_owned(), network.get_node_value(v).unwrap());
        }
    }

    pub fn set_signal(&mut self, name: &str, value: f32) {
        let node_key = self.sensors.get(name).unwrap();
        self.signals.insert(*node_key, value);
    }

    pub fn get_action(&self, name: &str) -> f32 {
        return *self.actions.get(name).unwrap();
    }

    pub fn get_signal_list(&self) -> Vec<(String, f32)> {
        let mut signal_list: Vec<(String, f32)> = vec![];
        for (s, k) in self.sensors.iter() {
            let v = self.signals.get(k).unwrap();
            signal_list.push((s.to_owned(), *v));
        }
        return signal_list;
    }

    pub fn get_action_list(&self) -> Vec<(String, f32)> {
        let mut action_list: Vec<(String, f32)> = vec![];
        for (s, v) in self.actions.iter() {
            action_list.push((s.to_owned(), *v));
        }
        return action_list;
    }

}

pub struct Agent {
    pub key: u64,
    pub pos: Vec2,
    pub rot: f32,
    pub mass: f32,
    vel: f32,
    ang_vel: f32,
    pub size: f32,
    pub vision_range: f32,
    pub max_eng: f32,
    pub eng: f32,
    color: color::Color,
    pub shape: SharedShape,
    analize_timer: Timer,
    pub network: Network,
    pub alife: bool,
    pub lifetime: f32,
    pub generation: u32,
    pub contacts: Vec<(RigidBodyHandle, u64, f32)>,
    pub detected: Option<Detected>,
    pub enemy: Option<RigidBodyHandle>,
    pub enemy_position: Option<Vec2>,
    pub enemy_dir: Option<f32>,
    pub resource: Option<RigidBodyHandle>,
    pub resource_position: Option<Vec2>,
    pub resource_dir: Option<f32>,
    pub physics_handle: RigidBodyHandle,
    //pub neuro_table: NeuroTable,
    pub neuro_map: NeuroMap,
    pub childs: usize,
    pub specie: String,
    pub attacking: bool,
    pub points: f32,
    //pub hit_list: HitList,
}



impl Agent {
    
    pub fn new(physics: &mut PhysicsWorld) -> Self {
        let settings = get_settings();
        let key = gen_range(u64::MIN, u64::MAX);
        let size = rand::gen_range(settings.agent_size_min, settings.agent_size_max) as f32;
        let pos = random_position(settings.world_w as f32, settings.world_h as f32);
        let shape = SharedShape::ball(size);
        let rbh = physics.add_dynamic(key, &pos, 0.0, shape.clone(), PhysicsProperities::default(), InteractionGroups { memberships: Group::GROUP_1, filter: Group::GROUP_2 | Group::GROUP_1 });
        let color = random_color();
        let mut network = Network::new(1.0);
        let inp_labs = vec!["CON", "ENG", "TGL", "TGR", "DST", "REL", "RER", "RED"];
        let out_labs = vec!["MOV", "LFT", "RGT", "ATK"];
        network.build(inp_labs.len(), inp_labs, 5, out_labs.len(), out_labs, settings.neurolink_rate);
        let input_pairs = network.get_input_pairs();
        let output_pairs = network.get_output_pairs();
        let mut neuro_map = NeuroMap::new();
        neuro_map.add_sensors(input_pairs);
        neuro_map.add_effectors(output_pairs);
        Self {
            key: gen_range(u64::MIN, u64::MAX),
            pos,
            rot: random_rotation(),
            mass: 0.0,
            vel: 0.0,
            ang_vel: 0.0,
            size,
            vision_range: (rand::gen_range(0.5, 1.5) * settings.agent_vision_range).round(),
            max_eng: size.powi(2) * 10.0,
            eng: size.powi(2) * 10.0,
            color,
            shape,
            analize_timer: Timer::new(1.0, true, true, true),
            network,
            alife: true,
            lifetime: 0.0,
            generation: 0,
            detected: None,
            enemy: None,
            enemy_position: None,
            enemy_dir: None,
            resource: None,
            resource_position: None,
            resource_dir: None,
            contacts: Vec::new(),
            physics_handle: rbh,
            //neuro_table: NeuroTable { inputs: vec![], outputs: vec![] },
            neuro_map,
            childs: 0,
            specie: create_name(4),
            attacking: false,
            points: 0.0,
        }
    }

    pub fn from_sketch(sketch: AgentSketch, physics: &mut PhysicsWorld) -> Agent {
        let key = gen_range(u64::MIN, u64::MAX);
        let settings = get_settings();
        let pos = random_position(settings.world_w as f32, settings.world_h as f32);
        let color = Color::new(sketch.color[0], sketch.color[1], sketch.color[2], sketch.color[3]);
        let shape = match sketch.shape {
            MyShapeType::Ball => {
                SharedShape::ball(sketch.size)
            },
            MyShapeType::Cuboid => {
                SharedShape::cuboid(sketch.size, sketch.size)
            },
            _ => {
                SharedShape::ball(sketch.size)
            },
        };
        let gen = sketch.generation + 1;
        let mut network = sketch.network.from_sketch();
        network.mutate(settings.mutations);
        let input_pairs = network.get_input_pairs();
        let output_pairs = network.get_output_pairs();
        let mut neuro_map = NeuroMap::new();
        neuro_map.add_sensors(input_pairs);
        neuro_map.add_effectors(output_pairs);
        let rbh = physics.add_dynamic(key, &pos, 0.0, shape.clone(), PhysicsProperities::default(), InteractionGroups { memberships: Group::GROUP_1, filter: Group::GROUP_2 | Group::GROUP_1 });
        Agent {
            key,
            pos,
            rot: random_rotation(),
            mass: 0.0,
            vel: 0.0,
            ang_vel: 0.0,
            size: sketch.size,
            vision_range: sketch.vision_range,
            max_eng: sketch.size.powi(2) * 10.0,
            eng: sketch.size.powi(2) * 10.0,
            color,
            shape,
            analize_timer: Timer::new(1.0, true, true, true),
            network,
            alife: true,
            lifetime: 0.0,
            generation: gen,
            detected: None,
            enemy: None,
            enemy_position: None,
            enemy_dir: None,
            resource: None,
            resource_position: None,
            resource_dir: None,
            contacts: Vec::new(),
            physics_handle: rbh,
            //neuro_table: NeuroTable { inputs: vec![], outputs: vec![] },
            neuro_map,
            childs: 0,
            specie: create_name(4),
            attacking: false,
            points: 0.0,
        }
    }

    pub fn draw(&self, selected: bool, font: &Font) {
        let settings = get_settings();
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        if settings.agent_eng_bar {
            let e = self.eng/self.max_eng;
            self.draw_status_bar(e, SKYBLUE, ORANGE, Vec2::new(0.0, self.size*1.5+4.0));
        }
        draw_circle(x0, y0, self.size, self.color);
        self.draw_front();
        if selected {
            self.draw_info(&font);
            self.draw_target();
        } else if settings.show_specie {
            self.draw_info(&font);
        }
    }    

    pub fn update(&mut self, dt: f32, physics: &mut PhysicsWorld) -> bool {
        self.lifetime += dt;
        if self.analize_timer.update(dt) {
            self.watch(physics);
            self.update_contacts(physics);
            self.analize();
        }

        self.update_physics(physics);
        //self.calc_timers(dt);
        //self.network.update();
        self.calc_energy(dt);
        return self.alife;
    }

    pub fn attack(&self) -> Vec<RigidBodyHandle> {
        let dt = get_frame_time();
        let mut hits: Vec<RigidBodyHandle> = vec![];
        if !self.attacking { return hits; }
        for (rbh, id, ang) in self.contacts.to_vec() {
            if ang <= PI/4.0 && ang >= -PI/4.0 {
                hits.push(rbh);
            }
        }
        return hits;
    }

    fn prep_input(&mut self) {
        let mut contact: f32;
        if self.contacts.len() > 0 {
            contact = 1.0; 
        } else {
            contact = 0.0;
        }
        //let mut contact = clamp(self.contacts.len(), 0, 1) as f32;
        
        let tg_dist = match self.enemy_position {
            None => 0.0,
            Some(pos2) => {
                let dist = pos2.distance(self.pos);
                1.0-(dist/self.vision_range)
            },
        };
        let mut tg_ang = match self.enemy_dir {
            None => PI,
            Some(dir) => {
                dir
            },
        };
        tg_ang = tg_ang/PI;
        let mut tgr: f32=0.0; let mut tgl: f32=0.0;
        if tg_ang > 0.0 {
            tgr = 1.0 - clamp(tg_ang, 0.0, 1.0);
        } else if tg_ang < 0.0 {
            tgl = 1.0-clamp(tg_ang, -1.0, 0.0).abs();
        }
        
        let res_dist = match self.enemy_position {
            None => 0.0,
            Some(pos2) => {
                let dist = pos2.distance(self.pos);
                1.0-(dist/self.vision_range)
            },
        };
        let mut res_ang = match self.enemy_dir {
            None => PI,
            Some(dir) => {
                dir
            },
        };
        res_ang = res_ang/PI;
        let mut resr: f32=0.0; let mut resl: f32=0.0;
        if res_ang > 0.0 {
            resr = 1.0 - clamp(res_ang, 0.0, 1.0);
        } else if res_ang < 0.0 {
            resl = 1.0-clamp(res_ang, -1.0, 0.0).abs(); 
        }
        
        let hp = self.eng/self.max_eng;
        //let val: Vec<Option<f32>> = vec![contact, hp, tgl, tgr, tg_dist, resl, resr, res_dist];
        //vec!["CON", "ENG", "TGL", "TGR", "DST", "REL", "RER", "RED"];
        let input_values: [f32; 8] = [contact, hp, tgl, tgr, tg_dist, resl, resr, res_dist];
        self.neuro_map.set_signal("CON", contact);
        self.neuro_map.set_signal("ENG", hp);
        self.neuro_map.set_signal("TGL", tgl);
        self.neuro_map.set_signal("TGR", tgr);
        self.neuro_map.set_signal("DST", tg_dist);
        self.neuro_map.set_signal("REL", resl);
        self.neuro_map.set_signal("RER", resr);
        self.neuro_map.set_signal("RED", res_dist);
    }

    fn analize(&mut self) {

        self.network.deactivate_nodes();
        self.prep_input();
        self.neuro_map.send_signals(&mut self.network);
        self.network.calc();
        self.neuro_map.recv_actions(&self.network);

        //vec!["MOV", "LFT", "RGT", "ATK"];
        if self.neuro_map.get_action("MOV") > 0.0 {
            self.vel = self.neuro_map.get_action("MOV");
        } else {
            self.vel = 0.0;
        }
        
        self.ang_vel = -self.neuro_map.get_action("LFT")+self.neuro_map.get_action("RGT");
        if self.neuro_map.get_action("ATK") >= 0.5 {
            self.attacking = true;
        } else {
            self.attacking = false;
        }
    }

/*     fn analize2(&mut self) {

        self.network.deactivate_nodes();
        let iii = self.prep_input();
        self.network.input(iii.clone());
        self.network.calc();
        let mut outputs2 = self.network.get_outputs2();
        self.neuro_table.inputs = iii.clone();
        //self.neuro_table.outputs = outputs2.clone();
        if *outputs2.get("MOV").unwrap() >= 0.0 {
            self.vel = *outputs2.get("MOV").unwrap();
        } else {
            self.vel = 0.0;
        }
        self.ang_vel = -*outputs2.get("LFT").unwrap()+*outputs2.get("RGT").unwrap();
        if *outputs2.get("ATK").unwrap() >= 0.5 {
            self.attacking = true;
        } else {
            self.attacking = false;
        }
    } */

    fn draw_front(&self) {
        let dir = Vec2::from_angle(self.rot);
        let left = Vec2::from_angle(self.rot-PI/10.0);
        let right = Vec2::from_angle(self.rot+PI/10.0);
        let l0 = self.pos + left*self.size;
        let r0 = self.pos + right*self.size;
        let l1 = self.pos + left*self.size*1.7;
        let r1 = self.pos + right*self.size*1.7;
        let mut yaw_color = LIGHTGRAY;
        if self.attacking {
            yaw_color = RED;
        }
        draw_line(l0.x, l0.y, l1.x, l1.y, self.size/3.0, yaw_color);
        draw_line(r0.x, r0.y, r1.x, r1.y, self.size/3.0, yaw_color);
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
        if let Some(_rb) = self.resource {
            if let Some(resource_position) = self.resource_position {
                let v0l = Vec2::from_angle(self.rot - PI / 2.0) * self.size;
                let v0r = Vec2::from_angle(self.rot + PI / 2.0) * self.size;
                let x0l = self.pos.x + v0l.x;
                let y0l = self.pos.y + v0l.y;
                let x0r = self.pos.x + v0r.x;
                let y0r = self.pos.y + v0r.y;
                let x1 = resource_position.x;
                let y1 = resource_position.y;
                draw_line(x0l, y0l, x1, y1, 1.0, self.color);
                draw_line(x0r, y0r, x1, y1, 1.0, self.color);
            }
        }
    }

    fn draw_info(&self, font: &Font) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let text_cfg = TextParams {
            font: *font,
            font_size: 10,
            color: LIGHTGRAY,
            ..Default::default()
        };
        let rot = self.rot;
        //let mass = self.mass;
        let info = format!("{} [{}]", self.specie.to_uppercase(), self.generation);
        //let info = format!("rot: {}", (rot * 10.0).round() / 10.0);
        //let info_mass = format!("mass: {}", mass.round());
        let txt_center = get_text_center(&info, Some(*font), 10, 1.0, 0.0);
        draw_text_ex(&info, x0 - txt_center.x, y0 - txt_center.y + self.size * 2.0 + 8.0, text_cfg.clone());
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
        let settings = get_settings();
        self.update_enemy_position(physics);
        let physics_data = physics.get_physics_data(self.physics_handle);
        self.pos = physics_data.position;
        self.rot = physics_data.rotation;
        self.mass = physics_data.mass;
        match physics.rigid_bodies.get_mut(self.physics_handle) {
            Some(body) => {
                let dir = Vec2::from_angle(self.rot);
                let v = dir * self.vel * settings.agent_speed;
                let rot = self.ang_vel * settings.agent_rotate;
                body.set_linvel(Vector2::new(v.x, v.y), true);
                body.set_angvel(rot, true);
                self.check_edges(body);
            }
            None => {}
        }
    }

    fn check_edges(&mut self, body: &mut RigidBody) {
        let settings = get_settings();
        let mut raw_pos = matrix_to_vec2(body.position().translation);
        let mut out_of_edge = false;
        if raw_pos.x < -5.0 {
            raw_pos.x = 0.0;
            out_of_edge = true;
        } else if raw_pos.x > settings.world_w as f32 + 5.0 {
            raw_pos.x = settings.world_w as f32;
            out_of_edge = true;
        }
        if raw_pos.y < -5.0 {
            raw_pos.y = 0.0;
            out_of_edge = true;
        } else if raw_pos.y > settings.world_h as f32 + 5.0 {
            raw_pos.y = settings.world_h as f32;
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
        if let Some(rb) = self.resource {
            if let Some(resource_position) = physics.get_object_position(rb) {
                self.resource_position = Some(resource_position);
                let rel_pos = resource_position - self.pos;
                let resource_dir = rel_pos.angle_between(Vec2::from_angle(self.rot));
                self.resource_dir = Some(resource_dir);
            } else {
                self.resource = None;
                self.resource_position = None;
                self.resource_dir = None;
            }
        } else if self.resource_position.is_some() {
            self.resource_position = None;
            self.resource_dir = None;
        }
    }

    fn update_contacts(&mut self, physics: &mut PhysicsWorld) {
        self.contacts.clear();
        let contacts = physics.get_contacts_set(self.physics_handle, self.size);
        for contact in contacts {
            if let Some(pos2) = physics.get_object_position(contact) {
                let mut rel_pos = pos2 - self.pos;
                rel_pos = rel_pos.normalize_or_zero();
                let target_angle = rel_pos.angle_between(Vec2::from_angle(self.rot));
                match physics.get_key_for_body(&contact) {
                    Some(key) => {
                        self.contacts.push((contact, key, target_angle));
                    },
                    None => {},
                }
            }

        }
    }

    fn watch(&mut self, physics: &PhysicsWorld) {
        if let Some(tg) = physics.get_closesd_agent(self.physics_handle, self.vision_range) {
            self.enemy = Some(tg);
        } else {
            self.enemy = None;
            self.enemy_position = None;
            self.enemy_dir = None;
        }
        if let Some(tg) = physics.get_closesd_resource(self.physics_handle, self.vision_range) {
            self.resource = Some(tg);
            //self.update_enemy_position(physics);
        } else {
            self.resource = None;
            self.resource_position = None;
            self.resource_dir = None;
        }
        self.update_enemy_position(physics);
    }

/*     fn calc_timers(&mut self, _dt: f32) {

    } */

    fn calc_energy(&mut self, dt: f32) {
        let settings = get_settings();
        let base_cost = settings.base_energy_cost;
        let move_cost = settings.move_energy_cost;
        let attack_cost = settings.attack_energy_cost;
        let basic_loss = self.size * base_cost;
        let move_loss = self.vel * self.size * move_cost;
        let attack_loss = match self.attacking {
            true => attack_cost * self.size,
            false => 0.0,
        };
        let loss = (basic_loss + move_loss + attack_loss) * dt;
        if self.eng > 0.0 {
            self.eng -= loss;
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

    pub fn replicate(&self, physics: &mut PhysicsWorld) -> Self {
        let settings = get_settings();
        let key = gen_range(u64::MIN, u64::MAX);
        let mut size = self.size;
        if rand::gen_range(0, 9) == 0 {
            size += rand::gen_range(-1, 1) as f32;
        }
        size = clamp(size, settings.agent_size_min as f32, settings.agent_size_max as f32);
        let color = self.color.to_owned();
        let shape = SharedShape::ball(size);
        let rot = random_rotation();
        let pos = random_position(settings.world_w as f32, settings.world_h as f32);
        let rbh = physics.add_dynamic(key, &pos, rot, shape.clone(), PhysicsProperities::default(), InteractionGroups::new(Group::GROUP_1, Group::GROUP_2 | Group::GROUP_1 ));
        let network = self.network.replicate();
        let input_pairs = network.get_input_pairs();
        let output_pairs = network.get_output_pairs();
        let mut neuro_map = NeuroMap::new();
        neuro_map.add_sensors(input_pairs);
        neuro_map.add_effectors(output_pairs);
        Self {
            key,
            pos,
            rot,
            mass: 0.0,
            vel: 0.0,
            ang_vel: 0.0,
            size,
            vision_range: self.vision_range,
            max_eng: self.max_eng,
            eng: self.max_eng,
            color,
            shape,
            analize_timer: self.analize_timer.to_owned(),
            network,
            alife: true,
            lifetime: 0.0,
            generation: self.generation + 1,
            detected: None,
            enemy: None,
            enemy_position: None,
            enemy_dir: None,
            resource: None,
            resource_position: None,
            resource_dir: None,
            contacts: Vec::new(),
            physics_handle: rbh,
            neuro_map,
            childs: 0,
            specie: self.specie.to_owned(),
            attacking: false,
            points: 0.0,
        }
    }

    pub fn get_sketch(&self) -> AgentSketch {
        AgentSketch { 
            specie: self.specie.to_owned(),
            generation: self.generation, 
            size: self.size, 
            shape: match self.shape.shape_type() {
                ShapeType::Ball => MyShapeType::Ball,
                _ => MyShapeType::Cuboid,
            },
            color: self.color.to_vec().to_array(), 
            vision_range: self.vision_range, 
            network: self.network.get_sketch(),
            points: self.points, 
        }
    }
}


pub struct Detected {
    pub target_handle: RigidBodyHandle,
    pub dist: f32,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MyShapeType {
    Ball,
    Cuboid,
    Segment,
}



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentSketch {
    pub specie: String,
    pub generation: u32,
    pub size: f32,
    pub shape: MyShapeType,
    pub color: [f32; 4],
    pub vision_range: f32,
    pub network: NetworkSketch,
    pub points: f32,

}

