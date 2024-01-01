//#![allow(unused)]
#![windows_subsystem = "windows"]

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
mod terrain;
//mod agent2;
mod monit;

use std::env;
//use std::path::Path;
//use std::path::PathBuf;
use crate::sim::*;
use crate::globals::*;
use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;
use util::MyIcon;
use util::generate_seed;


fn app_configuration() -> Conf {
    let ico = MyIcon::color_filled(GREEN);
    Conf {
        window_title: env!("CARGO_PKG_NAME").to_string().to_uppercase(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        sample_count: 16,
        window_resizable: true,
        //icon: Some(image::io::Reader::open("assets/ico/molecular.ico").unwrap().decode().unwrap().into()),
        icon: Some(Icon {
            small: ico.small,
            medium: ico.medium,
            big: ico.big,
        }),
        ..Default::default()
    }
}

fn setup() {
    set_global_settings(Settings::default());
    set_global_signals(Signals::new());
    set_mutations(MutationStats::new(0.0, 0.0));
}

#[macroquad::main(app_configuration)]
async fn main() {
    setup();
    let seed = generate_seed();
    rand::srand(seed);
    let font = Font::default();
    let mut sim = Simulation::new(font.clone());
    sim.ui.load_textures();
    let mut args = env::args();
    match args.nth(1) {
        Some(save_path) => {
            sim.running = true;
            sim.load_sim(&save_path, true);
        },
        None => {},
        //let font = load_ttf_font("assets/fonts/firacode.ttf").await;
    }

    loop {
        sim.input();
        sim.process_ui();
        if sim.is_running() {
            sim.update();
            sim.draw();
        } else {
            sim.check_signals();
        }
        sim.draw_ui();
        next_frame().await;
    }
}
