use macroquad::prelude::*;
use rapier2d::pipeline::*;


pub struct MacroRapierDebugger;


impl MacroRapierDebugger {

/*     pub fn new() -> Self {
        Self
    } */

}

impl DebugRenderBackend for MacroRapierDebugger {

    fn draw_line(&mut self, _object: DebugRenderObject, a: rapier2d::prelude::Point<rapier2d::prelude::Real>, b: rapier2d::prelude::Point<rapier2d::prelude::Real>, color: [f32; 4]) {
        let v1 = vec2(a.x, a.y);
        let v2 = vec2(b.x, b.y);
        let line_color = color_u8!(
            (color[0] as u32 * 0xff) as u8,
            (color[1] as u32 * 0xff) as u8,
            (color[2] as u32 * 0xff) as u8,
            (color[3] as u32 * 0xff) as u8
        );
        draw_line(v1.x, v1.y, v2.x, v2.y, 1.0, line_color);
    }

}