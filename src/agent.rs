//#![allow(unused)]


use std::collections::HashMap;
use std::f32::consts::PI;
use crate::neuro::*;
use crate::timer::*;
use crate::util::*;
use macroquad::prelude::*;
use macroquad::rand::*;
use rapier2d::geometry::*;
use rapier2d::na::Vector2;
use rapier2d::prelude::{RigidBody, RigidBodyHandle};
use crate::settings::*;
use crate::stats::*;
use crate::misc::*;
use crate::sketch::*;
use crate::phyx::physics::Physics;
use crate::phyx::physics_misc::PhysicsMaterial;

#[derive(Clone)]
pub struct Agent {
    pub key: u64,
    pub pos: Vec2,
    pub rot: f32,
    pub mass: f32,
    vel: f32,
    ang_vel: f32,
    pub size: f32,
    pub vision_range: f32,
    pub vision_angle: f32,
    pub max_eng: f32,
    pub eng: f32,
    color: Color,
    color_second: Color,
    pub shape: SharedShape,
    timer_analize: Timer,
    timer_contact: Timer,
    pub network: Network,
    pub alife: bool,
    pub lifetime: f32,
    pub generation: u32,
    pub contacts: Vec<(RigidBodyHandle, f32)>,
    pub contact_agent: bool,
    pub contact_resource: bool,
    pub enemy: Option<RigidBodyHandle>,
    pub enemy_family: Option<bool>,
    pub enemy_position: Option<Vec2>,
    pub enemy_dir: Option<f32>,
    pub enemy_size: Option<f32>,
    enemy_mood: Option<Color>,
    pub resource: Option<RigidBodyHandle>,
    pub resource_position: Option<Vec2>,
    pub resource_dir: Option<f32>,
    pub physics_handle: RigidBodyHandle,
    pub neuro_map: NeuroMap,
    pub childs: usize,
    pub kills: usize,
    pub specie: String,
    pub attacking: bool,
    pub eating: bool,
    pub points: f32,
    pub pain: bool,
    pub run: bool,
    pub power: i32,
    pub speed: i32,
    pub shell: i32,
    pub mutations: i32,
    pub eyes: i32,
    pub mood: Color,
    ancestors: Ancestors,
    pub eng_cost: EnergyCost,
    blocked: f32,
    attack_visual: bool,
    eat_visual: bool,
}



impl Agent {
    
    pub fn new(physics: &mut Physics) -> Self {
        let settings = get_settings();
        let size = rand::gen_range(settings.agent_size_min, settings.agent_size_max) as f32;
        let rot = random_rotation();
        let eyes = gen_range(0, 10);
        let pos = random_position(settings.world_w as f32, settings.world_h as f32);
        let shape = SharedShape::ball(size);
        let rbh = physics.add_dynamic_object(&pos, rot, shape.clone(), PhysicsMaterial::default(), InteractionGroups { memberships: Group::GROUP_1, filter: Group::GROUP_2 | Group::GROUP_1 });
        let color = random_color();
        let color_second = random_color();
        let mut network = Network::new(1.0);
        let inp_labs = vec!["CON", "ENY", "RES", "ENG", "TGL", "TGR", "DST", "DNG", "FAM", "REL", "RER", "RED", "PAI", "WAL", "RED", "GRE", "BLU", "WAL", "E-R", "E-G", "E-B"];
        let out_labs = vec!["MOV", "LFT", "RGT", "ATK", "EAT", "RUN", "RED", "GRE", "BLU"];
        let hid = settings.hidden_nodes_num;
        let hid1 = rand::gen_range(1, hid);
        let hid2 = rand::gen_range(1, hid);
        network.build(inp_labs.len(), inp_labs, vec![hid1, hid2], out_labs.len(), out_labs, settings.neurolink_rate);
        let input_pairs = network.get_input_pairs();
        let output_pairs = network.get_output_pairs();
        let mut neuro_map = NeuroMap::new();
        neuro_map.add_sensors(input_pairs);
        neuro_map.add_effectors(output_pairs);

        let mut agent = Agent {
            key: gen_range(u64::MIN, u64::MAX),
            pos,
            rot,
            mass: 0.0,
            vel: 0.0,
            ang_vel: 0.0,
            size,
            vision_range:  Self::calc_vision_range(eyes),
            vision_angle: Self::calc_vision_angle(eyes),
            max_eng: 0.0,
            eng: 0.0,
            color,
            color_second,
            shape,
            timer_analize: Timer::new(settings.neuro_duration, true, true, true),
            timer_contact: Timer::new(0.07, true, true, true),
            network,
            alife: true,
            lifetime: 0.0,
            generation: 0,
            enemy: None,
            enemy_family: None,
            enemy_position: None,
            enemy_mood: None,
            enemy_dir: None,
            enemy_size: None,
            resource: None,
            resource_position: None,
            resource_dir: None,
            contacts: Vec::new(),
            contact_agent: false,
            contact_resource: false,
            physics_handle: rbh,
            neuro_map,
            childs: 0,
            kills: 0,
            specie: create_name(4),
            attacking: false,
            eating: false,
            points: 0.0,
            pain: false,
            run: false,
            speed: gen_range(0, 10),
            power: gen_range(0, 10),
            shell: gen_range(0, 10),
            mutations: gen_range(0, 10),
            eyes,
            mood: Color::new(0.0, 0.0, 0.0, 1.0),
            ancestors: Ancestors::new(),
            eng_cost: EnergyCost::default(),
            blocked: 0.0,
            attack_visual: false,
            eat_visual: false,
        };
        agent.ancestors.add_ancestor(Ancestor::new(&agent.specie, agent.generation as i32, 0));
        agent.calc_hp();
        return agent;
    }

    pub fn calc_vision_range(eyes: i32) -> f32 {
        let settings = get_settings();
        return 80.0 + settings.agent_vision_range*(eyes as f32)/10.0;
    }

    pub fn calc_vision_angle(eyes: i32) -> f32 {
        return 1.8*PI * ((11.0 - eyes as f32)/11.0);
    }

    pub fn get_mood(&self) -> Color {
        return self.mood.to_owned();
    }

    fn mod_specie(&mut self) {
        let settings = get_settings();
        if rand::gen_range(0, settings.rare_specie_mod) == 0 {
            let s = create_name(1);
            let i = rand::gen_range(0, 3)*2;
            let mut name = self.specie.to_owned();
            name.replace_range(i..=i+1, &s);
            self.specie = name.to_owned();
            self.ancestors.add_ancestor(Ancestor::new(&self.specie, self.generation as i32, 0));
            if random_unit_unsigned() < 0.25 {
                self.color_second = random_color();
            } else if random_unit_unsigned() < 0.25 {
                self.color = random_color();
            }
        }

    }

    pub fn ancestors(&self) -> Vec<Ancestor> {
        return self.ancestors.get_ancestors();
    }

    pub fn from_sketch(sketch: AgentSketch, physics: &mut Physics) -> Agent {
        let key = gen_range(u64::MIN, u64::MAX);
        let settings = get_settings();
        let pos = random_position(settings.world_w as f32, settings.world_h as f32);
        let color = Color::new(sketch.color[0], sketch.color[1], sketch.color[2], sketch.color[3]);
        let color_second = Color::new(sketch.color_second[0], sketch.color_second[1], sketch.color_second[2], sketch.color_second[3]);
        let size = sketch.size;
        let eyes = sketch.eyes;
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
        let network = sketch.network.from_sketch();
        //let parts: Vec<Box<dyn AgentPart>> = vec![];
        let rbh = physics.add_dynamic_object(&pos, 0.0, shape.clone(), PhysicsMaterial::default(), InteractionGroups { memberships: Group::GROUP_1, filter: Group::GROUP_2 | Group::GROUP_1 });
        let mut agent = Agent {
            key,
            pos,
            rot: random_rotation(),
            mass: 0.0,
            vel: 0.0,
            ang_vel: 0.0,
            size,
            vision_range: 0.0,
            vision_angle: 0.0,
            max_eng: 0.0,
            eng: 0.0,
            color,
            color_second,
            shape,
            timer_analize: Timer::new(settings.neuro_duration, true, true, true),
            timer_contact: Timer::new(0.07, true, true, true),
            network,
            alife: true,
            lifetime: 0.0,
            generation: gen,
            //detected: None,
            enemy: None,
            enemy_family: None,
            enemy_position: None,
            enemy_mood: None,
            enemy_dir: None,
            enemy_size: None,
            resource: None,
            resource_position: None,
            resource_dir: None,
            contacts: Vec::new(),
            contact_agent: false,
            contact_resource: false,
            physics_handle: rbh,
            neuro_map: sketch.neuro_map.clone(),
            childs: 0,
            kills: 0,
            specie: sketch.specie.to_owned(),
            attacking: false,
            eating: false,
            points: 0.0,
            pain: false,
            run: false,
            power: sketch.power,
            speed: sketch.speed,
            shell: sketch.shell,
            mutations: sketch.mutations,
            eyes,
            mood: Color::new(0.0, 0.0, 0.0, 1.0),
            ancestors: sketch.ancestors.to_owned(),
            eng_cost: EnergyCost::default(),
            blocked: 0.0,
            attack_visual: false,
            eat_visual: false,
        };
        agent.mod_specie();
        agent.mutate();
        agent.calc_hp();
        return agent;
    }

    pub fn draw(&self, selected: bool, font: &Font) {
        let settings = get_settings();
        if settings.agent_eng_bar {
            let e = self.eng/self.max_eng;
            self.draw_status_bar(e, SKYBLUE, ORANGE, Vec2::new(0.0, self.size*1.5+4.0));
        }
        self.draw_body();
        self.draw_front();
        self.draw_eyes(selected);
        if selected {
            self.draw_info(&font);
            self.draw_target(selected);
        } else {
            self.draw_info(&font);
        }
    }    

    fn draw_body(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let rv = Vec2::from_angle(self.rot+PI);
        let x1 = x0+rv.x*self.size;
        let y1 = y0+rv.y*self.size;
        let shell = self.size + (self.shell as f32)*0.4;
        draw_circle(x0, y0, shell, LIGHTGRAY);
        draw_circle(x1, y1, shell*0.6, LIGHTGRAY);
        draw_circle(x1, y1, self.size*0.6, self.color_second);
        draw_circle(x0, y0, self.size, self.color);
        draw_circle(x0, y0, self.size/2.0, self.mood);
        if self.run {
            let mut shadow = self.color;
            shadow.a = 0.6;
            let xs0 = x0 + rv.x * self.size * self.vel;
            let ys0 = y0 + rv.y * self.size * self.vel;
            let xs1 = x1 + rv.x * self.size * 1.3 * self.vel;
            let ys1 = y1 + rv.y * self.size * 1.3 * self.vel;
            let xs2 = x1 + rv.x * self.size * 1.7 * self.vel;
            let ys2 = y1 + rv.y * self.size * 1.7 * self.vel;
            draw_circle(xs2, ys2, self.size*0.5, shadow);
            draw_circle(xs1, ys1, self.size*0.8, shadow);
            draw_circle(xs0, ys0, self.size, shadow);
        }
    }

    pub fn update(&mut self, other: &HashMap<RigidBodyHandle, Agent>, physics: &mut Physics) -> bool {
        let dt = get_frame_time()*sim_speed();
        self.lifetime += dt;
        //if self.timer_contact.update(dt) {
            //}
        self.update_contacts(other, physics);
        if self.timer_analize.update(dt) {
            self.watch(physics);
            self.update_enemy_mood(other);
            self.analize();
            self.contacts_clear();
        }

        self.update_physics(physics);
        if self.pos.x.is_nan() || self.pos.y.is_nan() {
            self.alife = false;
            return self.alife;
        }
        self.calc_energy(dt);
        return self.alife;
    }

    fn update_enemy_mood(&mut self, other: &HashMap<RigidBodyHandle, Agent>) {
        match self.enemy {
            None => {
                self.enemy_mood = Some(Color::new(0.0, 0.0, 0.0, 1.0));
            },
            Some(rbh) => {
                match other.get(&rbh) {
                    None => {
                        self.enemy_mood = Some(Color::new(0.0, 0.0, 0.0, 1.0));
                    },
                    Some(agent) => {
                        self.enemy_mood = Some(agent.get_mood());
                    },
                }
            },
        }
    }


    pub fn eat(&self) -> Vec<RigidBodyHandle> {
        let mut hits: Vec<RigidBodyHandle> = vec![];
        for (rbh, ang) in self.contacts.to_vec() {
            if ang <= PI/4.0 && ang >= -PI/4.0 {
                hits.push(rbh);
            }
        }
        return hits;
    }

    pub fn attack(&self) -> Vec<RigidBodyHandle> {
        //let dt = get_frame_time()*sim_speed();
        let mut hits: Vec<RigidBodyHandle> = vec![];
        if !self.attacking { return hits; }
        for (rbh, ang) in self.contacts.to_vec() {
            if ang <= PI/4.0 && ang >= -PI/4.0 {
                hits.push(rbh);
            }
        }
        return hits;
    }

    fn prep_input(&mut self) {
        let contact: f32;
        let contact_agent = self.contact_agent as i32 as f32;
        let contact_resource = self.contact_resource as i32 as f32;
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
                dist/self.vision_range
            },
        };
        let tg_ang = match self.enemy_dir {
            None => 0.0,
            Some(dir) => {
                dir
            },
        };
        let tg_dng = match self.enemy_size {
            None => 0.0,
            Some(size2) => {
                ((size2/(size2+self.size))-0.5)/0.5
            },
        };

        let mut tgr: f32=0.0; let mut tgl: f32=0.0;
        if tg_ang > 0.0 {
            tgr = 1.0 - clamp(tg_ang, 0.0, 1.0);
        } else if tg_ang < 0.0 {
            tgl = 1.0-clamp(tg_ang, -1.0, 0.0).abs();
        }
        
        let res_dist = match self.resource_position {
            None => 0.0,
            Some(pos2) => {
                let dist = pos2.distance(self.pos);
                dist/self.vision_range
            },
        };
        let res_ang = match self.resource_dir {
            None => 0.0,
            Some(dir) => {
                dir
            },
        };
        let mut resr: f32=0.0; let mut resl: f32=0.0;
        if res_ang > 0.0 {
            resr = 1.0 - clamp(res_ang, 0.0, 1.0);
        } else if res_ang < 0.0 {
            resl = 1.0-clamp(res_ang, -1.0, 0.0).abs(); 
        }
        
        let fam: f32 = match self.enemy_family {
            None => 0.0,
            Some(family) => {
                let mut f = 0.0;
                if family { 
                    f = 1.0; 
                } else if !family {
                    f = 0.0;
                }
                f
            }
        };
        let wall = self.blocked;
        self.blocked = 0.0;
        let hp = self.eng/self.max_eng;
        let red = self.mood.r;
        let blu = self.mood.b;
        let gre = self.mood.g;
        let en_color = match self.enemy_mood {
            None => Color::new(0.0, 0.0, 0.0, 1.0),
            Some(color) => color,
        };
        let e_r = en_color.r;
        let e_g = en_color.g;
        let e_b = en_color.b;
        let mut pain = 0.0;
        if self.pain { pain = 1.0; }
        self.pain = false;
        self.pain = false;
        self.neuro_map.set_signal("CON", contact);
        self.neuro_map.set_signal("ENY", contact_agent);
        self.neuro_map.set_signal("RES", contact_resource);
        self.neuro_map.set_signal("ENG", hp);
        self.neuro_map.set_signal("TGL", tgl);
        self.neuro_map.set_signal("TGR", tgr);
        self.neuro_map.set_signal("DST", tg_dist);
        self.neuro_map.set_signal("DNG", tg_dng);
        self.neuro_map.set_signal("FAM", fam);
        self.neuro_map.set_signal("REL", resl);
        self.neuro_map.set_signal("RER", resr);
        self.neuro_map.set_signal("RED", res_dist);
        self.neuro_map.set_signal("PAI", pain);
        self.neuro_map.set_signal("WAL", wall);
        self.neuro_map.set_signal("RED", red);
        self.neuro_map.set_signal("GRE", gre);
        self.neuro_map.set_signal("BLU", blu);
        self.neuro_map.set_signal("E-R", e_r);
        self.neuro_map.set_signal("E-G", e_g);
        self.neuro_map.set_signal("E-B", e_b);
        
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
        
        if self.neuro_map.get_action("RUN") >= 0.9 {
            self.run = true;
        }

        if self.neuro_map.get_action("LFT") > self.neuro_map.get_action("RGT") {
            self.ang_vel = -self.neuro_map.get_action("LFT");
        } else if self.neuro_map.get_action("LFT") < self.neuro_map.get_action("RGT") {
            self.ang_vel = self.neuro_map.get_action("RGT");
        } else {
            self.ang_vel = 0.0;
        }
        
        if self.neuro_map.get_action("ATK") >= 0.75 {
            self.attacking = true;
        } else {
            self.attacking = false;
        }

        if self.neuro_map.get_action("EAT") >= 0.6 {
            self.eating = true;
        } else {
            self.eating = false;
        }
        self.eat_visual = !self.eat_visual;
        self.attack_visual = !self.attack_visual;
        let r = clamp(self.neuro_map.get_action("RED"), 0.0, 1.0);
        let g = clamp(self.neuro_map.get_action("GRE"), 0.0, 1.0);
        let b = clamp(self.neuro_map.get_action("BLU"), 0.0, 1.0);
        self.mood.r = (self.mood.r+r)/2.0;
        self.mood.g = (self.mood.g+g)/2.0;
        self.mood.b = (self.mood.b+b)/2.0;
    }

    fn draw_front(&self) {
        let mut yaw_color = LIGHTGRAY;
        let mut left: Vec2 = Vec2::from_angle(self.rot-PI/10.0);
        let mut right: Vec2 = Vec2::from_angle(self.rot+PI/10.0);
        let l0 = self.pos + left * self.size;
        let r0 = self.pos + right * self.size;
        let mut l1 = self.pos + left * self.size*1.5;
        let mut r1 = self.pos + right * self.size*1.5;
        if self.attacking {
            yaw_color = RED;
            if self.attack_visual {
                l1 = self.pos + left * self.size*2.0;
                r1 = self.pos + right * self.size*2.0;
            }
            draw_line(l0.x, l0.y, l1.x, l1.y, self.size/3.0, yaw_color);
            draw_line(r0.x, r0.y, r1.x, r1.y, self.size/3.0, yaw_color);
        } else if self.eating {
            yaw_color = BLUE;
            if self.eat_visual {
                left = Vec2::from_angle(self.rot-PI/18.0);
                right = Vec2::from_angle(self.rot+PI/18.0);
            }
            l1 = self.pos + left * self.size*1.1;
            r1 = self.pos + right * self.size*1.1;
            draw_circle(l1.x, l1.y, self.size*0.33, yaw_color);
            draw_circle(r1.x, r1.y, self.size*0.33, yaw_color);
        } else {
            draw_line(l0.x, l0.y, l1.x, l1.y, self.size/3.0, yaw_color);
            draw_line(r0.x, r0.y, r1.x, r1.y, self.size/3.0, yaw_color);
        }
    }

    fn draw_eyes(&self, selected: bool) {
        let ang = self.vision_angle/2.0;
        let range = self.vision_range;
        let left_vision_border = Vec2::from_angle(self.rot - ang);
        let right_vision_border = Vec2::from_angle(self.rot + ang);
        let mut color = LIGHTGRAY;
        if self.eating { color = BLUE; }
        if self.attacking { color = RED; }
        let eye_l = Vec2::from_angle(self.rot - PI / 3.0) * self.size*0.66;
        let eye_r = Vec2::from_angle(self.rot + PI / 3.0) * self.size*0.66;
        let xl = self.pos.x + eye_l.x;
        let yl = self.pos.y + eye_l.y;
        let xr = self.pos.x + eye_r.x;
        let yr = self.pos.y + eye_r.y;
        let s = self.size*0.33;
        let vl0 = self.pos + left_vision_border*range*0.1;
        let vr0 = self.pos + right_vision_border*range*0.1;
        let vl1 = self.pos + left_vision_border*range;
        let vr1 = self.pos + right_vision_border*range;
        draw_circle(xl, yl, s, color);
        draw_circle(xr, yr, s, color);
        if selected {
            draw_line(vl0.x, vl0.y, vl1.x, vl1.y, 0.5, SKYBLUE);
            draw_line(vr0.x, vr0.y, vr1.x, vr1.y, 0.5, SKYBLUE);
            draw_smooth_arc(range, self.pos, self.rot, self.vision_angle/2.0, 10.0, 0.5, SKYBLUE);
            draw_smooth_arc(range*0.1, self.pos, self.rot+PI, PI-ang, 10.0, 0.5, ORANGE);
        }
    }

    fn draw_target(&self, _selected: bool) {
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
        let settings = get_settings();
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let text_cfg = TextParams {
            font: *font,
            font_size: 16,
            color: WHITE,
            ..Default::default()
        };
        //let mut info: String;
        let mut info = "".to_string();
        let mut info_gen = "".to_string();
        if settings.show_specie {
            info = format!("{}", self.specie.to_uppercase());
        } 
        if settings.show_generation {
            info_gen = format!("[{}]", self.generation);
        }
        let mut row = 1;
        if settings.show_specie {
            let txt_center = get_text_center(&info, Some(*font), 14, 1.0, 0.0);
            draw_text_ex(&info, x0 - txt_center.x, y0 - txt_center.y + self.size * 2.0 + 16.0, text_cfg.clone());
            row += 1;
        }
        if settings.show_generation {
            let txt_center = get_text_center(&info_gen, Some(*font), 18, 1.0, 0.0);
            draw_text_ex(&info_gen, x0 - txt_center.x, y0 - txt_center.y + self.size * 2.0 + (16.0*row as f32), text_cfg.clone());
        }
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
        self.update_enemy_position(physics);
        let physics_data = physics.get_object_state(self.physics_handle);
        self.pos = physics_data.position;
        self.rot = physics_data.rotation;
        self.mass = physics_data.mass;
        match physics.get_object_mut(self.physics_handle) {
            Some(body) => {
                let dt = get_frame_time()*sim_speed();
                let dir = Vec2::from_angle(self.rot);
                //let rel_speed = self.speed as f32 - (self.shell as f32)/6.0;
                let mut v = dir * self.vel * self.speed as f32 * settings.agent_speed * dt * 10.0;
                if self.run {
                    v *= 1.5;
                }
                let rot = self.ang_vel * settings.agent_rotate * dt / self.shell as f32;
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
        let mut out_of_edge: f32 = 0.0;
        if raw_pos.x <= 0.0 {
            raw_pos.x = 0.0;
            out_of_edge = 1.0;
        } else if raw_pos.x >= settings.world_w as f32 + 0.0 {
            raw_pos.x = settings.world_w as f32;
            out_of_edge = 1.0;
        }
        if raw_pos.y <= 0.0 {
            raw_pos.y = 0.0;
            out_of_edge = 1.0;
        } else if raw_pos.y >= settings.world_h as f32 + 0.0 {
            raw_pos.y = settings.world_h as f32;
            out_of_edge = 1.0;
        }
        if out_of_edge == 1.0 {
            body.set_position(make_isometry(raw_pos.x, raw_pos.y, rot+PI), true);
        }
        if out_of_edge == 1.0 {
            self.blocked = 1.0;
        }
    }

    fn update_enemy_position(&mut self, physics: &Physics) {
        if let Some(rb) = self.enemy {
            if let Some(enemy_position) = physics.get_object_position(rb) {
                self.enemy_position = Some(enemy_position);
                let rel_pos = enemy_position - self.pos;
                let enemy_dir = rel_pos.angle_between(Vec2::from_angle(self.rot))/(1.0*PI);
                self.enemy_dir = Some(enemy_dir);
                if let Some(enemy_size) = physics.get_object_size(rb) {
                    self.enemy_size = Some(enemy_size);
                }
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
                let resource_dir = rel_pos.angle_between(Vec2::from_angle(self.rot))/(1.0*PI);
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

    fn update_contacts(&mut self, other: &HashMap<RigidBodyHandle, Agent>, physics: &mut Physics) {
        self.contacts.clear();
        self.contact_agent = false;
        self.contact_resource = false;
        let contacts = physics.get_contacts_set(self.physics_handle, self.size);
        for contact in contacts {
            if other.contains_key(&contact) {
                self.contact_agent = true;
            } else {
                self.contact_resource = true;
            }
            if let Some(pos2) = physics.get_object_position(contact) {
                let mut rel_pos = pos2 - self.pos;
                rel_pos = rel_pos.normalize_or_zero();
                let target_angle = rel_pos.angle_between(Vec2::from_angle(self.rot));
                self.contacts.push((contact, target_angle));
            }
        }
    }

    fn contacts_clear(&mut self) {
        self.contacts.clear();
        self.contact_agent = false;
        self.contact_resource = false;
    }

    fn watch(&mut self, physics: &Physics) {
        let direction = Vec2::from_angle(self.rot);
        if let Some(tg) = physics.get_closest_agent(self.physics_handle, self.vision_range, self.vision_angle, direction) {
            self.enemy = Some(tg);
        } else {
            self.enemy_family = None;
            self.enemy = None;
            self.enemy_position = None;
            self.enemy_dir = None;
        }
        if let Some(tg) = physics.get_closest_resource(self.physics_handle, self.vision_range, self.vision_angle, direction) {
            self.resource = Some(tg);
        } else {
            self.resource = None;
            self.resource_position = None;
            self.resource_dir = None;
        }
        self.update_enemy_position(physics);
    }

    fn calc_energy(&mut self, dt: f32) {
        let settings = get_settings();
        let base_cost = settings.base_energy_cost;
        let move_cost = settings.move_energy_cost;
        let attack_cost = settings.attack_energy_cost;
        let size_cost = self.size * settings.size_cost;
        let mut basic_loss = (self.shell as f32 + size_cost) * base_cost;
        if self.eating {
            basic_loss += size_cost * base_cost;
        }
        let mut move_loss = self.vel * (self.speed as f32 + size_cost) * move_cost;
        if self.run {
            move_loss *= 2.0;
        }
        let attack_loss = match self.attacking {
            true => attack_cost * self.power as f32,
            false => 0.0,
        };
        self.eng_cost.basic = basic_loss;
        self.eng_cost.movement = move_loss;
        self.eng_cost.attack = attack_loss;
        let loss = (basic_loss + move_loss + attack_loss) * dt;
        if self.eng > 0.0 {
            self.eng -= loss;
        } else {
            self.eng = 0.0;
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

    fn mutate_one(v: i32, m: f32) -> i32 {
        let mut vm: i32 = v;
        if random_unit_unsigned() < m {
            let r = rand::gen_range(0, 2);
            if r == 1 {
                vm += 1;
            } else if r == 0 {
                vm -= 1;
            }
            vm = clamp(vm, 1_i32, 10_i32);
        }
        return vm;
    }

    pub fn mutate(&mut self) {
        let settings = get_settings();
        let m = ((self.mutations - 5) as f32) / 10.0;
        let mut_rate = settings.mutations + settings.mutations * m;
        self.mutations = Self::mutate_one(self.mutations, mut_rate);
        self.size = Self::mutate_one(self.size as i32, mut_rate) as f32;
        self.power = Self::mutate_one(self.power, mut_rate);
        self.speed = Self::mutate_one(self.speed, mut_rate);
        self.shell = Self::mutate_one(self.shell, mut_rate);
        self.eyes = Self::mutate_one(self.eyes, mut_rate);
        self.network.mutate(m);
        self.calc_hp();
        self.vision_angle = Self::calc_vision_angle(self.eyes);
        self.vision_range = Self::calc_vision_range(self.eyes);
    }

    fn calc_hp(&mut self) {
        let settings = get_settings();
        let eng = self.size * settings.size_to_hp + settings.base_hp as f32;
        self.max_eng = eng;
        self.eng = eng*settings.born_eng;
    }

    pub fn replicate(&self, physics: &mut Physics) -> Agent {
        //let settings = get_settings();
        let key = gen_range(u64::MIN, u64::MAX);
        let color = self.color.to_owned();
        let color_second = self.color_second.to_owned();
        let shape = SharedShape::ball(self.size);
        let rot = random_rotation();
        let pos = self.pos;
        let interactions = InteractionGroups::new(Group::GROUP_1, Group::GROUP_2 | Group::GROUP_1 );
        let rbh = physics.add_dynamic_object(&pos, rot, shape.clone(), PhysicsMaterial::default(), interactions);
        let network = self.network.replicate();
        let input_pairs = network.get_input_pairs();
        let output_pairs = network.get_output_pairs();
        let mut neuro_map = NeuroMap::new();
        neuro_map.add_sensors(input_pairs);
        neuro_map.add_effectors(output_pairs);
        let mut agent = Agent {
            key,
            pos: pos + random_unit_vec2()*30.0,
            rot,
            mass: 0.0,
            vel: 0.0,
            ang_vel: 0.0,
            size: self.size,
            vision_range: 0.0,
            vision_angle: 0.0,
            max_eng: 0.0,
            eng: 0.0,
            color,
            color_second,
            shape,
            timer_analize: self.timer_analize.to_owned(),
            timer_contact: self.timer_contact.to_owned(),
            network,
            alife: true,
            lifetime: 0.0,
            generation: self.generation + 1,
            enemy: None,
            enemy_family: None,
            enemy_position: None,
            enemy_mood: None,
            enemy_dir: None,
            enemy_size: None,
            resource: None,
            resource_position: None,
            resource_dir: None,
            contacts: Vec::new(),
            contact_agent: false,
            contact_resource: false,
            physics_handle: rbh,
            neuro_map,
            childs: 0,
            kills: 0,
            specie: self.specie.to_owned(),
            attacking: false,
            eating: false,
            points: 0.0,
            pain: false,
            run: false,
            power: self.power,
            speed: self.speed,
            shell: self.shell,
            mutations: self.mutations,
            eyes: self.eyes,
            mood: Color::new(0.0, 0.0, 0.0, 1.0),
            ancestors: self.ancestors.to_owned(),
            eng_cost: EnergyCost::default(),
            blocked: 0.0,
            attack_visual: false,
            eat_visual: false,
        };
        agent.mod_specie();
        agent.mutate();
        agent.calc_hp();
        return agent;
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
            color_second: self.color_second.to_vec().to_array(),  
            network: self.network.get_sketch(),
            points: self.points, 
            neuro_map: self.neuro_map.clone(),
            power: self.power,
            speed: self.speed,
            shell: self.shell,
            mutations: self.mutations,
            eyes: self.eyes,
            ancestors: self.ancestors.to_owned(),
        }
    }
}

