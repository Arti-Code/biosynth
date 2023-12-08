//#![allow(unused)]

use rapier2d::geometry::*;
use macroquad::{prelude::*, color};
use rapier2d::prelude::*;
use crate::util::*;
use crate::physics::*;
use crate::globals::*;

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
}

impl Resource {
    pub fn new(physics: &mut Physics) -> Self {
        let settings = get_settings();
        //let key = rand::gen_range(u64::MIN, u64::MAX);
        let pos = random_position(settings.world_w as f32, settings.world_h as f32);
        //let color = random_color();
        let size = rand::gen_range(6, 8) as f32;
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
        }
    }
    pub fn draw(&self) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        draw_circle(x0, y0, self.size, self.color);
    }
    pub fn update(&mut self, physics: &mut Physics){
        let dt = get_frame_time();
        self.time -= dt;
        self.update_physics(physics);
        self.pos = wrap_around(&self.pos);
        if self.eng <= 0.0 || self.time <= 0.0 {
            self.eng = 0.0;
            self.alife = false;
        }
    }

    pub fn drain_eng(&mut self, eng_loss: f32) {
        self.eng -= eng_loss;
    }

    fn update_physics(&mut self, physics: &mut Physics) {
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
