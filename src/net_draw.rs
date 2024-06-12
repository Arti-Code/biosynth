use macroquad::prelude::*;

use crate::{agent::Agent, neuro::NeuronTypes, util::*}; 
use crate::globals::*;


pub fn draw_network(agent: &Agent, timer: f32, cam_offset: Vec2) {
    //let period = timer/get_settings().neuro_duration;
    let t = timer;
    let network = &agent.network;
    //let w = 340.0; 
    //let h = 400.0; 
    let resize = Vec2::new(4.0, 4.0);
    let offset = Vec2::new((SCREEN_WIDTH as f32)/2.0-520.0, (SCREEN_HEIGHT as f32)/2.0-450.0);
    let zero = Vec2::ZERO+offset+cam_offset;
    let wi = 1.0;
    for (_, link) in network.links.iter() {
        let (coord0, coord1, coord_t) = link.get_coords(&network.nodes, t);
        //let ui_coord0 = vec2_to_uivec2(&coord0);
        //let ui_coord1 = vec2_to_uivec2(&coord1);
        //let ui_coord_t = vec2_to_uivec2(&coord_t);
        let w = link.get_width()*wi;
        let p1 = coord0*resize+zero;
        let p2 = coord1*resize+zero;
        let pt = coord_t*resize+zero;
        let (_, color1) = link.get_colors();
        //let c0 = color_to_color32(color0);
        //let c1 = color_to_color32(color1);
        //let points1 = [p1, p2];
        draw_line(p1.x, p1.y, p2.x, p2.y, w, color1);
        draw_circle(pt.x, pt.y, w, YELLOW);
        //painter.line_segment(points1, Stroke { color: c1, width: w });
        //painter.circle_filled(pt, w, Color32::YELLOW);
    }
    for (key, node) in network.nodes.iter() {
        let (_, color1) = node.get_colors();
        let (r0, _) = node.get_size();
        let mut mem = node.get_mem_size();
        let mut pos: Vec2 = node.pos.as_vec2();
        pos = pos*resize+zero;
        //let p1 = vec2_to_pos2(&ipos);
        //let c0 = color_to_color32(color1);
        let label = node.get_label();
        let v = match network.get_node_value(key) {
            None => 0.0,
            Some(v) => v,
        };
        //painter.circle_filled(p1, r0,  Color32::BLACK);
        //let w1 = 0.5 + 0.35*r1;
        //painter.circle_stroke(p1, r1, Stroke { color: Color32::GREEN, width: w1 });
        let w0 = 0.25 + 0.25*r0;
        draw_circle_lines(pos.x, pos.y, w0, 1.0, color1);
        //painter.circle_stroke(p1, r0, Stroke { color: c0, width: w0 });
        if mem > 0.0 {
            mem = clamp(mem, -1.0, 1.0);
            //painter.circle_stroke(p1, 1.0+mem*5.0, Stroke { color: Color32::GREEN, width: 1.0 });
            draw_circle_lines(pos.x, pos.y, 1.0+mem*5.0, 1.0, GREEN);
        }
        //let mut font = FontId::default();
        //font.size = 8.0;
        let txt = format!("{}: {:.1}", label, v);
        match node.node_type {
            NeuronTypes::INPUT => {
                //painter.text(p1+UIVec2{x: 8.0, y: 0.0}, Align2::LEFT_CENTER, txt, font, Color32::WHITE);
                draw_text(txt.as_str(), pos.x+8.0, pos.y, 12.0, WHITE);
            },
            NeuronTypes::OUTPUT => {
                //painter.text(p1+UIVec2{x: -50.0, y: 0.0}, Align2::LEFT_CENTER, txt, font, Color32::WHITE);
                draw_text(txt.as_str(), pos.x-50.0, pos.y, 12.0, WHITE);
            },
            _ => {},
        }
    } 
}
