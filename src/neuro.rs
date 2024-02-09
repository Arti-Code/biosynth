//#![allow(unused)]

use macroquad::prelude::*;
use macroquad::rand::*;
use std::collections::HashMap;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use crate::statistics::*;
use crate::util::*;
use crate::settings::*;


pub trait Neural {
    fn get_links_t0_draw(&self) -> HashMap<u64, (Vec2, Vec2, Color, Color)>;
    fn get_nodes_t0_draw(&self) -> HashMap<u64, (Vec2, Color, Color)>;
    fn new_random(&mut self, node_num: usize, link_rate: f32);
    fn get_random_io_keys(&self, n: usize) -> Vec<u64>;
    fn send_input(&mut self, inputs: Vec<(u64, f32)>);
    fn recv_output(&self) -> Vec<(u64, f32)>;
    fn analize(&mut self);
}

pub fn generate_id() -> u64 {
    return rand::gen_range(u64::MIN, u64::MAX);
}

#[derive(Clone, Copy, Serialize, Deserialize)]
struct NeuroMargins {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

impl Debug for NeuroMargins {
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
    pub pos: IVec2,
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

#[derive(Clone)]
pub struct Network {
    pub nodes: HashMap<u64, Node>,
    pub links: HashMap<u64, Link>,
    margins: NeuroMargins,
    pub input_keys: Vec<u64>,
    pub output_keys: Vec<u64>,
    //duration: f32
}

impl Node {

    pub fn new(position: IVec2, neuron_type: NeuronTypes, label: &str) -> Self {
        Self {
            id: generate_id(),
            pos: position,
            bias: rand::gen_range(-1.0, 1.0),
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

    pub fn get_sketch(&self) -> NodeSketch {
        NodeSketch { id: self.id, pos: MyPos2 { x: self.pos.x as f32, y: self.pos.y as f32 }, bias: self.bias, node_type: self.node_type.to_owned(), label: self.label.to_owned() }
    }

    pub fn from_sketch(sketch: NodeSketch) -> Node {
        Node { id: sketch.id, pos: sketch.pos.to_ivec2(), bias: sketch.bias, val: 0.0, sum: 0.0, selected: false, node_type: sketch.node_type, last: 0.0, active: false, label: sketch.label.to_string(), new_mut: false }
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
        let (color0, color1) = match self.val {
            n if n>0.0 => { 
                let v0 = clamp(255.0*n, 0.0, 255.0);
                let v = v0 as u8;
                let c1 = color_u8!(255, g, 0, v);
                let c0 = color_u8!(255, g, 0, 255);
                (c0, c1) 
            },
            n if n<0.0 => { 
                let v0 = clamp(255.0*n.abs(), 0.0, 255.0);
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
            w: rand::gen_range(-1.0, 1.0),
            signal: 0.0,
        }
    }

    pub fn get_coords(&self, nodes: &HashMap<u64, Node>, timer: f32) -> (Vec2, Vec2, Vec2) {
        let n0 = self.node_from;
        let n1 = self.node_to;
        let node0 = nodes.get(&n0).unwrap();
        let node1 = nodes.get(&n1).unwrap();
        let p0 = vec2(node0.pos.x as f32, node0.pos.y as f32);
        let p1 = vec2(node1.pos.x as f32, node1.pos.y as f32);
        let l = p1.distance(p0).abs();
        let dir = (p1-p0).normalize_or_zero();
        let mut pt = p0 + (l*(timer)*dir);
        if !node0.active { pt = p0 }
        return (p0, p1, pt);
    }

    pub fn get_colors(&self) -> (Color, Color) {
        let s = clamp(self.signal, -1.0, 1.0);
        let color0: Color = Color::new(0.78, 0.78, 0.78, 0.50); //LIGHTGRAY;
        let mut color1: Color = Color::new(0.15, 0.15, 0.15, 1.00); //GRAY;
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

    pub fn get_sketch(&self) -> LinkSketch {
        LinkSketch { id: self.id, w: self.w, node_from: self.node_from, node_to: self.node_to }
    }

    pub fn from_sketch(sketch: LinkSketch) -> Link {
        Link { id: sketch.id, w: sketch.w, node_from: sketch.node_from, node_to: sketch.node_to, signal: 0.0 }
    }
}


impl Network {

    pub fn new(_duration: f32) -> Self {
        Self {
            nodes: HashMap::new(),
            links: HashMap::new(),
            margins: NeuroMargins { x_min: 0.01, x_max: 0.99, y_min: 0.01, y_max: 0.99 },
            input_keys: vec![],
            output_keys: vec![],
        }
    }

    pub fn build(&mut self,input_num: usize, input_labels: Vec<&str>, hidden_num: Vec<usize>, output_num: usize, output_labels: Vec<&str>, link_rate: f32) {
        self.create_nodes2(input_num, input_labels, hidden_num, output_num, output_labels);
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

    pub fn get_nodes_links_number(&self) -> (i32, i32) {
        return (self.nodes.len() as i32, self.links.len() as i32);
    }

    fn create_nodes2(&mut self, input: usize, input_labels: Vec<&str>, hidden: Vec<usize>, output: usize, output_labels: Vec<&str>) {
        let deep_n = hidden.len()+1;
        let hi = 100.0 / (input+1) as f32;
        let ho = 100.0 / (output+1) as f32;
        let wd = (100/deep_n) as i32;
        for i in 0..input {
            let node = Node::new(IVec2::new(0, (hi+hi*i as f32) as i32), NeuronTypes::INPUT, input_labels[i]);
            let id = node.id;
            self.nodes.insert(id, node);
        }
        
        for deep in 0..hidden.len() {
            let hd = 100.0 / (hidden[deep]+1) as f32;
            for d in 0..hidden[deep] {
                let node = Node::new(IVec2::new(wd*(deep as i32+1), (hd+hd*d as f32) as i32), NeuronTypes::DEEP, "");
                let id = node.id;
                self.nodes.insert(id, node);
            }
        }

        for o in 0..output {
            let node = Node::new(IVec2::new(100, (ho+ho*o as f32) as i32), NeuronTypes::OUTPUT, output_labels[o]);
            let id = node.id;
            self.nodes.insert(id, node);
        }
    }
    
    fn create_links(&mut self, links: f32) {
        let nodes_id2: Vec<u64> = self.nodes.keys().copied().collect();
        let nodes_id1: Vec<u64> = self.nodes.keys().copied().collect();
        for id in nodes_id1.iter().copied() {
            for id2 in nodes_id2.iter().copied() {
                match self.nodes.get(&id2).unwrap().node_type {
                    NeuronTypes::INPUT => { continue; },
                    NeuronTypes::DEEP => {
                        match self.nodes.get(&id).unwrap().node_type {
                            NeuronTypes::OUTPUT => { continue; },
                            NeuronTypes::DEEP => {
                                let x1 = self.nodes.get(&id).unwrap().pos.x;
                                let x2 = self.nodes.get(&id2).unwrap().pos.x;
                                if x2 > x1 {
                                    if rand::gen_range(0.0, 1.0) <= links {
                                        self.add_link(id, id2)
                                    }
                                }
                            },
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
                    },
                }  
            }
        }
    }
        
    pub fn add_link(&mut self, node_from: u64, node_to: u64) {
        let link = Link::new(node_from, node_to);
        self.links.insert(link.id, link);
    }

    pub fn calc(&mut self) {
        for (_id, link) in self.links.iter_mut() {
            link.calc(&mut self.nodes);
        }

        for (_, node) in self.nodes.iter_mut() {
            node.calc();
        }
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
        self.links.retain(|_, v| {
            if v.node_from == id || v.node_to == id {
                return false;
            } else {
                return true;
            }
        });
        self.nodes.remove(&id);
    }

    pub fn replicate(&self) -> Self {
        let mut nodes_map: HashMap<u64, Node> = HashMap::new();
        nodes_map.clone_from(&self.nodes);
        let mut links_map: HashMap<u64, Link> = HashMap::new();
        links_map.clone_from(&self.links);
        Self {
            nodes: nodes_map,
            links: links_map,
            margins: NeuroMargins { x_min: 25.0, x_max: 25.0, y_min: 375.0, y_max: 375.0 },
            input_keys: self.input_keys.to_owned(),
            output_keys: self.output_keys.to_owned(),
        }
    }

    pub fn get_sketch(&self) -> NetworkSketch {
        let mut nodes_sketch: HashMap<u64, NodeSketch> = HashMap::new();
        let mut links_sketch: HashMap<u64, LinkSketch> = HashMap::new();

        for (_, node) in self.nodes.iter() {
            let n = node.get_sketch();
            nodes_sketch.insert(n.id, n);
        }

        for (_, link) in self.links.iter() {
            let l = link.get_sketch();
            links_sketch.insert(l.id, l);
        }

        NetworkSketch { 
            nodes: nodes_sketch, 
            links: links_sketch,
            margins: self.margins.to_owned() 
        }
    }

    pub fn mutate(&mut self, m: f32) {
        let settings = get_settings();
        let mut_node_add = settings.mut_add_node + settings.mut_add_node*m;
        let mut_node_del = settings.mut_del_node + settings.mut_del_node*m;
        let mut_link_add = settings.mut_add_link + settings.mut_add_link*m;
        let mut_link_del = settings.mut_del_link + settings.mut_del_link*m;
        let mut_change_val = settings.mut_change_val + settings.mut_change_val*m;
        self.delete_random_link(mut_link_del);
        let al = self.add_random_link(mut_link_add);
        let w = self.mutate_link_weight(mut_change_val);
        let (an, dn, al2, dl, b) = self.mutate_nodes(mut_node_add, mut_node_del, mut_change_val);
        let mut stats = get_mutations();
        stats.add_values(an as i32, dn as i32, (al+al2) as i32, dl as i32, b as i32, w as i32);
        set_mutations(stats);
    }

    fn mutate_nodes(&mut self, mut_add: f32, mut_del: f32, mut_mod: f32) -> (usize, usize, usize, usize, usize) {
        let (dn, dl) = self.del_random_node(mut_del);
        let (an, al) = self.add_random_node(mut_add);
        let b = self.mutate_nodes_bias(mut_mod);
        return (an, dn, al, dl, b);
    }

    fn mutate_nodes_bias(&mut self, mutation_rate: f32) -> usize{
        let mut counter = 0;
        let node_keys: Vec<u64> = self.nodes.keys().copied().collect();
        if random_unit_unsigned() < mutation_rate {
            let k = *node_keys.choose().unwrap();
            let node = self.nodes.get_mut(&k).unwrap();
            node.bias = node.bias + rand::gen_range(-1.0, 1.0) * 0.2;
            node.bias = clamp(node.bias, -1.0, 1.0);
            counter += 1;
        }
        return counter;
    }

    fn mutate_link_weight(&mut self, mutation_rate: f32) -> usize {
        let mut counter = 0;
        let link_keys: Vec<u64> = self.links.keys().copied().collect();
        if random_unit_unsigned() < mutation_rate {
            let k = *link_keys.choose().unwrap();
            let link = self.links.get_mut(&k).unwrap();
            link.w = link.w + rand::gen_range(-1.0, 1.0)*0.2;
            link.w = clamp(link.w, -1.0, 1.0);
            counter += 1;
        }
        return counter;
    }

    fn add_random_node(&mut self, mutation_rate: f32) -> (usize, usize) {
        if random_unit_unsigned() <= mutation_rate {
            let link_keys: Vec<u64> = self.links.keys().copied().collect();
            let num = link_keys.len();
            let rand_key = rand::gen_range(0, num);
            let link_key = link_keys[rand_key];
            let link = self.links.get_mut(&link_key).unwrap();
            let n0 = link.node_from;
            let n1 = link.node_to;
            let node0 = self.nodes.get(&n0).unwrap();
            let pos0 = node0.pos;
            let node1 = self.nodes.get(&n1).unwrap();
            let pos1 = node1.pos;
            let posx = pos0 + (pos1-pos0)/2;
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
        let mut nodes_to_del: Vec<u64> = vec![]; 
        let mut links_to_del: Vec<u64> = vec![]; 
        if random_unit_unsigned() < mutation_rate {
            let (_, deep_keys, _) = self.get_node_keys_by_type();
            if !deep_keys.is_empty() {
                match deep_keys.choose() {
                    Some(k) => {
                        match self.nodes.get(k) {
                            None => {
                                println!("No node found with key {}", k);
                            },
                            Some(n) => {
                                match n.node_type {
                                    NeuronTypes::DEEP => {
                                        let node_key = *k;
                                        nodes_to_del.push(node_key);
                                        let (links_from, links_to) = self.find_connected_links(node_key);
                                        for key in links_from.iter() {
                                            if links_to_del.contains(key) { continue; }
                                            links_to_del.push(*key);
                                        }
                                        for key in links_to.iter() {
                                            if links_to_del.contains(key) { continue; }
                                            links_to_del.push(*key);
                                        }
                                    },
                                    _ => {},
                                }
                            },
                        }
                    },
                    None => {
                        println!("No deep node keys found. (neuro.rs: 'match deep_keys.choose()')");
                    },
                }
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
        if random_unit_unsigned() <= mutation_rate {
            let r0 = rand::gen_range(0, num);
            let r1 = rand::gen_range(0, num);
            if r0 == r1 { return counter; }
            let n0 = node_keys[r0];
            let n1 = node_keys[r1];
            let node0 = self.nodes.get(&n0).unwrap();
            let node1 = self.nodes.get(&n1).unwrap();
            match node0.node_type {
                NeuronTypes::INPUT => {
                    match node1.node_type {
                        NeuronTypes::INPUT => { return counter; },
                        _ => {
                            self.add_link(n0, n1);
                            counter += 1;
                        },
                    }
                },
                NeuronTypes::DEEP => {
                    match node1.node_type {
                        NeuronTypes::OUTPUT => {
                            self.add_link(n0, n1);
                            counter += 1;
                        },
                        NeuronTypes::DEEP => {
                            let x0 = node0.pos.x;
                            let x1 = node1.pos.x;
                            if x1 > x0 {
                                self.add_link(n0, n1);
                                counter += 1;
                            }
                        },
                        _ => { return counter; }
                    }
                },
                NeuronTypes::OUTPUT => { return counter; },
                NeuronTypes::ANY => { return counter; },
            }
        }
        return counter;
    }

    fn delete_random_link(&mut self, mutation_rate: f32) -> usize {
        let mut counter: usize = 0;
        if random_unit_unsigned() < mutation_rate {
            let link_keys: Vec<u64> = self.links.keys().copied().collect();
            let link_key = link_keys.choose().unwrap();
            //let num = link_keys.len();
            //let r = rand::gen_range(0, num);
            //let link_key = link_keys[r];
            self.links.remove(link_key);
            counter += 1;
        }
        return counter;
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

    pub fn to_ivec2(&self) -> IVec2 {
        return IVec2::new(self.x as i32, self.y as i32);
    }

    pub fn from_vec(vec2: &Vec2) -> Self {
        Self {
            x: vec2.x,
            y: vec2.y,
        }
    }

/*     pub fn from_ivec(ivec2: &IVec2) -> Self {
        Self {
            x: ivec2.x as f32,
            y: ivec2.y as f32,
        }
    } */
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
    margins: NeuroMargins,
}

impl NetworkSketch {
    
    pub fn from_sketch(&self) -> Network {
        let mut nodes: HashMap<u64, Node> = HashMap::new();
        let mut links: HashMap<u64, Link> = HashMap::new();
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
            margins: self.margins.to_owned(), 
            input_keys: vec![], 
            output_keys: vec![], 
        };

        let (mut i, _, mut o) = net.get_node_keys_by_type();
        net.input_keys.append(&mut i);
        net.output_keys.append(&mut o);
        return net;
    }

}