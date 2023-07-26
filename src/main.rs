//#![allow(unused)]

mod agent;
mod camera;
mod consts;
mod kinetic;
mod neuro;
mod particle;
mod progress_bar;
mod sim;
mod timer;
mod ui;
mod util;
mod world;
mod being;
mod plant;

use crate::consts::*;
use crate::sim::*;
use macroquad::prelude::*;

fn app_configuration() -> Conf {
    Conf {
        window_title: "BIO-SYNTH".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        sample_count: 16,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(app_configuration)]
async fn main() {
    let cfg = SimConfig::default();
    let font = load_ttf_font("firacode.ttf")
        .await
        .expect("can't load font resource!");
    let mut sim = Simulation::new(cfg, font.clone());
    sim.ui.load_textures();
    sim.init();
    sim.autorun_new_sim();

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
