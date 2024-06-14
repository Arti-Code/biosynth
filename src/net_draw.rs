use macroquad::prelude::*;

use crate::{
    agent::Agent, 
    neuro::NeuronTypes, 
    util::*,
    settings::*,
}; 


pub fn draw_network(agent: &Agent, timer: f32, cam_offset: Vec2) {
    let t = timer;
    let network = &agent.network;
    let resize = Vec2::new(4.0, 4.0);
    let offset = Vec2::new((SCREEN_WIDTH as f32)/2.0-520.0, (SCREEN_HEIGHT as f32)/2.0-450.0);
    let zero = Vec2::ZERO+offset+cam_offset;
    draw_rectangle(zero.x, zero.y, resize.x*100.0, resize.y*100.0, Color::new(0.0, 0.0, 0.0, 0.65));
    let wi = 0.75;
    for (_, link) in network.links.iter() {
        let (coord0, coord1, coord_t) = link.get_coords(&network.nodes, t);
        let w = link.get_width()*wi;
        let p1 = coord0*resize+zero;
        let p2 = coord1*resize+zero;
        let pt = coord_t*resize+zero;
        let (_, color1) = link.get_colors();
        draw_line(p1.x, p1.y, p2.x, p2.y, w, color1);
        draw_circle(pt.x, pt.y, w, color1);
    }
    for (key, node) in network.nodes.iter() {
        let (_, color1) = node.get_colors();
        let (r0, _) = node.get_size();
        let mut mem = node.get_mem_size();
        let mut pos: Vec2 = node.pos.as_vec2();
        pos = pos*resize+zero;
        let label = node.get_label();
        let v = match network.get_node_value(key) {
            None => 0.0,
            Some(v) => v,
        };
        let w0 = 1.0 + 1.0*r0;
        draw_circle_lines(pos.x, pos.y, w0, 1.0, color1);
        if mem > 0.0 {
            mem = clamp(mem, -1.0, 1.0);
            draw_circle_lines(pos.x, pos.y, 1.0+mem*5.0, 1.0, GREEN);
        }
        let txt = format!("{}: {:.1}", label, v);
        match node.node_type {
            NeuronTypes::INPUT => {
                draw_text(txt.as_str(), pos.x+8.0, pos.y, 12.0, WHITE);
            },
            NeuronTypes::OUTPUT => {
                draw_text(txt.as_str(), pos.x-50.0, pos.y, 12.0, WHITE);
            },
            _ => {},
        }
    } 
}
