#![allow(unused)]

use macroquad::prelude::*;
use crate::timer::*;
use crate::settings::*;

pub struct PerformanceMonitor {
    fps_list: Vec<i32>,
    fps: i32,
    dt: f32,
    timer: Timer,
}

impl PerformanceMonitor {

    pub fn new(dur: f32) -> PerformanceMonitor {
        Self { 
            fps_list: vec![], 
            fps: 0,
            dt: 0.0,
            timer: Timer::new(dur, true, true, false),
        }
    }

    pub fn monitor(&mut self) {
        let fps = (get_fps() as f32/sim_speed()) as i32;
        let dt = get_frame_time()*sim_speed();
        self.fps_list.push(fps);
        if self.timer.update(dt) {
            let sum: i32 = self.fps_list.iter().sum();
            self.fps = sum / self.fps_list.len() as i32;
            self.dt = 1. / self.fps as f32;
            self.fps_list.clear();
        }
    }

    pub fn fps(&self) -> i32 {
        return self.fps;
    }

    pub fn dt(&self) -> f32 {
        return self.dt;
    }

}