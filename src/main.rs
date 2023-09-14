//#![allow(unused)]

mod camera;
mod consts;
mod neuro;
mod sim;
mod timer;
mod ui;
mod util;
mod physics;
mod part;
mod unit;
mod collector;

use crate::consts::*;
use crate::sim::*;
use crate::util::*;
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

#[macroquad::main(app_configuration)]
async fn main() {
    let cfg = Settings {
        world_w: WORLD_W as i32,
        world_h: WORLD_H as i32,
        agent_eng_bar: true,
        agent_init_num: 40,
        agent_min_num: 30,
        agent_rotate: 2.0,
        agent_speed: 100.0,
        agent_size_min: 5,
        agent_size_max: 12,
        agent_vision_range: 300.0,
        show_network: true,
        show_specie: false,
    };
    let font = load_ttf_font("assets/fonts/firacode.ttf")
        .await
        .expect("can't load font resource!");
    let mut sim = Simulation::new(cfg, font.clone());
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
