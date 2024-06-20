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
use crate::statistics::*;
use crate::misc::*;
use crate::sketch::*;
use crate::phyx::physics::Physics;
use crate::phyx::physics_misc::PhysicsMaterial;
use ::rand::prelude::*;

#[derive(Clone)]
pub struct Agent {
    pub key: u64,
    pub pos: Vec2,
    pub rot: f32,
    mass: f32,
    vel: f32,
    ang_vel: f32,
    pub size: f32,
    vision_range: f32,
    vision_angle: f32,
    pub max_eng: f32,
    pub eng: f32,
    pub max_hp: f32,
    pub hp: f32,
    color: Color,
    color_second: Color,
    pub shape: SharedShape,
    timer_analize: Timer,
    timer_contact: Timer,
    pub network: Network,
    pub alife: bool,
    pub lifetime: f32,
    pub repro_time: f32,
    pub generation: u32,
    pub contacts: Vec<(RigidBodyHandle, f32)>,
    pub contact_agent: bool,
    pub contact_plant: bool,
    pub enemy: Option<RigidBodyHandle>,
    pub enemy_family: Option<bool>,
    pub enemy_position: Option<Vec2>,
    pub enemy_dir: Option<f32>,
    pub enemy_size: Option<f32>,
    enemy_mood: Option<Color>,
    pub plant: Option<RigidBodyHandle>,
    pub plant_position: Option<Vec2>,
    pub plant_dir: Option<f32>,
    pub rbh: RigidBodyHandle,
    colliders: Vec<ColliderHandle>,
    pub neuro_map: NeuroMap,
    pub childs: usize,
    pub kills: usize,
    pub specie: String,
    pub attacking: bool,
    pub eating: bool,
    pub points: f32,
    pub pain: f32,
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
    water: i32,
}



impl Agent {
    
    pub fn new(physics: &mut Physics) -> Self {
        let settings = get_settings();
        let size = rand::gen_range(settings.agent_size_min, settings.agent_size_max) as f32;
        let rot = 0.0; //random_rotation();
        let eyes = gen_range(0, 10);
        let pos = random_position(settings.world_w as f32, settings.world_h as f32);
        let shape = SharedShape::ball(size);
        let rbh = physics.add_dynamic_object(
            &pos, 
            rot, 
            shape.clone(), 
            PhysicsMaterial::agent(), 
            InteractionGroups { 
                memberships: Group::GROUP_1, 
                filter: Group::GROUP_2 | Group::GROUP_1 
            },
            false,
        );
        let color = random_color();
        let color_second = random_color();
        let mut network = Network::new(1.0);
        let inp_labs = vec![
            "CON", "ENY", "RES", "HP", "ENG", "TGL", "TGR", "DST", 
            "DNG", "FAM", "REL", "RER", "RED", "PAI", "WAL", "H2O",
            "RED", "GRE", "BLU", "WAL", "E-R", "E-G", "E-B"
        ];
        let out_labs = vec![
            "MOV", "LFT", "RGT", "ATK", 
            "EAT", "RUN", "RED", "GRE", "BLU"
        ];
        let hid = settings.hidden_nodes_num;
        let hid_layers = settings.hidden_layers_num;
        let hid_range = 0..=hid;
        let layers_range = 0..=hid_layers;
        //let h = thread_rng().gen_range(hid_range);
        let l = thread_rng().gen_range(layers_range);
        let mut deep = vec![];
        for _ in 0..l {
            let node_num = thread_rng().gen_range(hid_range.clone());
            deep.push(node_num);
        }
        network.build(
            inp_labs.len(), 
            inp_labs, 
            deep, 
            out_labs.len(), 
            out_labs, 
            settings.neurolink_rate
        );
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
            max_hp: 100.0,
            hp: 100.0,
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
            repro_time: 0.0,
            generation: 0,
            enemy: None,
            enemy_family: None,
            enemy_position: None,
            enemy_mood: None,
            enemy_dir: None,
            enemy_size: None,
            plant: None,
            plant_position: None,
            plant_dir: None,
            contacts: Vec::new(),
            contact_agent: false,
            contact_plant: false,
            rbh,
            colliders: vec![],
            neuro_map,
            childs: 0,
            kills: 0,
            specie: create_name(4),
            attacking: false,
            eating: false,
            points: 0.0,
            pain: 0.0,
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
            water: 0,
        };
        agent.ancestors.add_ancestor(Ancestor::new(&agent.specie, agent.generation as i32, 0));
        agent.calc_hp();
        //let new_rot = rot-PI;
        let yaw = SharedShape::ball(size/4.0);
        let left: Vec2 = Vec2::from_angle(rot-PI-PI/3.0)*size;
        let right: Vec2 = Vec2::from_angle(rot-PI+PI/3.0)*size;
        let colh_left = physics.add_collider(
            agent.rbh, 
            &left, 
            0.0, 
            yaw.clone(), 
            PhysicsMaterial::agent(), 
            InteractionGroups { 
                memberships: Group::GROUP_1, 
                filter: Group::GROUP_2 | Group::GROUP_1 
            }
        );
        let colh_right = physics.add_collider(
            agent.rbh, 
            &right, 
            0.0, 
            yaw.clone(), 
            PhysicsMaterial::agent(), 
            InteractionGroups { 
                memberships: Group::GROUP_1, 
                filter: Group::GROUP_2 | Group::GROUP_1 
            }
        );
        agent.colliders.push(colh_left);
        agent.colliders.push(colh_right);
        return agent;
    }

    pub fn calc_vision_range(eyes: i32) -> f32 {
        let settings = get_settings();
        return 120.0 + settings.agent_vision_range*(eyes as f32)/10.0;
    }

    pub fn calc_vision_angle(eyes: i32) -> f32 {
        return 1.8*PI * ((11.0 - eyes as f32)/11.0);
    }

    pub fn get_mood(&self) -> Color {
        return self.mood.to_owned();
    }

    pub fn get_nodes_links_num(&self) -> (i32, i32) {
        return self.network.get_nodes_links_number();
    }

    fn mod_specie(&mut self, time: f64) {
        let settings = get_settings();
        if rand::gen_range(0, settings.rare_specie_mod) == 0 {
            let s = create_name(1);
            let i = rand::gen_range(0, 3)*2;
            let mut name = self.specie.to_owned();
            name.replace_range(i..=i+1, &s);
            self.specie = name.to_owned();
            self.ancestors.add_ancestor(Ancestor::new(&self.specie, self.generation as i32, time.round() as i32));
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

    pub fn from_sketch(sketch: AgentSketch, physics: &mut Physics, time: f64) -> Agent {
        let key = gen_range(u64::MIN, u64::MAX);
        let settings = get_settings();
        let pos = vec2(sketch.pos[0], sketch.pos[1])+random_unit_vec2()*100.0;
        //let pos = random_position(settings.world_w as f32, settings.world_h as f32);
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
        let rot = 0.0; //random_rotation();
        let gen = sketch.generation + 1;
        let network = sketch.network.from_sketch();
        let rbh = physics.add_dynamic_object(
            &pos, 
            0.0, 
            shape.clone(), 
            PhysicsMaterial::default(), 
            InteractionGroups { memberships: Group::GROUP_1, filter: Group::GROUP_2 | Group::GROUP_1 },
            false,
        );
        let mut agent = Agent {
            key,
            pos,
            rot,
            mass: 0.0,
            vel: 0.0,
            ang_vel: 0.0,
            size,
            vision_range: 0.0,
            vision_angle: 0.0,
            max_hp: 100.0,
            hp: 100.0,
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
            repro_time: 0.0,
            generation: gen,
            enemy: None,
            enemy_family: None,
            enemy_position: None,
            enemy_mood: None,
            enemy_dir: None,
            enemy_size: None,
            plant: None,
            plant_position: None,
            plant_dir: None,
            contacts: Vec::new(),
            contact_agent: false,
            contact_plant: false,
            rbh,
            colliders: vec![],
            neuro_map: sketch.neuro_map.clone(),
            childs: 0,
            kills: 0,
            specie: sketch.specie.to_owned(),
            attacking: false,
            eating: false,
            points: 0.0,
            pain: 0.0,
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
            water: 0,
        };
        agent.mod_specie(time);
        agent.mutate();
        agent.calc_hp();
        let yaw = SharedShape::ball(size/4.0);
        let left: Vec2 = Vec2::from_angle(rot-PI-PI/4.0)*size;
        let right: Vec2 = Vec2::from_angle(rot-PI+PI/4.0)*size;
        let colh_left = physics.add_collider(
            agent.rbh, 
            &left, 
            0.0, 
            yaw.clone(), 
            PhysicsMaterial::agent(), 
            InteractionGroups { 
                memberships: Group::GROUP_1, 
                filter: Group::GROUP_2 | Group::GROUP_1 
            }
        );
        let colh_right = physics.add_collider(
            agent.rbh, 
            &right, 
            0.0, 
            yaw.clone(), 
            PhysicsMaterial::agent(), 
            InteractionGroups { 
                memberships: Group::GROUP_1, 
                filter: Group::GROUP_2 | Group::GROUP_1 
            }
        );
        agent.colliders.push(colh_left);
        agent.colliders.push(colh_right);
        return agent;
    }

    pub fn draw(&self, selected: bool, font: &Font, _physics: &Physics) {
        let settings = get_settings();
        if settings.agent_eng_bar {
            let e = self.eng/self.max_eng;
            let hp = self.hp/self.max_hp;
            self.draw_status_bar(hp, GREEN, RED, Vec2::new(0.0, self.size*1.5+4.0));
            self.draw_status_bar(e, SKYBLUE, YELLOW, Vec2::new(0.0, self.size*1.5+6.0));
        }
        self.draw_body();
        self.draw_front();
        self.draw_eyes(selected);
        //self.draw_limbs(physics);
        if selected {
            self.draw_info(&font);
            self.draw_target(selected);
        } else {
            self.draw_info(&font);
        }
    }    

    fn draw_limbs(&self, physics: &Physics) {
        let colh_l = physics.core.colliders.get(self.colliders[0]).unwrap();
        //let tl_l = colh_l.position_wrt_parent().unwrap().translation;
        //let rot_l = colh_l.rotation().angle();
        let (mut pos_l, _) = iso_to_vec2_rot(colh_l.position_wrt_parent().unwrap());
        
        pos_l.rotate(Vec2::from_angle(self.rot));
        pos_l += self.pos;
        //let pos_l = self.pos+Vec2::from_angle(self.rot+rot_l)*(self.size+self.size/5.0);
        //let loc_l = vec2(tl_l.x, tl_l.y) + self.pos;
        let colh_r = physics.core.colliders.get(self.colliders[1]).unwrap();
        let tl_r = colh_r.position_wrt_parent().unwrap().translation;
        let loc_r = vec2(tl_r.x, tl_r.y) + self.pos;
        draw_circle(pos_l.x, pos_l.y, self.size/2.0, PINK);
        draw_circle(loc_r.x, loc_r.y, self.size/2.0, PINK);
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
        let dt = dt()*sim_speed();
        self.lifetime += dt;
        if self.repro_time < get_settings().repro_time {
            self.repro_time += dt;
        }
        if self.timer_analize.update(dt) {
            self.update_contacts(other, physics);
            self.watch(physics);
            self.update_enemy_mood(other);
            self.analize();
            //self.contacts_clear();
        }

        self.update_physics(physics);
        if self.pos.x.is_nan() || self.pos.y.is_nan() {
            self.alife = false;
            return self.alife;
        }
        self.calc_energy();
        self.calc_health();
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

    pub fn set_water_tile(&mut self, water: i32) {
        self.water = water;
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
        //let dt = dt()*sim_speed();
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
        let contact_plant = self.contact_plant as i32 as f32;
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
        
        let res_dist = match self.plant_position {
            None => 0.0,
            Some(pos2) => {
                let dist = pos2.distance(self.pos);
                dist/self.vision_range
            },
        };
        let res_ang = match self.plant_dir {
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
        let eng = self.eng/self.max_eng;
        let hp = self.hp/self.max_hp;
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
        //let mut pain = 0.0;
        //if self.pain { pain = 1.0; }
        //self.pain = false;
        //self.pain = false;
        let mut water = 0.0;
        if self.water == 1 {
            water = 0.5;
        } else if self.water > 1 {
            water = 1.0;
        }
        self.neuro_map.set_signal("CON", contact);
        self.neuro_map.set_signal("ENY", contact_agent);
        self.neuro_map.set_signal("RES", contact_plant);
        self.neuro_map.set_signal("HP", hp);
        self.neuro_map.set_signal("ENG", eng);
        self.neuro_map.set_signal("TGL", tgl);
        self.neuro_map.set_signal("TGR", tgr);
        self.neuro_map.set_signal("DST", tg_dist);
        self.neuro_map.set_signal("DNG", tg_dng);
        self.neuro_map.set_signal("FAM", fam);
        self.neuro_map.set_signal("REL", resl);
        self.neuro_map.set_signal("RER", resr);
        self.neuro_map.set_signal("RED", res_dist);
        self.neuro_map.set_signal("PAI", self.pain);
        self.neuro_map.set_signal("WAL", wall);
        self.neuro_map.set_signal("H2O", water);
        self.neuro_map.set_signal("RED", red);
        self.neuro_map.set_signal("GRE", gre);
        self.neuro_map.set_signal("BLU", blu);
        self.neuro_map.set_signal("E-R", e_r);
        self.neuro_map.set_signal("E-G", e_g);
        self.neuro_map.set_signal("E-B", e_b);
        self.pain = clamp(self.pain - get_settings().neuro_duration/2.0, 0.0, 1.0);
        
    }

    fn analize(&mut self) {

        self.network.deactivate_nodes();
        self.prep_input();
        self.neuro_map.send_signals(&mut self.network);
        self.network.calc();
        self.neuro_map.recv_actions(&self.network);

        if self.neuro_map.get_action("MOV") > 0.0 {
            self.vel = self.neuro_map.get_action("MOV");
        } else {
            self.vel = self.neuro_map.get_action("MOV")*0.2;
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
        
        if self.neuro_map.get_action("ATK") >= 0.7 {
            self.attacking = true;
        } else {
            self.attacking = false;
        }

        if self.neuro_map.get_action("EAT") >= 0.7 {
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
        let mut small_vision = get_settings().agent_vision_range*get_settings().peripheral_vision;
        let range = self.vision_range;
        small_vision = clamp(small_vision, 0.0, range);
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
        let vl0 = self.pos + left_vision_border*small_vision;
        let vr0 = self.pos + right_vision_border*small_vision;
        let vl1 = self.pos + left_vision_border*range;
        let vr1 = self.pos + right_vision_border*range;
        draw_circle(xl, yl, s, color);
        draw_circle(xr, yr, s, color);
        if selected {
            draw_line(vl0.x, vl0.y, vl1.x, vl1.y, 0.5, SKYBLUE);
            draw_line(vr0.x, vr0.y, vr1.x, vr1.y, 0.5, SKYBLUE);
            draw_smooth_arc(range, self.pos, self.rot, self.vision_angle/2.0, 10.0, 0.5, SKYBLUE);
            draw_smooth_arc(small_vision, self.pos, self.rot+PI, PI-ang, 10.0, 0.5, SKYBLUE);
        }
    }

    fn draw_target(&self, _selected: bool) {
        self.enemy.inspect(|_| {
            self.enemy_position.inspect(|enemy_position| {
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
            });
        });
        self.plant.inspect(|_| {
            self.plant_position.inspect(|plant_position| {
                let v0l = Vec2::from_angle(self.rot - PI / 2.0) * self.size;
                let v0r = Vec2::from_angle(self.rot + PI / 2.0) * self.size;
                let x0l = self.pos.x + v0l.x;
                let y0l = self.pos.y + v0l.y;
                let x0r = self.pos.x + v0r.x;
                let y0r = self.pos.y + v0r.y;
                let x1 = plant_position.x;
                let y1 = plant_position.y;
                draw_line(x0l, y0l, x1, y1, 1.0, self.color);
                draw_line(x0r, y0r, x1, y1, 1.0, self.color);
            });
        });
    }

    fn draw_info(&self, font: &Font) {
        let settings = get_settings();
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let text_cfg = TextParams {
            font: *font,
            font_size: 14,
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
            info_gen = format!("{}", self.generation);
        }
        let mut row = 1;
        if settings.show_specie {
            let txt_center = get_text_center(&info, Some(*font), 14, 1.0, 0.0);
            draw_text_ex(&info, x0 - txt_center.x, y0 - txt_center.y + self.size * 2.0 + 14.0, text_cfg.clone());
            row += 1;
        }
        if settings.show_generation {
            let txt_center = get_text_center(&info_gen, Some(*font), 14, 1.0, 0.0);
            draw_text_ex(&info_gen, x0 - txt_center.x, y0 - txt_center.y + self.size * 2.0 + (14.0*row as f32), text_cfg.clone());
        }
    }

    fn draw_status_bar(&self, percent: f32, color1: Color, color2: Color, offset: Vec2) {
        let xc = self.pos.x + offset.x; let yc = self.pos.y + offset.y;
        let x0 = xc-10.0; let y0 = yc -0.75;
        let w = 20.0*percent;
        draw_rectangle(x0, y0, 20.0, 1.5, color2);
        draw_rectangle(x0, y0, w, 1.5, color1);
    }

    fn update_physics(&mut self, physics: &mut Physics) {
        let settings = get_settings();
        self.update_enemy_position(physics);
        let physics_data = physics.get_object_state(self.rbh);
        self.pos = physics_data.position;
        self.rot = physics_data.rotation;
        self.mass = physics_data.mass;
        match physics.get_object_mut(self.rbh) {
            Some(body) => {
                let dt = dt()*sim_speed();
                let dir = Vec2::from_angle(self.rot);
                let v =(self.speed as f32 + 5.0) * settings.agent_speed * 5.0;
                let mut vel = dir * self.vel * v * dt;
                //let mut v = dir * self.vel * (self.speed as f32/2.0) * settings.agent_speed * dt * 20.0;
                let w = clamp(self.water, 0, 4);
                if w > 1 {
                    vel = vel/4.0;
                }
                if self.run && self.water == 0 {
                    vel *= 1.5;
                }
                let rot = self.ang_vel * settings.agent_rotate * dt / (self.shell as f32 * 0.5);
                body.set_linvel(Vector2::new(vel.x, vel.y), true);
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
        if let Some(rb) = self.plant {
            if let Some(plant_position) = physics.get_object_position(rb) {
                self.plant_position = Some(plant_position);
                let rel_pos = plant_position - self.pos;
                let plant_dir = rel_pos.angle_between(Vec2::from_angle(self.rot))/(1.0*PI);
                self.plant_dir = Some(plant_dir);
            } else {
                self.plant = None;
                self.plant_position = None;
                self.plant_dir = None;
            }
        } else if self.plant_position.is_some() {
            self.plant_position = None;
            self.plant_dir = None;
        }
    }

    fn update_contacts(&mut self, other: &HashMap<RigidBodyHandle, Agent>, physics: &mut Physics) {
        self.contacts_clear();
        let contacts = physics.get_contacts_set(self.rbh, self.size);
        for contact in contacts {
            if other.contains_key(&contact) {
                self.contact_agent = true;
            } else {
                self.contact_plant = true;
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
        self.contact_plant = false;
    }

    fn watch(&mut self, physics: &Physics) {
        let direction = Vec2::from_angle(self.rot);
        if let Some(tg) = physics.get_closest_agent(
            self.rbh, 
            self.vision_range, 
            self.vision_angle, 
            direction
        ) {
            self.enemy = Some(tg);
        } else {
            self.enemy_family = None;
            self.enemy = None;
            self.enemy_position = None;
            self.enemy_dir = None;
        }
        if let Some(tg) = physics.get_closest_plant(
            self.rbh, 
            self.vision_range, 
            self.vision_angle, 
            direction
        ) {
            self.plant = Some(tg);
        } else {
            self.plant = None;
            self.plant_position = None;
            self.plant_dir = None;
        }
        self.update_enemy_position(physics);
    }

    fn calc_energy(&mut self) {
        let settings = get_settings();
        let base_cost = settings.base_energy_cost;
        let move_cost = settings.move_energy_cost;
        let attack_cost = settings.attack_energy_cost;
        let size_cost = self.size * settings.size_cost;
        let mut basic_loss = size_cost * base_cost;
        if self.eating {
            basic_loss += attack_cost * self.size;
        }
        let shell_loss = self.shell as f32 * 0.25; 
        let speed_loss = self.speed as f32 * 3.0;
        let mut move_loss = self.vel.abs() * (shell_loss + speed_loss + size_cost) * move_cost;
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
        let loss = (basic_loss + move_loss + attack_loss) * dt();
        if self.eng > 0.0 {
            self.eng -= loss;
        } else {
            self.eng = 0.0;
        }
        self.check_alife();
    }

    fn calc_health(&mut self) {
        let e = self.eng/self.max_eng;
        if e >= get_settings().eng_bias {
            self.hp += e * dt() * 1.0;
        } else {
            self.hp += (e - 1.0) * dt() * 2.5; 
        }
        self.hp = clamp(self.hp, 0.0, self.max_hp);
    }

    fn check_alife(&mut self) {
        self.alife = true;
        if self.hp <= 0.0 {
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

    pub fn get_hit(&mut self, e: f32) {
        self.hp -= e;
        if self.hp > self.max_hp {
            self.hp = self.max_hp;
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
        let m = ((self.mutations - 5) as f32) / 20.0;
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

    pub fn replicate(&self, physics: &mut Physics, time: f64) -> Agent {
        //let settings = get_settings();
        let key = gen_range(u64::MIN, u64::MAX);
        let color = self.color.to_owned();
        let color_second = self.color_second.to_owned();
        let shape = SharedShape::ball(self.size);
        let rot = 0.0; //random_rotation();
        let pos = self.pos + random_unit_vec2()*100.0;
        let interactions = InteractionGroups::new(
            Group::GROUP_1, 
            Group::GROUP_2 | Group::GROUP_1 
        );
        let rbh = physics.add_dynamic_object(
            &pos, 
            rot, 
            shape.clone(), 
            PhysicsMaterial::default(), 
            interactions, 
            false
        );
        let network = self.network.replicate();
        let input_pairs = network.get_input_pairs();
        let output_pairs = network.get_output_pairs();
        let mut neuro_map = NeuroMap::new();
        neuro_map.add_sensors(input_pairs);
        neuro_map.add_effectors(output_pairs);
        let mut agent = Agent {
            key,
            pos,
            rot,
            mass: 0.0,
            vel: 0.0,
            ang_vel: 0.0,
            size: self.size,
            vision_range: 0.0,
            vision_angle: 0.0,
            max_hp: 100.0,
            hp: 100.0,
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
            repro_time: 0.0,
            generation: self.generation + 1,
            enemy: None,
            enemy_family: None,
            enemy_position: None,
            enemy_mood: None,
            enemy_dir: None,
            enemy_size: None,
            plant: None,
            plant_position: None,
            plant_dir: None,
            contacts: Vec::new(),
            contact_agent: false,
            contact_plant: false,
            rbh,
            colliders: vec![],
            neuro_map,
            childs: 0,
            kills: 0,
            specie: self.specie.to_owned(),
            attacking: false,
            eating: false,
            points: 0.0,
            pain: 0.0,
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
            water: 0,
        };
        agent.mod_specie(time);
        agent.mutate();
        agent.calc_hp();
        let yaw = SharedShape::ball(agent.size/4.0);
        let left: Vec2 = Vec2::from_angle(rot-PI-PI/4.0) * (agent.size);
        let right: Vec2 = Vec2::from_angle(rot-PI+PI/4.0) * (agent.size);
        let colh_left = physics.add_collider(
            agent.rbh, 
            &left, 
            0.0, 
            yaw.clone(), 
            PhysicsMaterial::agent(), 
            InteractionGroups { 
                memberships: Group::GROUP_1, 
                filter: Group::GROUP_2 | Group::GROUP_1 
            }
        );
        let colh_right = physics.add_collider(
            agent.rbh, 
            &right, 
            0.0, 
            yaw.clone(), 
            PhysicsMaterial::agent(), 
            InteractionGroups { 
                memberships: Group::GROUP_1, 
                filter: Group::GROUP_2 | Group::GROUP_1 
            }
        );
        agent.colliders.push(colh_left);
        agent.colliders.push(colh_right);
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
            pos: [self.pos.x, self.pos.y],
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

    pub fn get_water(&self) -> i32 {
        return self.water;
    }

}

