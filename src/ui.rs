//#![allow(unused)]

use std::fs;
use std::path::{Path, PathBuf};
use egui_macroquad;
use egui_macroquad::egui::*;
use egui_macroquad::egui::plot::{Legend, PlotPoints, Line};
use egui_macroquad::egui::widgets::{Slider, Button};
use egui_macroquad::egui::{
    Checkbox,
    Vec2 as UIVec2,
    FontFamily::Proportional,
    FontId,
    TextStyle::*,
    plot::Plot,
};
use macroquad::prelude::*;
use macroquad::math::vec2;
use base64::prelude::*;
use crate::plant::{Plant, PlantType};
use crate::util::*;
use crate::agent::*;
use crate::neuro::*;
use crate::settings::*;
use crate::statistics::*;
use crate::signals::*;
use crate::sketch::*;
use crate::ranking::Ranking;


struct TempValues {
    pub world_size: Option<macroquad::prelude::Vec2>,
}

impl Default for TempValues {
    fn default() -> Self {
        Self { world_size: None }
    }
}

pub struct UISystem {
    pub state: UIState,
    pub pointer_over: bool,
    temp_sim_name: String,
    logo: Option<egui_macroquad::egui::TextureHandle>,
    big_logo: Option<egui_macroquad::egui::TextureHandle>,
    //title: Option<egui_macroquad::egui::TextureHandle>,
    dice: Option<egui_macroquad::egui::TextureHandle>,
    temp_values: TempValues,
    timer: f32,
}

impl UISystem {

    pub fn new() -> Self {
        Self {
            state: UIState::new(),
            pointer_over: false,
            temp_sim_name: String::new(),
            logo: None,
            big_logo: None,
            //title: None,
            temp_values: TempValues::default(),
            dice: None,
            timer: 0.0,
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
            let img =  Self::load_image(Path::new("assets/img/hexagon32.png")).unwrap();
            self.logo = Some(egui_ctx.load_texture("logo".to_string(), img, Default::default()));
            let img2 =  Self::load_image(Path::new("assets/img/hexagon128.png")).unwrap();
            self.big_logo = Some(egui_ctx.load_texture("big_logo".to_string(), img2, Default::default()));
            //let img3 =  Self::load_image(Path::new("assets/img/hexagon128.png")).unwrap();
            //self.title = Some(egui_ctx.load_texture("title".to_string(), img3, Default::default()));
            let img4 =  Self::load_image(Path::new("assets/img/hexagon24.png")).unwrap();
            self.dice = Some(egui_ctx.load_texture("dice".to_string(), img4, Default::default()));
        });
    }

    fn set_fonts_styles(&mut self, egui_ctx: &Context) {
        let mut style = (*egui_ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(13.0, Proportional)),
            (Name("Heading2".into()), FontId::new(11.0, Proportional)),
            (Name("Context".into()), FontId::new(11.0, Proportional)),
            (Body, FontId::new(10.0, Proportional)),
            (Monospace, FontId::new(10.0, Proportional)),
            (Button, FontId::new(12.0, Proportional)),
            (Small, FontId::new(9.0, Proportional)),
        ].into();
        egui_ctx.set_style(style);
    }

    pub fn ui_process(&mut self, sim_state: &SimState, signals: &mut Signals, camera2d: &Camera2D, agent: Option<&Agent>, res: Option<&Plant>, ranking: &Ranking, statistics: &Statistics) {
        self.timer += dt();
        self.timer = self.timer%1.0;
        //self.timer = self.timer%get_settings().neuro_duration;
        egui_macroquad::ui(|egui_ctx| {
            self.set_fonts_styles(egui_ctx);
            self.pointer_over = egui_ctx.is_pointer_over_area();
            self.build_top_menu(egui_ctx, &sim_state.sim_name, signals);
            self.build_quit_window(egui_ctx);
            self.build_debug_window(egui_ctx, camera2d, &sim_state, agent);
            self.build_new_sim_window(egui_ctx, signals);
            match agent {
                Some(agent) => {
                    self.build_ancestors_window(egui_ctx, agent);
                    //self.build_network(egui_ctx, agent)
                },
                None => {},
            }
            match res {
                Some(res) => {
                    self.build_plant_window(egui_ctx, res);
                },
                None => {},
            }
            self.build_about_window(egui_ctx);
            self.build_settings_enviro_window(egui_ctx, signals);
            self.build_settings_agent_window(egui_ctx, signals);
            self.build_load_sim_window(egui_ctx);
            self.build_main_menu_win(egui_ctx);
            self.build_load_agent_window(egui_ctx);
            self.build_settings_neuro_window(egui_ctx, signals);
            self.build_info_window(egui_ctx);
            self.build_resize_world_window(egui_ctx);
            self.build_left_panel(egui_ctx, &sim_state, agent, ranking);
            self.build_right_panel(egui_ctx, agent, statistics);
            //self.build_bottom_panel(egui_ctx, statistics);
        });
    }

    fn build_top_menu(&mut self, egui_ctx: &Context, sim_name: &str, signals: &mut Signals) {
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
                
                menu::menu_button(ui, RichText::new("MENU").strong(), |ui| {
                    if ui.button(RichText::new("New Simulation").strong().color(Color32::GREEN)).clicked() {
                        self.state.new_sim = true;
                    }
                    if ui.button(RichText::new("Load Simulation").strong().color(Color32::LIGHT_GREEN)).clicked() {
                        //signals.load_sim = true;
                        self.state.load_sim = true;
                    }
                    if ui.button(RichText::new("Save Simulation").weak().color(Color32::LIGHT_BLUE)).clicked() {
                        signals.save_sim = true;
                    }
                    if ui.button(RichText::new("Load Agent").weak().color(Color32::LIGHT_GREEN)).clicked() {
                        self.state.load_agent = true;
                    }
                    if ui.button(RichText::new("Save Agent").strong().color(Color32::LIGHT_BLUE),).clicked() {
                        //let mut signals = mod_signals();
                        signals.save_selected = true;
                    }
                    if ui.button(RichText::new("Export Settings").strong().color(Color32::GRAY),).clicked() {
                        //let mut signals = mod_signals();
                        signals.export_settings = true;
                    }
                    if ui.button(RichText::new("Import Settings").strong().color(Color32::GRAY),).clicked() {
                        //let mut signals = mod_signals();
                        signals.import_settings = true;
                    }
                    if ui.button(RichText::new("Resize World").strong().color(Color32::LIGHT_RED),).clicked() {
                        if !self.state.resize_world {
                            let settings = get_settings();
                            self.temp_values.world_size = Some(macroquad::prelude::Vec2::new(settings.world_w as f32, settings.world_h as f32));
                        }
                        self.state.resize_world = !self.state.resize_world;
                    }
                    if ui.button(RichText::new("Quit").color(Color32::RED).strong()).clicked() {
                        self.state.quit = true;
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                let speed = sim_speed();
                let accel_label = format!("SPEED [+]");
                let mut deccel_color = Color32::YELLOW;
                let mut deccel_label = format!("SPEED [-]");
                if speed == 1.0 {
                    deccel_color = Color32::GRAY;
                    deccel_label = format!("---");
                }
                let mut pause_color = Color32::YELLOW;
                let mut pause_label = format!("Pause");
                if get_settings().pause {
                    pause_color = Color32::GREEN;
                    pause_label = format!("Run");
                }

                menu::menu_button(ui, RichText::new("SIMULATE").strong(), |ui| {
                    if ui.button(RichText::new("Normal").strong().color(Color32::GREEN)).clicked() {
                        let mut settings = get_settings();
                        settings.sim_speed = 1.0;
                        set_settings(settings);
                    }
                    if ui.button(RichText::new(accel_label).strong().color(Color32::LIGHT_BLUE)).clicked() {
                        let mut settings = get_settings();
                        settings.sim_speed += 1.0;
                        set_settings(settings);
                    }
                    if ui.button(RichText::new(deccel_label).strong().color(deccel_color)).clicked() {
                        if sim_speed() > 1.0 {
                            let mut settings = get_settings();
                            settings.sim_speed -= 1.0;
                            set_settings(settings);
                        }
                    }
                    if ui.button(RichText::new(pause_label).strong().color(pause_color)).clicked() {
                            let mut settings = get_settings();
                            settings.pause = !settings.pause;
                            set_settings(settings);
                    }
                });


                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                
                menu::menu_button(ui, RichText::new("SELECTION").strong(), |ui| {
                    if ui.button(RichText::new("Random").strong().color(Color32::GREEN)).clicked() {
                        let mut settings = get_settings();
                        settings.select_mode = SelectMode::RANDOM;
                        set_settings(settings);
                    }
                    if ui.button(RichText::new("Points").strong().color(Color32::GREEN)).clicked() {
                        let mut settings = get_settings();
                        settings.select_mode = SelectMode::POINTS;
                        set_settings(settings);
                    }
                    if ui.button(RichText::new("Lifetime").strong().color(Color32::GREEN)).clicked() {
                        let mut settings = get_settings();
                        settings.select_mode = SelectMode::LIFETIME;
                        set_settings(settings);
                    }
                    if ui.button(RichText::new("Childs").strong().color(Color32::GREEN)).clicked() {
                        let mut settings = get_settings();
                        settings.select_mode = SelectMode::CHILDS;
                        set_settings(settings);
                    }
                    if ui.button(RichText::new("Kills").strong().color(Color32::GREEN)).clicked() {
                        let mut settings = get_settings();
                        settings.select_mode = SelectMode::KILLS;
                        set_settings(settings);
                    }
                });


                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                menu::menu_button(ui, RichText::new("SETTINGS").strong(), |ui| {
                    if ui.button(RichText::new("Agent Settings").strong().color(Color32::YELLOW)).clicked() {
                        self.state.set_agent = !self.state.set_agent;
                    }
                    if ui.button(RichText::new("Enviroment Settings").strong().color(Color32::YELLOW)).clicked() {
                        self.state.environment = !self.state.environment;
                    }
                    if ui.button(RichText::new("Neuro Settings").strong().color(Color32::YELLOW)).clicked() {
                        self.state.neuro_settings = !self.state.neuro_settings;
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                menu::menu_button(ui, RichText::new("CAMERA").strong(), |ui| {
                    if ui.button(RichText::new("Follow Mode").strong().color(Color32::GOLD)).clicked() {
                        let mut settings = get_settings();
                        settings.follow_mode = !settings.follow_mode;
                        set_settings(settings);
                    }
                    if ui.button(RichText::new("Show Name").strong().color(Color32::GOLD)).clicked() {
                        let mut settings = get_settings();
                        settings.show_specie = !settings.show_specie;
                        set_settings(settings);
                    }
                    if ui.button(RichText::new("Show Generation").strong().color(Color32::GOLD)).clicked() {
                        let mut settings = get_settings();
                        settings.show_generation = !settings.show_generation;
                        set_settings(settings);
                    }
                    if ui.button(RichText::new("Show Energy Bar").strong().color(Color32::GOLD)).clicked() {
                        let mut settings = get_settings();
                        settings.agent_eng_bar = !settings.agent_eng_bar;
                        set_settings(settings);
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                menu::menu_button(ui, RichText::new("VIEW").strong(), |ui| {
                    if ui.button(RichText::new("Left Panel").strong().color(Color32::LIGHT_BLUE)).clicked() {
                        self.state.left_panel = !self.state.left_panel;
                    }
                    if ui.button(RichText::new("Right Panel").strong().color(Color32::LIGHT_BLUE)).clicked() {
                        self.state.right_panel = !self.state.right_panel;
                    }
                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);
                    if ui.button(RichText::new("Plot: attributes").strong().color(Color32::GOLD)).clicked() {
                        self.state.plot_attributes = !self.state.plot_attributes;
                    }
                    if ui.button(RichText::new("Plot: population/kills").strong().color(Color32::GOLD)).clicked() {
                        self.state.plot_population = !self.state.plot_population;
                    }
                    if ui.button(RichText::new("Plot: lifetime/points").strong().color(Color32::GOLD)).clicked() {
                        self.state.plot_lifetime = !self.state.plot_lifetime;
                    }
                    if ui.button(RichText::new("Plot: nodes/links").strong().color(Color32::GOLD)).clicked() {
                        self.state.plot_neuro = !self.state.plot_neuro;
                    }
                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);
                    if ui.button(RichText::new("Monitor").strong().color(Color32::LIGHT_GREEN)).clicked() {
                        self.state.monit = !self.state.monit;
                    }
                    if ui.button(RichText::new("Inspector").strong().color(Color32::LIGHT_GREEN)).clicked() {
                        self.state.inspect = !self.state.inspect;
                    }
                    if ui.button(RichText::new("Neural Network").strong().color(Color32::LIGHT_GREEN)).clicked() {
                        //self.state.neuro_lab = !self.state.neuro_lab;
                        let mut settings = get_settings();
                        settings.show_network = !settings.show_network;
                        set_settings(settings);
                    }
                    if ui.button(RichText::new("Ranking").strong().color(Color32::LIGHT_GREEN)).clicked() {
                        self.state.ranking = !self.state.ranking;
                    }
                    if ui.button(RichText::new("Ancestors").strong().color(Color32::LIGHT_GREEN)).clicked() {
                        self.state.ancestors = !self.state.ancestors;
                    }
                    if ui.button(RichText::new("Plants").strong().color(Color32::LIGHT_GREEN)).clicked() {
                        self.state.plants = !self.state.plants;
                    }
                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);
                    if ui.button(RichText::new("Debug Info").strong().color(Color32::LIGHT_RED)).clicked() {
                        self.state.mouse = !self.state.mouse;
                    }
                    if ui.button(RichText::new("Show Mutations Stats").strong().color(Color32::LIGHT_RED)).clicked() {
                        self.state.info = !self.state.info;
                    }
                    if ui.button(RichText::new("Physic Debuger").strong().color(Color32::LIGHT_BLUE)).clicked() {
                        let mut settings = get_settings();
                        settings.debug = ! settings.debug;
                        set_settings(settings);
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                menu::menu_button(ui, RichText::new("ABOUT").strong(), |ui| {
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

    fn build_main_menu_win(&mut self, egui_ctx: &Context) {
        if self.state.main_menu {
            Window::new("EVOLVE 2").default_pos((SCREEN_WIDTH / 2.0 - 180.0, SCREEN_HEIGHT / 4.0)).default_width(360.0).show(egui_ctx, |ui| {
                let big_logo = self.big_logo.clone().unwrap();
                ui.vertical_centered(|pic| {
                    pic.image(big_logo.id(), big_logo.size_vec2());
                });
                ui.add_space(10.0);
                ui.vertical_centered(|title| {
                    title.heading(RichText::new("EVOLUTION 2").strong().color(Color32::GREEN).strong());
                });
                ui.add_space(6.0);
                ui.vertical_centered(|author| {
                    author.label(RichText::new("Artur Gwoździowski 2023-2024").color(Color32::BLUE).strong());
                });
                ui.add_space(6.0);
                ui.vertical_centered(|author| {
                    author.label(RichText::new(format!("version {}", env!("CARGO_PKG_VERSION"))).color(Color32::YELLOW).italics());
                });
                ui.add_space(10.0);
                ui.vertical_centered(|row| {
                    row.heading(RichText::new("MAIN MENU").strong().color(Color32::LIGHT_GRAY));
                });
                ui.add_space(16.0);
                ui.vertical_centered(|row| {
                    row.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::new(2.0, Color32::GREEN);
                    row.style_mut().visuals.widgets.active.bg_stroke = Stroke::new(5.0, Color32::GREEN);
                    row.style_mut().visuals.widgets.active.weak_bg_fill = Color32::DARK_GREEN;
                    row.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::DARK_GREEN;
                    if row.add(Button::new(RichText::new("NEW SIMULATION").strong()).min_size(UIVec2::new(160., 35.))).clicked() {
                        self.state.main_menu = false;
                        self.state.new_sim = true;
                        self.state.gen_random_name = true;
                    }
                });
                ui.add_space(16.0);
                ui.vertical_centered(|row| {
                    row.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::new(2.0, Color32::BLUE);
                    row.style_mut().visuals.widgets.active.bg_stroke = Stroke::new(5.0, Color32::BLUE);
                    row.style_mut().visuals.widgets.active.weak_bg_fill = Color32::DARK_BLUE;
                    row.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::DARK_BLUE;
                    if row.add(Button::new(RichText::new("LOAD SIMULATION").strong()).min_size(UIVec2::new(160., 35.))).clicked() {
                        self.state.main_menu = false;
                        self.state.load_sim = true;
                    }
                });
                ui.add_space(16.0);
                ui.vertical_centered(|row| {
                    row.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::new(2.0, Color32::RED);
                    row.style_mut().visuals.widgets.active.bg_stroke = Stroke::new(5.0, Color32::RED);
                    row.style_mut().visuals.widgets.active.weak_bg_fill = Color32::DARK_RED;
                    row.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::DARK_RED;
                    if row.add(Button::new(RichText::new("QUIT").strong()).min_size(UIVec2::new(160., 35.))).clicked() {
                        self.state.main_menu = false;
                        self.state.quit = true;
                    }
                });
                ui.add_space(4.0);
            });
        }
    }

    fn build_load_sim_window(&mut self, egui_ctx: &Context) {
        if self.state.load_sim {
            let mut signals = get_signals();
            let mut saved_sims: Vec<String> = vec![];
            let path = Path::new("saves\\simulations\\");
            let sims =  fs::read_dir(path).unwrap();
            for entry in sims {
                if let Ok(sim_dir) = entry {
                    let path = sim_dir.path();
                    if path.is_dir() {
                        let sim_name = path.file_name().unwrap().to_str().unwrap().to_owned();
                        saved_sims.push(sim_name);
                    }
                }
            }
            Window::new("LOAD SIMULATION").default_pos((SCREEN_WIDTH / 2.0 - 65.0, SCREEN_HEIGHT / 4.0)).default_width(260.0).show(egui_ctx, |ui| {
                for sim in saved_sims {
                    ui.vertical_centered(|row| {
                        row.columns(2, |columns| {
                            columns[0].label(RichText::new(sim.to_owned().to_uppercase()).strong().color(Color32::WHITE));
                            columns[1].horizontal(|col| {
                                    if col.button(RichText::new("[LOAD]").strong().color(Color32::GREEN)).clicked()  {
                                        signals.load_sim_name = Some(String::from(&sim));
                                        set_global_signals(signals.clone());
                                        self.state.load_sim = false;
                                    }
                                    col.separator();
                                    if col.button(RichText::new("[DEL]").strong().color(Color32::RED)).clicked()  {
                                        signals.del_sim_name = Some(String::from(&sim));
                                        set_global_signals(signals.clone());
                                        self.state.load_sim = false;
                                    }
                            })
                        })
                    });
                    ui.add_space(4.0);
                }
                ui.add_space(16.0);
                
                ui.vertical_centered(|ctn| {
                    if ctn.button(RichText::new("CLOSE").strong().color(Color32::YELLOW)).clicked() {
                        self.state.load_sim = false;
                    }
                })
            });
        }
    }

    fn build_load_agent_window(&mut self, egui_ctx: &Context) {
        if self.state.load_agent {
            let mut signals = get_signals();
            let mut saved_agents: Vec<String> = vec![];
            let path = Path::new("saves\\agents\\");
            let agents =  fs::read_dir(path).unwrap();
            for entry in agents {
                if let Ok(agent_dir) = entry {
                    let path = agent_dir.path();
                    if path.is_file() {
                        let ext = path.extension();
                        match ext {
                            None => {},
                            Some(ext) => {
                                if ext == "agent" {
                                    let agent_name = path.file_name().unwrap().to_str().unwrap().to_owned();
                                    saved_agents.push(agent_name);
                                }
                            },
                        }
                    }
                }
            }
            let mut list_of_files: Vec<PathBuf> = vec![];
            for agent_name in saved_agents.iter() {
                let p = format!("saves\\agents\\{}", &agent_name);
                let path_to_agent = Path::new(&p);
                list_of_files.push(path_to_agent.to_path_buf());
            }

            let mut sketches: Vec<AgentSketch> = vec![];
            for f in list_of_files {
                match fs::read_to_string(f) {
                    Ok(file) => {
                        match BASE64_STANDARD.decode(file.clone().into_bytes()) {
                            Err(e) => {
                                println!("ERROR: can't decode base64 of saved agent");
                                eprintln!("{}", e);
                            },
                            Ok(decoded) => {
                                let save = String::from_utf8(decoded).expect("error during decode Vec<u8> to String");
                                match serde_json::from_str::<AgentSketch>(&save) {
                                    Ok(sketch) => {
                                        sketches.push(sketch.clone());
                                    },
                                    Err(e) => {
                                        eprintln!("Error deserializing save file: {:?}", e);
                                    },
                                }
                            },
                        }
                    },
                    Err(e) => {
                        eprintln!("Error reading save file: {:?}", e);
                    },
                }
            }

            Window::new("LOAD AGENT").default_pos((SCREEN_WIDTH / 2.0 - 65.0, SCREEN_HEIGHT / 4.0)).default_width(260.0).show(egui_ctx, |ui| {
                for agent in sketches {
                    ui.vertical_centered(|row| {
                        row.columns(2, |columns| {
                            let txt = format!("{} | G:{} ", agent.specie.to_uppercase(), agent.generation);
                            let filename = format!("{}-{}.agent", agent.specie, agent.generation);
                            columns[0].label(RichText::new(txt).strong().color(Color32::WHITE));
                            columns[1].horizontal(|col| {
                                if col.button(RichText::new("[LOAD]").strong().color(Color32::GREEN)).clicked()  {
                                    signals.load_agent_name = Some(filename.clone());
                                    set_global_signals(signals.clone());
                                }
                                col.separator();
                                if col.button(RichText::new("[DEL]").strong().color(Color32::RED)).clicked()  {
                                    signals.del_agent_name = Some(String::from(filename.clone()));
                                    set_global_signals(signals.clone());
                                }
                            });
                        });
                    });
                    ui.add_space(4.0);
                }
                ui.add_space(16.0);
                
                ui.vertical_centered(|ctn| {
                    if ctn.button(RichText::new("CLOSE").strong().color(Color32::YELLOW)).clicked() {
                        self.state.load_agent = false;
                    }
                })
            });
        }
    }

/*     fn build_load_sim_window(&mut self, egui_ctx: &Context) {
        if self.state.load_sim {
            let mut signals = get_signals();
            let mut saved_sims: Vec<SimulationSketch> = vec![];
            let path = Path::new("saves\\simulations\\");
            let sims =  fs::read_dir(path).unwrap();
            for entry in sims {
                if let Ok(sim_dir) = entry {
                    let mut path = sim_dir.path();
                    let p = path.join("last.sim").as_path().to_owned();
                    let last = Path::from(*p);
                    let f = fs::read_to_string(path).unwrap();
                    match BASE64_STANDARD.decode(f.clone().into_bytes()) {
                        Ok(decoded) => {
                            let save = String::from_utf8(decoded).expect("error during converting from utf8");
                            match serde_json::from_str::<SimulationSketch>(&save) {
                                Err(_) => {},
                                Ok(sim_state) => {
                                    saved_sims.push(sim_state);
                                },
                            }
                        },
                        Err(_) => {},
                    }
                }
            }
            Window::new("LOAD SIMULATION").default_pos((SCREEN_WIDTH / 2.0 - 65.0, SCREEN_HEIGHT / 4.0)).default_width(260.0).show(egui_ctx, |ui| {
                for sim in saved_sims {
                    ui.vertical_centered(|row| {
                        row.columns(2, |columns| {
                            let sim_info = format!("{} T:{}", sim.simulation_name, sim.sim_time);
                            columns[0].label(RichText::new(sim_info.to_owned().to_uppercase()).strong().color(Color32::WHITE));
                            columns[1].horizontal(|col| {
                                    if col.button(RichText::new("[LOAD]").strong().color(Color32::GREEN)).clicked()  {
                                        signals.load_sim_name = Some(String::from(&sim.simulation_name));
                                        set_global_signals(signals.clone());
                                        self.state.load_sim = false;
                                    }
                                    col.separator();  
                                    if col.button(RichText::new("[DEL]").strong().color(Color32::RED)).clicked()  {
                                        signals.del_sim_name = Some(String::from(&sim.simulation_name));
                                        set_global_signals(signals.clone());
                                        self.state.load_sim = false;
                                    }
                            })
                        })
                    });
                    ui.add_space(4.0);
                }
                ui.add_space(16.0);
                
                ui.vertical_centered(|ctn| {
                    if ctn.button(RichText::new("CLOSE").strong().color(Color32::YELLOW)).clicked() {
                        self.state.load_sim = false;
                    }
                })
            });
        }
    } */

    fn build_debug_window(&self, egui_ctx: &Context, camera2d: &Camera2D, sim_state: &SimState, agent: Option<&Agent>) {
        if self.state.mouse {
            let (mouse_x, mouse_y) = mouse_position();
            Window::new("DEBUG INFO").default_pos((375.0, 5.0)).default_width(175.0).show(egui_ctx, |ui| {
                let fps = sim_state.fps;
                let delta = sim_state.dt;
                //ui.label(RichText::new("PERFORMANCE").strong().color(Color32::BLUE));
                ui.label(format!("FPS: {} dT: {}ms", fps, (delta * 1000.0).round()));
                ui.separator();
                ui.label(RichText::new("MOUSE").strong().color(Color32::YELLOW));
                ui.label(format!("coords [x: {} | y: {}]", mouse_x.round(), mouse_y.round()));
                ui.separator();
                ui.label(RichText::new("CAMERA").strong().color(Color32::LIGHT_BLUE));
                ui.label(format!("target [x:{} | Y:{}]", camera2d.target.x.round(), camera2d.target.y.round()));
                ui.label(format!("offset [x:{} | Y:{}]", camera2d.offset.x.round(), camera2d.offset.y.round()));
                ui.label(format!("zoom [x:{} | Y:{}]", (camera2d.zoom.x * 10000.).round() / 10., (camera2d.zoom.y * 10000.).round() / 10.));
                ui.label(format!("rotation: {}", camera2d.rotation.round()));
                ui.separator();
                ui.label(RichText::new("PHYSICS CORE").strong().color(Color32::RED));
                ui.label(format!("RIGID: {}", sim_state.rigid_num));
                ui.label(format!("COLLIDERS: {}", sim_state.colliders_num));
                match agent {
                    None => {},
                    Some(agent) => {
                        ui.separator();
                        ui.label(RichText::new("AGENT").strong().color(Color32::LIGHT_BLUE));
                        match agent.enemy_dir {
                            None => {},
                            Some(enemy_dir) => {
                                ui.label(format!("target angle: {:.2}", enemy_dir));
                            },
                        }
                        match agent.enemy_position {
                            None => {},
                            Some(enemy_pos) => {
                                let rel_pos = enemy_pos - agent.pos;
                                ui.label(format!("target pos: [x: {} | y: {}]", rel_pos.x.round(), rel_pos.y.round()));
                            },
                        }
                        match agent.plant_dir {
                            None => {},
                            Some(res_dir) => {
                                ui.label(format!("plant angle: {:.2}", res_dir));
                            },
                        }
                        match agent.plant_position {
                            None => {},
                            Some(res_pos) => {
                                let rel_pos = res_pos - agent.pos;
                                ui.label(format!("plant pos: [x: {} | y: {}]", rel_pos.x.round(), rel_pos.y.round()));
                            },
                        }
                    },
                }
            });
        }
    }

    fn build_quit_window(&mut self, egui_ctx: &Context) {
        if self.state.quit {
            Window::new("QUIT").default_pos((SCREEN_WIDTH / 2.0 - 65.0, SCREEN_HEIGHT / 4.0)).default_width(200.0).show(egui_ctx, |ui| {
                ui.horizontal(|head| {
                    head.vertical_centered(|head| {
                        head.heading("Are you sure?");
                    });
                });
                ui.horizontal(|mid| {
                    mid.columns(2, |columns| {
                        columns[0].style_mut().visuals.widgets.inactive.bg_stroke = Stroke::new(2.0, Color32::BLUE);
                        columns[0].style_mut().visuals.widgets.active.bg_stroke = Stroke::new(5.0, Color32::BLUE);
                        columns[0].style_mut().visuals.widgets.active.weak_bg_fill = Color32::DARK_BLUE;
                        columns[0].style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::DARK_BLUE;
                        if columns[0].add(Button::new(RichText::new("NO").strong()).min_size(UIVec2::new(75.0, 40.0))).clicked() {
                            self.state.quit = false;
                        }
                        columns[1].style_mut().visuals.widgets.inactive.bg_stroke = Stroke::new(2.0, Color32::RED);
                        columns[1].style_mut().visuals.widgets.active.bg_stroke = Stroke::new(5.0, Color32::RED);
                        columns[1].style_mut().visuals.widgets.active.weak_bg_fill = Color32::DARK_RED;
                        columns[1].style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::DARK_RED;
                        //let text = columns[1].style_mut().text_styles();
                        
                        if columns[1].add(Button::new(RichText::new("YES").strong()).min_size(UIVec2::new(75.0, 40.0))).clicked() {
                            std::process::exit(0);
                        }
                    });
                });
            });
        }
    }

    fn build_new_sim_window(&mut self, egui_ctx: &Context, signals: &mut Signals) {
        if self.state.new_sim {
            let names0 = vec![
                "NEW", "IDEAL", "DANGER", "DARK", "FIRST", "EXPERIMENTAL", 
                "RANDOM", "STRANGE", "MODERN", "OLD", "SIMPLE", "HISTORICAL",
                "DEEP", "DIGITAL", "SECOND", "GREAT", "LAST", 
                "ANOTHER", "NEXT", "TOXIC"
            ];
            let names1 = vec![
                "SIMULATION", "UNIVERSE", "WORLD", "LAND", "PLANET", "SIM",
                "REALITY", "ISLAND", "NATURE", "ECOSYSTEM",
                "SYSTEM", "TERRAIN", "GLOBE", "REALM", "COSMOS"
            ];

            let mut settings = get_settings();
            let w = 500.0; let h = 220.0;
            Window::new("EVOLVE").default_pos((SCREEN_WIDTH / 2.0 - w/2.0, 100.0)).default_size([w, h]).show(egui_ctx, |ui| {
                let big_logo = self.big_logo.clone().unwrap();
                //let title = self.title.clone().unwrap();
                ui.vertical_centered(|pic| {
                    pic.image(big_logo.id(), big_logo.size_vec2());
                });
                /* ui.add_space(4.0);
                ui.vertical_centered(|pic| {
                    pic.image(title.id(), title.size_vec2()*0.7);
                }); */
                ui.add_space(1.0);
                ui.vertical_centered(|author| {
                    let txt = format!("Artur Gwoździowski 2023  |  ver.{}", env!("CARGO_PKG_VERSION"));
                    author.label(RichText::new(txt).color(Color32::RED).strong());
                });
                ui.add_space(6.0);
                ui.separator();
                ui.add_space(6.0);
                ui.vertical_centered(|head| {
                    head.heading(RichText::new("NEW SIMULATION").color(Color32::GREEN).strong());
                });
                ui.add_space(3.0);
                ui.vertical_centered(|txt| {
                    let response = txt.text_edit_singleline(&mut self.temp_sim_name);
                    let rnd_btn = txt.add(
                        Button::image_and_text(
                            self.dice.clone().unwrap().id(), 
                            egui_macroquad::egui::Vec2::new(24.0, 24.0), 
                            WidgetText::from("")
                        )
                        .fill(Color32::TRANSPARENT).stroke(Stroke::new(1.5, Color32::GREEN))
                        .min_size(egui_macroquad::egui::Vec2::new(32.0, 32.0))
                    );
                    if rnd_btn.clicked() {
                        self.state.gen_random_name = true;
                    }
                    if self.state.gen_random_name {
                        self.state.gen_random_name = false;
                        let l0 = names0.len();
                        let l1 = names1.len();
                        let n0 = rand::gen_range(0, l0);
                        let n1 = rand::gen_range(0, l1);
                        let name0 = names0.get(n0).unwrap();
                        let name1 = names1.get(n1).unwrap();
                        //let id = rand::gen_range(100, 999);
                        self.temp_sim_name = format!("{} {}",name0.to_uppercase(), name1.to_uppercase());
                    }
                    if response.gained_focus() {
                    }
                    //if response.changed() {
                    //    response.
                    //}
                    if response.lost_focus() && txt.input(|i| i.key_pressed(Key::Enter)) {
                        self.state.new_sim = false;
                        signals.new_sim = true;
                        signals.new_sim_name = String::from(&self.temp_sim_name);
                        //self.temp_sim_name = String::new();
                    }
                });
                ui.add_space(3.0);
                ui.spacing();
                ui.vertical_centered(|row| {
                    row.label("WORLD SIZE [X | Y]");
                });
                ui.add_space(2.0);
                ui.vertical_centered(|row| {
                    row.style_mut().spacing.slider_width = 220.0;
                    let mut w = settings.world_w;
                    let mut h = settings.world_h;
                    row.columns(2, |columns| {
                        if columns[0].add(Slider::new(&mut w, 800..=10000).step_by(100.0)).changed() {
                            settings.world_w = w;
                            set_settings(settings.clone());
                        }
                        if columns[1].add(Slider::new(&mut h, 600..=7500).step_by(100.0)).changed() {
                            settings.world_h = h;
                            set_settings(settings.clone());
                        }
                    });
                });
                ui.add_space(4.0);
                ui.spacing();
                ui.vertical_centered(|mid| {
                    mid.columns(2, |columns| {
                        if columns[0].button(RichText::new("NO").color(Color32::YELLOW).strong()).clicked() {
                            self.state.new_sim = false;
                            self.temp_sim_name = String::new();
                        }
                        if columns[1].button(RichText::new("YES").color(Color32::BLUE).strong()).clicked() {
                            self.state.new_sim = false;
                            signals.new_sim = true;
                            signals.new_sim_name = String::from(&self.temp_sim_name);
                            self.temp_sim_name = String::new();
                        }
                    });
                });
                ui.add_space(3.0);
            });
        }
    }

    fn build_resize_world_window(&mut self, egui_ctx: &Context) {
        if self.state.resize_world {
            let win_w = 500.0; let win_h = 220.0;
            //let mut settings = get_settings();
            let xy = self.temp_values.world_size.unwrap_or(vec2(1800.0, 900.0));
            let mut w = xy.x; let mut h = xy.y;
            Window::new("WORD RESIZE").default_pos((SCREEN_WIDTH / 2.0 - win_w/2.0, 100.0)).default_size([win_w, win_h]).show(egui_ctx, |ui| {
                //let title = self.title.clone().unwrap();
                ui.vertical_centered(|head| {
                    head.heading(RichText::new("WORD RESIZE").color(Color32::GREEN).strong());
                });
                ui.add_space(3.0);
                ui.vertical_centered(|row| {
                    row.label("WORLD SIZE [X | Y]");
                });
                ui.add_space(2.0);
                let mut new_xy = xy;
                ui.vertical_centered(|row| {
                    row.style_mut().spacing.slider_width = 220.0;
                    row.columns(2, |columns| {
                        if columns[0].add(Slider::new(&mut w, 800.0..=10000.0).step_by(100.0)).changed() {
                            new_xy = vec2(w, h);
                        }
                        if columns[1].add(Slider::new(&mut h, 600.0..=7500.0).step_by(100.0)).changed() {
                            new_xy = vec2(w, h);
                        }
                    });
                });
                self.temp_values.world_size = Some(new_xy);
                ui.add_space(4.0);
                ui.spacing();
                ui.vertical_centered(|mid| {
                    mid.columns(2, |columns| {
                        if columns[0].button(RichText::new("CANCEL").color(Color32::YELLOW).strong()).clicked() {
                            self.state.resize_world = false;
                            self.temp_values.world_size = None;
                            let mut signals = get_signals();
                            signals.resize_world = None;
                            set_global_signals(signals);
                        }
                        if columns[1].button(RichText::new("APPLY").color(Color32::BLUE).strong()).clicked() {
                            self.state.resize_world = false;
                            let mut signals = get_signals();
                            signals.resize_world = self.temp_values.world_size;
                            set_global_signals(signals);
                        }
                    });
                });
                ui.add_space(3.0);
            });
        }
    }

    fn build_ancestors_window(&self, egui_ctx: &Context, agent: &Agent) {
        if self.state.ancestors {
            let ancestors = agent.ancestors();
            Window::new(RichText::new("Ancestors").strong().color(Color32::WHITE)).default_pos((800.0, 0.0)).min_width(280.0).show(egui_ctx, |ui| {
                for a in ancestors.iter() {
                    let (name, gen, time) = a.get_name_gen_time();
                    ui.horizontal(|row| {
                        row.label(RichText::new(format!("{} | G:{} | T:{}", name.to_uppercase(), gen, time)).strong().color(Color32::WHITE));
                    });
                }
            });
        }
    }

    fn build_plant_window(&self, egui_ctx: &Context, plant: &Plant) {
        if self.state.plants {
            let size = plant.size as i32;
            let max_eng = plant.max_eng;
            let eng = plant.eng;
            let lifetime = plant.time;
            let attributes = format!("ENG: {:.0}/{:.0} | SIZE: {} | LIFETIME: {:.0}",eng, max_eng, size, lifetime);
            let title_txt = format!("Plants"); 
            Window::new(RichText::new(title_txt).strong().color(Color32::GREEN)).default_pos((800.0, 0.0)).min_width(100.0).show(egui_ctx, |ui| {
                ui.horizontal(|row| {
                    row.label(RichText::new(attributes).strong());
                });
            });
        }
    }

    fn build_network(&mut self, egui_ctx: &Context, agent: &Agent) {
        if self.state.neuro_lab {
            let w = 300.0; let h = 360.0; let resize = egui_macroquad::egui::Vec2::new(3.0, 3.6);
            Window::new(RichText::new("NEURO NETWORK").strong().color(Color32::GREEN)).default_pos((SCREEN_WIDTH-w, 0.0)).fixed_size((w, h)).show(egui_ctx, |ui| {
                let network = &agent.network;
                let offset = UIVec2::new(0.0, 0.0);
                let (response, painter) = ui.allocate_painter(UIVec2::new(w, h), Sense::hover());
                let rect = response.rect;
                let zero = rect.left_top().to_vec2()+offset;
                //let (input_node_keys, hidden_node_keys, output_node_keys) = network.get_node_keys_by_type();
                
                for (_, link) in network.links.iter() {
                    let (coord0, coord1, _coord_t) = link.get_coords(&network.nodes, 0.0);
                    let ui_coord0 = vec2_to_uivec2(&coord0);
                    let ui_coord1 = vec2_to_uivec2(&coord1);
                    let w = link.get_width()*1.2;
                    let p1 = vec2_to_pos2(&(ui_coord0*resize+zero));
                    let p2 = vec2_to_pos2(&(ui_coord1*resize+zero));
                    let (_, color1) = link.get_colors();
                    let c1 = color_to_color32(color1);
                    let points1 = [p1, p2];
                    painter.line_segment(points1, Stroke { color: c1, width: w });
                }
                for (key, node) in network.nodes.iter() {
                    let (_, color1) = node.get_colors();
                    let (mut r0, mut r1) = node.get_size();
                    r0 = r0*1.2;
                    r1 = r1*1.2;
                    let ipos = egui_macroquad::egui::Vec2::new(node.pos.x as f32, node.pos.y as f32)*resize+zero;
                    let p1 = vec2_to_pos2(&ipos);
                    let c0 = color_to_color32(color1);
                    //let c1 = color_to_color32(color0);
                    let label = node.get_label();
                    let v = match network.get_node_value(key) {
                        None => 0.0,
                        Some(v) => v,
                    };
                    painter.circle_filled(p1, r0,  Color32::BLACK);
                    //painter.circle_filled(p1, r,  c1);
                    let w = 0.75 + 0.24*r0;
                    painter.circle_stroke(p1, r0, Stroke { color: c0, width: w });
                    //painter.circle_filled(p1, r0,  Color32::BLACK);
                    //painter.circle_filled(p1, r,  c1);
                    let w = 0.75 + 0.24*r1;
                    painter.circle_stroke(p1, r1, Stroke { color: c0, width: w });
                    let mut font = FontId::default();
                    font.size = 8.0;
                    let txt = format!("{}: {:.1}", label, v);
                    match node.node_type {
                        NeuronTypes::INPUT => {
                            painter.text(p1+UIVec2{x: 8.0, y: 0.0}, Align2::LEFT_CENTER, txt, font, Color32::WHITE);
                        },
                        NeuronTypes::OUTPUT => {
                            painter.text(p1+UIVec2{x: -50.0, y: 0.0}, Align2::LEFT_CENTER, txt, font, Color32::WHITE);
                        },
                        _ => {},
                    }
                }
            });
        }
    }

    fn build_about_window(&mut self, egui_ctx: &Context) {
        if self.state.about {
            Window::new("ABOUT").resizable(false).default_pos((SCREEN_WIDTH/2.-150., SCREEN_HEIGHT/6.)).min_height(380.).min_width(300.)
            .title_bar(true).show(egui_ctx, |ui| {
                let big_logo = self.big_logo.clone().unwrap();
                ui.vertical_centered(|pic| {
                    pic.image(big_logo.id(), big_logo.size_vec2());
                });
                ui.add_space(10.0);
                ui.vertical_centered(|author| {
                    author.label(RichText::new("Artur Gwoździowski 2023-2024").color(Color32::GREEN).strong());
                });
                ui.add_space(10.0);
                ui.vertical_centered(|author| {
                    author.label(RichText::new(format!("version {}", env!("CARGO_PKG_VERSION"))).color(Color32::YELLOW).italics());
                });
                ui.add_space(10.0);
                ui.vertical_centered(|author| {
                    author.label(RichText::new(format!("{}", env!("CARGO_PKG_DESCRIPTION"))).color(Color32::WHITE).italics());
                });
                ui.add_space(10.0);
                ui.vertical_centered(|closer| {
                    if closer.button(RichText::new("CLOSE").color(Color32::LIGHT_BLUE).strong()).clicked() {
                        self.state.about = false;
                    }
                });
            });
        }
    }

    fn build_info_window(&mut self, egui_ctx: &Context) {
        if self.state.info {
            let mutations = get_mutations();
            let na = mutations.nodes_added; let nd = mutations.nodes_deleted; let la = mutations.links_added; let ld = mutations.links_deleted;
            let w = mutations.weights_changed; let b = mutations.biases_changed;
            let text = format!("NODES: [added: {na} | del: {nd}] LINKS: [added: {la} | del: {ld}] MOD: [w: {w} | b: {b}]");
            Window::new("INFO").resizable(false).default_pos((SCREEN_WIDTH/2.-150., SCREEN_HEIGHT/3.)).min_height(380.).min_width(300.)
            .title_bar(true).show(egui_ctx, |ui| {
                ui.vertical_centered(|row| {
                    row.label(RichText::new(text).color(Color32::LIGHT_BLUE).strong());
                });
                ui.add_space(10.0);
                ui.vertical_centered(|closer| {
                    if closer.button(RichText::new("OK").color(Color32::GREEN).strong()).clicked() {
                        self.state.info = false;
                    }
                });
            });
        }
    }

    fn build_settings_agent_window(&mut self, egui_ctx: &Context, signals: &mut Signals) {
        if !self.state.set_agent {
            return;
        }
        let mut settings = get_settings();
        Window::new("AGENT SETTINGS").id("agent_settings_win".into()).default_pos((SCREEN_WIDTH/2., SCREEN_HEIGHT/2.)).fixed_size([380., 400.])
        .title_bar(true).show(egui_ctx, |ui| {
            ui.heading("AGENT SETTINGS");
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut agent_speed: i32 = settings.agent_speed as i32;
                column[0].label(RichText::new("SPEED").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut agent_speed, 0..=60)).changed() {
                    settings.agent_speed = agent_speed as f32;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut agent_rotate = settings.agent_rotate;
                column[0].label(RichText::new("AGILITY").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut agent_rotate, 0.0..=50.0).step_by(1.0)).changed() {
                    settings.agent_rotate = agent_rotate;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut base_energy_cost: f32 = settings.base_energy_cost;
                column[0].label(RichText::new("BASE ENG COST").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut base_energy_cost, 0.0..=1.0).step_by(0.01)).changed() {
                    settings.base_energy_cost = base_energy_cost;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut move_energy_cost: f32 = settings.move_energy_cost;
                column[0].label(RichText::new("MOVE ENG COST").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut move_energy_cost, 0.0..=1.0).step_by(0.01)).changed() {
                    settings.move_energy_cost = move_energy_cost;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut attack_energy_cost: f32 = settings.attack_energy_cost;
                column[0].label(RichText::new("ATTACK ENG COST").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut attack_energy_cost, 0.0..=1.0).step_by(0.01)).changed() {
                    settings.attack_energy_cost = attack_energy_cost;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut size_cost: f32 = settings.size_cost;
                column[0].label(RichText::new("SIZE COST").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut size_cost, 0.0..=5.0).step_by(0.1)).changed() {
                    settings.size_cost = size_cost;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut eng_bias: f32 = settings.eng_bias;
                column[0].label(RichText::new("ENG BIAS").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut eng_bias, 0.0..=1.0).step_by(0.05)).changed() {
                    settings.eng_bias = eng_bias;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut base_hp: i32 = settings.base_hp;
                column[0].label(RichText::new("BASE HP").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut base_hp, 0..=1000).step_by(10.0)).changed() {
                    settings.base_hp = base_hp;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut size_to_hp: f32 = settings.size_to_hp;
                column[0].label(RichText::new("SIZE TO HP").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut size_to_hp, 0.0..=200.0).step_by(5.0)).changed() {
                    settings.size_to_hp = size_to_hp;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut damage: i32 = settings.damage as i32;
                column[0].label(RichText::new("DAMAGE").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut damage, 0..=100).step_by(1.0)).changed() {
                    settings.damage = damage as f32;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut agent_vision_range: i32 = settings.agent_vision_range as i32;
                column[0].label(RichText::new("VISION").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut agent_vision_range, 10..=1000)).changed() {
                    settings.agent_vision_range = agent_vision_range as f32;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut peripheral_vision = settings.peripheral_vision;
                column[0].label(RichText::new("PERIPHERAL VISION").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut peripheral_vision, 0.0..=1.0).step_by(0.05)).changed() {
                    settings.peripheral_vision = peripheral_vision;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut mutations: f32 = settings.mutations;
                column[0].label(RichText::new("MUTATIONS").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut mutations, 0.0..=0.5).step_by(0.05)).changed() {
                    settings.mutations = mutations;
                    signals.new_settings = true;
                }
            });
            ui.style_mut().spacing.slider_width = 75.0;
            ui.columns(3, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(140., 75.));
                column[2].set_max_size(UIVec2::new(140., 75.));
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
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut atk_to_eng = settings.atk_to_eng;
                column[0].label(RichText::new("ATACK TO ENG").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut atk_to_eng, 0.1..=2.0).step_by(0.05)).changed() {
                    settings.atk_to_eng = atk_to_eng;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut eat_to_eng = settings.eat_to_eng;
                column[0].label(RichText::new("EAT TO ENG").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut eat_to_eng, 0.0..=15.0).step_by(0.1)).changed() {
                    settings.eat_to_eng = eat_to_eng;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut rare_specie_mod = settings.rare_specie_mod;
                column[0].label(RichText::new("SPECIE MODIFY RARITY").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut rare_specie_mod, 0..=10000).step_by(100.0)).changed() {
                    settings.rare_specie_mod = rare_specie_mod;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut dmg_to_hp = settings.dmg_to_hp;
                column[0].label(RichText::new("DMG_TO_HP").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut dmg_to_hp, 0.0..=2.0).step_by(0.1)).changed() {
                    settings.dmg_to_hp = dmg_to_hp;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut born_eng = settings.born_eng;
                column[0].label(RichText::new("CHILD BORN ENERGY").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut born_eng, 0.0..=1.0).step_by(0.05)).changed() {
                    settings.born_eng = born_eng;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut born_eng_min = settings.born_eng_min;
                column[0].label(RichText::new("MIN REPRODUCE ENERGY").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut born_eng_min, 0.0..=1.0).step_by(0.05)).changed() {
                    settings.born_eng_min = born_eng_min;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut born_eng_cost = settings.born_eng_cost;
                column[0].label(RichText::new("REPRODUCTION COST").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut born_eng_cost, 0.0..=1.0).step_by(0.05)).changed() {
                    settings.born_eng_cost = born_eng_cost;
                    signals.new_settings = true;
                }
            });
            ui.add_space(2.0);
            ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::new(2.0, Color32::DARK_GREEN);
            ui.vertical_centered(|closer| {
                if closer.button(RichText::new("CLOSE").color(Color32::GREEN).strong()).clicked() {
                    self.state.set_agent = false;
                    set_settings(settings.clone());
                }
            });
        });
        set_settings(settings.clone());
    }

    fn build_settings_enviro_window(&mut self, egui_ctx: &Context, signals: &mut Signals) {
        if !self.state.environment {
            return;
        }
        let mut settings = get_settings();
        Window::new("ENVIROMENT SETTINGS").id("enviroment_settings_win".into()).default_pos((SCREEN_WIDTH/2., SCREEN_HEIGHT/2.)).fixed_size([380., 400.])
        .title_bar(true).show(egui_ctx, |ui| {
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut ranking_size: i32 = settings.ranking_size as i32;
                column[0].label(RichText::new("RANKING SIZE").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut ranking_size, 0..=100)).changed() {
                    settings.ranking_size = ranking_size as usize;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut agents_num: i32 = settings.agent_min_num as i32;
                column[0].label(RichText::new("AGENT MIN NUMBER").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut agents_num, 0..=200)).changed() {
                    settings.agent_min_num = agents_num as usize;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut agent_init_num: i32 = settings.agent_init_num as i32;
                column[0].label(RichText::new("AGENT INIT NUMBER").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut agent_init_num, 0..=200)).changed() {
                    settings.agent_init_num = agent_init_num as usize;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut res_init_num: i32 = settings.plant_init_num as i32;
                column[0].label(RichText::new("PLANT INIT NUMBER").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut res_init_num, 0..=500)).changed() {
                    settings.plant_init_num = res_init_num as usize;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut res_min_num: i32 = settings.plant_min_num as i32;
                column[0].label(RichText::new("PLANT MIN NUMBER").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut res_min_num, 0..=500)).changed() {
                    settings.plant_min_num = res_min_num as usize;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut plant_balance: i32 = settings.plant_balance as i32;
                column[0].label(RichText::new("PLANT BALANCE").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut plant_balance, 0..=100)).changed() {
                    settings.plant_balance = plant_balance as usize;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut growth: f32 = settings.growth;
                column[0].label(RichText::new("PLANT GROWTH").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut growth, 0.0..=10.0).step_by(0.5)).changed() {
                    settings.growth = growth;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut plant_lifetime = settings.plant_lifetime;
                column[0].label(RichText::new("PLANT LIFETIME").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut plant_lifetime, 0.0..=600.0).step_by(0.01)).changed() {
                    settings.plant_lifetime = plant_lifetime;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut plant_clone_size = settings.plant_clone_size;
                column[0].label(RichText::new("PLANT CLONE SIZE").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut plant_clone_size, 1..=16).step_by(0.01)).changed() {
                    settings.plant_clone_size = plant_clone_size;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut repro_time = settings.repro_time as i32;
                column[0].label(RichText::new("REPRODUCTION TIME").color(Color32::WHITE).strong());
                if column[1].add(Slider::new::<i32>(&mut repro_time, 0..=400).step_by(1.0)).changed() {
                    settings.repro_time = repro_time as f32;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut new_one_probability = settings.new_one_probability;
                column[0].label(RichText::new("NEW AGENT PROBABILITY").color(Color32::WHITE).strong());
                if column[1].add(Slider::new::<f32>(&mut new_one_probability, 0.0..=0.5).step_by(0.01)).changed() {
                    settings.new_one_probability = new_one_probability;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut water_lvl = settings.water_lvl;
                column[0].label(RichText::new("WATER LEVEL").color(Color32::WHITE).strong());
                if column[1].add(Slider::new::<i32>(&mut water_lvl, 0..=10)).changed() {
                    settings.water_lvl = water_lvl;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(120., 75.));
                column[1].set_max_size(UIVec2::new(120., 75.));
                let mut agent_eng_bar: bool = settings.agent_eng_bar;
                column[0].label(RichText::new("SHOW ENG BAR").color(Color32::WHITE).strong());
                if column[1].add(Checkbox::without_text(&mut agent_eng_bar)).changed() {
                    settings.agent_eng_bar = agent_eng_bar;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(120., 75.));
                column[1].set_max_size(UIVec2::new(120., 75.));
                let mut show_specie: bool = settings.show_specie;
                column[0].label(RichText::new("SHOW SPECIE NAME").color(Color32::WHITE).strong());
                if column[1].add(Checkbox::without_text(&mut show_specie)).changed() {
                    settings.show_specie = show_specie;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(120., 75.));
                column[1].set_max_size(UIVec2::new(120., 75.));
                let mut show_cells: bool = settings.show_cells;
                column[0].label(RichText::new("SHOW OCCUPIED CELLS").color(Color32::WHITE).strong());
                if column[1].add(Checkbox::without_text(&mut show_cells)).changed() {
                    settings.show_cells = show_cells;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(120., 75.));
                column[1].set_max_size(UIVec2::new(120., 75.));
                let mut show_res_rad: bool = settings.show_plant_rad;
                column[0].label(RichText::new("SHOW PLANT RADIUS").color(Color32::WHITE).strong());
                if column[1].add(Checkbox::without_text(&mut show_res_rad)).changed() {
                    settings.show_plant_rad = show_res_rad;
                    //signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut grid_size = settings.grid_size;
                column[0].label(RichText::new("GRID-SIZE").color(Color32::WHITE).strong());
                if column[1].add(Slider::new::<u32>(&mut grid_size, 0..=250).step_by(10.0)).changed() {
                    settings.grid_size = grid_size;
                    signals.new_settings = true;
                }
            });
            ui.add_space(2.0);
            ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::new(2.0, Color32::DARK_GREEN);
            ui.vertical_centered(|closer| {
                //let mut stylus = closer.style();
                if closer.button(RichText::new("CLOSE").color(Color32::GREEN).strong()).clicked() {
                    self.state.environment = false;
                    set_settings(settings.clone());
                }
            });
        });
        set_settings(settings.clone());
    }

    fn build_settings_neuro_window(&mut self, egui_ctx: &Context, signals: &mut Signals) {
        if !self.state.neuro_settings {
            return;
        }
        let mut settings = get_settings();
        Window::new("NEURO SETTINGS").id("neuro_settings_win".into()).default_pos((SCREEN_WIDTH/2., SCREEN_HEIGHT/2.)).fixed_size([380., 400.])
        .title_bar(true).show(egui_ctx, |ui| {
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut neurolink_rate: f32 = settings.neurolink_rate;
                column[0].label(RichText::new("NEURON LINKS RATE").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut neurolink_rate, 0.0..=1.0).step_by(0.05)).changed() {
                    settings.neurolink_rate = neurolink_rate;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut neuro_duration: f32 = settings.neuro_duration;
                column[0].label(RichText::new("ANALIZE PERIOD").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut neuro_duration, 0.01..=1.0).step_by(0.01)).changed() {
                    settings.neuro_duration = neuro_duration;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut hidden_layers_num = settings.hidden_layers_num as i32;
                column[0].label(RichText::new("DEEP LAYERS NUMBER").color(Color32::WHITE).strong());
                if column[1].add(Slider::new::<i32>(&mut hidden_layers_num, 0..=10).step_by(1.0)).changed() {
                    settings.hidden_layers_num = hidden_layers_num as usize;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut hidden_nodes_num = settings.hidden_nodes_num as i32;
                column[0].label(RichText::new("DEEP NEURONS NUMBER").color(Color32::WHITE).strong());
                if column[1].add(Slider::new::<i32>(&mut hidden_nodes_num, 0..=10).step_by(1.0)).changed() {
                    settings.hidden_nodes_num = hidden_nodes_num as usize;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut mut_add_link = settings.mut_add_link;
                column[0].label(RichText::new("MUTATIONS: ADD LINK").color(Color32::WHITE).strong());
                if column[1].add(Slider::new::<f32>(&mut mut_add_link, 0.0..=0.05).step_by(0.001)).changed() {
                    settings.mut_add_link = mut_add_link;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut mut_del_link = settings.mut_del_link;
                column[0].label(RichText::new("MUTATIONS: DEL LINK").color(Color32::WHITE).strong());
                if column[1].add(Slider::new::<f32>(&mut mut_del_link, 0.0..=0.05).step_by(0.001)).changed() {
                    settings.mut_del_link = mut_del_link;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut mut_add_node = settings.mut_add_node;
                column[0].label(RichText::new("MUTATIONS: ADD NODE").color(Color32::WHITE).strong());
                if column[1].add(Slider::new::<f32>(&mut mut_add_node, 0.0..=0.05).step_by(0.001)).changed() {
                    settings.mut_add_node = mut_add_node;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut mut_del_node = settings.mut_del_node;
                column[0].label(RichText::new("MUTATIONS: DEL NODE").color(Color32::WHITE).strong());
                if column[1].add(Slider::new::<f32>(&mut mut_del_node, 0.0..=0.05).step_by(0.001)).changed() {
                    settings.mut_del_node = mut_del_node;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut mut_change_val = settings.mut_change_val;
                column[0].label(RichText::new("MUTATIONS: MOD VALUE").color(Color32::WHITE).strong());
                if column[1].add(Slider::new::<f32>(&mut mut_change_val, 0.0..=0.05).step_by(0.001)).changed() {
                    settings.mut_change_val = mut_change_val;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(120., 75.));
                column[1].set_max_size(UIVec2::new(120., 75.));
                let mut show_network: bool = settings.show_network;
                column[0].label(RichText::new("SHOW NETWORK").color(Color32::WHITE).strong());
                if column[1].add(Checkbox::without_text(&mut show_network)).changed() {
                    settings.show_network = show_network;
                    signals.new_settings = true;
                }
            });
            ui.add_space(2.0);
            ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::new(2.0, Color32::DARK_GREEN);
            ui.vertical_centered(|closer| {
                if closer.button(RichText::new("CLOSE").color(Color32::GREEN).strong()).clicked() {
                    self.state.neuro_settings = false;
                    set_settings(settings.clone());
                }
            });
        });
        set_settings(settings.clone());
    }

    fn build_left_panel(&mut self, egui_ctx: &Context, state: &SimState, agent: Option<&Agent>, ranking: &Ranking) {
        if !self.state.left_panel {
            return;
        }
        SidePanel::left("Sidebar").width_range(100.0..=400.0).show(egui_ctx, |ui| {
            if !self.pointer_over {
                self.pointer_over = ui.ui_contains_pointer();
            }
            if self.state.monit {
                ui.vertical(|ui| {
                    ui.collapsing("Monitor", |ui| {
                        self.inside_monitor(ui, state)
                    });
                });
            }
            if self.state.inspect {
                ui.vertical(|ui| {
/*                     let name = match agent {
                        Some(agent) => {
                            agent.specie.to_uppercase()
                        },
                        None => String::from("Inspector"),
                    }; */
                    ui.collapsing("Inspector", |ui| {
                        self.inside_agent(ui, agent);
                    });
                });
            }
            if self.state.ranking {
                ui.vertical(|ui| {
                    ui.collapsing("Ranking", |ui| {
                        self.inside_ranking2(ui, ranking);
                    });
                });
            }
        });
    }

    fn build_right_panel(&mut self, egui_ctx: &Context, _agent: Option<&Agent>, statistics: &Statistics) {
        if !self.state.right_panel {
            return;
        }
        SidePanel::right("Rightbar").max_width(520.0).show(egui_ctx, |ui| {
            if !self.pointer_over {
                self.pointer_over = ui.ui_contains_pointer();
            }
            //if self.state.neuro_lab {
            //    ui.vertical(|ui| {
            //        ui.collapsing("Network", |ui| {
            //            self.inside_network(ui, agent);
            //        });
            //    });
            //}
            if self.state.plot_population {
                ui.vertical(|ui| {
                    ui.set_height(125.0);
                    self.inside_plot_borns(ui, statistics);
                });
            }
            if self.state.plot_attributes {
                ui.vertical(|ui| {
                    ui.set_height(125.0);
                    self.inside_plot_attributes(ui, statistics);
                });
            }
            if self.state.plot_lifetime {
                ui.vertical(|ui| {
                    ui.set_height(125.0);
                    self.inside_plot_lifetimes(ui, statistics);
                });
            }
            if self.state.plot_neuro {
                ui.vertical(|ui| {
                    ui.set_height(125.0);
                    self.inside_plot_neuro(ui, statistics);
                });
            }
        });
    }

    fn build_bottom_panel(&mut self, egui_ctx: &Context, statistics: &Statistics) {
        if !self.state.bottom_panel {
            return;
        }
        let col_num = self.state.plot_attributes as i32 + self.state.plot_population as i32 + self.state.plot_lifetime as i32 + self.state.plot_neuro as i32;
        let mut c: usize = 0;
        TopBottomPanel::bottom("bottom").height_range(100.0..=400.0).show(egui_ctx, |ui| {
            if !self.pointer_over {
                self.pointer_over = ui.ui_contains_pointer();
            }
            ui.columns(col_num as usize, |col| {
                if self.state.plot_population {
                    col[c].vertical(|ui| {
                        self.inside_plot_borns(ui, statistics);
                    });
                    c += 1;
                }
                if self.state.plot_attributes {
                    col[c].vertical(|ui| {
                        self.inside_plot_attributes(ui, statistics);
                    });
                    c += 1;
                }
                if self.state.plot_lifetime {
                    col[c].vertical(|ui| {
                        self.inside_plot_lifetimes(ui, statistics);
                    });
                    c += 1;
                }
                if self.state.plot_neuro {
                    col[c].vertical(|ui| {
                        self.inside_plot_neuro(ui, statistics);
                    });
                    c += 1;
                }
            });
        });
    }

    fn inside_plot_lifetimes(&mut self, ui: &mut Ui, statistics: &Statistics) {
        let legend = Legend {
            position: plot::Corner::LeftTop,
            ..Default::default()
        };
        //let statistics = statistics.get_statistics();
        let plot_lifetimes = Plot::new("lifetimes").legend(legend);
        let lifetimes = statistics.get_data_as_slice("lifetimes");
        let agents = statistics.get_data_as_slice("agents");
        let plants = statistics.get_data_as_slice("plants");
        let inner = plot_lifetimes.show(ui, |plot_ui| {
            plot_ui.line(Line::new(PlotPoints::from(lifetimes)).name("lifetime").color(Color32::YELLOW));
            plot_ui.line(Line::new(PlotPoints::from(agents)).name("agents").color(Color32::BLUE));
            plot_ui.line(Line::new(PlotPoints::from(plants)).name("plants").color(Color32::GREEN));
        });
        _ = Some(inner.response.rect);
    }

    fn inside_plot_borns(&mut self, ui: &mut Ui, statistics: &Statistics) {
        let legend = Legend {
            position: plot::Corner::LeftTop,
            ..Default::default()
        };
        //let statistics = statistics.get_statistics();
        let born_plot = Plot::new("borns").legend(legend);
        let born = statistics.get_data_as_slice("borns");
        let points = statistics.get_data_as_slice("points");
        let k = statistics.get_data_as_slice("kills");
        let inner = born_plot.show(ui, |plot_ui| {
            plot_ui.line(Line::new(PlotPoints::from(born)).name("borns").color(Color32::GREEN));
            plot_ui.line(Line::new(PlotPoints::from(points)).name("points").color(Color32::BLUE));
            plot_ui.line(Line::new(PlotPoints::from(k)).name("kills").color(Color32::RED));
        });
        _ = Some(inner.response.rect);
    }

    fn inside_plot_neuro(&mut self, ui: &mut Ui, statistics: &Statistics) {
        let legend = Legend {
            position: plot::Corner::LeftTop,
            ..Default::default()
        };
        let neuro_plot = Plot::new("neuro").legend(legend);
        let nodes = statistics.get_data_as_slice("nodes");
        let links = statistics.get_data_as_slice("links");
        let inner = neuro_plot.show(ui, |plot_ui| {
            plot_ui.line(Line::new(PlotPoints::from(nodes)).name("nodes").color(Color32::BLUE));
            plot_ui.line(Line::new(PlotPoints::from(links)).name("links").color(Color32::GREEN));
        });
        _ = Some(inner.response.rect);
    }

    fn inside_plot_attributes(&mut self, ui: &mut Ui, statistics: &Statistics) {
        //let w = 500.0; let h = 120.0;
        let legend = Legend {
            position: plot::Corner::LeftTop,
            ..Default::default()
        };
        let my_plot = Plot::new("attributes").legend(legend);
        //let statistics = statistics.get_statistics();
        let sizes = statistics.get_data_as_slice("sizes");
        let powers = statistics.get_data_as_slice("powers");
        let speeds = statistics.get_data_as_slice("speeds");
        let eyes = statistics.get_data_as_slice("eyes");
        let shells = statistics.get_data_as_slice("shells");
        let mutations = statistics.get_data_as_slice("mutations");
        let inner = my_plot.show(ui, |plot_ui| {
            plot_ui.line(Line::new(PlotPoints::from(sizes)).name("size").color(Color32::BLUE));
            plot_ui.line(Line::new(PlotPoints::from(powers)).name("power").color(Color32::GREEN));
            plot_ui.line(Line::new(PlotPoints::from(speeds)).name("speed").color(Color32::YELLOW));
            plot_ui.line(Line::new(PlotPoints::from(eyes)).name("eye").color(Color32::RED));
            plot_ui.line(Line::new(PlotPoints::from(shells)).name("shell").color(Color32::DARK_GRAY));
            plot_ui.line(Line::new(PlotPoints::from(mutations)).name("mutation").color(Color32::LIGHT_BLUE));
        });
        _ = Some(inner.response.rect);
    }

    fn inside_monitor(&mut self, ui: &mut Ui, sim_state: &SimState) {
        let time = sim_state.sim_time;
        let agents_num = sim_state.agents_num;
        let sources_num = sim_state.sources_num;
        let fps = sim_state.fps;
        let dt = sim_state.dt;
        ui.horizontal(|ui| {
            ui.set_max_height(16.0);
            ui.label(RichText::new(format!("TIME: {}", time.round())).monospace());
            ui.label(RichText::new(format!("FPS: {}", fps)).monospace());
            ui.label(RichText::new(format!("dT: {:.0}ms", dt*1000.0)).monospace());
            ui.label(RichText::new(format!("SPEED: x{}", sim_speed())).monospace());
        });
        ui.horizontal(|ui| {
            ui.set_max_height(16.0);
            ui.label(RichText::new(format!("AGENT: {}", agents_num)).monospace());
            ui.label(RichText::new(format!("PLANT: {}", sources_num)).monospace());
        });
    }

    fn inside_agent(&mut self, ui: &mut Ui, agent: Option<&Agent>) {
        if let Some(agent) = agent {
            let contacts_num = agent.contacts.len();
            let lifetime = agent.lifetime.round();
            let generation = agent.generation;
            let childs = agent.childs;
            let kills = agent.kills;
            let attack = agent.attacking;
            let eat = agent.eating;
            let points = agent.points;
            let run = agent.run;
            let name = agent.specie.to_owned().to_uppercase();
            let mut states: Vec<String> = vec![];
            if attack { states.push("ATK".to_string()) }
            if eat { states.push("EAT".to_string()) }
            if run { states.push("RUN".to_string()) }
            if contacts_num > 0 { states.push(format!("CON({})", contacts_num)) }
            let mut status_txt = String::from("| ");
            if states.len() == 0 { status_txt.push_str("... |"); }
            for s in states {
                status_txt.push_str(&s);
                status_txt.push_str(" |");
            }
            let size = agent.size as i32;
            let power = agent.power;
            let speed = agent.speed;
            let shell = agent.shell;
            let mutations = agent.mutations;
            let eyes = agent.eyes;
            let water: bool = if agent.get_water() == 0 {
                false
            } else {
                true
            };
            let attributes = format!("S: {} | M: {} | P: {} | D: {} | X: {} | V: {}", size, speed, power, shell, mutations, eyes);
            ui.horizontal(|ui| {
                ui.set_max_height(16.0);
                ui.label(RichText::new(format!("{} ({})", &name, generation)).strong().size(14.0).color(Color32::GREEN));
            });
            ui.horizontal(|ui| {
                ui.set_max_height(14.0);
                ui.label(RichText::new(format!("[HP: {}%]", agent.hp.round())).strong().monospace().color(Color32::GREEN));
                ui.separator();
                ui.label(RichText::new(format!("[ENG: {}/{}]", agent.eng.round(), agent.max_eng.round())).strong().monospace().color(Color32::YELLOW));
            });
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("T: {}", lifetime)).monospace());
                ui.separator();
                ui.label(RichText::new(format!("PTS: {}", points.round())).monospace());
            });
            ui.horizontal(|ui| {
                ui.set_max_height(14.0);
                ui.label(RichText::new(format!("BORN: {}", childs)).monospace());
                ui.separator();
                ui.label(RichText::new(format!("KILL: {}", kills)).monospace());
                ui.separator();
                ui.label(RichText::new("■■■").strong().color(color_to_color32(agent.mood)).monospace())
            });
            ui.horizontal(|ui| {
                ui.set_max_height(14.0);
                ui.label(RichText::new(attributes).strong());
            });
            ui.horizontal(|ui| {
                ui.set_max_height(14.0);
                let txt = format!("B: {} | M: {} | A: {} ", agent.eng_cost.basic.round(), agent.eng_cost.movement.round(), agent.eng_cost.attack.round());
                ui.label(RichText::new(txt).strong().color(Color32::RED));
                ui.separator();
                ui.label(RichText::new(status_txt).strong().color(Color32::LIGHT_BLUE));
            });
            ui.horizontal(|ui| {
                ui.set_max_height(14.0);
                let txt = if water {
                    "water".to_string()
                } else {
                    "".to_string()
                };
                ui.label(RichText::new(txt).strong().color(Color32::BLUE));
            });
        }
    }

    fn inside_network(&mut self, ui: &mut Ui, agent: Option<&Agent>) {
        if let Some(agent) = agent {
            //let period = self.timer/get_settings().neuro_duration;
            let t = self.timer;
            let network = &agent.network;
            let w = 340.0; let h = 380.0; let resize = egui_macroquad::egui::Vec2::new(3.2, 3.6);
            let offset = UIVec2::new(5.0, 0.0);
            let (response, painter) = ui.allocate_painter(UIVec2::new(w, h), Sense::hover());
            let rect = response.rect;
            let zero = rect.left_top().to_vec2()+offset;
            let wi = 1.0;
            for (_, link) in network.links.iter() {
                let (coord0, coord1, coord_t) = link.get_coords(&network.nodes, t);
                let ui_coord0 = vec2_to_uivec2(&coord0);
                let ui_coord1 = vec2_to_uivec2(&coord1);
                let ui_coord_t = vec2_to_uivec2(&coord_t);
                let w = link.get_width()*wi;
                let p1 = vec2_to_pos2(&(ui_coord0*resize+zero));
                let p2 = vec2_to_pos2(&(ui_coord1*resize+zero));
                let pt = vec2_to_pos2(&(ui_coord_t*resize+zero));
                let (_, color1) = link.get_colors();
                //let c0 = color_to_color32(color0);
                let c1 = color_to_color32(color1);
                let points1 = [p1, p2];
                painter.line_segment(points1, Stroke { color: c1, width: w });
                painter.circle_filled(pt, w, Color32::YELLOW);
            }
            for (key, node) in network.nodes.iter() {
                let (_, color1) = node.get_colors();
                let (r0, _) = node.get_size();
                let mut mem = node.get_mem_size();
                let ipos = egui_macroquad::egui::Vec2::new(node.pos.x as f32, node.pos.y as f32)*resize+zero;
                let p1 = vec2_to_pos2(&ipos);
                let c0 = color_to_color32(color1);
                let label = node.get_label();
                let v = match network.get_node_value(key) {
                    None => 0.0,
                    Some(v) => v,
                };
                painter.circle_filled(p1, r0,  Color32::BLACK);
                //let w1 = 0.5 + 0.35*r1;
                //painter.circle_stroke(p1, r1, Stroke { color: Color32::GREEN, width: w1 });
                let w0 = 0.25 + 0.25*r0;
                painter.circle_stroke(p1, r0, Stroke { color: c0, width: w0 });
                if mem > 0.0 {
                    mem = clamp(mem, -1.0, 1.0);
                    painter.circle_stroke(p1, 1.0+mem*5.0, Stroke { color: Color32::GREEN, width: 1.0 });
                }
                let mut font = FontId::default();
                font.size = 8.0;
                let txt = format!("{}: {:.1}", label, v);
                match node.node_type {
                    NeuronTypes::INPUT => {
                        painter.text(p1+UIVec2{x: 8.0, y: 0.0}, Align2::LEFT_CENTER, txt, font, Color32::WHITE);
                    },
                    NeuronTypes::OUTPUT => {
                        painter.text(p1+UIVec2{x: -50.0, y: 0.0}, Align2::LEFT_CENTER, txt, font, Color32::WHITE);
                    },
                    _ => {},
                }
            } 
        }
    }

    fn inside_ranking(&mut self, ui: &mut Ui, ranking: &Vec<AgentSketch>) {
        if self.state.ranking {
            let mut i = 0;
            for rank in ranking.iter() {
                i += 1;
                ui.horizontal(|ui| {
                    let msg1 = format!("{}.{}", i, rank.specie.to_uppercase());
                    let msg3 = format!("{}  |  ({})", rank.points.round(), rank.generation);
                    ui.columns(2, |column| {
                        
                        column[0].set_width(80.0);
                        column[0].label(RichText::new(msg1).monospace());
                        column[1].set_width(55.0);
                        column[1].label(RichText::new(msg3).color(Color32::WHITE).monospace().strong());
                    });
                });
            }
        }
    }

    fn inside_ranking2(&mut self, ui: &mut Ui, ranking: &Ranking) {
        let rank = ranking.get_general_rank();
        let school = ranking.get_school_rank();
        if self.state.ranking {
            let mut i = 0;
            for r in rank.iter() {
                i += 1;
                ui.horizontal(|ui| {
                    let msg1 = format!("{}.{}", i, r.specie.to_uppercase());
                    let msg3 = format!("{}  |  ({})", r.points.round(), r.generation);
                    ui.columns(2, |column| {
                        
                        column[0].set_width(80.0);
                        column[0].label(RichText::new(msg1).monospace());
                        column[1].set_width(55.0);
                        column[1].label(RichText::new(msg3).color(Color32::WHITE).monospace().strong());
                    });
                });
            }
            ui.separator();
            ui.separator();
            let mut i = 0;
            for r in school.iter() {
                i += 1;
                ui.horizontal(|ui| {
                    let msg1 = format!("{}.{}", i, r.specie.to_uppercase());
                    let msg3 = format!("{}  |  ({})", r.points.round(), r.generation);
                    ui.columns(2, |column| {
                        
                        column[0].set_width(80.0);
                        column[0].label(RichText::new(msg1).monospace());
                        column[1].set_width(55.0);
                        column[1].label(RichText::new(msg3).color(Color32::WHITE).monospace().strong());
                    });
                });
            }
        }
    }

    pub fn ui_draw(&self) {
        egui_macroquad::draw();
    }
    
}

pub struct UIState {
    pub ancestors: bool,
    pub new_sim_name: String,
    pub monit: bool,
    pub inspect: bool,
    pub mouse: bool,
    pub quit: bool,
    pub agents_num: i32,
    pub new_sim: bool,
    pub credits: bool,
    pub docs: bool,
    pub net: bool,
    pub about: bool,
    pub environment: bool,
    pub neuro_lab: bool,
    pub resize_world: bool,
    pub ranking: bool,
    pub set_agent: bool,
    pub load_sim: bool,
    pub load_agent: bool,
    pub attributes: bool,
    pub main_menu: bool,
    pub energy_cost: bool,
    pub neuro_settings: bool,
    pub info: bool,
    pub plants: bool,
    pub plot_attributes: bool,
    pub plot_population: bool,
    pub plot_lifetime: bool,
    pub plot_neuro: bool,
    pub left_panel: bool,
    pub right_panel: bool,
    pub bottom_panel: bool,
    pub deaths: bool,
    pub gen_random_name: bool,
}

impl UIState {

    pub fn new() -> Self {
        Self {
            ancestors: false,
            new_sim_name: String::new(),
            monit: true,
            inspect: true,
            mouse: false,
            quit: false,
            agents_num: 0,
            new_sim: false,
            credits: false,
            docs: false,
            net: false,
            about: false,
            environment: false,
            neuro_lab: true,
            resize_world: false,
            ranking: true,
            set_agent: false,
            load_sim: false,
            load_agent: false,
            attributes: false,
            main_menu: true,
            energy_cost: false,
            neuro_settings: false,
            info: false,
            plants: false,
            plot_attributes: true,
            plot_population: true,
            plot_lifetime: true,
            plot_neuro: true,
            left_panel: true,
            right_panel: false,
            deaths: false,
            bottom_panel: true,
            gen_random_name: false,
        }
    }
}