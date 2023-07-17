//use macroquad::rand::*;
use rand::{thread_rng, Rng};

pub struct Timer {
    pub time: f32,
    pub duration: f32,
    pub repeat: bool,
    run: bool,
}

impl Timer {
    pub fn new(duration: f32, repeat: bool, autostart: bool, random_start: bool) -> Self {
        let mut start_time: f32 = 0.0;
        if random_start {
            start_time = thread_rng().gen_range(0.0..duration);
            //start_time = gen_range::<f32>(0.0, duration);
        }
        Self {
            time: start_time,
            duration,
            repeat,
            run: autostart,
        }
    }
    pub fn update(&mut self, dt: f32) -> bool {
        if self.run {
            self.time += dt;
            if self.time >= self.duration {
                if self.repeat {
                    self.time -= self.duration;
                    return true;
                } else {
                    self.run = false;
                    self.time = 0.0;
                    return true;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    pub fn start(&mut self) {
        self.run = true;
        //self.time = 0.0;
    }
    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration;
    }
    pub fn set_repeating(&mut self, repeat: bool) {
        self.repeat = repeat;
    }
    pub fn stop(&mut self) {
        self.run = false;
        self.time = 0.0;
    }
    pub fn restart(&mut self) {
        self.stop();
        self.run = true
    }
}
