use macroquad::prelude::*;
use macroquad::rand::*;
use std::collections::HashMap;
use std::f32::consts::PI;


fn rand_position(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Vec2 {
    let x = rand::gen_range(x_min, x_max);
    let y = rand::gen_range(y_min, y_max);
    return Vec2::new(x, y);
}

fn generate_id() -> u64 {
    return rand::gen_range(u64::MIN, u64::MAX);
}

struct Margins {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}


pub enum NeuronTypes {
    INPUT,
    DEEP,
    OUTPUT,
    ANY,
}


pub struct DummyNetwork {
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
}



pub struct Node {
    pub id: u64,
    pub pos: Vec2,
    //pub links_to: Vec<u64>,
    bias: f32,
    pub val: f32,
    sum: f32,
    pub selected: bool,
    pub node_type: NeuronTypes,
}

pub struct Link {
    pub id: u64,
    pub w: f32,
    pub node_from: u64,
    pub node_to: u64,
    signal: f32,
}

pub struct Network {
    pub nodes: HashMap<u64, Node>,
    pub links: HashMap<u64, Link>,
    pub timer: f32,
    margins: Margins,
    input_keys: Vec<u64>,
    output_keys: Vec<u64>,
}

impl Node {

    pub fn new(position: Vec2, neuron_type: NeuronTypes) -> Self {
        Self {
            id: generate_id(),
            pos: position,
            //links_to: vec![],
            bias: rand::gen_range(-1.0, 1.0),
            val: rand::gen_range(-1.0, 1.0),
            sum: 0.0,
            selected: false,
            node_type: neuron_type,
        }
    }

    pub fn draw(&self, t:f32) {
        let (mut color0, color) = match self.val {
            n if n>0.0 => { 
                let v = (155.0*n) as u8;
                let c = color_u8!(255, 0, 0, v);
                let c0 = color_u8!(255, 0, 0, 255);
                (c0, c) 
            },
            n if n<0.0 => { 
                let v = (255.0*n.abs()) as u8;
                let c = color_u8!(0, 0, 255, v);
                let c0 = color_u8!(0, 0, 255, 255);
                (c0, c) 
            },
            _ => {
                (WHITE, WHITE)
            }
        };
        let r = 5.0;
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

    pub fn get_val(&self) -> f32 {
        return self.val;
    }

    pub fn add_to_sum(&mut self, v: f32) {
        self.sum += v;
    }

    pub fn calc(&mut self) {
        let sum: f32 = self.sum + self.bias;
        let v = sum.tanh();
        self.val = v;
        self.sum = 0.0;
        //self.sum = 0.0;
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
        let mut color0: Color = WHITE;
        let mut color1: Color = WHITE;
        if w >= 0.0 {
            color0 = color_u8!(255, 75, 75, (200.0*w) as u8);
        }
        if w < 0.0 {
            color0 = color_u8!(75, 75, 255, (200.0*w.abs()) as u8);
        }
        if s >= 0.0 {
            color1 = color_u8!(255, 0, 0, (100.0+155.0*s) as u8);
        }
        if s < 0.0 {
            color1 = color_u8!(0, 0, 255, (100.0+155.0*s.abs()) as u8);
        }
        let n0 = self.node_from;
        let n1 = self.node_to;
        let p0 = nodes.get(&n0).unwrap().pos;
        let p1 = nodes.get(&n1).unwrap().pos;
        let l = p1.distance(p0).abs();
        let dir = (p1-p0).normalize_or_zero();
        let flow1 = l*(timer/2.0)*dir;
        //let flow2 = l*(timer/2.0)*dir*0.96;
        draw_line(p0.x, p0.y, p1.x, p1.y, 2.0+3.0*w.abs(), color0);
        draw_line(p0.x, p0.y, p0.x+flow1.x, p0.y+flow1.y, 2.0+4.0*s.abs(), color1);
        
    }

    pub fn calc(&mut self, nodes: &mut HashMap<u64, Node>) {
        let n0 = self.node_from;
        let n1 = self.node_to;
        let w = self.w;
        let node0 = nodes.get(&n0).unwrap();
        let v = node0.get_val()*w;
        let node1 = nodes.get_mut(&n1).unwrap();
        node1.add_to_sum(v);
        self.signal = v;
    }
}

impl Network {

    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            links: HashMap::new(),
            timer: 0.0,
            margins: Margins { x_min: 25.0, x_max: 25.0, y_min: 375.0, y_max: 375.0 },
            input_keys: vec![],
            output_keys: vec![],
        }
    }
    
    pub fn build(&mut self,input_num: usize, hidden_num: usize, output_num: usize, link_rate: f32) {
        self.create_nodes(input_num, hidden_num, output_num);
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
                    node.sum = *value;
                },
            }
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
        let hi = self.margins.y_max / input as f32;
        let ho = self.margins.y_max / output as f32;
        for i in 0..input {
            let node = Node::new(Vec2::new(25.0, hi/2.0+hi*i as f32), NeuronTypes::INPUT);
            let id = node.id;
            self.nodes.insert(id, node);
        }
        for _ in 0..hidden {
            let node = Node::new(rand_position(self.margins.x_min, self.margins.x_max, self.margins.y_min, self.margins.y_max), NeuronTypes::DEEP);
            let id = node.id;
            self.nodes.insert(id, node);
        }

        for o in 0..output {
            let node = Node::new(Vec2::new(self.margins.x_max, ho/2.0+ho*o as f32), NeuronTypes::OUTPUT);
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
                    _ => {
                        if id != id2 {
                            if rand::gen_range(0.0, 1.0) <= links {
                                self.add_link(id, id2)
                            }
                        }
                    }
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
        for (_, link) in self.links.iter() {
            link.draw(&self.nodes, self.timer);
        }
        for (_, node) in self.nodes.iter() {
            node.draw(self.timer);
        }
    }

    pub fn update(&mut self) {
        self.timer += get_frame_time();
        if self.timer > 2.0 {
            self.timer -= 2.0;
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
                    output_values.push((*key, node.val));
                },
                _ => {},
            }
        }
        return output_values;
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
}