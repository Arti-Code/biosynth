#![allow(unused)]

use std::collections::HashMap;
use std::f32::consts::PI;
use crate::neuro::*;
use crate::timer::*;
use crate::util::*;
use crate::physics::*;
use crate::globals::*;
use crate::part::*;
use macroquad::{color, prelude::*};
use macroquad::rand::*;
use rapier2d::geometry::*;
use rapier2d::na::Vector2;
use rapier2d::prelude::{RigidBody, RigidBodyHandle};
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use crate::agent::*;




#[derive(Clone)]
pub struct Agent2 {
    pub pos: Vec2,
    pub rot: f32,
    pub mass: f32,
    vel: f32,
    ang_vel: f32,
    pub size: f32,
    pub max_eng: f32,
    pub eng: f32,
    color: color::Color,
    shape: SharedShape,
    analize_timer: Timer,
    pub network: Network,
    pub alife: bool,
    pub lifetime: f32,
    pub generation: u32,
    pub physics_handle: RigidBodyHandle,
    pub neuro_map: NeuroMap,
    pub kills: usize,
    pub specie: String,
    pub attacking: bool,
    pub points: f32,
    pub pain: bool,
    pub power: i32,
    pub speed: i32,
    pub shell: i32,
    //parts: Vec<Box<dyn AgentPart>>,
}



impl Agent2 {
    
    pub fn new(physics: &mut Physics) -> Self {
        let settings = get_settings();
        let key = gen_range(u64::MIN, u64::MAX);
        let size = rand::gen_range(settings.agent_size_min, settings.agent_size_max) as f32;
        let pos = random_position(settings.world_w as f32, settings.world_h as f32);
        let shape = SharedShape::ball(size);
        let rbh = physics.add_dynamic_object(&pos, 0.0, shape.clone(), PhysicsMaterial::default(), InteractionGroups { memberships: Group::GROUP_1, filter: Group::GROUP_2 | Group::GROUP_1 });
        let color = random_color();
        let mut network = Network::new(1.0);
        let inp_labs = vec!["CON", "ENG", "TGL", "TGR", "DST", "DNG", "FAM", "REL", "RER", "RED", "PAI"];
        let out_labs = vec!["MOV", "LFT", "RGT", "ATK", "RUN"];
        network.build(inp_labs.len(), inp_labs, settings.hidden_nodes_num, out_labs.len(), out_labs, settings.neurolink_rate);
        let input_pairs = network.get_input_pairs();
        let output_pairs = network.get_output_pairs();
        let mut neuro_map = NeuroMap::new();
        neuro_map.add_sensors(input_pairs);
        neuro_map.add_effectors(output_pairs);
        let mut parts: Vec<Box<dyn AgentPart>> = vec![];
        let tail = Tail::new(Vec2::from_angle(PI)*size, size*0.7, color);
        let tail = Box::new(tail);
        //parts.push(tail);
        Self {
            pos,
            rot: random_rotation(),
            mass: 0.0,
            vel: 0.0,
            ang_vel: 0.0,
            size,
            max_eng: size.powi(2) * 10.0 + 200.0,
            eng: size.powi(2) * 10.0 + 200.0,
            color,
            shape,
            analize_timer: Timer::new(settings.neuro_duration, true, true, true),
            network,
            alife: true,
            lifetime: 0.0,
            generation: 0,
            physics_handle: rbh,
            neuro_map,
            kills: 0,
            specie: create_name(4),
            attacking: false,
            points: 0.0,
            pain: false,
            speed: gen_range(0, 10),
            power: gen_range(0, 10),
            shell: gen_range(0, 10),
        }
    }

    pub fn from_sketch(sketch: AgentSketch2, physics: &mut Physics) -> Agent2 {
        let key = gen_range(u64::MIN, u64::MAX);
        let settings = get_settings();
        let pos = random_position(settings.world_w as f32, settings.world_h as f32);
        let color = Color::new(sketch.color[0], sketch.color[1], sketch.color[2], sketch.color[3]);
        let mut size = Self::mutate_one(sketch.size as i32) as f32;
        let shape = match sketch.shape {
            MyShapeType::Ball => {
                SharedShape::ball(size)
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
        let mut parts: Vec<Box<dyn AgentPart>> = vec![];
        let tail = Tail::new(Vec2::from_angle(PI)*size, size*0.7, color);
        let tail = Box::new(tail);
        //parts.push(tail);
        let rbh = physics.add_dynamic_object(&pos, 0.0, shape.clone(), PhysicsMaterial::default(), InteractionGroups { memberships: Group::GROUP_1, filter: Group::GROUP_2 | Group::GROUP_1 });
        Agent2 {
            pos,
            rot: random_rotation(),
            mass: 0.0,
            vel: 0.0,
            ang_vel: 0.0,
            size,
            max_eng: sketch.size.powi(2) * 10.0 +200.0,
            eng: sketch.size.powi(2) * 10.0 + 200.0,
            color,
            shape,
            analize_timer: Timer::new(settings.neuro_duration, true, true, true),
            network,
            alife: true,
            lifetime: 0.0,
            generation: gen,
            physics_handle: rbh,
            neuro_map: sketch.neuro_map.clone(),
            kills: 0,
            specie: sketch.specie.to_owned(),
            attacking: false,
            points: 0.0,
            pain: false,
            power: Self::mutate_one(sketch.power),
            speed: Self::mutate_one(sketch.speed),
            shell: Self::mutate_one(sketch.shell),
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
        self.draw_body();
        self.draw_front();
        self.draw_eyes();
        if selected {
            self.draw_info(&font);
            self.draw_target(selected);
        } else if settings.show_specie {
            self.draw_info(&font);
        }
    }    

    fn draw_body(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let rv = Vec2::from_angle(self.rot+PI);
        let x1 = x0+rv.x*self.size*0.8;
        let y1 = y0+rv.y*self.size*0.8;
        let shell = self.size + (self.shell as f32)*0.4;
        draw_circle(x0, y0, shell, WHITE);
        draw_circle(x1, y1, shell*0.6, WHITE);
        draw_circle(x1, y1, self.size*0.6, self.color);
        draw_circle(x0, y0, self.size, self.color);
    }

    pub fn update(&mut self, physics: &mut Physics) -> bool {
        let dt = get_frame_time();
        self.lifetime += dt;
        if self.analize_timer.update(dt) {
            self.analize();
        }

        self.update_physics(physics);
        //self.calc_timers(dt);
        //self.network.update();
        self.calc_energy(dt);
        return self.alife;
    }

    pub fn eat(&self) -> Vec<RigidBodyHandle> {
        let mut hits: Vec<RigidBodyHandle> = vec![];
/*         for (rbh, ang) in self.contacts.to_vec() {
            if ang <= PI/4.0 && ang >= -PI/4.0 {
                hits.push(rbh);
            }
        } */
        return hits;
    }

    pub fn attack(&self) -> Vec<RigidBodyHandle> {
        //let dt = get_frame_time();
        let mut hits: Vec<RigidBodyHandle> = vec![];
        if !self.attacking { return hits; }
/*         for (rbh, ang) in self.contacts.to_vec() {
            if ang <= PI/4.0 && ang >= -PI/4.0 {
                hits.push(rbh);
            }
        } */
        return hits;
    }

    fn prep_input(&mut self) {
        let hp = self.eng/self.max_eng;
        //let val: Vec<Option<f32>> = vec![contact, hp, tgl, tgr, tg_dist, resl, resr, res_dist];
        //vec!["CON", "ENG", "TGL", "TGR", "DST", "REL", "RER", "RED", "PAI"];
        //let input_values: [f32; 8] = [contact, hp, tgl, tgr, tg_dist, resl, resr, res_dist];
        let mut pain = 0.0;
        if self.pain { pain = 1.0; }
        self.pain = false;
        self.pain = false;
        self.neuro_map.set_signal("ENG", hp);
        self.neuro_map.set_signal("PAI", pain);
    }

    fn analize(&mut self) {

        self.network.deactivate_nodes();
        self.prep_input();
        self.neuro_map.send_signals(&mut self.network);
        self.network.calc();
        self.neuro_map.recv_actions(&self.network);

        //vec!["MOV", "LFT", "RGT", "ATK", "RUN"];
        if self.neuro_map.get_action("MOV") > 0.0 {
            self.vel = self.neuro_map.get_action("MOV");
        } else {
            self.vel = 0.0;
        }
    }

    fn draw_front(&self) {
        //let dir = Vec2::from_angle(self.rot);
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

/*     fn draw_circle(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        draw_circle_lines(x0, y0, self.size, 4.0, self.color);
        //self.draw_front();
    } */

    fn draw_eyes(&self) {
        let mut color = SKYBLUE;
        if self.attacking { color = RED; }
        let eye_l = Vec2::from_angle(self.rot - PI / 3.0) * self.size*0.66;
        let eye_r = Vec2::from_angle(self.rot + PI / 3.0) * self.size*0.66;
        let xl = self.pos.x + eye_l.x;
        let yl = self.pos.y + eye_l.y;
        let xr = self.pos.x + eye_r.x;
        let yr = self.pos.y + eye_r.y;
        let s = self.size*0.33;
        draw_circle(xl, yl, s, color);
        draw_circle(xr, yr, s, color);
    }

    fn draw_target(&self, selected: bool) {

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
        let info = format!("{} [{}]", self.specie.to_uppercase(), self.generation);
        let txt_center = get_text_center(&info, Some(*font), 10, 1.0, 0.0);
        draw_text_ex(&info, x0 - txt_center.x, y0 - txt_center.y + self.size * 2.0 + 8.0, text_cfg.clone());
    }

    fn draw_status_bar(&self, percent: f32, color1: Color, color2: Color, offset: Vec2) {
        let xc = self.pos.x + offset.x; let yc = self.pos.y + offset.y;
        let x0 = xc-20.0; let y0 = yc -1.5;
        let w = 40.0*percent;
        draw_rectangle(x0, y0, 40.0, 3.0, color2);
        draw_rectangle(x0, y0, w, 3.0, color1);
    }

    fn update_physics(&mut self, physics: &mut Physics) {
        let settings = get_settings();
        let physics_data = physics.get_object_state(self.physics_handle);
        self.pos = physics_data.position;
        self.rot = physics_data.rotation;
        self.mass = physics_data.mass;
        match physics.get_object_mut(self.physics_handle) {
            Some(body) => {
                let dt = get_frame_time();
                let dir = Vec2::from_angle(self.rot);
                let rel_speed = ((self.speed as f32) - (self.shell as f32)/2.0);
                let mut v = dir * self.vel * self.speed as f32 * settings.agent_speed * dt * 10.0;
                let rot = self.ang_vel * settings.agent_rotate * dt *50.0;
                body.set_linvel(Vector2::new(v.x, v.y), true);
                body.set_angvel(rot, true);
                self.check_edges(body);
            }
            None => {}
        }
    }

    fn check_edges(&mut self, body: &mut RigidBody) {
        let settings = get_settings();
        let (mut raw_pos, rot ) = iso_to_vec2_rot(body.position());
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
            body.set_position(make_isometry(raw_pos.x, raw_pos.y, rot), true);
            //body.set_linvel([0.0, 0.0].into(), true);
            //self.vel = 0.0;
        }
    }

    fn calc_energy(&mut self, dt: f32) {
        let settings = get_settings();
        let base_cost = settings.base_energy_cost;
        let move_cost = settings.move_energy_cost;
        let attack_cost = settings.attack_energy_cost;
        let basic_loss = (self.power as f32 + self.shell as f32) * base_cost * (self.size*0.2);
        let mut move_loss = self.vel * (self.size + self.speed as f32) * move_cost;
        let attack_loss = match self.attacking {
            true => attack_cost * self.size,
            false => 0.0,
        };
        let loss = (basic_loss + move_loss + attack_loss) * dt;
        if self.eng > 0.0 {
            self.eng -= loss;
        } else {
            self.eng = 0.0;
            //self.alife = false;
        }
        self.check_alife();
    }

    fn check_alife(&mut self) {
        if self.eng > 0.0 {
            self.alife = true;
        } else {
            self.alife = false;
        }
    }

    pub fn is_death(&self) -> bool {
        return !self.alife;
    }

    pub fn add_energy(&mut self, e: f32) {
        self.eng += e;
        if self.eng > self.max_eng {
            self.eng = self.max_eng;
        }
        self.check_alife();
    }

    fn mutate_one(v: i32) -> i32 {
        let mut vm: i32 = v;
        let mut r = rand::gen_range(0, 20);
        if r == 1 {
            vm += 1;
            //println!("{} -> {}", v, vm);
        } else if r == 2 {
            vm -= 1;
            //println!("{} -> {}", v, vm);
        }
        vm = clamp(vm, 1_i32, 10_i32);
        return vm;
    }

    pub fn mutate(&mut self) {
        println!("Mutate");
        let settings = get_settings();
        let mut size = self.size;
        let mut r = rand::gen_range(0, 9);
        if r == 1 {
            println!("r: {}", r);
            size += rand::gen_range(-1, 1) as f32;
        }
        size = clamp(size, settings.agent_size_min as f32, settings.agent_size_max as f32);
        r = rand::gen_range(0, 9);
        let mut power = self.power;
        if r == 1 {
            println!("r: {}", r);
            power += rand::gen_range(-1, 1);
        }
        power = clamp(power, 0, 10);
        r = rand::gen_range(0, 9);
        let mut speed = self.speed;
        if r == 1 {
            println!("r: {}", r);
            speed += rand::gen_range(-1, 1);
        }
        speed = clamp(speed, 0, 10);
        r = rand::gen_range(0, 9);
        let mut shell = self.shell;
        if r == 1 {
            println!("r: {}", r);
            shell += rand::gen_range(-1, 1);
        }
        shell = clamp(shell, 0, 10);
        if self.size != size {
            println!("{} -> {}", self.size, size);
        }
        if self.power != power {
            println!("{} -> {}", self.power, power);
        }
        if self.speed != speed {
            println!("{} -> {}", self.speed, speed);
        }
        if self.shell != shell {
            println!("{} -> {}", self.shell, shell);
        }
        self.size = size;
        self.power = power;
        self.speed = speed;
        self.shell = shell;
    }

    pub fn replicate(&self, physics: &mut Physics) -> Agent2 {
        let settings = get_settings();
        let mut size = Self::mutate_one(self.size as i32) as f32;
        let mut power = Self::mutate_one(self.power);
        let mut speed = Self::mutate_one(self.speed);
        let mut shell = Self::mutate_one(self.shell);
        let color = self.color.to_owned();
        let shape = SharedShape::ball(size);
        let rot = random_rotation();
        let pos = random_position(settings.world_w as f32, settings.world_h as f32);
        let interactions = InteractionGroups::new(Group::GROUP_1, Group::GROUP_2 | Group::GROUP_1 );
        let rbh = physics.add_dynamic_object(&pos, rot, shape.clone(), PhysicsMaterial::default(), interactions);
        let network = self.network.replicate();
        let input_pairs = network.get_input_pairs();
        let output_pairs = network.get_output_pairs();
        let mut neuro_map = NeuroMap::new();
        neuro_map.add_sensors(input_pairs);
        neuro_map.add_effectors(output_pairs);
        let mut parts: Vec<Box<dyn AgentPart>> = vec![];
        let tail = Tail::new(Vec2::from_angle(PI)*size, size*0.7, color);
        let tail = Box::new(tail);
        //parts.push(tail);
        Agent2 {
            pos: pos + random_unit_vec2()*40.0,
            rot,
            mass: 0.0,
            vel: 0.0,
            ang_vel: 0.0,
            size,
            max_eng: self.max_eng,
            eng: self.max_eng,
            color,
            shape,
            analize_timer: self.analize_timer.to_owned(),
            network,
            alife: true,
            lifetime: 0.0,
            generation: self.generation + 1,
            physics_handle: rbh,
            neuro_map,
            specie: self.specie.to_owned(),
            attacking: false,
            points: 0.0,
            pain: false,
            kills: 0,
            power,
            speed,
            shell,
        }
    }

    pub fn get_sketch(&self) -> AgentSketch2 {
        AgentSketch2 { 
            specie: self.specie.to_owned(),
            generation: self.generation, 
            size: self.size, 
            shape: match self.shape.shape_type() {
                ShapeType::Ball => MyShapeType::Ball,
                _ => MyShapeType::Cuboid,
            },
            color: self.color.to_vec().to_array(), 
            network: self.network.get_sketch(),
            points: self.points, 
            neuro_map: self.neuro_map.clone(),
            power: self.power,
            speed: self.speed,
            shell: self.shell,
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentSketch2 {
    pub specie: String,
    pub generation: u32,
    pub size: f32,
    pub shape: MyShapeType,
    pub color: [f32; 4],
    pub network: NetworkSketch,
    pub points: f32,
    pub neuro_map: NeuroMap,
    pub power: i32,
    pub speed: i32,
    pub shell: i32,
}

