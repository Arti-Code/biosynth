//#![allow(unused)]

use rapier2d::geometry::*;
use macroquad::{prelude::*, color};
use rapier2d::prelude::*;
use crate::timer::Timer;
use crate::util::*;
use crate::phyx::physics::Physics;
use crate::phyx::physics_misc::PhysicsMaterial;
use crate::settings::*;


pub trait PlantType {
    fn new(physics: &mut Physics) -> Self;
    fn draw(&self, _show_range: bool);
    fn update(&mut self, physics: &mut Physics);
    fn resize(&mut self, physics: &mut Physics);
    fn drain_eng(&mut self, eng_loss: f32);
    fn update_physics(&mut self, physics: &mut Physics, _resize: bool);
    fn update_cloning(&mut self, plant_num: i32, physics: &mut Physics) -> Option<Plant>;
    fn check_edges(&mut self, body: &mut RigidBody);
    fn is_alive(&self) -> bool;
    fn get_body_handle(&self) -> RigidBodyHandle;
    fn get_lifetime(&self) -> f32;
}

#[derive(Clone, Copy)]
pub struct Plant {
    pub pos: Vec2,
    pub rot: f32,
    pub size: f32,
    pub max_eng: f32,
    pub eng: f32,
    color: color::Color,
    shape: Ball,
    physics_handle: RigidBodyHandle,
    alife: bool,
    pub time: f32,
    clone_timer: Timer,
    growth_timer: Timer,
    life_length: f32,
    clone_ready: bool,
}


impl PlantType for Plant {
    
    fn new(physics: &mut Physics) -> Self {
        let settings = get_settings();
        let pos = random_position(settings.world_w as f32, settings.world_h as f32);
        let size = 2.0;
        let shape = SharedShape::ball(size);
        let rbh = physics.add_dynamic_object(
            &pos, 
            0.0, 
            shape.clone(), 
            PhysicsMaterial::plant(), 
            InteractionGroups::new(Group::GROUP_2, Group::GROUP_1 | Group::GROUP_2), 
            true
        );
        let max_life = settings.plant_lifetime + settings.plant_lifetime * random_unit() / 4.0;
        Self {
            pos,
            rot: 0.0,
            size,
            max_eng: size.powi(2)*10.0,
            eng: size.powi(2)*10.0,
            color: YELLOW,
            shape: Ball { radius: size },
            physics_handle: rbh,
            life_length: max_life,
            time: max_life,
            alife: true,
            clone_timer: Timer::new(10.0, true, true, true),
            growth_timer: Timer::new(10.0, true, true, true),
            clone_ready: false,
        }
    }
    
    fn draw(&self, _show_range: bool) {
        let x0 = self.pos.x;
        let y0 = self.pos.y;
        let age = self.time/self.life_length;
        let g = clamp(1.0, 0., 1.,);
        let r = clamp(-0.25+(1.5-age), 0., 0.75,);
        let b = clamp(0., 0., 1.,);
        let color = Color::new(r, g, b, 1.0);
        draw_circle(x0, y0, self.size, color);
    }
    
    fn update(&mut self, physics: &mut Physics){
        let dt = dt()*sim_speed();
        let settings = get_settings();
        let mut resize = false;
        self.time -= dt;
        self.eng += settings.growth * dt;
        if self.growth_timer.update(dt) {
            if self.eng >= self.size.powi(2)*10.0 {
                self.size += 1.0;
                resize = true;
                if self.size >= settings.plant_clone_size as f32 {
                    self.clone_ready = true;
                }
            } else if self.eng < (self.size-1.0).powi(2)*10.0 && self.size >= 1.0 {
                self.size -= 1.0;
                resize = true;
            }
        }
        self.update_physics(physics, resize);
        self.pos = wrap_around(&self.pos);
        if self.eng <= 0.0 || self.time <= 0.0 {
            self.eng = 0.0;
            self.alife = false;
            return;
        }
        if resize {
            self.resize(physics);
            self.max_eng = self.size.powi(2)*10.0;
            if self.eng > self.max_eng {
                self.eng = self.max_eng;
            }
        }
    }

    fn resize(&mut self, physics: &mut Physics) {
        let c = physics.get_first_collider_mut(self.physics_handle);
        c.set_shape(SharedShape::ball(self.size));
    }

    fn drain_eng(&mut self, eng_loss: f32) {
        self.eng -= eng_loss;
    }

    fn update_physics(&mut self, physics: &mut Physics, _resize: bool) {
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

    fn update_cloning(&mut self, plant_num: i32, physics: &mut Physics) -> Option<Plant> {
        if self.clone_timer.update(dt()*sim_speed()) {
            if self.clone_ready {
                let plant_balance = get_settings().plant_balance as f32;
                let r = plant_balance/((plant_num as f32));
                if random_unit_unsigned() > r { return None; }
                self.clone_ready = false;
                let mut plant = Plant::new(physics);
                plant.pos = self.pos + random_unit_vec2() * 25.0;
                return Some(plant);
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
        }
    }    

    fn is_alive(&self) -> bool {
        return self.alife;
    }

    fn get_lifetime(&self) -> f32 {
        return self.time;
    }

    fn get_body_handle(&self) -> RigidBodyHandle {
        return self.physics_handle;
    }

}
