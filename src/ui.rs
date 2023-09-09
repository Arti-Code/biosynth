#![allow(unused)]

use std::collections::BTreeMap;
use std::f32::consts::PI;
use std::path::Path;
use egui_macroquad;
use egui_macroquad::egui::*;
use egui_macroquad::egui::widgets::Slider;
use macroquad::math::Vec2 as Vec2;
use macroquad::prelude::*;
use image::{io::*, *};
use macroquad::ui::StyleBuilder;
use macroquad::ui::widgets::Texture;
use crate::consts::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::sim::{*, self};
use crate::util::*;
use crate::unit::*;

pub struct UISystem {
    pub state: UIState,
    pub pointer_over: bool,
    temp_sim_name: String,
    logo: Option<egui_macroquad::egui::TextureHandle>,
    big_logo: Option<egui_macroquad::egui::TextureHandle>,
    title: Option<egui_macroquad::egui::TextureHandle>,
}

impl UISystem {

    pub fn new() -> Self {
        Self {
            state: UIState::new(),
            pointer_over: false,
            temp_sim_name: String::new(),
            logo: None,
            big_logo: None,
            title: None,
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
            let img2 =  Self::load_image(Path::new("assets/img/microbes.png")).unwrap();
            self.big_logo = Some(egui_ctx.load_texture("big_logo".to_string(), img2, Default::default()));
            let img3 =  Self::load_image(Path::new("assets/img/evolve.png")).unwrap();
            self.title = Some(egui_ctx.load_texture("title".to_string(), img3, Default::default()));
        });
    }

    pub fn ui_process(&mut self, settings: &mut Settings, sim_state: &SimState, signals: &mut Signals, camera2d: &Camera2D, agent: Option<&Unit>) {
        egui_macroquad::ui(|egui_ctx| {
            self.pointer_over = egui_ctx.is_pointer_over_area();
            self.build_top_menu(egui_ctx, &sim_state.sim_name);
            self.build_quit_window(egui_ctx);
            self.build_monit_window(egui_ctx, &sim_state);
            self.build_debug_window(egui_ctx, camera2d);
            self.build_new_sim_window(egui_ctx, signals, settings);
            match agent {
                Some(agent) => {
                    self.build_inspect_window(egui_ctx, agent);
                    self.build_io_window(egui_ctx, agent.neuro_table.inputs.clone(), agent.neuro_table.outputs.clone());
                },
                None => {}
            }
            self.build_net_graph(egui_ctx);
            self.build_about_window(egui_ctx);
            self.build_enviroment_window(egui_ctx, settings, signals);
            self.build_static_rect_win(egui_ctx, signals);
            self.build_neurolab(egui_ctx);
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
                ui.separator();
                ui.label(RichText::new(sim_name).heading().strong().color(Color32::GREEN));
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

                menu::menu_button(ui, RichText::new("VIEW").strong(), |ui| {
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
                });
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                menu::menu_button(ui, RichText::new("TOOLS").strong(), |ui| {
                    if ui
                        .button(RichText::new("Static Object").strong().color(Color32::WHITE))
                        .clicked()
                    {
                        self.state.static_rect = !self.state.static_rect;
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                menu::menu_button(ui, RichText::new("NEUROLOGY").strong(), |ui| {
                    if ui
                        .button(RichText::new("NeuroLab").strong().color(Color32::WHITE))
                        .clicked()
                    {
                        self.state.neuro_lab = !self.state.neuro_lab;
                    }
                    if ui
                        .button(RichText::new("I/O").strong().color(Color32::WHITE))
                        .clicked()
                    {
                        self.state.io = !self.state.io;
                    }
                });


                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                menu::menu_button(ui, RichText::new("SETTINGS").strong(), |ui| {
                    if ui.button(RichText::new("Settings").strong().color(Color32::YELLOW)).clicked() {
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
            Window::new("MONITOR").default_pos((5.0, 5.0)).default_width(200.0).show(egui_ctx, |ui| {
                ui.label(format!("DELTA: {}ms", (delta * 1000.0).round()));
                ui.separator();
                ui.label(format!("FPS: {}", fps));
                ui.separator();
                ui.label(format!("TIME: {}", time.round()));
                ui.separator();
                ui.label(format!("TOTAL MASS: {}", total_mass.round()));
                ui.separator();
                ui.label(format!("OBJECTS: {}", physics_num));
            });
        }
    }

    fn build_io_window(&self, egui_ctx: &Context, inputs: Vec<(u64, f32)>, outputs: Vec<(u64, f32)>) {
        if self.state.io {
            Window::new("INPUT&OUTPUT").default_pos((5.0, 5.0)).default_width(200.0).show(egui_ctx, |ui| {
                ui.horizontal(|horizont| {
                    horizont.columns(2, |col| {
                        for (id, v) in inputs.iter() {
                            let i = (v*100.0).round()/100.0;
                            col[0].label(format!("{}", i));
                        }
                        for (id, v) in outputs.iter() {
                            let o = (v*100.0).round()/100.0;
                            col[1].label(format!("{o}"));
                        }
                    })
                });
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

    fn build_new_sim_window(&mut self, egui_ctx: &Context, signals: &mut Signals, settings: &mut Settings) {
        if self.state.new_sim {
            Window::new("NEW SIMULATION").default_pos((SCREEN_WIDTH / 2.0 - 65.0, SCREEN_HEIGHT / 4.0)).fixed_size([380., 100.]).show(egui_ctx, |ui| {
                ui.vertical_centered(|head| {
                    head.label("Start new simulation");
                });
                ui.vertical_centered(|txt| {
                    let response = txt.add(widgets::TextEdit::singleline(&mut self.temp_sim_name));
                    self.temp_sim_name = String::from("SIMULATION");
                    if response.gained_focus() {
                    }
                    if response.changed() {
                    }
                    if response.lost_focus() && txt.input(|i| i.key_pressed(Key::Enter)) {
                        self.state.new_sim = false;
                        signals.new_sim = true;
                        signals.new_sim_name = String::from(&self.temp_sim_name);
                        self.temp_sim_name = String::new();
                    }
                });
                ui.spacing();
                ui.vertical_centered(|row| {
                    row.label("WORLD SIZE [X | Y]");
                });
                ui.vertical_centered(|row| {
                    row.style_mut().spacing.slider_width = 140.0;
                    let mut w = settings.world_w;
                    let mut h = settings.world_h;
                    row.columns(2, |columns| {
                        if columns[0].add(Slider::new(&mut w, 400..=4800)).changed() {
                            settings.world_w = w;
                        }
                        if columns[1].add(Slider::new(&mut h, 300..=3600)).changed() {
                            settings.world_h = h;
                        }
                    });
                });
                ui.spacing();
                ui.vertical_centered(|mid| {
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

    fn build_inspect_window(&self, egui_ctx: &Context, agent: &Unit) {
        if self.state.inspect {
            let rot = agent.rot;
            let size = agent.size;
            let tg_pos = agent.enemy_position;
            let tg_ang = agent.enemy_dir;
            let pos = agent.pos;
            let contacts_num = agent.contacts.len();
            let lifetime = agent.lifetime.round();
            let generation = agent.generation;
            Window::new("INSPECT").default_pos((175.0, 5.0)).default_width(200.0).show(egui_ctx, |ui| {
                ui.label(RichText::new("AGENT").strong().color(Color32::GREEN));
                ui.label(format!("lifetime: [{}]", lifetime));
                ui.label(format!("generation: [{}]", generation));
                ui.label(format!("direction: [{}]", ((rot * 10.0).round()) / 10.0));
                ui.label(format!("size: [{}]", size));
                ui.label(format!("position: [X: {} | Y:{}]", pos.x.round(), pos.y.round()));
                ui.label(RichText::new(format!("energy: {}/{}", agent.eng.round(), agent.max_eng.round())).strong().color(Color32::BLUE));
                ui.separator();
                ui.label(RichText::new("ENEMY").strong().color(Color32::RED));
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

    fn build_neurolab(&mut self, egui_ctx: &Context) {
        if self.state.neuro_lab {
            Window::new("Neuro Lab").default_pos((SCREEN_WIDTH/2.-400., SCREEN_HEIGHT/2.-500.)).min_height(600.).min_width(800.)
            .title_bar(true).show(egui_ctx, |ui| {
                let (response, painter) = ui.allocate_painter(egui_macroquad::egui::Vec2::new(800., 600.), Sense::hover());
                let rect = response.rect;
                let c = rect.center();
                painter.circle_stroke(c, 16.0, Stroke::new(3.0, Color32::BLUE));
                painter.arrow(Pos2::new(c.x-64.0, c.y), egui_macroquad::egui::Vec2::new(48.0, 0.0), Stroke::new(5.0, Color32::RED));
                painter.arrow(Pos2::new(c.x+64.0, c.y), egui_macroquad::egui::Vec2::new(-48.0, 0.0), Stroke::new(5.0, Color32::RED));
            });
        }
    }

    fn build_about_window(&mut self, egui_ctx: &Context) {
        if self.state.about {
            Window::new("ABOUT").resizable(false).default_pos((SCREEN_WIDTH/2.-150., SCREEN_HEIGHT/6.)).min_height(380.).min_width(300.)
            .title_bar(true).show(egui_ctx, |ui| {
                let big_logo = self.big_logo.clone().unwrap();
                let title = self.title.clone().unwrap();
                ui.vertical_centered(|pic| {
                    pic.image(title.id(), title.size_vec2());
                });
                ui.add_space(10.0);
                ui.vertical_centered(|pic| {
                    pic.image(big_logo.id(), big_logo.size_vec2());
                });
                ui.add_space(10.0);
                ui.vertical_centered(|author| {
                    author.label(RichText::new("Artur Gwoździowski 2023").color(Color32::BLUE).strong());
                });
                ui.add_space(10.0);
                ui.vertical_centered(|author| {
                    author.label(RichText::new(format!("version {}", env!("CARGO_PKG_VERSION"))).color(Color32::YELLOW).italics());
                });
                ui.add_space(10.0);
                ui.vertical_centered(|closer| {
                    let mut stylus = closer.style();
                    if closer.button(RichText::new("CLOSE").color(Color32::RED).strong()).clicked() {
                        self.state.about = false;
                        self.state.new_sim = true;
                    }
                });
            });
        }
    }

    fn build_enviroment_window(&mut self, egui_ctx: &Context, settings: &mut Settings, signals: &mut Signals) {
        if !self.state.enviroment {
            return;
        }
        Window::new("SETTINGS").id("settings_win".into()).default_pos((SCREEN_WIDTH/2., SCREEN_HEIGHT/2.)).fixed_size([380., 300.])
        .title_bar(true).show(egui_ctx, |ui| {
            ui.heading("AGENTS");
            ui.columns(2, |column| {
                column[0].set_max_size(egui_macroquad::egui::Vec2::new(80., 75.));
                column[1].set_max_size(egui_macroquad::egui::Vec2::new(280., 75.));
                unsafe {
                    let mut agents_num: i32 = settings.agent_min_num as i32;
                    column[0].label(RichText::new("MIN NUMBER").color(Color32::WHITE).strong());
                    if column[1].add(Slider::new(&mut agents_num, 0..=100)).changed() {
                        settings.agent_min_num = agents_num as usize;
                        signals.new_settings = true;
                    }
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(egui_macroquad::egui::Vec2::new(80., 75.));
                column[1].set_max_size(egui_macroquad::egui::Vec2::new(280., 75.));
                unsafe {
                    let mut agent_init_num: i32 = settings.agent_init_num as i32;
                    column[0].label(RichText::new("INIT NUMBER").color(Color32::WHITE).strong());
                    if column[1].add(Slider::new(&mut agent_init_num, 0..=100)).changed() {
                        settings.agent_init_num = agent_init_num as usize;
                        signals.new_settings = true;
                    }
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(egui_macroquad::egui::Vec2::new(80., 75.));
                column[1].set_max_size(egui_macroquad::egui::Vec2::new(280., 75.));
                unsafe {
                    let mut agent_vision_range: i32 = settings.agent_vision_range as i32;
                    column[0].label(RichText::new("VISION").color(Color32::WHITE).strong());
                    if column[1].add(Slider::new(&mut agent_vision_range, 10..=1000)).changed() {
                        settings.agent_vision_range = agent_vision_range as f32;
                        signals.new_settings = true;
                    }
                }
            });
            ui.style_mut().spacing.slider_width = 75.0;
            ui.columns(3, |column| {
                column[0].set_max_size(egui_macroquad::egui::Vec2::new(80., 75.));
                column[1].set_max_size(egui_macroquad::egui::Vec2::new(140., 75.));
                column[2].set_max_size(egui_macroquad::egui::Vec2::new(140., 75.));
                unsafe {
                    let mut agent_size_min: i32 = settings.agent_size_min as i32;
                    let mut agent_size_max: i32 = (settings.agent_size_max as i32).max(agent_size_min);
                    column[0].label(RichText::new("SIZE [MIN|MAX]").color(Color32::WHITE).strong());
                    if column[1].add(Slider::new(&mut agent_size_min, 1..=40)).changed() {
                        settings.agent_size_min = agent_size_min as i32;
                        signals.new_settings = true;
                    }
                    if column[2].add(Slider::new(&mut agent_size_max, agent_size_min..=40)).changed() {
                        settings.agent_size_max = agent_size_max as i32;
                        signals.new_settings = true;
                    }
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(egui_macroquad::egui::Vec2::new(120., 75.));
                column[1].set_max_size(egui_macroquad::egui::Vec2::new(120., 75.));
                unsafe {
                    let mut agent_eng_bar: bool = settings.agent_eng_bar;
                    column[0].label(RichText::new("SHOW ENG BAR").color(Color32::WHITE).strong());
                    if column[1].add(egui_macroquad::egui::Checkbox::without_text(&mut agent_eng_bar)).changed() {
                        settings.agent_eng_bar = agent_eng_bar;
                        signals.new_settings = true;
                    }
                }
            });
            ui.add_space(10.0);
            ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::new(3.0, Color32::BLUE);
            ui.vertical_centered(|closer| {
                let mut stylus = closer.style();
                if closer.button(RichText::new("CLOSE").color(Color32::LIGHT_RED).strong()).clicked() {
                    self.state.enviroment = false;
                }
            });
        });
    }

    fn build_static_rect_win(&mut self, egui_ctx: &Context, signals: &mut Signals) {
        if self.state.static_rect {
            Window::new("Build Static Rect").title_bar(true).default_pos((SCREEN_WIDTH/2.-150., SCREEN_HEIGHT/6.)).default_size([300., 260.]).show(egui_ctx, |ui| {
                ui.label("Rect Size");
                ui.columns(2, |column| {
                    let mut w: i32 = 100;
                    let mut h: i32 = 60;
                    if column[1].add(Slider::new(&mut w, 10..=400).text("width")).changed() {
                    }
                    if column[0].add(Slider::new(&mut h, 10..=400).text("height")).changed() {
                    }
                });
                ui.spacing();
                ui.label("Rect Position");
                ui.columns(2, |column| {
                    let mut x: i32 = 250;
                    let mut y: i32 = 250;
                    if column[1].add(Slider::new(&mut x, 10..=400).text("xpos")).changed() {
                    }
                    if column[0].add(Slider::new(&mut y, 10..=400).text("ypos")).changed() {
                    }
                });
                ui.spacing();
                ui.vertical_centered(|btn| {
                    if btn.button(RichText::new("CREATE").color(Color32::BLUE).strong()).clicked() {
                        self.state.static_rect = false;
                    }
                }); 
            });
        }
    }

    pub fn ui_draw(&self) {
        egui_macroquad::draw();
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