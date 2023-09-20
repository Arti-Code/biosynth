#![allow(unused)]

use macroquad::prelude::*;
use macroquad::rand::*;
use serde::ser::SerializeStruct;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use serde_json::{self, *};
use std::fs;


fn rand_position(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Vec2 {
    let x = rand::gen_range(x_min as i32, x_max as i32);
    let y = rand::gen_range(y_min as i32, y_max as i32);
    return Vec2::new(x as f32, y as f32);
}

fn rand_position_rel() -> Vec2 {
    let mut x: f32 = rand::gen_range(0.0, 1.0);
    let mut y: f32 = rand::gen_range(0.0, 1.0);
    x = (x*100.0).round()/100.0;
    y = (y*100.0).round()/100.0;
    return Vec2::new(x, y);
}

fn generate_id() -> u64 {
    return rand::gen_range(u64::MIN, u64::MAX);
}


#[derive(Clone, Copy)]
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

/* impl Serialize for NeuronTypes {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut s = serializer.serialize_struct("NeuronTypes", 1)?;
        let v = match self {
            Self::ANY => "any",
            Self::DEEP => "deep",
            Self::INPUT => "input",
            Self::OUTPUT => "output",
        };
        s.serialize_field("type", v);
        s.end()
    }
} */


/* pub struct VisualNeuron {
    pub loc1: Vec2,
    pub color1: Color,
    pub color2: Color,
}

pub struct VisualConnection {
    pub loc1: Vec2,
    pub loc2: Vec2,
    pub loc_t: Vec2,
    pub color1: Color,
    pub color2: Color,
}

pub struct NeuroVisual {
    pub neurons: Vec<VisualNeuron>,
    pub connections: Vec<VisualConnection>
}

impl NeuroVisual {
    
    pub fn new() -> Self {
        Self { neurons: vec![], connections: vec![] }
    }

    pub fn add_node(&mut self, location: Vec2, color1: Color, color2: Color) {
        let e = VisualNeuron {loc1: location, color1, color2 };
        self.neurons.push(e);
    }

    pub fn add_link(&mut self, location1: Vec2, location2: Vec2, location_timing: Vec2, color1: Color, color2: Color) {
        let e = VisualConnection {loc1: location1, loc2: location2, loc_t: location_timing, color1, color2 };
        self.connections.push(e);
    }

} */

/* pub struct DummyNetwork {
    outputs: usize,
}

impl DummyNetwork {

    pub fn new(outputs_num: usize) -> Self {
        Self {
            outputs: outputs_num,
        }
    }

    pub fn analize(&self) -> Vec<f32> {
        let mut outputs: Vec<f32> = vec![];
        for _ in 0..self.outputs {
            let out = gen_range(-1.0, 1.0);
            outputs.push(out);
        }
        return outputs;
    }
} */


#[derive(Clone, Copy)]
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
    timer: f32,
    margins: Margins,
    pub input_keys: Vec<u64>,
    pub output_keys: Vec<u64>,
    duration: f32
}

impl Node {

    pub fn new(position: Vec2, neuron_type: NeuronTypes) -> Self {
        Self {
            id: generate_id(),
            pos: (position*100.0).round()/100.0,
            //links_to: vec![],
            bias: rand::gen_range(-1.0, 1.0),
            val: rand::gen_range(0.0, 0.0),
            sum: 0.0,
            selected: false,
            node_type: neuron_type,
            last: 0.0,
            active: false,
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
        }
    }

    pub fn get_sketch(&self) -> NodeSketch {
        NodeSketch { id: self.id, pos: MyPos2 { x: self.pos.x, y: self.pos.y }, bias: self.bias, node_type: self.node_type.to_owned() }
    }

    pub fn get_colors(&self) -> (Color, Color) {
        if !self.active {
            return (LIGHTGRAY, GRAY);
        }
        let (mut color0, color1) = match self.last {
            n if n>0.0 => { 
                let v = (155.0*n) as u8;
                let c1 = color_u8!(255, 0, 0, v);
                let c0 = color_u8!(255, 0, 0, 255);
                (c0, c1) 
            },
            n if n<0.0 => { 
                let v = (255.0*n.abs()) as u8;
                let c1 = color_u8!(0, 0, 255, v);
                let c0 = color_u8!(0, 0, 255, 255);
                (c0, c1) 
            },
            _ => {
                (WHITE, WHITE)
            }
        };
        return (color0, color1);
    }

    pub fn draw(&self, t:f32) {
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
    }

/*     pub fn add_link_to(&mut self, link_id: u64) {
        self.links_to.push(link_id);
    } */

    pub fn send_impulse(&self) -> f32 {
        return self.val;
    }

    pub fn receiv_signal(&mut self, v: f32) {
        self.sum += v;
        self.active = true;
    }

    pub fn calc(&mut self) {
        let sum: f32 = self.sum + self.bias;
        let v = sum.tanh();
        self.last = self.val;
        self.val = v;
        //self.sum = 0.0;
        self.sum = 0.0;
    }

}

impl Link {
    pub fn new(node_from: u64, node_to: u64) -> Self {
        Self {
            id: generate_id(),
            node_from,
            node_to,
            w: rand::gen_range(-1.0, 1.0),
            signal: 0.0,
        }
    }

    pub fn draw(&self, nodes: &HashMap<u64, Node>, timer: f32) {
        let w = self.w;
        let s = clamp(self.signal, -1.0, 1.0);
        let (color0, color1) = self.get_colors();
        let (p0, p1, pt) = self.get_coords(nodes, timer);
        //let flow2 = l*(timer/2.0)*dir*0.96;
        draw_line(p0.x, p0.y, p1.x, p1.y, 2.0+3.0*w.abs(), color0);
        draw_line(p0.x, p0.y, pt.x, pt.y, 2.0+4.0*s.abs(), color1);
        
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
        //let w = self.w;
        let s = clamp(self.signal, -1.0, 1.0);
        let mut color0: Color = LIGHTGRAY;
        let mut color1: Color = GRAY;
        if s == 0.0 {
            return (color0, color1);
        }
        //if w >= 0.0 {
        //    color0 = color_u8!(255, 0, 0, (150.0*w) as u8);
        //}
        //if w < 0.0 {
        //    color0 = color_u8!(0, 0, 255, (150.0*w.abs()) as u8);
        //}
        if s > 0.0 {
            color1 = color_u8!(255, 0, 0, (100.0+155.0*s) as u8);
        }
        if s < 0.0 {
            color1 = color_u8!(0, 0, 255, (100.0+155.0*s.abs()) as u8);
        }
        return (color0, color1);
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
        node1.receiv_signal(v);
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
}


impl Network {

    pub fn new(duration: f32) -> Self {
        Self {
            nodes: HashMap::new(),
            links: HashMap::new(),
            timer: 0.0,
            margins: Margins { x_min: 0.01, x_max: 0.99, y_min: 0.01, y_max: 0.99 },
            input_keys: vec![],
            output_keys: vec![],
            duration,
        }
    }
    
    pub fn build(&mut self,input_num: usize, hidden_num: usize, output_num: usize, link_rate: f32) {
        self.create_nodes(input_num, hidden_num, output_num);
        self.create_links(link_rate);
        let (i, _, o) = self.get_node_keys_by_type();
        self.input_keys = i;
        self.output_keys = o;
    }

    pub fn input(&mut self, input_values: Vec<(u64, Option<f32>)>) {
        for (key, value) in input_values.iter() {
            match self.nodes.get_mut(key) {
                None => warn!("input node {} not found", key),
                Some(node) => {
                    match value {
                        Some(v) => {
                            node.sum = *v;
                            node.active = true;
                        },
                        None => {
                            node.sum = 0.0;
                        }
                    }

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

    fn create_nodes(&mut self, input: usize, hidden: usize, output: usize) {
        let hi = (self.margins.y_max / input as f32);
        let ho = (self.margins.y_max / output as f32);
        let hd = (self.margins.y_max / hidden as f32);
        let wd = (self.margins.x_max)/2.0 + self.margins.x_min;
        let h0 = self.margins.y_min;
        for i in 0..input {
            let node = Node::new(Vec2::new(self.margins.x_min, (hi/2.0+hi*i as f32)+h0), NeuronTypes::INPUT);
            let id = node.id;
            self.nodes.insert(id, node);
        }
        for d in 0..hidden {
            let node = Node::new(Vec2::new(wd, (hd/2.0+hd*d as f32)+h0), NeuronTypes::DEEP);
            //let node = Node::new(rand_position(self.margins.x_min, self.margins.x_max, self.margins.y_min, self.margins.y_max), NeuronTypes::DEEP);
            let id = node.id;
            self.nodes.insert(id, node);
        }

        for o in 0..output {
            let node = Node::new(Vec2::new(self.margins.x_max, (ho/2.0+ho*o as f32)+h0), NeuronTypes::OUTPUT);
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

    pub fn add_node(&mut self, position: Vec2) {
        let node = Node::new(position, NeuronTypes::DEEP);
        let id = node.id;
        self.nodes.insert(id, node);
        println!("[NODE CREATE] id: {}", id);
    }

    pub fn draw(&self) {
        let t = self.timer/self.duration;
        for (_, link) in self.links.iter() {
            link.draw(&self.nodes, t);
        }
        for (_, node) in self.nodes.iter() {
            node.draw(t);
        }
    }

    pub fn update(&mut self) {
        self.timer += get_frame_time();
        if self.timer > self.duration {
            self.timer -= self.duration;
            self.calc();
        }
    }

    pub fn calc(&mut self) -> Vec<(u64, f32)> {
        for (_id, link) in self.links.iter_mut() {
            link.calc(&mut self.nodes);
        }

        for (_, node) in self.nodes.iter_mut() {
            node.calc();
        }

        let mut output_values: Vec<(u64, f32)> = vec![];
        for (key, node) in self.nodes.iter() {
            match node.node_type {
                NeuronTypes::OUTPUT => {
                    match node.active {
                        true => {
                            output_values.push((*key, node.val));
                        },
                        false => {
                            output_values.push((*key, 0.0));
                        },
                    }
                },
                _ => {},
            }
        }
        return output_values;
    }

    pub fn get_outputs(&self) -> Vec<(u64, f32)>{
        let mut outputs: Vec<(u64, f32)> = vec![];
        for key in self.output_keys.iter() {
            let node = self.nodes.get(key).unwrap();
            let val = node.val;
            outputs.push((*key, val));
        }
        return outputs;
    }

    pub fn del_node(&mut self, id: u64) {
        self.links.retain(|k, v| {
            if v.node_from == id || v.node_to == id {
                println!("[LINK DEL] id: {}", k);
                return false;
            } else {
                return true;
            }
        });
        self.nodes.remove(&id);
        println!("[NODE DEL] id: {}", id);
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
            timer: 0.0,
            margins: Margins { x_min: 25.0, x_max: 25.0, y_min: 375.0, y_max: 375.0 },
            input_keys: self.input_keys.to_owned(),
            output_keys: self.output_keys.to_owned(),
            duration: self.duration,
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
            //margins: self.margins.to_owned() 
        }
    }

    pub fn mutate(&mut self, mutation_rate: f32) {
        for (id, link) in self.links.iter_mut() {
            let r = rand::gen_range(0.0, 1.0);
            if r < mutation_rate {
                link.w = rand::gen_range(-1.0, 1.0);
            }
        }

        for (id, node) in self.nodes.iter_mut() {
            let r = rand::gen_range(0.0, 1.0);
            if r < mutation_rate {
                node.bias = rand::gen_range(-1.0, 1.0);
            }
        }

        self.delete_random_link(mutation_rate/3.0);
        self.add_random_link(mutation_rate/1.0);
    }

    fn add_random_link(&mut self, mutation_rate: f32) {
        let node_keys: Vec<u64> = self.nodes.keys().copied().collect();
        let num = node_keys.len();
        loop {
            if rand::gen_range(0.0, 1.0) <= mutation_rate {
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
                                info!("NEW LINK");
                                break;
                            },
                        }
                    },
                    NeuronTypes::DEEP => {
                        match node1.node_type {
                            NeuronTypes::OUTPUT => {
                                self.add_link(n0, n1);
                                info!("NEW LINK");
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
    }

    fn delete_random_link(&mut self, mutation_rate: f32) {
        if rand::gen_range(0.0, 1.0) <= mutation_rate {
            let link_keys: Vec<u64> = self.links.keys().copied().collect();
            let num = link_keys.len();
            let r = rand::gen_range(0, num);
            let link_key = link_keys[r];
            self.links.remove(&link_key);
            warn!("DEL LINK");
        }

    }

/*     pub fn get_visual_sketch(&self) -> NeuroVisual {
        let t = self.timer/self.duration;
        let mut sketch = NeuroVisual::new();
        for (id, node) in self.nodes.iter() {
            sketch.add_node(node.pos, node.get_colors().0, node.get_colors().1);
        }
        for (id, link) in self.links.iter() {
            let pos1 = self.nodes.get(&link.node_from).unwrap().pos;
            let pos2 = self.nodes.get(&link.node_to).unwrap().pos;
            let (loc1, loc2, loc_t) = link.get_coords(&self.nodes, t);
            let (color0, color1) = link.get_colors();
            sketch.add_link(pos1, pos2, loc_t, color0, color1);
        }
        return sketch;
    } */


}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MyPos2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct NodeSketch {
    id: u64,
    pos: MyPos2,
    bias: f32,
    node_type: NeuronTypes,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct LinkSketch {
    pub id: u64,
    pub w: f32,
    pub node_from: u64,
    pub node_to: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkSketch {
    nodes: HashMap<u64, NodeSketch>,
    links: HashMap<u64, LinkSketch>,
    //margins: Margins,
}

#[test]
fn test_ser_de() {
    let mut net  = Network::new(0.5);
    net.build(4, 10, 5, 0.1);
    let sketch = net.get_sketch();
    let json_net = serde_json::to_string_pretty(&sketch);
    let s = match json_net {
        Ok(net) => {
            println!("{}", &net);
            net
        },
        Err(_) => {
            println!("Serialization Error");
            String::new()
        }
    };
    fs::write("neuro.json", &s);
    println!("{}", &s);

}

#[test]
fn u64_to_u128() {
    let mut uint64: u64 = u64::MAX;
    println!("u64: {}", uint64);
    let mut uint128: u128 = uint64 as u128;
    println!("u128: {}", uint128);
    uint64 = uint128 as u64;
    println!("u64: {}", uint64);
}