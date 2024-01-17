//#![allow(unused)]

use rapier2d::geometry::*;
use macroquad::{prelude::*, color};
use rapier2d::prelude::*;
use crate::timer::Timer;
use crate::util::*;
use crate::physics::*;
//use crate::globals::*;
use crate::settings::*;


#[derive(Clone, Copy)]
pub struct Resource {
    pub pos: Vec2,
    pub rot: f32,
    pub size: f32,
    pub max_eng: f32,
    pub eng: f32,
    pub color: color::Color,
    pub shape: Ball,
    pub physics_handle: RigidBodyHandle,
    pub alife: bool,
    pub time: f32,
    clone_timer: Timer,
    growth_timer: Timer,
}

impl Resource {
    pub fn new(physics: &mut Physics) -> Self {
        let settings = get_settings();
        //let key = rand::gen_range(u64::MIN, u64::MAX);
        let pos = random_position(settings.world_w as f32, settings.world_h as f32);
        //let color = random_color();
        //let size = rand::gen_range(6, 8) as f32;
        let size = 2.0;
        let shape = SharedShape::ball(size);
        let rbh = physics.add_dynamic_object(&pos, 0.0, shape.clone(), PhysicsMaterial::high_inert(), InteractionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_2));
        Self {
            pos,
            rot: 0.0,
            size,
            max_eng: size.powi(2)*10.0,
            eng: size.powi(2)*10.0,
            color: YELLOW,
            shape: Ball { radius: size },
            physics_handle: rbh,
            time: 64.0,
            alife: true,
            clone_timer: Timer::new(10.0, true, true, true),
            growth_timer: Timer::new(10.0, true, true, true),
        }
    }
    pub fn draw(&self, show_range: bool) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        draw_circle(x0, y0, self.size, self.color);
        if show_range {
            let settings = get_settings();
            draw_circle_lines(x0, y0, settings.res_detection_radius, 0.5, Color { r: 0.78, g: 0.78, b: 0.78, a: 0.25 });
        }
    }
    pub fn update(&mut self, physics: &mut Physics){
        let dt = get_frame_time();
        let settings = get_settings();
        let mut resize = false;
        self.time -= dt;
        self.eng += settings.growth * dt;
        if self.growth_timer.update(dt) {
            if self.eng >= self.size.powi(2)*10.0 {
                self.size += 1.0;
                resize = true;
                self.resize(physics);
                self.max_eng = self.size.powi(2)*10.0;
                if self.eng > self.max_eng {
                    self.eng = self.max_eng;
                }
            }
        }
        self.update_physics(physics, resize);
        self.pos = wrap_around(&self.pos);
        if self.eng <= 0.0 || self.time <= 0.0 {
            self.eng = 0.0;
            self.alife = false;
        }
    }

    fn resize(&mut self, physics: &mut Physics) {
        let c = physics.get_first_collider_mut(self.physics_handle);
        c.set_shape(SharedShape::ball(self.size));
    }

    pub fn drain_eng(&mut self, eng_loss: f32) {
        self.eng -= eng_loss;
    }

    fn update_physics(&mut self, physics: &mut Physics, _resize: bool) {
        //let settings = get_settings();
        let physics_data = physics.get_object_state(self.physics_handle);
        self.pos = physics_data.position;
        self.rot = physics_data.rotation;
        match physics.get_object_mut(self.physics_handle) {
            Some(body) => {
                self.check_edges(body);
            }
            None => {}
        }
    }

    pub fn update_cloning(&mut self, physics: &mut Physics) -> Option<Resource> {
        if self.clone_timer.update(get_frame_time()) {
            let settings = get_settings();
            let b = settings.res_balance as f32;
            let n = physics.count_near_resources(self.physics_handle, settings.res_detection_radius) as f32;
            let mut p = settings.resource_probability;
            if n > b {
                p = p - p*((n-b)/b);
            }
            if random_unit_unsigned() <= p {
                let mut res = Resource::new(physics);
                res.pos = self.pos + random_unit_vec2() * 150.0;
                return Some(res);
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    fn check_edges(&mut self, body: &mut RigidBody) {
        let settings = get_settings();
        let mut raw_pos = matrix_to_vec2(body.position().translation);
        let mut out_of_edge = false;
        if raw_pos.x < -5.0 {
            raw_pos.x = 0.0;
            out_of_edge = true;
        } else if raw_pos.x > settings.world_w as f32 + 5.0 {
            raw_pos.x = settings.world_w as f32;
            out_of_edge = true;
        }
        if raw_pos.y < -5.0 {
            raw_pos.y = 0.0;
            out_of_edge = true;
        } else if raw_pos.y > settings.world_h as f32 + 5.0 {
            raw_pos.y = settings.world_h as f32;
            out_of_edge = true;
        }
        if out_of_edge {
            body.set_position(make_isometry(raw_pos.x, raw_pos.y, self.rot), true);
            //body.set_linvel([0.0, 0.0].into(), true);
            //self.vel = 0.0;
        }
    }    

}
