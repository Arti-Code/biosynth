#![allow(unused)]

use std::f32::consts::PI;
use crate::consts::*;
use macroquad::{color, prelude::*};
use rapier2d::prelude::*;
use rapier2d::parry::query::contact; 
use rapier2d::na::{Isometry2, Vector2, Translation, Point2};

pub fn random_unit() -> f32 {
    return rand::gen_range(-1.0, 1.0);
}

pub fn random_position(x_max: f32, y_max: f32) -> Vec2 {
    let x = rand::gen_range(0.0, x_max);
    let y = rand::gen_range(0.0, y_max);
    return Vec2::new(x, y);
}

pub fn random_rotation() -> f32 {
    let rot = rand::gen_range(0.0, PI * 2.0);
    return rot;
}

pub fn random_unit_vec2() -> Vec2 {
    let x = rand::gen_range(-1.0, 1.0);
    let y = rand::gen_range(-1.0, 1.0);
    return Vec2::new(x, y).normalize_or_zero();
}

pub fn random_color() -> color::Color {
    let colors = vec![RED, GREEN, BLUE, YELLOW, ORANGE, GRAY, SKYBLUE, LIME];
    let num = colors.len();
    let c = rand::gen_range(0, num);
    return colors[c];
}

pub fn random_color5() -> color::Color {
    let colors = [RED, BLUE, GREEN, YELLOW, WHITE];
    //let num = colors.len();
    let c = rand::gen_range(0, 5);
    return colors[c];
}

pub fn angle2vec2(angle: f32) -> Vec2 {
    let (x, y) = angle.sin_cos();
    let mut v = Vec2::new(x, y).normalize_or_zero();
    return v;
}

pub fn wrap_around(v: &Vec2) -> Vec2 {
    let tolerance = 5.0;
    let mut vr = Vec2::new(v.x, v.y);
    if vr.x > WORLD_W + tolerance {
        vr.x = 0.0 - tolerance;
    } else if vr.x < 0.0 - tolerance {
        vr.x = WORLD_W + tolerance;
    }
    if vr.y > WORLD_H + tolerance {
        vr.y = 0.0 - tolerance;
    } else if vr.y < 0.0 - tolerance {
        vr.y = WORLD_H + tolerance;
    }
    return vr;
}

pub fn make_isometry(posx: f32, posy: f32, rotation: f32) -> Isometry2<f32> {
    let iso = Isometry2::new(Vector2::new(posx, posy), rotation);
    return iso;
}

pub fn matrix_to_vec2(translation: Translation<f32, 2>) -> Vec2 {
    return Vec2::new(translation.x, translation.y);
}

pub fn map_polygon(n: usize, r: f32, dev: f32) -> Vec<Vec2> {
    let mut points: Vec<Vec2> = vec![];
    let s = 2.0 * PI / (n as f32);
    let mut a = 2.0 * PI;
    for i in 0..n {
        a = s * i as f32;
        let x = a.sin();
        let y = a.cos();
        let v = Vec2::new(x, y)*r;
        points.push(v);
    }
    return points;
}

fn vec2_to_point2(v: &Vec2) -> Point2<f32> {
    return Point2::new(v.x, v.y);
}

pub fn vec2_to_point2_collection(vec2_list: &Vec<Vec2>) -> Vec<Point2<f32>> {
    let mut points: Vec<Point2<f32>> = vec![];
    for v in vec2_list.iter() {
        let p = Point2::new(v.x, v.y);
        points.push(p);
    }
    //let d = points.as_chunks();
    return points;
}

pub fn vec2_to_point2_array(vec2_list: &Vec<Vec2>) -> Matrix<Point2<f32>> {
    let l = vec2_list.len();
    let mut points: Matrix<Point2<f32>>;
    let vecs = vec2_to_point2_collection(vec2_list);
    points = Matrix::from_vec(vecs);
    return points;
}

pub fn contact_mouse(mouse_pos: Vec2, target_pos: Vec2, target_rad: f32) -> bool {
    let v1 = Vec2::new(mouse_pos.x, mouse_pos.y);
    let v2 = Vec2::new(target_pos.x, target_pos.y);
    let pos1 = make_isometry(v1.x, v1.y, 0.0);
    let pos2 = make_isometry(v2.x, v2.y, 0.0);
    let ball1 = Ball::new(2.0);
    let ball2 = Ball::new(target_rad);
    match contact(&pos1, &ball1, &pos2, &ball2, 0.0).unwrap() {
        Some(_) => true,
        None => false,
    }
}

//?         [[[SIGNALS]]]
pub struct Signals {
    pub spawn_agent: bool,
    pub spawn_plant: bool,
    pub spawn_asteroid: bool,
    pub spawn_jet: bool,
    pub spawn_particles: bool,
    pub new_sim: bool,
    pub new_sim_name: String,
    pub new_settings: bool,
}

impl Signals {
    
    pub fn new() -> Self {
        Self {
            spawn_agent: false,
            spawn_plant: false,
            spawn_asteroid: false,
            spawn_jet: false,
            spawn_particles: false,
            new_sim: false,
            new_sim_name: String::new(),
            new_settings: false,
        }
    }
}


#[derive(Clone, Copy)]
pub struct Settings {
    pub agent_min_num: usize,
    pub agent_init_num: usize,
    pub agent_speed: f32,
    pub agent_vision_range: f32,
    pub agent_rotate: f32,
    pub agent_eng_bar: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            agent_min_num: AGENTS_NUM_MIN,
            agent_init_num: AGENTS_NUM,
            agent_speed: AGENT_SPEED,
            agent_rotate: AGENT_ROTATE,
            agent_vision_range: AGENT_VISION_RANGE,
            agent_eng_bar: true,
        }
    }
}

impl Settings {
    pub fn new(agent_init_num: usize, agent_min_num: usize, agent_speed: f32, agent_turn: f32, vision_range: f32, agent_energy_bar: bool) -> Self {
        Self {
            agent_init_num: agent_init_num,
            agent_min_num: agent_min_num,
            agent_speed: agent_speed,
            agent_rotate: agent_turn,
            agent_vision_range: vision_range,
            agent_eng_bar: agent_energy_bar,
        }
    }
}


pub struct SimState {
    pub sim_name: String,
    pub ver: String,
    pub agents_num: i32,
    pub plants_num: i32,
    pub lifes_num: i32,
    pub physics_num: i32,
    pub total_mass: f32,
    pub total_eng: f32,
    pub sim_time: f64,
    pub fps: i32,
    pub dt: f32,
    pub total_kin_eng: f32,
    pub contacts_info: (i32, i32),
}

impl SimState {
    pub fn new() -> Self {
        Self {
            sim_name: String::new(),
            ver: String::from(env!("CARGO_PKG_VERSION")),
            agents_num: AGENTS_NUM as i32,
            plants_num: AGENTS_NUM as i32,
            lifes_num: 0,
            physics_num: 0,
            total_mass: 0.0,
            total_eng: 0.0,
            sim_time: 0.0,
            fps: 0,
            dt: 0.0,
            total_kin_eng: 0.0,
            contacts_info: (0, 0),
        }
    }
}


pub struct MouseState {
    pub pos: Vec2,
}