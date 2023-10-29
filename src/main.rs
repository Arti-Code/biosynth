//#![allow(unused)]

mod camera;
mod neuro;
mod sim;
mod timer;
mod ui;
mod util;
mod physics;
mod part;
mod agent;
mod collector;
mod food;
mod globals;
mod resource;
mod neural;


use std::time::{SystemTime, UNIX_EPOCH/* , Duration, Instant */};
use crate::sim::*;
use crate::globals::*;
use macroquad::prelude::*;


fn app_configuration() -> Conf {
    Conf {
        window_title: env!("CARGO_PKG_NAME").to_string().to_uppercase(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        sample_count: 16,
        window_resizable: false,
        //icon: Some(image::io::Reader::open("assets/ico/molecular.ico").unwrap().decode().unwrap().into()),
        ..Default::default()
    }
}

fn setup() {
    init_global_settings(Settings::default());
    init_global_signals(Signals::new());
}

#[macroquad::main(app_configuration)]
async fn main() {
    setup();
    let t = SystemTime::now();
    let s = t.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let s2 = s / 1000000;
    let seed = s%s2;
    println!("SEED: {}", seed); 
    rand::srand(seed);
    let font = load_ttf_font("assets/fonts/firacode.ttf")
        .await
        .expect("can't load font resource!");
    let mut sim = Simulation::new(font.clone());
    sim.ui.load_textures();

    loop {
        sim.input();
        sim.process_ui();
        if sim.is_running() {
            sim.update();
            sim.draw();
        } else {
            sim.signals_check();
        }
        sim.draw_ui();
        next_frame().await;
    }
}