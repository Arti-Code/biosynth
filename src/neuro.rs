#![allow(unused)]

use egui_macroquad::egui::InputState;
use macroquad::prelude::*;
use macroquad::rand::*;
use rapier2d::na::ComplexField;
use serde::ser::SerializeStruct;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use serde_json::{self, *};
use std::fs;
use crate::globals::get_mutations;
use crate::globals::set_mutations;
use crate::util::*;


pub trait Neural {
    fn get_links_t0_draw(&self) -> HashMap<u64, (Vec2, Vec2, Color, Color)>;
    fn get_nodes_t0_draw(&self) -> HashMap<u64, (Vec2, Color, Color)>;
    fn new_random(&mut self, node_num: usize, link_rate: f32);
    fn get_random_io_keys(&self, n: usize) -> Vec<u64>;
    fn send_input(&mut self, inputs: Vec<(u64, f32)>);
    fn recv_output(&self) -> Vec<(u64, f32)>;
    fn analize(&mut self);
}


pub fn rand_position(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Vec2 {
    let x = rand::gen_range(x_min as i32, x_max as i32);
    let y = rand::gen_range(y_min as i32, y_max as i32);
    return Vec2::new(x as f32, y as f32);
}

pub fn rand_position_rel() -> Vec2 {
    let mut x: f32 = rand::gen_range(0.0, 1.0);
    let mut y: f32 = rand::gen_range(0.0, 1.0);
    x = (x*100.0).round()/100.0;
    y = (y*100.0).round()/100.0;
    return Vec2::new(x, y);
}

pub fn generate_id() -> u64 {
    return rand::gen_range(u64::MIN, u64::MAX);
}




#[derive(Clone, Copy, Serialize, Deserialize)]
struct Margins {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

impl Debug for Margins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.x_min).field(&self.x_max).field(&self.y_min).field(&self.y_max).finish()
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum NeuronTypes {
    INPUT,
    DEEP,
    OUTPUT,
    ANY,
}


#[derive(Clone)]
pub struct Node {
    pub id: u64,
    pub pos: Vec2,
    bias: f32,
    pub val: f32,
    sum: f32,
    pub selected: bool,
    pub node_type: NeuronTypes,
    last: f32,
    active: bool,
    pub label: String,
    new_mut: bool,
}

#[derive(Clone, Copy)]
pub struct Link {
    pub id: u64,
    pub w: f32,
    pub node_from: u64,
    pub node_to: u64,
    signal: f32,
}

//#[derive(Clone, Copy)]
pub struct Network {
    pub nodes: HashMap<u64, Node>,
    pub links: HashMap<u64, Link>,
    //timer: f32,
    margins: Margins,
    pub input_keys: Vec<u64>,
    pub output_keys: Vec<u64>,
    //duration: f32
}

impl Node {

    pub fn new(position: Vec2, neuron_type: NeuronTypes, label: &str) -> Self {
        Self {
            id: generate_id(),
            pos: (position*100.0).round()/100.0,
            //links_to: vec![],
            bias: rand::gen_range(0.0, 1.0)*rand::gen_range(0.0, 1.0),
            val: rand::gen_range(0.0, 0.0),
            sum: 0.0,
            selected: false,
            node_type: neuron_type,
            last: 0.0,
            active: false,
            label: label.to_string(),
            new_mut: false,
        }
    }

    pub fn replicate(&self) -> Self {
        Self {
            id: self.id,
            pos: self.pos.clone(),
            bias: self.bias,
            node_type: self.node_type.clone(),
            selected: false,
            sum: 0.0,
            val: 0.0,
            last: 0.0,
            active: false,
            label: self.label.to_owned(),
            new_mut: false,
        }
    }

    pub fn get_sketch(&self) -> NodeSketch {
        NodeSketch { id: self.id, pos: MyPos2 { x: self.pos.x, y: self.pos.y }, bias: self.bias, node_type: self.node_type.to_owned(), label: self.label.to_owned() }
    }

    pub fn from_sketch(sketch: NodeSketch) -> Node {
        Node { id: sketch.id, pos: sketch.pos.to_vec2(), bias: sketch.bias, val: 0.0, sum: 0.0, selected: false, node_type: sketch.node_type, last: 0.0, active: false, label: sketch.label.to_string(), new_mut: false }
    }

    pub fn get_size(&self) -> f32 {
        if !self.active {
            return 2.0;
        } else {
            return 2.0 + 6.0*self.val.abs();
        }
    }

    pub fn get_colors(&self) -> (Color, Color) {
        if !self.active {
            return (LIGHTGRAY, GRAY);
        }
        let mut g = 0;
        if self.new_mut { g = 255; }
        let (mut color0, color1) = match self.val {
            n if n>0.0 => { 
                let v0 = clamp(255.0*n, 0.0, 255.0);
                let v = v0 as u8;
                let c1 = color_u8!(255, g, 0, v);
                let c0 = color_u8!(255, g, 0, 255);
                (c0, c1) 
            },
            n if n<0.0 => { 
                let v0 = clamp((255.0*n.abs()), 0.0, 255.0);
                let v = v0 as u8;
                let c1 = color_u8!(0, g, 255, v);
                let c0 = color_u8!(0, g, 255, 255);
                (c0, c1) 
            },
            _ => {
                (WHITE, WHITE)
            }
        };
        return (color0, color1);
    }

    pub fn get_label(&self) -> String {
        return self.label.to_owned();
    }

/*     pub fn draw(&self) {
        let (color0, color) = self.get_colors();
        let r = 3.0;
        draw_circle(self.pos.x, self.pos.y, r, BLACK);
        draw_circle(self.pos.x, self.pos.y, r, color);
        draw_circle_lines(self.pos.x, self.pos.y, r, 3.0, color0);
        if self.selected {
            let mark = r + r*(PI*t).sin()*1.5;
            draw_circle_lines(self.pos.x, self.pos.y, mark, 2.0, YELLOW);
        }
        let value = format!("{}", (self.val*100.0).round()/100.0);
        draw_text(&value, self.pos.x-8.0, self.pos.y+18.0, 18.0, WHITE);
    } */

    pub fn send_impulse(&self) -> f32 {
        return self.val;
    }

    pub fn recv_signal(&mut self, v: f32) {
        if v == 0.0 { return; }
        self.sum += v;
        self.active = true;
    }

    pub fn recv_input(&mut self, v: f32) {
        self.val = v;
        if v == 0.0 { 
            self.active = false;
        } else {
            self.active = true;
        }
    }

    pub fn calc(&mut self) {
        if !self.active { 
            self.sum = 0.0;
            self.val = 0.0;
            self.last = 0.0;
            return;
        }
        let sum: f32 = self.sum + self.bias;
        let v = sum.tanh();
        self.last = self.val;
        self.val = v;
        self.sum = 0.0;
    }

}

impl Link {

    pub fn new(node_from: u64, node_to: u64) -> Self {
        Self {
            id: generate_id(),
            node_from,
            node_to,
            w: rand::gen_range(0.0, 1.0),
            signal: 0.0,
        }
    }

    pub fn draw(&self, nodes: &HashMap<u64, Node>) {
        let w = self.w;
        let s = clamp(self.signal, 0.0, 1.0);
        let (color0, color1) = self.get_colors();
        let (p0, p1, _pt) = self.get_coords(nodes, 1.0);
        //let flow2 = l*(timer/2.0)*dir*0.96;
        draw_line(p0.x, p0.y, p1.x, p1.y, 1.0+4.0*w.abs(), color1);
        //draw_line(p0.x, p0.y, pt.x, pt.y, 2.0+4.0*s.abs(), color1);
        
    }

    pub fn get_coords(&self, nodes: &HashMap<u64, Node>, timer: f32) -> (Vec2, Vec2, Vec2) {
        let n0 = self.node_from;
        let n1 = self.node_to;
        let node0 = nodes.get(&n0).unwrap();
        let p0 = node0.pos;
        let p1 = nodes.get(&n1).unwrap().pos;
        let l = p1.distance(p0).abs();
        let dir = (p1-p0).normalize_or_zero();
        let mut pt = p0 + (l*(timer)*dir);
        if !node0.active { pt = p0 }
        return (p0, p1, pt);
    }

    pub fn get_colors(&self) -> (Color, Color) {
        let s = clamp(self.signal, -1.0, 1.0);
        let mut color0: Color = LIGHTGRAY;
        let mut color1: Color = GRAY;
        if s == 0.0 {
            return (color0, color1);
        }
        if s > 0.0 {
            let mut r = 100 + (155.0 * s) as u8;
            r = clamp(r, 0, 255);
            color1 = color_u8!(r, 0, 0, (100.0+155.0*s) as u8);
        }
        if s < 0.0 {
            let mut b = 100 + (155.0 * s.abs()) as u8;
            b = clamp(b, 0, 255);
            color1 = color_u8!(0, 0, b, (100.0+155.0*s.abs()) as u8);
        }
        return (color0, color1);
    }
    
    pub fn get_width(&self) -> f32 {
        let s = clamp(self.signal.abs(), 0.0, 1.0);
        return 1.0 + s * 4.0;
    }

    pub fn calc(&mut self, nodes: &mut HashMap<u64, Node>) {
        let n0 = self.node_from;
        let n1 = self.node_to;
        let w = self.w;
        let node0 = nodes.get(&n0).unwrap();
        if !node0.active { 
            self.signal = 0.0;
            return;
        }
        let v = node0.send_impulse()*w;
        let node1 = nodes.get_mut(&n1).unwrap();
        node1.recv_signal(v);
        self.signal = v;
    
    }

    pub fn replicate(&self) -> Self {
        Self {
            id: self.id,
            node_from: self.node_from,
            node_to: self.node_to,
            signal: 0.0,
            w: self.w,
        }
    }

    pub fn get_sketch(&self) -> LinkSketch {
        LinkSketch { id: self.id, w: self.w, node_from: self.node_from, node_to: self.node_to }
    }

    pub fn from_sketch(sketch: LinkSketch) -> Link {
        Link { id: sketch.id, w: sketch.w, node_from: sketch.node_from, node_to: sketch.node_to, signal: 0.0 }
    }
}


impl Network {

    pub fn new(duration: f32) -> Self {
        Self {
            nodes: HashMap::new(),
            links: HashMap::new(),
            //timer: 0.0,
            margins: Margins { x_min: 0.01, x_max: 0.99, y_min: 0.01, y_max: 0.99 },
            input_keys: vec![],
            output_keys: vec![],
            //duration,
        }
    }

    pub fn build(&mut self,input_num: usize, input_labels: Vec<&str>, hidden_num: usize, output_num: usize, output_labels: Vec<&str>, link_rate: f32) {
        self.create_nodes(input_num, input_labels, hidden_num, output_num, output_labels);
        self.create_links(link_rate);
        let (i, _, o) = self.get_node_keys_by_type();
        self.input_keys = i;
        self.output_keys = o;
    }

    pub fn input(&mut self, input_values: Vec<(u64, f32)>) {
        for (key, value) in input_values.iter() {
            match self.nodes.get_mut(key) {
                None => warn!("input node {} not found", key),
                Some(node) => {
                    node.recv_input(*value);
                },
            }
        }
    }

    pub fn deactivate_nodes(&mut self) {
        for (_, node) in self.nodes.iter_mut() {
            node.active = false;
        }
    }

    pub fn get_node_keys_by_type(&self) -> (Vec<u64>, Vec<u64>, Vec<u64>) {
        let mut input_keys: Vec<u64> = vec![];
        let mut deep_keys: Vec<u64> = vec![];
        let mut output_keys: Vec<u64> = vec![];
        for (key, node) in self.nodes.iter() {
            match node.node_type {
                NeuronTypes::INPUT => input_keys.push(*key),
                NeuronTypes::DEEP => deep_keys.push(*key),
                NeuronTypes::OUTPUT => output_keys.push(*key),
                NeuronTypes::ANY => {},
            }
        }
        return (input_keys, deep_keys, output_keys);
    }

    pub fn get_input_pairs(&self) -> Vec<(u64, String)> {
        let mut pairs: Vec<(u64, String)> = vec![];
        let (keys, _, _) = self.get_node_keys_by_type();
        for k in keys.iter().cloned() {
            let s = self.nodes.get(&k).unwrap().label.to_owned();
            pairs.push((k, s));
        }
        return pairs;
    }

    pub fn get_output_pairs(&self) -> Vec<(u64, String)> {
        let mut pairs: Vec<(u64, String)> = vec![];
        let (_, _, keys) = self.get_node_keys_by_type();
        for k in keys.iter().cloned() {
            let s = self.nodes.get(&k).unwrap().label.to_owned();
            pairs.push((k, s));
        }
        return pairs;
    }

    fn create_nodes(&mut self, input: usize, input_labels: Vec<&str>, hidden: usize, output: usize, output_labels: Vec<&str>) {
        let hi = (self.margins.y_max / input as f32);
        let ho = (self.margins.y_max / output as f32);
        let hd = (self.margins.y_max / hidden as f32);
        let wd = (self.margins.x_max)/2.0 + self.margins.x_min;
        let h0 = self.margins.y_min;
        for i in 0..input {
            let node = Node::new(Vec2::new(self.margins.x_min, (hi/2.0+hi*i as f32)+h0), NeuronTypes::INPUT, input_labels[i]);
            let id = node.id;
            self.nodes.insert(id, node);
        }
        for d in 0..hidden {
            let node = Node::new(Vec2::new(wd, (hd/2.0+hd*d as f32)+h0), NeuronTypes::DEEP, "");
            //let node = Node::new(rand_position(self.margins.x_min, self.margins.x_max, self.margins.y_min, self.margins.y_max), NeuronTypes::DEEP);
            let id = node.id;
            self.nodes.insert(id, node);
        }

        for o in 0..output {
            let node = Node::new(Vec2::new(self.margins.x_max, (ho/2.0+ho*o as f32)+h0), NeuronTypes::OUTPUT, output_labels[o]);
            let id = node.id;
            self.nodes.insert(id, node);
        }
    }
    
    fn create_links(&mut self, links: f32) {
        //let buf_links = self.links.
        let nodes_id2: Vec<u64> = self.nodes.keys().copied().collect();
        let nodes_id1: Vec<u64> = self.nodes.keys().copied().collect();
        for id in nodes_id1.iter().copied() {
            for id2 in nodes_id2.iter().copied() {
                match self.nodes.get(&id2).unwrap().node_type {
                    NeuronTypes::INPUT => { continue; },
                    NeuronTypes::DEEP => {
                        match self.nodes.get(&id).unwrap().node_type {
                            NeuronTypes::DEEP | NeuronTypes::OUTPUT => { continue; },
                            _ => {
                                if rand::gen_range(0.0, 1.0) <= links {
                                    self.add_link(id, id2)
                                }
                            },
                        }
                    },
                    NeuronTypes::OUTPUT => {
                        match self.nodes.get(&id).unwrap().node_type {
                            NeuronTypes::OUTPUT => { continue; },
                            _ => {
                                if rand::gen_range(0.0, 1.0) <= links {
                                    self.add_link(id, id2)
                                }
                            },
                        }
                    },
                    _ => {
                        /* if id != id2 {
                            if rand::gen_range(0.0, 1.0) <= links {
                                self.add_link(id, id2)
                            }
                        } */
                    },
                }  
            }
        }
    }
        
    pub fn add_link(&mut self, node_from: u64, node_to: u64) {
        let link = Link::new(node_from, node_to);
        //let node = self.nodes.get_mut(&node_to).unwrap();
        //node.add_link_to(link.id);
        self.links.insert(link.id, link);
    }

    pub fn add_node(&mut self, position: Vec2) -> u64 {
        let node = Node::new(position, NeuronTypes::DEEP, "");
        let id = node.id;
        self.nodes.insert(id, node);
        return id;
        //println!("[NODE CREATE] id: {}", id);
    }

    pub fn add_node_with_id(&mut self, id: u64, position: Vec2) -> u64 {
        let mut node = Node::new(position, NeuronTypes::DEEP, "");
        node.id = id;
        self.nodes.insert(id, node);
        return id;
    }

/*     pub fn draw(&self) {
        //let t = self.timer/self.duration;
        for (_, link) in self.links.iter() {
            link.draw(&self.nodes);
        }
        for (_, node) in self.nodes.iter() {
            node.draw();
        }
    } */

/*     pub fn update(&mut self) {
        self.timer += get_frame_time();
        if self.timer > self.duration {
            self.timer -= self.duration;
            self.calc();
        }
    } */

    pub fn calc(&mut self) {
        for (_id, link) in self.links.iter_mut() {
            link.calc(&mut self.nodes);
        }

        for (_, node) in self.nodes.iter_mut() {
            node.calc();
        }
    }

    pub fn get_outputs(&self) -> Vec<(u64, String, f32)>{
        let mut outputs: Vec<(u64, String, f32)> = vec![];
        for key in self.output_keys.iter() {
            let node = self.nodes.get(key).unwrap();
            if node.active {
                let val = node.val;
                outputs.push((*key, node.label.to_owned(), val));
            } else {
                let val = 0.0;
                outputs.push((*key, node.label.to_owned(), val));
            }
        }
        return outputs;
    }

    pub fn get_outputs2(&self) -> HashMap<String, f32>{
        let mut outputs: HashMap<String, f32> = HashMap::new();
        for key in self.output_keys.iter() {
            let node = self.nodes.get(key).unwrap();
            if node.active {
                let val = clamp(node.val, 0.0, 1.0);
                outputs.insert(node.label.to_owned(), val);
            } else {
                let val = 0.0;
                outputs.insert(node.label.to_owned(), val);
            }
        }
        return outputs;
    }

    fn get_node(&self, node_key: &u64) -> Option<&Node> {
        return self.nodes.get(node_key);
    }

    pub fn get_node_value(&self, node_key: &u64) -> Option<f32> {
        return match self.get_node(node_key) {
            Some(node) => {
                Some(node.val)
            },
            None => None,
        };
    }

    pub fn del_node(&mut self, id: u64) {
        self.links.retain(|k, v| {
            if v.node_from == id || v.node_to == id {
                //println!("[LINK DEL] id: {}", k);
                return false;
            } else {
                return true;
            }
        });
        self.nodes.remove(&id);
        //println!("[NODE DEL] id: {}", id);
    }

    pub fn unselect(&mut self) {
        for (_, node) in self.nodes.iter_mut() {
            node.selected = false;
        }
    }

    pub fn replicate(&self) -> Self {
        let mut nodes_map: HashMap<u64, Node> = HashMap::new();
        nodes_map.clone_from(&self.nodes);
        let mut links_map: HashMap<u64, Link> = HashMap::new();
        links_map.clone_from(&self.links);
        Self {
            nodes: nodes_map,
            links: links_map,
            //timer: 0.0,
            margins: Margins { x_min: 25.0, x_max: 25.0, y_min: 375.0, y_max: 375.0 },
            input_keys: self.input_keys.to_owned(),
            output_keys: self.output_keys.to_owned(),
            //duration: self.duration,
        }
    }

    pub fn get_sketch(&self) -> NetworkSketch {
        let mut nodes_sketch: HashMap<u64, NodeSketch> = HashMap::new();
        let mut links_sketch: HashMap<u64, LinkSketch> = HashMap::new();

        for (id, node) in self.nodes.iter() {
            let n = node.get_sketch();
            nodes_sketch.insert(n.id, n);
        }

        for (id, link) in self.links.iter() {
            let l = link.get_sketch();
            links_sketch.insert(l.id, l);
        }

        NetworkSketch { 
            nodes: nodes_sketch, 
            links: links_sketch,
            //duration: self.duration, 
            margins: self.margins.to_owned() 
        }
    }

    pub fn mutate(&mut self, mutation_rate: f32) {
        
        let mut an: usize = 0; let mut dn: usize = 0; let mut al: usize = 0; let mut dl: usize = 0; let mut b: usize = 0; let mut w: usize = 0; let mut al2: usize = 0;
        //self.delete_random_link(mutation_rate/3.0);
        al = self.add_random_link(mutation_rate);
        w = self.mutate_link_weight(mutation_rate);
        (an, dn, al2, dl, b) = self.mutate_nodes(mutation_rate);
        let mut stats = get_mutations();
        stats.add_values(an as i32, dn as i32, (al+al2) as i32, dl as i32, b as i32, w as i32);
        set_mutations(stats);
        //println!("MUTATIONS: add_node: {} | del_node {} | add_link: {} | del_link: {} | b: {} | w: {}", an, dn, al+al2, dl, b, w);
    }

    fn mutate_nodes(&mut self, mutation_rate: f32) -> (usize, usize, usize, usize, usize) {
        let (dn, dl) = self.del_random_node(mutation_rate/4.0);
        let (an, al) = self.add_random_node(mutation_rate/4.0);
        let b = self.mutate_nodes_bias(mutation_rate);
        return (an, dn, al, dl, b);
    }

    fn mutate_nodes_bias(&mut self, mutation_rate: f32) -> usize{
        let mut counter = 0;
        let n_num = self.nodes.len();
        let xfactor = mutation_rate/n_num as f32;
        for (id, node) in self.nodes.iter_mut() {
            if random_unit_unsigned() < xfactor {
                node.bias = rand::gen_range(-1.0, 1.0)*rand::gen_range(-1.0, 1.0);
                counter += 1;
            }
        }
        return counter;
    }

    fn mutate_link_weight(&mut self, mutation_rate: f32) -> usize {
        let mut counter = 0;
        let l_num = self.links.len();
        if l_num == 0 { return 0; }
        let xfactor = mutation_rate / ((l_num as f32)/2.0);
        for (id, link) in self.links.iter_mut() {
            if random_unit_unsigned() < xfactor {
                link.w = rand::gen_range(-1.0, 1.0);
                counter += 1;
            }
        }
        return counter;
    }

    fn add_random_node(&mut self, mutation_rate: f32) -> (usize, usize) {
        let link_keys: Vec<u64> = self.links.keys().copied().collect();
        let num = link_keys.len();
        //let n0: u64; let n1: u64; let nx: u64; let mut link: &Link;
        if random_unit_unsigned() <= mutation_rate {
            let rand_key = rand::gen_range(0, num);
            let link_key = link_keys[rand_key];
            let link = self.links.get_mut(&link_key).unwrap();
            let n0 = link.node_from;
            let n1 = link.node_to;

            let node0 = self.nodes.get(&n0).unwrap();
            let pos0 = node0.pos;
            let node1 = self.nodes.get(&n1).unwrap();
            let pos1 = node1.pos;
            let posx = pos0 + (pos1-pos0)/2.0;
            let nx = generate_id();
            let mut new_node = Node::new(posx, NeuronTypes::DEEP, "");
            new_node.id = nx;
            new_node.new_mut = true;
            self.nodes.insert(nx, new_node);
            link.node_to = nx;
            self.add_link(nx, n1);
            return (1,1);
        }
        return (0, 0);
    }

    fn find_connected_links(&self, n_key: u64) -> (Vec<u64>, Vec<u64>) {
        let mut links_from: Vec<u64> = vec![];
        let mut links_to: Vec<u64> = vec![];
        for (l_key, _) in self.links.iter() {
            let link = self.links.get(l_key).unwrap();
            if n_key == link.node_from { links_from.push(*l_key) }
            if n_key == link.node_to { links_to.push(*l_key) }
        }
        return (links_from, links_to);
    }

    fn del_random_node(&mut self, mutation_rate: f32) -> (usize, usize) {
        let mut counter_n = 0;
        let mut counter_l = 0;
        let link_keys: Vec<u64> = self.links.keys().copied().collect();
        let node_keys: Vec<u64> = self.nodes.keys().copied().collect();
        let mut nodes_to_del: Vec<u64> = vec![]; 
        let mut links_to_del: Vec<u64> = vec![]; 
        let l_num = link_keys.len();
        let n_num = node_keys.len();
        let xfactor = mutation_rate/n_num as f32;
        for nk in node_keys.iter() {
            match self.nodes.get(nk).unwrap().node_type {
                NeuronTypes::DEEP => {
                        if random_unit_unsigned() <= xfactor {
                            let node_key = *nk;
                            nodes_to_del.push(node_key);
                            let (mut links_from, mut links_to) = self.find_connected_links(node_key);
                            for key in links_from.iter() {
                                if links_to_del.contains(key) { continue; }
                                links_to_del.push(*key);
                            }
                            for key in links_to.iter() {
                                if links_to_del.contains(key) { continue; }
                                links_to_del.push(*key);
                            }
                        }
                },
                _ => {},
            }
        }
        for l_key in links_to_del.iter() {
            self.links.remove(l_key);
            counter_l += 1;
        }
        for n_key in nodes_to_del.iter() {
            self.del_node(*n_key);
            counter_n += 1;
        }
        return (counter_n, counter_l);
    }

    fn add_random_link(&mut self, mutation_rate: f32) -> usize {
        let mut counter = 0;
        let node_keys: Vec<u64> = self.nodes.keys().copied().collect();
        let num = node_keys.len();
        
        loop {
            if random_unit_unsigned() <= mutation_rate {
                let r0 = rand::gen_range(0, num);
                let r1 = rand::gen_range(0, num);
                if r0 == r1 { continue; }
                let n0 = node_keys[r0];
                let n1 = node_keys[r1];
                let node0 = self.nodes.get(&n0).unwrap();
                let node1 = self.nodes.get(&n1).unwrap();
                match node0.node_type {
                    NeuronTypes::INPUT => {
                        match node1.node_type {
                            NeuronTypes::INPUT => { continue; },
                            _ => {
                                self.add_link(n0, n1);
                                counter += 1;
                                break;
                            },
                        }
                    },
                    NeuronTypes::DEEP => {
                        match node1.node_type {
                            NeuronTypes::OUTPUT => {
                                self.add_link(n0, n1);
                                counter += 1;
                                break;
                            },
                            _ => { continue; }
                        }
                    },
                    NeuronTypes::OUTPUT => { continue; },
                    NeuronTypes::ANY => { continue; },
                }
            } else {
                break;
            }
        }
        return counter;
    }

    fn delete_random_link(&mut self, mutation_rate: f32) {
        if rand::gen_range(0.0, 1.0) <= mutation_rate {
            let link_keys: Vec<u64> = self.links.keys().copied().collect();
            let num = link_keys.len();
            let r = rand::gen_range(0, num);
            let link_key = link_keys[r];
            self.links.remove(&link_key);
            //warn!("DEL LINK");
        }

    }


}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MyPos2 {
    pub x: f32,
    pub y: f32,
}

impl MyPos2 {
    pub fn to_vec2(&self) -> Vec2 {
        return Vec2::new(self.x, self.y);
    }

    pub fn from_vec(vec2: &Vec2) -> Self {
        Self {
            x: vec2.x,
            y: vec2.y,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeSketch {
    id: u64,
    pos: MyPos2,
    bias: f32,
    node_type: NeuronTypes,
    label: String,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct LinkSketch {
    pub id: u64,
    pub w: f32,
    pub node_from: u64,
    pub node_to: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkSketch {
    nodes: HashMap<u64, NodeSketch>,
    links: HashMap<u64, LinkSketch>,
    //duration: f32,
    margins: Margins,
}

impl NetworkSketch {
    
    pub fn from_sketch(&self) -> Network {
        let mut nodes: HashMap<u64, Node> = HashMap::new();
        let mut links: HashMap<u64, Link> = HashMap::new();
        let margins = Margins { x_min: 0.01, x_max: 0.99, y_min: 0.01, y_max: 0.99 };
        for (key, sketch_node) in self.nodes.iter() {
            let node = Node::from_sketch(sketch_node.to_owned());
            nodes.insert(*key, node);
        }

        for (key, sketch_link) in self.links.iter() {
            let link = Link::from_sketch(sketch_link.to_owned());
            links.insert(*key, link);
        }

        let mut net = Network { 
            nodes: nodes.to_owned(), 
            links: links.to_owned(), 
            //timer: random_unit().abs(), 
            margins: self.margins.to_owned(), 
            input_keys: vec![], 
            output_keys: vec![], 
            //duration: self.duration 
        };

        let (mut i, _, mut o) = net.get_node_keys_by_type();
        net.input_keys.append(&mut i);
        net.output_keys.append(&mut o);
        return net;
    }

}