#![allow(unused)]

use std::f32::consts::PI;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use egui_macroquad;
use egui_macroquad::egui::*;
use egui_macroquad::egui::widgets::{Slider, Button};
use egui_macroquad::egui::Checkbox;
use egui_macroquad::egui::Vec2 as UIVec2;
use egui_macroquad::egui::FontFamily::Proportional;
use egui_macroquad::egui::FontId;
use egui_macroquad::egui::TextStyle::*;
use macroquad::prelude::*;
use crate::resource::Resource;
use crate::util::*;
use crate::agent::*;
use crate::neuro::*;
use crate::globals::*;

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
            let img =  Self::load_image(Path::new("assets/img/biome32.png")).unwrap();
            self.logo = Some(egui_ctx.load_texture("logo".to_string(), img, Default::default()));
            let img2 =  Self::load_image(Path::new("assets/img/biome128.png")).unwrap();
            self.big_logo = Some(egui_ctx.load_texture("big_logo".to_string(), img2, Default::default()));
            let img3 =  Self::load_image(Path::new("assets/img/evolve.png")).unwrap();
            self.title = Some(egui_ctx.load_texture("title".to_string(), img3, Default::default()));
        });
    }

    fn set_fonts_styles(&mut self, egui_ctx: &Context) {
        let mut style = (*egui_ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(14.0, Proportional)),
            (Name("Heading2".into()), FontId::new(12.0, Proportional)),
            (Name("Context".into()), FontId::new(11.0, Proportional)),
            (Body, FontId::new(11.0, Proportional)),
            (Monospace, FontId::new(11.0, Proportional)),
            (Button, FontId::new(12.0, Proportional)),
            (Small, FontId::new(9.0, Proportional)),
        ].into();
        egui_ctx.set_style(style);
    }

    pub fn ui_process(&mut self, sim_state: &SimState, signals: &mut Signals, camera2d: &Camera2D, agent: Option<&Agent>, res: Option<&Resource>, ranking: &Vec<AgentSketch>) {
        egui_macroquad::ui(|egui_ctx| {
            self.set_fonts_styles(egui_ctx);
            self.pointer_over = egui_ctx.is_pointer_over_area();
            self.build_top_menu(egui_ctx, &sim_state.sim_name, signals);
            self.build_quit_window(egui_ctx);
            self.build_monit_window(egui_ctx, &sim_state);
            self.build_debug_window(egui_ctx, camera2d, &sim_state, agent);
            self.build_new_sim_window(egui_ctx, signals);
            match agent {
                Some(agent) => {
                    self.build_inspect_window(egui_ctx, agent);
                    self.build_inspect_network(egui_ctx, &agent.network);
                    self.build_attributes_window(egui_ctx, agent);
                    self.build_eng_cost_window(egui_ctx, agent);
                },
                None => {},
            }
            match res {
                Some(res) => {
                    self.build_res_window(egui_ctx, res);
                },
                None => {},
            }
            self.build_about_window(egui_ctx);
            self.build_settings_window(egui_ctx, signals);
            self.build_agent_settings_window(egui_ctx, signals);
            self.build_ranking_window(egui_ctx, ranking);
            self.build_load_sim_window(egui_ctx);
            self.build_main_menu_win(egui_ctx);
            self.build_load_agent_window(egui_ctx);
            self.build_neuro_settings_window(egui_ctx, signals);
            self.build_info_window(egui_ctx);
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
                
                menu::menu_button(ui, RichText::new("SIMULATION").strong(), |ui| {
                    if ui.button(RichText::new("New Simulation").strong().color(Color32::BLUE)).clicked() {
                        self.state.new_sim = true;
                    }
                    if ui.button(RichText::new("Load Simulation").strong().color(Color32::GREEN)).clicked() {
                        //signals.load_sim = true;
                        self.state.load_sim = true;
                    }
                    if ui.button(RichText::new("Save Simulation").weak().color(Color32::GREEN)).clicked() {
                        signals.save_sim = true;
                    }
                    if ui.button(RichText::new("Load Agent").weak().color(Color32::BLUE)).clicked() {
                        self.state.load_agent = true;
                    }
                    if ui.button(RichText::new("Save Agent").strong().color(Color32::BLUE),).clicked() {
                        //let mut signals = mod_signals();
                        signals.save_selected = true;
                    }
                    if ui.button(RichText::new("Quit").color(Color32::RED).strong()).clicked() {
                        self.state.quit = true;
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                menu::menu_button(ui, RichText::new("VIEW").strong(), |ui| {
                    if ui.button(RichText::new("Monitor").strong().color(Color32::WHITE)).clicked() {
                        self.state.performance = !self.state.performance;
                    }
                    if ui.button(RichText::new("Inspector").strong().color(Color32::WHITE)).clicked() {
                        self.state.inspect = !self.state.inspect;
                    }
                    if ui.button(RichText::new("Attributes").strong().color(Color32::WHITE)).clicked() {
                        self.state.attributes = !self.state.attributes;
                    }
                    if ui.button(RichText::new("Energy Cost").strong().color(Color32::WHITE)).clicked() {
                        self.state.energy_cost = !self.state.energy_cost;
                    }
                    if ui.button(RichText::new("Debug Info").strong().color(Color32::WHITE)).clicked() {
                        self.state.mouse = !self.state.mouse;
                    }
                    if ui.button(RichText::new("Ranking").strong().color(Color32::WHITE)).clicked() {
                        self.state.ranking = !self.state.ranking;
                    }
                    if ui.button(RichText::new("Resource").strong().color(Color32::WHITE)).clicked() {
                        self.state.resource = !self.state.resource;
                    }
                    if ui.button(RichText::new("Print Mutations Stats").strong().color(Color32::WHITE)).clicked() {
                        self.state.info = !self.state.info;
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                
                menu::menu_button(ui, RichText::new("TOOLS").strong(), |_ui| {
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                menu::menu_button(ui, RichText::new("CAMERA").strong(), |ui| {
                    if ui.button(RichText::new("Follow Mode").strong().color(Color32::GOLD)).clicked() {
                        let mut settings = get_settings();
                        settings.follow_mode = !settings.follow_mode;
                        set_global_settings(settings);
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                menu::menu_button(ui, RichText::new("NEUROLOGY").strong(), |ui| {
                    if ui.button(RichText::new("Network Inspector").strong().color(Color32::WHITE)).clicked() {
                        self.state.neuro_lab = !self.state.neuro_lab;
                    }
                });


                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);

                menu::menu_button(ui, RichText::new("SETTINGS").strong(), |ui| {
                    if ui.button(RichText::new("Agent Settings").strong().color(Color32::YELLOW)).clicked() {
                        self.state.set_agent = !self.state.set_agent;
                    }
                    if ui.button(RichText::new("Sim Settings").strong().color(Color32::YELLOW)).clicked() {
                        self.state.environment = !self.state.environment;
                    }
                    if ui.button(RichText::new("Neuro Settings").strong().color(Color32::YELLOW)).clicked() {
                        self.state.neuro_settings = !self.state.neuro_settings;
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
                    author.label(RichText::new("Artur Gwoździowski 2023").color(Color32::BLUE).strong());
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


    fn build_monit_window(&self, egui_ctx: &Context, sim_state: &SimState) {
        if self.state.performance {
            let time = sim_state.sim_time;
            let agents_num = sim_state.agents_num;
            let sources_num = sim_state.sources_num;
            Window::new("MONITOR").default_pos((170.0, 0.0)).default_width(400.0).default_height(80.0).show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(format!("TIME: {}", time.round())).small());
                    ui.label(RichText::new(format!("AGENTS: {}", agents_num)).small());
                    ui.label(RichText::new(format!("SOURCES: {}", sources_num)).small());
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
                        let agent_name = path.file_name().unwrap().to_str().unwrap().to_owned();
                        saved_agents.push(agent_name);
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
                        match serde_json::from_str::<AgentSketch>(&file) {
                            Ok(sketch) => {
                                sketches.push(sketch.clone());
                            },
                            Err(_) => {},
                        }
                    },
                    Err(_) => {

                    },
                }
            }

            Window::new("LOAD AGENT").default_pos((SCREEN_WIDTH / 2.0 - 65.0, SCREEN_HEIGHT / 4.0)).default_width(260.0).show(egui_ctx, |ui| {
                for agent in sketches {
                    ui.vertical_centered(|row| {
                        row.columns(2, |columns| {
                            let txt = format!("{} | G:{} ", agent.specie.to_uppercase(), agent.generation);
                            let filename = format!("{}-{}.json", agent.specie, agent.generation);
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
                        match agent.resource_dir {
                            None => {},
                            Some(res_dir) => {
                                ui.label(format!("resource angle: {:.2}", res_dir));
                            },
                        }
                        match agent.resource_position {
                            None => {},
                            Some(res_pos) => {
                                let rel_pos = res_pos - agent.pos;
                                ui.label(format!("resource pos: [x: {} | y: {}]", rel_pos.x.round(), rel_pos.y.round()));
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
                        let text = columns[1].style_mut().text_styles();
                        
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
                "RANDOM", "STRANGE", "CRAZY", "FANTASTIC", "ALTERNATIVE",
                "DEEP", "DIGITAL", "FIRST", "GREAT", "LAST", "BROKEN", 
                "ANOTHER", "MIRROR", "BYTE", "NEXT", "TOXIC", "FLUENT"
            ];
            let names1 = vec![
                "SIMULATION", "UNIVERSE", "WORLD", "LAND", "LAB", "PLANET", "SIM",
                "REALITY", "BIOME", "LABOLATORY", "ROCK", "ISLAND", "NATURE", "ECOSYSTEM",
                "SYSTEM", "EXPERIMENT", "TERRAIN", "GLOBE", "SPHERE", "SANDBOX"
            ];

            let mut settings = get_settings();
            let w = 500.0; let h = 220.0;
            Window::new("EVOLVE").default_pos((SCREEN_WIDTH / 2.0 - w/2.0, 100.0)).default_size([w, h]).show(egui_ctx, |ui| {
                let big_logo = self.big_logo.clone().unwrap();
                let title = self.title.clone().unwrap();
                ui.vertical_centered(|pic| {
                    pic.image(big_logo.id(), big_logo.size_vec2());
                });
                ui.add_space(4.0);
                ui.vertical_centered(|pic| {
                    pic.image(title.id(), title.size_vec2()*0.7);
                });
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
                    let response = txt.add(widgets::TextEdit::singleline(&mut self.temp_sim_name));
                    if self.temp_sim_name.is_empty() {
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
                    if response.changed() {
                    }
                    if response.lost_focus() && txt.input(|i| i.key_pressed(Key::Enter)) {
                        self.state.new_sim = false;
                        signals.new_sim = true;
                        signals.new_sim_name = String::from(&self.temp_sim_name);
                        self.temp_sim_name = String::new();
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
                            set_global_settings(settings.clone());
                        }
                        if columns[1].add(Slider::new(&mut h, 600..=7500).step_by(100.0)).changed() {
                            settings.world_h = h;
                            set_global_settings(settings.clone());
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

    fn build_eng_cost_window(&self, egui_ctx: &Context, agent: &Agent) {
        if self.state.energy_cost {
            Window::new(RichText::new("ENERGY COST").strong().color(Color32::GREEN)).default_pos((170.0, 100.0)).min_width(380.0).show(egui_ctx, |ui| {
                ui.horizontal(|row| {
                    let txt = format!("[ BASE: {} | MOVE {} | ATTACK {} ]", agent.eng_cost.basic.round(), agent.eng_cost.movement.round(), agent.eng_cost.attack.round());
                    row.label(RichText::new(txt).strong().color(Color32::RED));
                });
            });
        }
    }

    fn build_inspect_window(&self, egui_ctx: &Context, agent: &Agent) {
        if self.state.inspect {
            let rot = agent.rot;
            let size = agent.size;
            //let tg_pos = agent.enemy_position;
            let pos = agent.pos;
            let name = agent.specie.to_owned();
            let contacts_num = agent.contacts.len();
            let lifetime = agent.lifetime.round();
            let generation = agent.generation;
            let childs = agent.childs;
            let kills = agent.kills;
            let attack = agent.attacking;
            let eat = agent.eating;
            let points = agent.points;
            let run = agent.run;
            //let is_resource: bool = agent.resource.is_some();
            let mut states: Vec<String> = vec![];
            if attack { states.push("ATK".to_string()) }
            if eat { states.push("EAT".to_string()) }
            if run { states.push("RUN".to_string()) }
            //if is_resource { states.push("SOUR".to_string()) }
            //if tg_pos.is_some() { states.push("TARG".to_string()) }
            if contacts_num > 0 { states.push(format!("CON({})", contacts_num)) }
            let mut status_txt = String::from("| ");
            if states.len() == 0 { status_txt.push_str("... |"); }
            for s in states {
                status_txt.push_str(&s);
                status_txt.push_str(" |");
            }
            let title_txt = format!("{}", name.to_uppercase()); 
            Window::new(RichText::new(title_txt).strong().color(Color32::WHITE)).default_pos((435.0, 0.0)).min_width(380.0).show(egui_ctx, |ui| {
                ui.horizontal(|row| {
                    row.label(RichText::new(format!("[ ENERGY: {} / {} ]", agent.eng.round(), agent.max_eng.round())).strong().color(Color32::RED));
                    row.separator();
                    row.label(RichText::new(status_txt).strong().color(Color32::LIGHT_BLUE));
                    row.separator();
                    row.label(RichText::new("■■■").strong().color(color_to_color32(agent.mood)).monospace())
                });
                ui.horizontal(|row| {
                    row.label(RichText::new(format!("GEN: [{}]", generation)).small());
                    row.separator();
                    row.label(RichText::new(format!("SIZE: [{}]", size)).small());
                    row.separator();
                    row.label(RichText::new(format!("TIME: [{}]", lifetime)).small());
                    row.separator();
                    row.label(RichText::new(format!("POINTS: [{}]", points.round())).small());
                });
                ui.horizontal(|row| {
                    row.label(RichText::new(format!("CHILD: [{}]", childs)).small());
                    row.separator();
                    row.label(RichText::new(format!("KILLS: [{}]", kills)).small());
                    row.separator();
                    row.label(RichText::new(format!("ORIENT: [{}]", ((rot * 10.0).round()) / 10.0)).small());
                    row.separator();
                    row.label(RichText::new(format!("COORD: [X{}|Y{}]", pos.x.round(), pos.y.round())).small());
                });
            });
        }
    }


    fn build_res_window(&self, egui_ctx: &Context, res: &Resource) {
        if self.state.resource {
            let size = res.size as i32;
            let max_eng = res.max_eng;
            let eng = res.eng;
            let attributes = format!("ENG: {:.0}/{:.0} | SIZE: {}",eng, max_eng, size);
            let title_txt = format!("Resource"); 
            Window::new(RichText::new(title_txt).strong().color(Color32::GREEN)).default_pos((800.0, 0.0)).min_width(100.0).show(egui_ctx, |ui| {
                ui.horizontal(|row| {
                    row.label(RichText::new(attributes).strong());
                });
            });
        }
    }


    fn build_attributes_window(&self, egui_ctx: &Context, agent: &Agent) {
        if self.state.attributes {
            let size = agent.size as i32;
            let power = agent.power;
            let speed = agent.speed;
            let shell = agent.shell;
            let mutations = agent.mutations;
            let eyes = agent.eyes;
            let name = &agent.specie;
            let attributes = format!("size: {} | speed: {} | power: {} | shell: {} | mutation: {} | eyes: {}", size, speed, power, shell, mutations, eyes);
            let title_txt = format!("Attributes: {}", name.to_uppercase()); 
            Window::new(RichText::new(title_txt).strong().color(Color32::GREEN)).default_pos((800.0, 0.0)).min_width(150.0).show(egui_ctx, |ui| {
                ui.horizontal(|row| {
                    row.label(RichText::new(attributes).strong());
                });
            });
        }
    }

    fn build_inspect_network(&mut self, egui_ctx: &Context, network: &Network) {
        if self.state.neuro_lab {
            let w = 360.0; let h = 360.0; let resize = macroquad::prelude::Vec2::new(360.0, 360.0);
            let offset = UIVec2::new(0.0, 0.0);
            Window::new("Network Inspector").default_pos((SCREEN_WIDTH-w, 0.0)).min_height(h).min_width(w).resizable(true)
                .title_bar(true).show(egui_ctx, |ui| {
                    
                    let (response, painter) = ui.allocate_painter(UIVec2::new(w, h), Sense::hover());
                    let rect = response.rect;
                    let zero = rect.left_top().to_vec2()+offset;
                    for (_, link) in network.links.iter() {
                        let (coord0, coord1, _coord_t) = link.get_coords(&network.nodes, 0.0);
                        let w = link.get_width()*1.5;
                        let p1 = vec2_to_pos2(coord0*resize)+zero;
                        let p2 = vec2_to_pos2(coord1*resize)+zero;
                        let (_, color1) = link.get_colors();
                        let c1 = color_to_color32(color1);
                        let points1 = [p1, p2];
                        painter.line_segment(points1, Stroke { color: c1, width: w });
                    }
                    for (key, node) in network.nodes.iter() {
                        let (color0, color1) = node.get_colors();
                        let r = node.get_size()*1.5;
                        let p1 = vec2_to_pos2(node.pos*resize)+zero;
                        let c0 = color_to_color32(color1);
                        let c1 = color_to_color32(color0);
                        let label = node.get_label();
                        let v = match network.get_node_value(key) {
                            None => 0.0,
                            Some(v) => v,
                        };
                        painter.circle_filled(p1, r,  Color32::BLACK);
                        //painter.circle_filled(p1, r,  c1);
                        let w = 0.75 + 0.24*r;
                        painter.circle_stroke(p1, r, Stroke { color: c0, width: w });
                        let mut font = FontId::default();
                        font.size = 10.0;
                        let txt = format!("{}: {:.1}", label, v);
                        match node.node_type {
                            NeuronTypes::INPUT => {
                                painter.text(p1+UIVec2{x: 10.0, y: 0.0}, Align2::LEFT_CENTER, txt, font, Color32::WHITE);
                            },
                            NeuronTypes::OUTPUT => {
                                painter.text(p1+UIVec2{x: -60.0, y: 0.0}, Align2::LEFT_CENTER, txt, font, Color32::WHITE);
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

    fn build_agent_settings_window(&mut self, egui_ctx: &Context, signals: &mut Signals) {
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
            ui.add_space(2.0);
            ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::new(2.0, Color32::DARK_GREEN);
            ui.vertical_centered(|closer| {
                if closer.button(RichText::new("CLOSE").color(Color32::GREEN).strong()).clicked() {
                    self.state.set_agent = false;
                    set_global_settings(settings.clone());
                }
            });
        });
        set_global_settings(settings.clone());
    }

    fn build_settings_window(&mut self, egui_ctx: &Context, signals: &mut Signals) {
        if !self.state.environment {
            return;
        }
        let mut settings = get_settings();
        Window::new("SETTINGS").id("settings_win".into()).default_pos((SCREEN_WIDTH/2., SCREEN_HEIGHT/2.)).fixed_size([380., 400.])
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
                if column[1].add(Slider::new(&mut agents_num, 0..=100)).changed() {
                    settings.agent_min_num = agents_num as usize;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut agent_init_num: i32 = settings.agent_init_num as i32;
                column[0].label(RichText::new("AGENT INIT NUMBER").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut agent_init_num, 0..=100)).changed() {
                    settings.agent_init_num = agent_init_num as usize;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut res_init_num: i32 = settings.res_init_num as i32;
                column[0].label(RichText::new("RES INIT NUMBER").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut res_init_num, 0..=250)).changed() {
                    settings.res_init_num = res_init_num as usize;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut res_min_num: i32 = settings.res_min_num as i32;
                column[0].label(RichText::new("RES MIN NUMBER").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut res_min_num, 0..=20)).changed() {
                    settings.res_min_num = res_min_num as usize;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut res_detection_radius: f32 = settings.res_detection_radius;
                column[0].label(RichText::new("RES DETECTION RANGE").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut res_detection_radius, 0.0..=500.0).step_by(5.0)).changed() {
                    settings.res_detection_radius = res_detection_radius;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut res_balance: i32 = settings.res_balance as i32;
                column[0].label(RichText::new("RES BALANCE NUMBER").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut res_balance, 0..=50)).changed() {
                    settings.res_balance = res_balance as usize;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut growth: f32 = settings.growth;
                column[0].label(RichText::new("GROWTH").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut growth, 0.0..=10.0).step_by(0.5)).changed() {
                    settings.growth = growth;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut resource_probability = settings.resource_probability;
                column[0].label(RichText::new("SOURCES RATE").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut resource_probability, 0.0..=1.0).step_by(0.01)).changed() {
                    settings.resource_probability = resource_probability;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut mutations: f32 = settings.mutations;
                column[0].label(RichText::new("MUTATIONS").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut mutations, 0.0..=1.0).step_by(0.05)).changed() {
                    settings.mutations = mutations;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut repro_time = settings.repro_time as i32;
                column[0].label(RichText::new("REPRODUCTION TIME").color(Color32::WHITE).strong());
                if column[1].add(Slider::new::<i32>(&mut repro_time, 0..=200).step_by(1.0)).changed() {
                    settings.repro_time = repro_time as f32;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut new_one_probability = settings.new_one_probability;
                column[0].label(RichText::new("NEW AGENT PROBABILITY").color(Color32::WHITE).strong());
                if column[1].add(Slider::new::<f32>(&mut new_one_probability, 0.0..=0.3).step_by(0.01)).changed() {
                    settings.new_one_probability = new_one_probability;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut water_lvl = settings.water_lvl as i32;
                column[0].label(RichText::new("WATER LEVEL").color(Color32::WHITE).strong());
                if column[1].add(Slider::new::<i32>(&mut water_lvl, 0..=10)).changed() {
                    settings.water_lvl = water_lvl as u8;
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
                let mut show_res_rad: bool = settings.show_res_rad;
                column[0].label(RichText::new("SHOW RES RADIUS").color(Color32::WHITE).strong());
                if column[1].add(Checkbox::without_text(&mut show_res_rad)).changed() {
                    settings.show_res_rad = show_res_rad;
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
                    set_global_settings(settings.clone());
                }
            });
        });
        set_global_settings(settings.clone());
    }

    fn build_neuro_settings_window(&mut self, egui_ctx: &Context, signals: &mut Signals) {
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
                if column[1].add(Slider::new(&mut neurolink_rate, 0.0..=0.5).step_by(0.05)).changed() {
                    settings.neurolink_rate = neurolink_rate;
                    signals.new_settings = true;
                }
            });
            ui.columns(2, |column| {
                column[0].set_max_size(UIVec2::new(80., 75.));
                column[1].set_max_size(UIVec2::new(280., 75.));
                let mut neuro_duration: f32 = settings.neuro_duration;
                column[0].label(RichText::new("ANALIZE PERIOD").color(Color32::WHITE).strong());
                if column[1].add(Slider::new(&mut neuro_duration, 0.01..=0.5).step_by(0.01)).changed() {
                    settings.neuro_duration = neuro_duration;
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
                    set_global_settings(settings.clone());
                }
            });
        });
        set_global_settings(settings.clone());
    }

    fn build_ranking_window(&mut self, egui_ctx: &Context, ranking: &Vec<AgentSketch>) {
        if self.state.ranking {
            Window::new("RANKING").default_pos((0.0, 0.0)).title_bar(true).default_width(120.0).show(egui_ctx, |ui| {
                let mut i = 0;
                ui.horizontal(|ui| {
                    ui.label(RichText::new("NAME").strong());
                    ui.separator();
                    ui.label(RichText::new("GEN").strong());
                    ui.separator();
                    ui.label(RichText::new("PTS").strong());
                });
                ui.separator();
                for rank in ranking.iter() {
                    i += 1;
                    ui.horizontal(|ui| {
                        let msg1 = format!("{}. {}",i, rank.specie.to_uppercase());
                        let msg2 = format!("{}", rank.generation);
                        let msg3 = format!("{}", rank.points.round());
                        ui.label(RichText::new(msg1).small());
                        ui.separator();
                        ui.label(RichText::new(msg2).small());
                        ui.separator();
                        ui.label(RichText::new(msg3).small());
                    });
                    ui.separator();
                }
            });
        }
    }

    pub fn ui_draw(&self) {
        egui_macroquad::draw();
    }
    
}

/* struct LogoImage {
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
} */