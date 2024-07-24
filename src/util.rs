#![allow(unused)]

use std::error::Error;
use std::f32::consts::PI;
use std::{fs, io};
use std::path::Path;
use std::time::{UNIX_EPOCH, SystemTime};

use egui_macroquad::egui::epaint::ahash::HashMap;
use egui_macroquad::egui::{Pos2, Color32};
use macroquad::{color, prelude::*};
use rapier2d::prelude::*;
use rapier2d::parry::query::contact; 
use rapier2d::na::{Isometry2, Vector2, Translation, Point2, Const};

use crate::settings::*;
use crate::sketch::*;
use crate::sketch::SimulationSketch;
use crate::statistics::Statistics;

static NAME_LIST: [&str; 529] = [
    "am","af", "ax", "ar", "av", "al", "aq", "ak", "ar", "at",
    "cu", "ca", "co", "cy", "cu", "ce", "co", "cv", "ce", "cd", "cf", "cf", "ct", "ci", "cj", "ck", "cl", "cr", "cs", "cz", "cw", "cm", "cu", "cp",
    "mu", "ma", "mo", "my", "mu", "me", "mo", "mv", "me", "md", "mf", "mf", "mt", "mi", "mj", "mk", "ml", "mr", "ms", "mz", "mw", "mm", "mu", "mp",
    "ju", "ja", "jo", "jy", "ju", "je", "jo", "jv", "je", "jd", "jf", "jf", "jt", "ji", "jj", "jk", "jl", "jr", "js", "jz", "jw", "jj", "ju", "jp",
    "du", "da", "do", "dy", "du", "de", "do", "dv", "de", "dd", "df", "df", "dt", "di", "dj", "dk", "dl", "dr", "ds", "dz", "dw", "dd", "du", "dp",
    "so", "su", "sa", "si", "se", "sy", "sl", "sj", "ss", "sk", "sr", "st", "sq", "sf", "sn",
    "nu", "na", "no", "ny", "nu", "ne", "no", "nv", "ne", "nd", "nf", "nf", "nt", "ni", "nj", "nk", "nl", "nr", "ns", "nz", "nw", "nn", "nu", "np",
    "vu", "va", "vo", "vy", "vu", "ve", "vo", "vv", "ve", "vd", "vf", "vf", "vt", "vi", "vj", "vk", "vl", "vr", "vs", "vz", "vw", "vv", "vu", "vp",
    "xu", "xa", "xo", "xy", "xu", "xe", "xo", "xv", "xe", "xd", "xf", "xf", "xt", "xi", "xj", "xk", "xl", "xr", "xs", "xz", "xw", "xx", "xu", "xp",
    "pu", "pa", "po", "py", "pu", "pe", "po", "pv", "pe", "pd", "pf", "pf", "pj", "pi", "pj", "pk", "pl", "pr", "ps", "pz", "pw", "pp", "pu", "pt",
    "lu", "la", "lo", "ly", "lu", "le", "lo", "lv", "le", "ld", "lf", "lf", "lt", "li", "lj", "lk", "ll", "lr", "ls", "lz", "lw", "ll", "lu", "lp", 
    "ku", "ka", "ko", "ky", "ku", "ke", "ko", "kv", "ke", "kd", "kf", "kf", "kt", "ki", "kj", "kk", "kl", "kr", "ks", "kz", "kw", "kk", "ku", "kp",
    "ru", "ra", "ro", "ry", "ru", "re", "ro", "rv", "re", "rd", "rf", "rf", "rt", "ri", "rj", "rk", "rl", "rr", "rs", "rz", "rw", "rr", "ru", "rp",
    "fu", "fa", "fo", "fy", "fu", "fe", "fo", "fv", "fe", "fd", "ff", "ff", "ft", "fi", "fj", "fk", "fl", "fr", "fs", "fz", "fw", "ff", "fu", "fp", 
    "ol", "oi", "oj", "od", "os", "ot", "ok", "on", "om", "oc", "ox", "oz", "op",
    "iu", "ia", "io", "iy", "iu", "ie", "io", "iv", "ie", "id", "if", "if", "it", "ii", "ij", "ik", "il", "ir", "is", "iz", "iw", "ii", "iu", "ip",
    "wu", "wa", "wo", "wy", "wu", "we", "wo", "wv", "we", "wd", "wf", "wf", "wt", "wi", "wj", "wk", "wl", "wr", "ws", "wz", "ww", "ww", "wu", "wp",
    "bu", "ba", "bo", "by", "bu", "be", "bo", "bv", "be", "bd", "bf", "bf", "bt", "bi", "bj", "bk", "bl", "br", "bs", "bz", "bw", "bb", "bu", "bp",
    "qu", "qa", "qo", "qy", "qu", "qe", "qo", "qv", "qe", "qd", "qf", "qf", "qt", "qi", "qj", "qk", "ql", "qr", "qs", "qz", "qw", "qq", "qu", "qp", 
    "uo", "ui", "ua", "us", "ud", "uf", "ug", "ug", "uj", "uk", "ul",
    "hu", "ha", "ho", "hy", "hu", "he", "ho", "hv", "he", "hd", "hf", "hf", "ht", "hi", "hj", "hk", "hl", "hr", "hs", "hz", "hw", "hh", "hu", "hp", 
    "su", "sa", "so", "sy", "su", "se", "so", "sv", "se", "sd", "sf", "sf", "st", "si", "sj", "sk", "sl", "sr", "ss", "sz", "sw", "ss", "su", "sp",
    "tu", "ta", "to", "ty", "tu", "te", "to", "tv", "te", "td", "tf", "tf", "th", "ti", "tj", "tk", "tl", "tr", "ts", "tz", "tw", "tt", "tu", "tp",
    "zu", "za", "zo", "zy", "zu", "ze", "zo", "zv", "ze", "zd", "zf", "zf", "zt", "zi", "zj", "zk", "zl", "zr", "zs", "zz", "zw", "zz", "zu", "zp"
];

#[doc = r"Random unit value in range -1.0..1.0."]
pub fn random_unit() -> f32 {
    return rand::gen_range(-1.0, 1.0);
}

#[doc = r"Random unit value in range 0.0..1.0."]
pub fn random_unit_unsigned() -> f32 {
    return rand::gen_range(0.0, 1.0);
}

#[doc = r"Random position vector2d in range between 0.0..max_value."]
pub fn random_position(x_max: f32, y_max: f32) -> Vec2 {
    let x = rand::gen_range(0.0, x_max);
    let y = rand::gen_range(0.0, y_max);
    return Vec2::new(x, y);
}

#[doc = r"Random rotation angle in radians ranged between 0.0..2.0*PI"]
pub fn random_rotation() -> f32 {
    let rot = rand::gen_range(0.0, PI * 2.0);
    return rot;
}

#[doc = r"Random unit vector2d with value in range -1.0..1.0."]
pub fn random_unit_vec2() -> Vec2 {
    let x = rand::gen_range(-1.0, 1.0);
    let y = rand::gen_range(-1.0, 1.0);
    return Vec2::new(x, y).normalize_or_zero();
}

pub fn dt() -> f32 {
    if get_settings().pause {
        return  0.0;
    }
    return  get_frame_time();
}

pub fn dt_force() -> f32 {
    return get_frame_time();
}

pub fn random_color() -> color::Color {
    let colors = vec![
        LIGHTGRAY, GRAY, DARKGRAY, YELLOW, GOLD, ORANGE, PINK, RED, 
        MAROON, GREEN, LIME, DARKGREEN, SKYBLUE, BLUE, DARKBLUE, PURPLE, 
        VIOLET, DARKPURPLE, BEIGE, BROWN, DARKBROWN, WHITE, MAGENTA
    ];
    //let colors = vec![RED, GREEN, BLUE, YELLOW, ORANGE, GRAY, SKYBLUE, LIME, ];
    let num = colors.len();
    let c = rand::gen_range(0, num-1);
    return colors[c];
}

pub fn random_color5() -> color::Color {
    let colors = [RED, BLUE, GREEN, YELLOW, WHITE];
    let c = rand::gen_range(0, 4);
    return colors[c];
}

pub fn angle2vec2(angle: f32) -> Vec2 {
    let (x, y) = angle.sin_cos();
    let mut v = Vec2::new(x, y).normalize_or_zero();
    return v;
}

pub fn wrap_around(v: &Vec2) -> Vec2 {
    let settings = get_settings();
    let world_w = settings.world_w as f32;
    let world_h = settings.world_h as f32;
    let tolerance = 0.0;
    let mut vr = Vec2::new(v.x, v.y);
    if vr.x > world_w + tolerance {
        vr.x = world_w - tolerance
    } else if vr.x < 0.0 - tolerance {
        vr.x = 0.0 + tolerance;
    }
    if vr.y > world_h + tolerance {
        vr.y = world_h - tolerance;
    } else if vr.y < 0.0 - tolerance {
        vr.y = 0.0 + tolerance;
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
        let deviation = rand::gen_range(-dev, dev);
        let radius = r + r * deviation;
        let v = Vec2::new(x, y)*radius;
        points.push(v);
    }
    return points;
}

pub fn vec2_to_point2(v: &Vec2) -> Point2<f32> {
    return Point2::new(v.x, v.y);
}

pub fn vec2_to_point2_collection(vec2_list: &Vec<Vec2>) -> Vec<Point2<f32>> {
    let mut points: Vec<Point2<f32>> = vec![];
    for v in vec2_list.iter() {
        let p = Point2::new(v.x, v.y);
        points.push(p);
    }
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

pub fn make_regular_poly(n: usize, r: f32, dev: Option<f32>) -> Vec<Vec2> {
    let s = 2.0*PI/n as f32;
    let mut verts: Vec<Vec2> = vec![];
    for i in 0..n {
        let d = match dev {
            Some(deviation) => rand::gen_range(-deviation, deviation),
            None => 0.0,
        };

        let a = s * i as f32;
        let x = a.cos();
        let y = a.sin();
        let v = Vec2::new(x, y)*r + r*d;
        verts.push(v);
    }
    return verts;
}

pub fn make_regular_poly_indices(n: usize, r: f32) -> (Vec<Vec2>, Vec<[u32; DIM]>) {
    let s = 2.0*PI/n as f32;
    let mut verts: Vec<Vec2> = vec![];
    let mut indices: Vec<[u32; DIM]> = vec![];
    for i in 0..n {
        let a = s * i as f32;
        let x = a.cos()*r;
        let y = a.sin()*r;
        let v = Vec2::new(x, y);
        if i == 0 {
            indices.push([(n-1) as u32, i as u32]);
        } else {
            indices.push([(i-1) as u32, i as u32]);
        }
        verts.push(v);
    }
    return (verts, indices);
}

pub fn vec2_to_uivec2(vec2: &Vec2) -> egui_macroquad::egui::Vec2 {
    return egui_macroquad::egui::Vec2::new(vec2.x, vec2.y);
}
pub fn generate_seed() -> u64 {
    let t = SystemTime::now();
    let s = t.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let s2 = s / 1000000;
    return s%s2;
}

pub fn create_name(num: usize) -> String {
    let names_list: Vec<&str> = vec![
        "am","af", "ax", "ar", "av", "al", "aq", "ak", "ar", "at",
        "cu", "ca", "co", "cy", "cu", "ce", "co", "cv", "ce", "cd", "cf", "cf", "ct", "ci", "cj", "ck", "cl", "cr", "cs", "cz", "cw", "cm", "cu", "cp",
        "mu", "ma", "mo", "my", "mu", "me", "mo", "mv", "me", "md", "mf", "mf", "mt", "mi", "mj", "mk", "ml", "mr", "ms", "mz", "mw", "mm", "mu", "mp",
        "ju", "ja", "jo", "jy", "ju", "je", "jo", "jv", "je", "jd", "jf", "jf", "jt", "ji", "jj", "jk", "jl", "jr", "js", "jz", "jw", "jj", "ju", "jp",
        "du", "da", "do", "dy", "du", "de", "do", "dv", "de", "dd", "df", "df", "dt", "di", "dj", "dk", "dl", "dr", "ds", "dz", "dw", "dd", "du", "dp",
        "so", "su", "sa", "si", "se", "sy", "sl", "sj", "ss", "sk", "sr", "st", "sq", "sf", "sn",
        "nu", "na", "no", "ny", "nu", "ne", "no", "nv", "ne", "nd", "nf", "nf", "nt", "ni", "nj", "nk", "nl", "nr", "ns", "nz", "nw", "nn", "nu", "np",
        "vu", "va", "vo", "vy", "vu", "ve", "vo", "vv", "ve", "vd", "vf", "vf", "vt", "vi", "vj", "vk", "vl", "vr", "vs", "vz", "vw", "vv", "vu", "vp",
        "xu", "xa", "xo", "xy", "xu", "xe", "xo", "xv", "xe", "xd", "xf", "xf", "xt", "xi", "xj", "xk", "xl", "xr", "xs", "xz", "xw", "xx", "xu", "xp",
        "pu", "pa", "po", "py", "pu", "pe", "po", "pv", "pe", "pd", "pf", "pf", "pj", "pi", "pj", "pk", "pl", "pr", "ps", "pz", "pw", "pp", "pu", "pt",
        "lu", "la", "lo", "ly", "lu", "le", "lo", "lv", "le", "ld", "lf", "lf", "lt", "li", "lj", "lk", "ll", "lr", "ls", "lz", "lw", "ll", "lu", "lp", 
        "ku", "ka", "ko", "ky", "ku", "ke", "ko", "kv", "ke", "kd", "kf", "kf", "kt", "ki", "kj", "kk", "kl", "kr", "ks", "kz", "kw", "kk", "ku", "kp",
        "ru", "ra", "ro", "ry", "ru", "re", "ro", "rv", "re", "rd", "rf", "rf", "rt", "ri", "rj", "rk", "rl", "rr", "rs", "rz", "rw", "rr", "ru", "rp",
        "fu", "fa", "fo", "fy", "fu", "fe", "fo", "fv", "fe", "fd", "ff", "ff", "ft", "fi", "fj", "fk", "fl", "fr", "fs", "fz", "fw", "ff", "fu", "fp", 
        "ol", "oi", "oj", "od", "os", "ot", "ok", "on", "om", "oc", "ox", "oz", "op",
        "iu", "ia", "io", "iy", "iu", "ie", "io", "iv", "ie", "id", "if", "if", "it", "ii", "ij", "ik", "il", "ir", "is", "iz", "iw", "ii", "iu", "ip",
        "wu", "wa", "wo", "wy", "wu", "we", "wo", "wv", "we", "wd", "wf", "wf", "wt", "wi", "wj", "wk", "wl", "wr", "ws", "wz", "ww", "ww", "wu", "wp",
        "bu", "ba", "bo", "by", "bu", "be", "bo", "bv", "be", "bd", "bf", "bf", "bt", "bi", "bj", "bk", "bl", "br", "bs", "bz", "bw", "bb", "bu", "bp",
        "qu", "qa", "qo", "qy", "qu", "qe", "qo", "qv", "qe", "qd", "qf", "qf", "qt", "qi", "qj", "qk", "ql", "qr", "qs", "qz", "qw", "qq", "qu", "qp", 
        "uo", "ui", "ua", "us", "ud", "uf", "ug", "ug", "uj", "uk", "ul",
        "hu", "ha", "ho", "hy", "hu", "he", "ho", "hv", "he", "hd", "hf", "hf", "ht", "hi", "hj", "hk", "hl", "hr", "hs", "hz", "hw", "hh", "hu", "hp", 
        "su", "sa", "so", "sy", "su", "se", "so", "sv", "se", "sd", "sf", "sf", "st", "si", "sj", "sk", "sl", "sr", "ss", "sz", "sw", "ss", "su", "sp",
        "tu", "ta", "to", "ty", "tu", "te", "to", "tv", "te", "td", "tf", "tf", "th", "ti", "tj", "tk", "tl", "tr", "ts", "tz", "tw", "tt", "tu", "tp",
        "zu", "za", "zo", "zy", "zu", "ze", "zo", "zv", "ze", "zd", "zf", "zf", "zt", "zi", "zj", "zk", "zl", "zr", "zs", "zz", "zw", "zz", "zu", "zp"
    ];
    let mut name = String::new();
    let size = names_list.len();
    for locus in 0..num {
        let i = rand::gen_range(0, size-1);
        let voice = names_list.get(i).unwrap();
        name.insert_str(locus*2, voice);
    }
    return name;
}

pub fn vec2_to_pos2(vec2: &egui_macroquad::egui::Vec2) -> Pos2 {
    return Pos2 { x: vec2.x, y: vec2.y };
}
pub fn vec2_to_ivec2(vec2: &egui_macroquad::egui::Vec2) -> Vec2 {
    return Vec2 { x: vec2.x, y: vec2.y };
}

pub fn ivec2_to_pos2(vec2: IVec2) -> Pos2 {
    return Pos2 { x: vec2.x as f32, y: vec2.y as f32};
}

pub fn color_to_color32(color: Color) -> Color32 {
    let r = (color.r*255.0) as u8;
    let g = (color.g*255.0) as u8;
    let b = (color.b*255.0) as u8;
    let af32 = clamp((color.a*255.0).round(), 0.0, 255.0);
    let a = af32 as u8;
    return Color32::from_rgba_unmultiplied(r, g, b, a);

}

pub fn iso_to_vec2_rot(isometry: &Isometry<Real>) -> (Vec2, f32) {
    let pos = Vec2::new(isometry.translation.x, isometry.translation.y);
    let rot = isometry.rotation.angle() + PI;
    return (pos, rot);
}

pub struct SimState {
    pub ver: String,
    pub agents_num: i32,
    pub sources_num: i32,
    pub plants_num: i32,
    pub points: Vec<[f64; 2]>, 
    pub physics_num: i32,
    pub rigid_num: usize,
    pub colliders_num: usize,
    pub total_mass: f32,
    pub total_eng: f32,
    pub sim_time: f64,
    pub fps: i32,
    pub dt: f32,
    pub total_kin_eng: f32,
    pub contacts_info: (i32, i32),
    pub lifetime: Vec<[f64; 2]>,
    pub sizes: Vec<[f64; 2]>,
    pub powers: Vec<[f64; 2]>,
    pub speeds: Vec<[f64; 2]>,
    pub eyes: Vec<[f64; 2]>,
    pub shells: Vec<[f64; 2]>,
    pub mutations: Vec<[f64; 2]>,
    pub update_terrain: bool,
    pub mod_spec: i32,
}

impl SimState {

    pub fn new() -> Self {
        Self {
            ver: String::from(env!("CARGO_PKG_VERSION")),
            agents_num: 0,
            sources_num: 0,
            plants_num: 0,
            //agents_num: 0,
            physics_num: 0,
            total_mass: 0.0,
            total_eng: 0.0,
            sim_time: 0.0,
            fps: 0,
            dt: 0.0,
            total_kin_eng: 0.0,
            contacts_info: (0, 0),
            rigid_num: 0,
            colliders_num: 0,
            lifetime: vec![],
            sizes: vec![],
            powers: vec![],
            speeds: vec![],
            eyes: vec![],
            shells: vec![],
            mutations: vec![],
            //stats: Statistics::new(limit),
            points: vec![],
            update_terrain: false,
            mod_spec: 0,
        }
    }

    /* pub fn get_statistics(&self) ->  &Statistics {
        return &self.stats;
    } */

}


pub struct MouseState {
    pub pos: Vec2,
}


pub struct MyIcon {
    pub small: [u8; 16*16*4],
    pub medium: [u8; 32*32*4],
    pub big: [u8; 64*64*4],
}

impl MyIcon {
    pub fn color_filled(color: Color) -> Self {
        let r = (color.r * 255.) as u8;
        let g = (color.g * 255.) as u8;
        let b = (color.b * 255.) as u8;
        let a = (color.a * 255.) as u8;
        let small = [r, g, b, a].repeat(16*16);
        let medium = [r, g, b, a].repeat(32*32);
        let big = [r, g, b, a].repeat(64*64);
        let mut s: [u8; 16*16*4] = [0; 16*16*4]; 
        let mut m: [u8; 32*32*4] = [0; 32*32*4]; 
        let mut l: [u8; 64*64*4] = [0; 64*64*4]; 
        for i in 0..s.len() {
            s[i] = small[i];
        }
        for i in 0..m.len() {
            m[i] = medium[i];
        }
        for i in 0..l.len() {
            l[i] = big[i];
        }
        Self {
            small: s,
            medium: m,
            big: l,
        }
    }
}

pub fn saved_sim_to_sketch(path: &Path) -> Option<SimulationSketch> {
    let sim = match fs::read_to_string(path) {
        Err(_) => { None },
        Ok(save) => {
            match serde_json::from_str::<SimulationSketch>(&save) {
                Err(_) => {
                    println!("error during deserialization of saved sim...");
                    return None;
                },
                Ok(sim_state) => {
                    Some(sim_state)
                },
            }
        },
    };
    return sim;
}

pub fn saved_agent_to_agent_sketch(file_name: &str) -> Option<AgentSketch> {
    let path_str = format!("saves/agents/{}.json", file_name);
    let path = Path::new(&path_str);
    let agent = match fs::read_to_string(path) {
        Err(_) => { None },
        Ok(save) => {
            match serde_json::from_str::<AgentSketch>(&save) {
                Err(_) => {
                    println!("error during deserialization of agent...");
                    return None;
                },
                Ok(agent_state) => {
                    Some(agent_state)
                },
            }
        },
    };
    return agent;
}

pub fn draw_smooth_circle(r: f32, center: Vec2, detail: f32, width: f32, color: Color) {
    let o = PI * r * 2.0;
    let s = o / detail;
    let a = 2.0 * PI / s;
    let mut angle = 0.0;
    while angle <= 2.0*PI {
        let p0 = center + Vec2::from_angle(angle) * r;
        angle += a;
        let p1 = center + Vec2::from_angle(angle) * r;
        draw_line(p0.x, p0.y, p1.x, p1.y, width, color);
    }
    let p0 = center + Vec2::from_angle(angle) * r;
    let p1 = center + Vec2::from_angle(0.0) * r;
    draw_line(p0.x, p0.y, p1.x, p1.y, width, color);
}

pub fn draw_smooth_arc(r: f32, center: Vec2, rotation: f32, half_angle: f32, detail: f32, width: f32, color: Color) {
    let rel_peri = (2.0*half_angle) / (2.0*PI);
    let o = PI * r * 2.0 * rel_peri;
    let s = o / (detail*rel_peri);
    let a = 2.0 * half_angle / s;
    let mut angle = rotation - half_angle;
    while angle + a <= rotation + half_angle {
        let p0 = center + Vec2::from_angle(angle) * r;
        angle += a;
        let p1 = center + Vec2::from_angle(angle) * r;
        draw_line(p0.x, p0.y, p1.x, p1.y, width, color);
    }
    angle = clamp(angle, rotation - half_angle, rotation + half_angle);
    let p0 = center + Vec2::from_angle(angle) * r;
    let p1 = center + Vec2::from_angle(rotation + half_angle) * r;
    draw_line(p0.x, p0.y, p1.x, p1.y, width, color);
}

pub fn specie_mod_rate(factor: i32, ancestor_num: i32) -> i32 {
    return (((factor * ancestor_num) as f32).log2() * 500.0) as i32 + 500;
}