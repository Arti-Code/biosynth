#![allow(unused)]

use std::collections::BTreeMap;
use std::f32::consts::PI;
use std::path::Path;
use egui_macroquad;
use egui_macroquad::egui::*;
use macroquad::prelude::*;
use image::{io::*, *};
use macroquad::ui::widgets::Texture;
use crate::consts::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::sim::{*, self};
use crate::util::*;
use crate::agent::*;
use crate::progress_bar;

pub struct UISystem {
    pub state: UIState,
    pub pointer_over: bool,
    temp_sim_name: String,
    logo: Option<egui_macroquad::egui::TextureHandle>,
}

impl UISystem {

    pub fn new() -> Self {
        Self {
            state: UIState::new(),
            pointer_over: false,
            temp_sim_name: String::new(),
            logo: None,
            //egui_extras::RetainedImage::from_color_image("mylogo.png", ColorImage::new([64, 256], Color32::WHITE)).
        }
    }

    fn load_image(path: &Path) -> Result<egui_macroquad::egui::ColorImage, image::ImageError> {
        let image = image::io::Reader::open(path)?.decode()?;
        let size = [image.width() as _, image.height() as _];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.as_flat_samples();
        Ok(egui_macroquad::egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice()))
    }

    pub fn load_textures(&mut self) {
        egui_macroquad::ui(|egui_ctx| {
            let img =  Self::load_image(Path::new("assets/img/microbes32.png")).unwrap();
            self.logo = Some(egui_ctx.load_texture("logo".to_string(), img, Default::default()));
        });
    }

    pub fn ui_process(&mut self, sim_state: &SimState, signals: &mut Signals, camera2d: &Camera2D, agent: Option<&Agent>) {
        egui_macroquad::ui(|egui_ctx| {
            //let mut style = (*egui_ctx.style()).clone();
            //style.text_styles. = [
            //    (egui::TextStyle::Button, FontId::new(15.0, Proportional)),
            //].into();
            //egui_ctx.set_style(style);
            self.pointer_over = egui_ctx.is_pointer_over_area();
            self.build_top_menu(egui_ctx, &sim_state.sim_name);
            self.build_quit_window(egui_ctx);
            self.build_monit_window(egui_ctx, &sim_state);
            self.build_debug_window(egui_ctx, camera2d);
            //self.build_create_window(egui_ctx, signals);
            self.build_new_sim_window(egui_ctx, signals);
            match agent {
                Some(agent) => self.build_inspect_window(egui_ctx, agent),
                None => {}
            }
            self.build_net_graph(egui_ctx);
            self.build_about_window(egui_ctx);
            self.build_enviroment_window(egui_ctx);
        });
    }

    fn build_top_menu(&mut self, egui_ctx: &Context, sim_name: &str) {
        TopBottomPanel::top("top_panel").default_height(100.0).show(egui_ctx, |ui| {
            if !self.pointer_over {
                self.pointer_over = ui.ui_contains_pointer();
            }
            menu::bar(ui, |ui| {
                let logo = self.logo.clone().unwrap();
                ui.image(logo.id(), logo.size_vec2());
                //ui.heading(RichText::new(sim_name).strong().color(Color32::GREEN));
                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);
                menu::menu_button(ui, RichText::new("SIMULATION").strong(), |ui| {
                    if ui
                        .button(
                            RichText::new("New Simulation")
                                .strong()
                                .color(Color32::BLUE),
                        )
                        .clicked()
                    {
                        self.state.new_sim = true;
                    }
                    if ui
                        .button(
                            RichText::new("Load Simulation")
                                .weak()
                                .color(Color32::from_gray(100)),
                        )
                        .clicked()
                    {}
                    if ui
                        .button(
                            RichText::new("Save Simulation")
                                .weak()
                                .color(Color32::from_gray(100)),
                        )
                        .clicked()
                    {}
                    if ui
                        .button(RichText::new("Quit").color(Color32::RED).strong())
                        .clicked()
                    {
                        self.state.quit = true;
                    }
                });
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                menu::menu_button(ui, RichText::new("TOOLS").strong(), |ui| {
                    if ui
                        .button(RichText::new("Monitor").strong().color(Color32::WHITE))
                        .clicked()
                    {
                        self.state.performance = !self.state.performance;
                    }
                    if ui
                        .button(RichText::new("Inspector").strong().color(Color32::WHITE))
                        .clicked()
                    {
                        self.state.inspect = !self.state.inspect;
                    }
                    if ui
                        .button(RichText::new("Debug Info").strong().color(Color32::WHITE))
                        .clicked()
                    {
                        self.state.mouse = !self.state.mouse;
                    }
                    if ui
                        .button(RichText::new("Create").strong().color(Color32::WHITE))
                        .clicked()
                    {
                        self.state.create = !self.state.create;
                    }
                });
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                menu::menu_button(ui, RichText::new("SETTINGS").strong(), |ui| {
                    if ui.button(RichText::new("Enviroment Settings").strong().color(Color32::YELLOW)).clicked() {
                        self.state.enviroment = !self.state.enviroment;
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                menu::menu_button(ui, RichText::new("ABOUT").strong(), |ui| {
                    if ui.button(RichText::new("Draw Neuro").strong().color(Color32::WHITE)).clicked() {
                        self.state.net = !self.state.net;
                    }
                    if ui.button(RichText::new("About").strong().color(Color32::WHITE)).clicked() {
                        self.state.about = !self.state.about;
                    }
                    if ui.button(RichText::new("Documentation").strong().color(Color32::WHITE)).clicked() {
                        self.state.docs = !self.state.docs;
                    }
                });
            });
        });
    }

    fn build_monit_window(&self, egui_ctx: &Context, sim_state: &SimState) {
        if self.state.performance {
            let total_mass = sim_state.total_mass;
            let fps = sim_state.fps;
            let delta = sim_state.dt;
            let time = sim_state.sim_time;
            let physics_num = sim_state.physics_num;
            //let contacts = sim_state.contacts_info;
            let mut eng = String::new();
            let eng2: Option<f32> = Some(sim_state.total_eng);
            match eng2 {
                Some(e) if e < 1000.0 => {
                    eng = e.to_string();
                    eng.push_str(" eV");
                },
                Some(e) if e < 1000000.0 => {
                    eng = (e/1000.0).round().to_string();
                    eng.push_str(" KeV");
                }
                Some(e) if e < 1000000000.0 => {
                    eng = (e/1000000.0).round().to_string();
                    eng.push_str(" MeV");
                }
                Some(e) if e < 1000000000000.0 => {
                    eng = (e/1000000000.0).round().to_string();
                    eng.push_str(" GeV");
                },
                Some(_) => {},
                None => {},
            }
            Window::new("MONITOR").default_pos((5.0, 5.0)).default_width(150.0).show(egui_ctx, |ui| {
                ui.label(format!("DELTA: {}ms", (delta * 1000.0).round()));
                ui.separator();
                ui.label(format!("FPS: {}", fps));
                ui.separator();
                ui.label(format!("TIME: {}", time.round()));
                ui.separator();
                ui.label(format!("TOTAL ENG: {}", eng));
                ui.separator();
                ui.label(format!("TOTAL MASS: {}", total_mass.round()));
                ui.separator();
                ui.label(format!("OBJECTS: {}", physics_num));
                //ui.separator();
                //ui.label(format!("CONTACTS: {} | {}", contacts.0, contacts.0));
            });
        }
    }

    fn build_debug_window(&self, egui_ctx: &Context, camera2d: &Camera2D) {
        if self.state.mouse {
            let (mouse_x, mouse_y) = mouse_position();
            Window::new("DEBUG INFO").default_pos((375.0, 5.0)).default_width(175.0).show(egui_ctx, |ui| {
                ui.label(RichText::new("MOUSE").strong());
                ui.label(format!("coords [x: {} | y: {}]", mouse_x.round(), mouse_y.round()));
                ui.separator();
                ui.label(RichText::new("CAMERA").strong());
                ui.label(format!("target [x:{} | Y:{}]", camera2d.target.x.round(), camera2d.target.y.round()));
                ui.label(format!("offset [x:{} | Y:{}]", camera2d.offset.x.round(), camera2d.offset.y.round()));
                ui.label(format!("zoom [x:{} | Y:{}]", (camera2d.zoom.x * 10000.).round() / 10., (camera2d.zoom.y * 10000.).round() / 10.));
                ui.label(format!("rotation: {}", camera2d.rotation.round()));
            });
        }
    }

    fn build_quit_window(&mut self, egui_ctx: &Context) {
        if self.state.quit {
            Window::new("QUIT").default_pos((SCREEN_WIDTH / 2.0 - 65.0, SCREEN_HEIGHT / 4.0)).default_width(125.0).show(egui_ctx, |ui| {
                ui.horizontal(|head| {
                    head.heading("Are you sure?");
                });
                ui.horizontal(|mid| {
                    mid.columns(2, |columns| {
                        if columns[0].button(RichText::new("No").color(Color32::WHITE)).clicked() {
                            self.state.quit = false;
                        }
                        if columns[1].button(RichText::new("Yes").color(Color32::RED)).clicked() {
                            std::process::exit(0);
                        }
                    });
                });
            });
        }
    }

    fn build_new_sim_window(&mut self, egui_ctx: &Context, signals: &mut Signals) {
        if self.state.new_sim {
            //let mut sim_name: String = String::new();
            Window::new("NEW SIMULATION").default_pos((SCREEN_WIDTH / 2.0 - 65.0, SCREEN_HEIGHT / 4.0)).default_width(125.0).show(egui_ctx, |ui| {
                ui.horizontal(|head| {
                    head.heading("Start new simulation?");
                });
                ui.horizontal(|txt| {
                    let response = txt.add(widgets::TextEdit::singleline(&mut self.temp_sim_name));
                    if response.gained_focus() {
                        self.temp_sim_name = String::new();
                    }
                    if response.changed() {
                        //self.temp_sim_name = String::from(&sim_name);
                        //println!("{:?}", sim_name);
                        //println!("{:?}", self.temp_sim_name);
                    }
                    if response.lost_focus() && txt.input(|i| i.key_pressed(Key::Enter)) {
                        self.state.new_sim = false;
                        signals.new_sim = true;
                        signals.new_sim_name = String::from(&self.temp_sim_name);
                        self.temp_sim_name = String::new();
                    }
                    //let response = txt.text_edit_singleline(&mut sim_name);
                });
                ui.horizontal(|mid| {
                    mid.columns(2, |columns| {
                        if columns[0].button(RichText::new("No").color(Color32::WHITE)).clicked() {
                            self.state.new_sim = false;
                            self.temp_sim_name = String::new();
                        }
                        if columns[1].button(RichText::new("Yes").color(Color32::BLUE)).clicked() {
                            self.state.new_sim = false;
                            signals.new_sim = true;
                            signals.new_sim_name = String::from(&self.temp_sim_name);
                            self.temp_sim_name = String::new();
                        }
                    });
                });
            });
        }
    }

    fn build_create_window(&self, egui_ctx: &Context, signals: &mut Signals) {
        if self.state.create {
            Window::new("CREATE").default_pos((600.0, 5.0)).default_width(275.0).min_height(250.0).show(egui_ctx, |ui| {
                ui.horizontal(|mid| {
                    mid.heading("CREATE PARTICLES");
                    mid.columns(1, |columns| {
                        if columns[0].button(RichText::new("PARTICLE").strong().color(Color32::WHITE)).clicked() {
                            signals.spawn_particles = true;
                        }
                    })
                });
            });
        }
    }

    fn build_inspect_window(&self, egui_ctx: &Context, agent: &Agent) {
        if self.state.inspect {
            let rot = agent.rot;
            let size = agent.size;
            let tg_pos = agent.enemy_position;
            let tg_ang = agent.enemy_dir;
            let pos = agent.pos;
            let contacts_num = agent.contacts.len();
            Window::new("INSPECT").default_pos((175.0, 5.0)).default_width(200.0).show(egui_ctx, |ui| {
                ui.label(RichText::new("AGENT").strong());
                ui.label(format!("direction: [{}]", ((rot * 10.0).round()) / 10.0));
                ui.label(format!("size: [{}]", size));
                ui.label(format!("position: [X: {} | Y:{}]", pos.x.round(), pos.y.round()));
                ui.separator();
                ui.label(RichText::new("ENEMY").strong());
                ui.label(format!("contacts: [{}]", contacts_num));
                match (tg_pos, tg_ang) {
                    (Some(target), Some(ang)) => {
                        ui.label(format!("position: [x: {} | y:{}]", target.x.round(), target.y.round()));
                        ui.label(format!("angle: [{}]", (ang*10.0).round()/10.0));
                    }
                    (None, None) => {
                        ui.label(format!("position: [---]"));
                        ui.label(format!("angle: [---]"));
                    }
                    (_, _) => {
                        ui.label(format!("position: [---]"));
                        ui.label(format!("angle: [---]"));
                    }
                }
                ui.separator();
                ui.label(format!("energy: {}/{}", agent.eng.round(), agent.max_eng.round()));
                let eng_prog = agent.eng / agent.max_eng;
                ui.add(progress_bar::ProgressBar::new(eng_prog).desired_width(100.0).fill(Color32::BLUE).show_percentage());
            });
        }
    }

    fn build_net_graph(&mut self, egui_ctx: &Context) {
        if self.state.net {
            Window::new("Neural Network").default_pos((SCREEN_WIDTH/2., SCREEN_HEIGHT/2.)).min_height(400.).min_width(400.)
            .title_bar(true).show(egui_ctx, |ui| {
                let (response, painter) = ui.allocate_painter(egui_macroquad::egui::Vec2::new(400., 400.), Sense::hover());
                let rect = response.rect;
                let c = rect.center();
                let s = 2. * PI / 6.;
                let l = 75.;
                for i in 0..6 {
                    let ang = s*i as f32;
                    let x1 = ang.sin()*l+c.x;
                    let y1 = ang.cos()*l+c.y;
                    let end = Pos2::new(x1, y1);
                    painter.line_segment([c,  end], Stroke {color: Color32::RED, width: 4.0});
                    painter.circle(end, 15., Color32::BLUE, Stroke::default());
                }
                painter.circle(c, 25., Color32::GREEN, Stroke::default());
            });
        }
    }

    fn build_about_window(&mut self, egui_ctx: &Context) {
        if self.state.about {
            Window::new("ABOUT SIMULATION").default_pos((SCREEN_WIDTH/2., SCREEN_HEIGHT/2.)).min_height(250.).min_width(300.)
            .title_bar(true).show(egui_ctx, |ui| {
                ui.separator();
                ui.label(RichText::new("BioSynth Simulation").color(Color32::LIGHT_BLUE).strong().heading());
                ui.separator();
                ui.label(RichText::new("Artur Gwoździowski").color(Color32::WHITE).italics());
                ui.separator();
                ui.label(RichText::new("2023").color(Color32::WHITE).italics());
            });
        }
    }

    fn build_enviroment_window(&mut self, egui_ctx: &Context) {
        if self.state.enviroment {
            Window::new("ENVIROMENT SETTINGS").default_pos((SCREEN_WIDTH/2., SCREEN_HEIGHT/2.)).min_height(260.).min_width(300.)
            .title_bar(true).show(egui_ctx, |ui| {
                ui.heading("AGENTS");
                ui.columns(2, |column| {
                    unsafe {
                        let mut agents_num: i32 = SIM_PARAMS.agent_min_num as i32;
                        column[0].label(RichText::new("MINIMAL NUMBER").color(Color32::WHITE).strong());
                        if column[1].add(egui_macroquad::egui::widgets::Slider::new(&mut agents_num, 0..=100)).changed() {
                            SIM_PARAMS.agent_min_num = agents_num as usize;
                        }
                    }
                });
                ui.columns(2, |column| {
                    unsafe {
                        let mut agent_init_num: i32 = SIM_PARAMS.agent_init_num as i32;
                        column[0].label(RichText::new("INIT NUMBER").color(Color32::WHITE).strong());
                        if column[1].add(egui_macroquad::egui::widgets::Slider::new(&mut agent_init_num, 0..=100)).changed() {
                            SIM_PARAMS.agent_init_num = agent_init_num as usize;
                        }
                    }
                });
            });
        }
    }

    pub fn ui_draw(&self) {
        egui_macroquad::draw();
    }
}

//?         [[[UISTATE]]]
pub struct UIState {
    pub new_sim_name: String,
    pub performance: bool,
    pub inspect: bool,
    pub mouse: bool,
    pub create: bool,
    pub quit: bool,
    pub agents_num: i32,
    pub new_sim: bool,
    pub credits: bool,
    pub docs: bool,
    pub net: bool,
    pub about: bool,
    pub enviroment: bool,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            new_sim_name: String::new(),
            performance: false,
            inspect: false,
            mouse: false,
            create: false,
            quit: false,
            agents_num: 0,
            new_sim: false,
            credits: false,
            docs: false,
            net: false,
            about: false,
            enviroment: false,
        }
    }
}

struct LogoImage {
    texture: Option<TextureHandle>,
}

impl LogoImage {
    fn ui(&mut self, ui: &mut Ui) {
        let texture: &TextureHandle = self.texture.get_or_insert_with(|| {
            ui.ctx().load_texture("my-image", ColorImage::example(), Default::default())
        });
        ui.add(egui_macroquad::egui::Image::new(texture, texture.size_vec2()));
        ui.image(texture, texture.size_vec2());
    }
}