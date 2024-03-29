#![allow(unused)]

use crate::agent::*;
use crate::camera::*;
use crate::neuro::MyPos2;
use crate::plant::Plant;
use crate::timer::Timer;
use crate::ui::*;
use crate::util::*;
use crate::physics::*;
use crate::collector::*;
use crate::globals::*;
use crate::terrain::*;
use base64::engine;
use macroquad::camera::Camera2D;
use macroquad::prelude::*;
use rapier2d::prelude::RigidBodyHandle;
use std::collections::HashMap;
use std::f32::consts::PI;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::path::Path;
use base64::prelude::*;
use crate::monit::PerformanceMonitor;
use crate::stats::Stats;
use crate::settings::*;
use crate::signals::*;

pub struct Simulation {
    pub simulation_name: String,
    pub world_size: Vec2,
    pub font: Font,
    pub physics: Physics,
    pub camera: Camera2D,
    pub running: bool,
    pub sim_time: f64,
    last_autosave: f64,
    pub ui: UISystem,
    pub sim_state: SimState,
    pub signals: Signals,
    select_phase: f32,
    pub selected: Option<RigidBodyHandle>,
    pub mouse_state: MouseState,
    pub agents: AgentBox,
    pub resources: ResBox,
    pub ranking: Vec<AgentSketch>,
    population_timer: Timer,
    pub terrain: Terrain,
    coord_timer: Timer,
    monitor: PerformanceMonitor,
    //tail: Tail,
    lifetimes: Vec<f32>,
    lifetime_stats: Vec<[i32; 2]>,
    sizes: Vec<f32>,
    eyes: Vec<f32>,
    speeds: Vec<f32>,
    powers: Vec<f32>,
    mutations: Vec<f32>,
    shells: Vec<f32>,
    plot_x: i32,
    stats: Stats,
    borns: [i32; 4],
    deaths: [i32; 2],
}

impl Simulation {
    
    pub fn new(font: Font) -> Self {
        let settings = get_settings();
        Self {
            simulation_name: format!("Simulation{}", rand::gen_range(u8::MIN, u8::MAX)),
            world_size: Vec2 {
                x: f32::NAN,
                y: f32::NAN,
            },
            font,
            physics: Physics::new(),
            camera: create_camera(),
            running: false,
            sim_time: 0.0,
            ui: UISystem::new(),
            sim_state: SimState::new(),
            signals: Signals::new(),
            selected: None,
            select_phase: 0.0,
            mouse_state: MouseState { pos: Vec2::NAN },
            agents: AgentBox::new(),
            resources: ResBox::new(),
            ranking: vec![],
            last_autosave: 0.0,
            population_timer: Timer::new(1.0, true, true, false),
            terrain: Terrain::new(0.0, 0.0, settings.grid_size as f32, settings.water_lvl),
            coord_timer: Timer::new(0.25, true, true, true),
            monitor: PerformanceMonitor::new(1.0),
            lifetimes: vec![],
            lifetime_stats: vec![],
            sizes: vec![],
            eyes: vec![],
            speeds: vec![],
            powers: vec![],
            mutations: vec![],
            shells: vec![],
            plot_x: 0,
            stats: Stats::new(),
            borns: [0, 0, 0, 0],
            deaths: [0, 0],
        }
    }

    fn init_stats(&mut self) {
        self.borns = [0, 0, 0, 0];
        self.sim_state.stats.add_data_type("New Creatures");
        self.sim_state.stats.add_data_type("Born Creatures");
        self.sim_state.stats.add_data_type("Rank Creatures");
        self.sim_state.stats.add_data_type("Zero Creatures");
        self.sim_state.stats.add_data_type("Deaths");
        self.sim_state.stats.add_data_type("Kills");
    }

    fn reset_sim(&mut self, sim_name: Option<&str>) {
        self.simulation_name = match sim_name {
            Some(name) => name.to_string(),
            None => format!("Simulation{}", rand::gen_range(u8::MIN, u8::MAX)),
        };
        let settings = get_settings();
        self.world_size = Vec2::new(settings.world_w as f32, settings.world_h as f32);
        self.physics = Physics::new();
        self.terrain = Terrain::new(settings.world_w as f32, settings.world_h as f32, settings.grid_size as f32, settings.water_lvl);
        self.agents.agents.clear();
        self.resources.resources.clear();
        self.sim_time = 0.0;
        self.sim_state = SimState::new();
        self.sim_state.sim_name = String::from(&self.simulation_name);
        self.signals = Signals::new();
        self.selected = None;
        self.select_phase = 0.0;
        self.mouse_state = MouseState { pos: Vec2::NAN };
        self.running = true;
        self.init();
    }

    fn clear_sim(&mut self) {
        let settings = get_settings();
        self.world_size = Vec2::new(settings.world_w as f32, settings.world_h as f32);
        self.physics = Physics::new();
        self.agents = AgentBox::new();
        self.resources = ResBox::new();
        self.ranking = vec![];
        self.sim_time = 0.0;
        self.sim_state = SimState::new();
        self.sim_state.sim_name = String::from("");
        self.signals = Signals::new();
        self.selected = None;
        self.select_phase = 0.0;
        self.mouse_state = MouseState { pos: Vec2::NAN };
        self.running = true;
    }

    pub fn init(&mut self) {
        let settings = get_settings();
        let agents_num = settings.agent_init_num;
        self.agents.add_many_agents(agents_num as usize, &mut self.physics);
        self.plot_x = (self.sim_state.sim_time/100.0) as i32;
        self.init_stats();
    }

    fn update_agents(&mut self) {
        let agents = self.agents.agents.clone();
        let cloned_agents = self.agents.agents.clone();
        for (_, agent) in self.agents.get_iter_mut() {
            if agent.enemy.is_some() {
                let enemy = cloned_agents.get(&agent.enemy.unwrap());
                match enemy {
                    None => {},
                    Some(enemy) => {
                        if agent.specie == enemy.specie {
                            agent.enemy_family = Some(true);
                        } else {
                            agent.enemy_family = Some(false);
                        }
                    }
                }
            }
            if !agent.update(&agents, &mut self.physics) {
                let lf = agent.lifetime.round();
                self.lifetimes.push(lf);
                self.sizes.push(agent.size);
                self.eyes.push(agent.eyes as f32);
                self.speeds.push(agent.speed as f32);
                self.powers.push(agent.power as f32);
                self.mutations.push(agent.mutations as f32);
                self.shells.push(agent.shell as f32);
                let mut sketch = agent.get_sketch();
                sketch.points = (sketch.points * 0.5).round();
                self.ranking.push(sketch);
                self.physics.remove_object(agent.physics_handle);
                self.deaths[0] += 1;
            }
        }
        self.agents.agents.retain(|_, agent| agent.alife == true);
        self.update_coordinates();
    }

    fn update_rank(&mut self) {
        let settings = get_settings();
        self.ranking.sort_by(|a, b| b.points.total_cmp(&a.points));
        let ranking_copy = self.ranking.to_vec();
        for elem1 in ranking_copy.iter() {
            self.ranking.retain(|elem2| {
                if elem1.specie == elem2.specie {
                    if elem1.points == elem2.points {
                        return true;
                    } else if elem2.points < elem1.points {
                        return false;
                    } else {
                        return true;
                    }
                } else {
                    return true;
                }
            });
        }
        if self.ranking.len() > settings.ranking_size {
            self.ranking.pop();
        }
    }

    fn update_res(&mut self) {
        let settings = get_settings();
        let mut new_resources: Vec<Plant> = vec![];
        for (_, res) in self.resources.get_iter_mut() {
            match res.update_cloning(&mut self.physics) {
                None => {},
                Some(new_res) => {
                    new_resources.push(new_res);
                }
            }
            res.update(&mut self.physics);
            if !res.alife {
                self.physics.remove_object(res.physics_handle);
            }
        }
        self.resources.resources.retain(|_, res| res.alife == true);
        for res in new_resources.iter() {
            self.resources.add_resource(res.to_owned())
        }
        if self.resources.count() < settings.res_min_num {
            self.resources.add_many_resources(2, &mut self.physics);
        }
    }

    fn update_coordinates(&mut self) {
        if self.coord_timer.update(get_frame_time()) {
            let mut coords: Vec<[i32; 2]> = vec![];
            for (_, agent) in self.agents.get_iter_mut() {
                let coordinates = self.terrain.pos_to_coord(&agent.pos);
                coords.push(coordinates);
            }
            self.terrain.set_occupied(coords);
        }
    }

    pub fn update(&mut self) {
        //self.tail.update(PI/2.5, &mut self.physics);
        self.check_signals();
        self.check_settings();
        self.update_sim_state();
        self.check_agents_num();
        self.update_res();
        self.calc_selection_time();
        self.attacks();
        self.eat();
        self.update_agents();
        self.update_rank();
        let n = self.agents.populate(&mut self.physics);
        self.borns[0] += n;
        self.borns[1] += n;
        self.monitor.monitor();
        self.physics.step();
    }

    fn attacks(&mut self) {
        let settings = get_settings();
        let dt = get_frame_time();
        let mut hits: HashMap<RigidBodyHandle, (f32, RigidBodyHandle)> = HashMap::new();
        for (id, agent) in self.agents.get_iter() {
            if !agent.attacking { continue; }
            let attacks = agent.attack();
            for tg in attacks.iter() {
                if let Some(target) = self.agents.agents.get(tg) {
                    let pow1 = agent.size + agent.power as f32;
                    let pow2 = target.size + target.power as f32;
                    let power1 = pow1 + pow1*random_unit();
                    let power2 = pow2 + pow2*random_unit();
                    if power1 > power2 {
                        let mut a = (agent.power as f32 + agent.size)/2.0;
                        a = a + a*random_unit();
                        let mut d = target.shell as f32;
                        //d = d + d*random_unit();
                        let mut dmg = (a - d) * dt * settings.damage;
                        //let def = 1.3-(target.shell as f32/10.0);
                        //dmg = dmg * def;
                        if dmg > 0.0 {
                            if hits.contains_key(id) {
                                let (old_dmg, _) = *hits.get_mut(id).unwrap();
                                dmg = dmg + old_dmg;
                                hits.insert(*id, (dmg, *tg));
                            } else {
                                hits.insert(*id, (dmg, *tg));
                            }
                            if hits.contains_key(tg) {
                                let (old_dmg, _) = *hits.get_mut(tg).unwrap();
                                let hit = -dmg + old_dmg;
                                hits.insert(*tg, (hit, *id));
                            } else {
                                let hit = -dmg;
                                hits.insert(*tg, (hit, *id));
                            }
                        }
                    }
                }
            }
        }
        let mut killers: Vec<RigidBodyHandle> = vec![];
        for (id1, (dmg, id2)) in hits.iter() {
            let agent1 = self.agents.agents.get_mut(id1).unwrap();
            let damage = *dmg;
            if damage >= 0.0 {
                let hp = damage * settings.atk_to_eng;
                agent1.add_energy(hp);
                agent1.points += hp*0.01;
            } else {
                agent1.add_energy(damage);
                agent1.pain = true;
                if agent1.is_death() { killers.push(*id2); }
            }
        }

        for killer_rbh in killers.iter() {
            let killer = self.agents.agents.get_mut(killer_rbh).unwrap();
            killer.points += 350.0;
            killer.kills += 1;
            self.deaths[0] += 1;
            self.deaths[1] += 1;
        }
    }

    fn eat(&mut self) {
        let settings = get_settings();
        let dt = get_frame_time();
        let mut hits: HashMap<RigidBodyHandle, f32> = HashMap::new();
        for (id, agent) in self.agents.get_iter() {
            if agent.eating && !agent.attacking {
                let attacks = agent.eat();
                for tg in attacks.iter() {
                if let Some(_target) = self.resources.resources.get(tg) {
                    let power1 = agent.size + agent.size*random_unit();
                        let mut food = settings.eat_to_eng * power1 * dt;
                        let mut bite = -food;
                        if hits.contains_key(id) {
                            let old_food = *hits.get_mut(id).unwrap();
                            food = old_food + food;
                            hits.insert(*id, food);
                        } else {
                            hits.insert(*id, food);
                        }
                        if hits.contains_key(tg) {
                            let old_food = *hits.get_mut(tg).unwrap();
                            bite = bite + old_food;
                            hits.insert(*tg, bite);
                        } else {
                            hits.insert(*tg, bite);
                        }
                    }
                }
            }
        }
        for (id, dmg) in hits.iter() {
            if *dmg > 0.0 {
                let eat = *dmg;
                let agent = self.agents.agents.get_mut(id).unwrap();
                agent.add_energy(eat);
                if eat > 0.0 {
                    agent.points += eat*0.1;
                }
            } else {
                match self.resources.resources.get_mut(id) {
                    None => println!("[WARN]: resource not exist"),
                    Some(source) => {
                        let damage = *dmg;
                        source.drain_eng(damage.abs());
                    },
                }
            }
        }
    }

    pub fn draw(&self) {
        //set_default_camera();
        set_camera(&self.camera);
        clear_background(color_u8!(35,35,35,255));
        draw_rectangle_lines(0.0, 0.0, self.world_size.x, self.world_size.y, 3.0, WHITE);
        self.draw_terrain();
        //self.draw_grid();
        self.draw_agents();
        self.draw_res();
    }

    pub fn draw_terrain(&self) {
        let settings = get_settings();
        self.terrain.draw(settings.show_cells);
    }

    fn draw_res(&self) {
        let settings = get_settings();
        for (_, res) in self.resources.get_iter() {
            res.draw(settings.show_res_rad);
        }
    }

    fn draw_agents(&self) {
        for (id, agent) in self.agents.get_iter() {
            let mut draw_field_of_view: bool = false;
            if self.selected.is_some() {
                if *id == self.selected.unwrap() {
                    draw_field_of_view = true;
                };
            }
            agent.draw(draw_field_of_view, &self.font);
        }

        match self.selected {
            Some(selected) => {
                match self.agents.get(selected) {
                    Some(selected_agent) => {
                        let pos = Vec2::new(selected_agent.pos.x, selected_agent.pos.y);
                        let s = selected_agent.size;
                        draw_circle_lines(
                            pos.x,
                            pos.y,
                            2.0 * s + (self.select_phase.sin() * s * 0.5),
                            1.0,
                            ORANGE,
                        );
                    },
                    None => {},
                }
            },
            None => {},
        }
    }

    fn draw_grid(&self) {
        let settings = get_settings();
        let cell_size = settings.grid_size;
        let w = self.world_size.x;
        let h = self.world_size.y;
        let col_num = (w / cell_size as f32).floor() as u32;
        let row_num = (h / cell_size as f32).floor() as u32;
        for x in 0..col_num + 1 {
            for y in 0..row_num + 1 {
                draw_circle((x * cell_size) as f32, (y * cell_size) as f32, 1.0, LIGHTGRAY);
            }
        }
    }

    pub fn check_signals(&mut self) {
        let sign = get_signals();
        let sign2 = sign.clone();
        let sign3 = sign.clone();
        if self.signals.spawn_agent {
            self.agents.add_many_agents(1, &mut self.physics);
            self.signals.spawn_agent = false;
        }
        if self.signals.new_sim {
            self.signals.new_sim = false;
            self.reset_sim(Some(&self.signals.new_sim_name.to_owned()));
        }
        if self.signals.new_settings {
            self.signals.new_settings = false;
        }
        if self.signals.save_selected {
            self.signals.save_selected = false;
            match self.selected {
                Some(handle) => {
                    self.save_encoded_agent(handle);
                },
                None => {},
            }
        }
        if self.signals.save_sim {
            self.signals.save_sim = false;
            self.save_sim();
        }
        if sign.load_sim_name.is_some() {
            match sign.load_sim_name.clone() {
                None => {},
                Some(name) => {
                    let sim_name = name.to_owned();
                    let mut signals = get_signals();
                    signals.load_sim_name = None;
                    set_global_signals(signals);
                    self.load_sim(&sim_name, false);
                    self.init_stats();
                },
            }
        }
        if sign2.del_sim_name.is_some() {
            match sign2.del_sim_name {
                None => {},
                Some(name) => {
                    let sim_name = name.to_owned();
                    let mut signals = get_signals();
                    self.delete_sim(&sim_name);
                    signals.del_sim_name = None;
                    set_global_signals(signals);
                },
            }
        }
        if sign3.load_agent_name.is_some() {
            match sign3.load_agent_name {
                Some(agent_file_name) => {
                    self.load_encoded_agent(&agent_file_name);
                    let mut signals = get_signals();
                    signals.load_agent_name = None;
                    set_global_signals(signals);
                },
                None => {},
            }
        }
        if sign3.del_agent_name.is_some() {
            match sign3.del_agent_name {
                Some(agent_file_name) => {
                    self.delete_agent(&agent_file_name);
                    let mut signals = get_signals();
                    signals.del_agent_name = None;
                    set_global_signals(signals);
                },
                None => {},
            }
        }
        if get_signals().resize_world.is_some() {
            let xy = get_signals().resize_world.unwrap();
            let mut settings = get_settings();
            settings.world_w = xy.x as i32; settings.world_h = xy.y as i32;
            set_settings(settings.clone());
            self.world_size = Vec2::new(settings.world_w as f32, settings.world_h as f32);
            self.terrain = Terrain::new(settings.world_w as f32, settings.world_h as f32, settings.grid_size as f32, settings.water_lvl);
            let mut signals = get_signals();
            signals.resize_world = None;
            set_global_signals(signals);
        }
    }

    fn save_sim(&self) {
        let data = SimulationSave::from_sim(self);
        let p = format!("saves/simulations/{}/", self.simulation_name.to_lowercase());
        match serde_json::to_string(&data) {
            Ok(serial) => {
                match fs::DirBuilder::new().recursive(true).create(p) {
                    Ok(_) => {
                        let f_n = format!("saves/simulations/{}/{}.sim", self.simulation_name.to_lowercase(), data.sim_time as i32);
                        let f = format!("saves/simulations/{}/last.sim", self.simulation_name.to_lowercase());
                        let encoded = BASE64_STANDARD.encode(serial.as_bytes());
                        match fs::write(f.clone(), &encoded) {
                            Ok(_) => {
                                println!("Simulation saved as last.sim.");
                            },
                            Err(e) => {
                                println!("{}", e);
                                println!("{}", f.clone());
                            },
                        }
                    },
                    Err(e) => {
                        error!("Error creating path: {}", e);
                    },
                }
            },
            Err(e) => {
                error!("Failed to serialize simulation: {:?}", e);
            },
        }
    }

    fn delete_sim(&self, sim_name: &str) {
        let f = format!("saves/simulations/{}", sim_name.to_lowercase());
        let path = Path::new(&f);
        match fs::remove_dir_all(path) {
            Err(_) => {
                println!("error during removing saved simulation");
            },
            Ok(_) => {},
        }
    }

    pub fn load_sim(&mut self, sim_name: &str, absolute_path: bool) {
        let path: &Path;
        let f: String;
        if absolute_path {
            f = sim_name.to_string().to_lowercase();
            path = Path::new(sim_name);
        } else {
            f = format!("saves/simulations/{}/last.sim", sim_name);
            path = Path::new(&f);
        }
        match fs::read_to_string(path) {
            Err(_) => {
                warn!("can't read from {}", path.to_str().unwrap());
            },
            Ok(save) => {
                match BASE64_STANDARD.decode(save.clone().into_bytes()) {
                    Err(_) => {
                        println!("error during decoding of saved sim...");
                    },
                    Ok(decoded) => {
                        let save = String::from_utf8(decoded).expect("error during decode Vec<u8> to String");
                        match serde_json::from_str::<SimulationSave>(&save) {
                            Err(_) => {
                                println!("error during deserialization of saved sim... [{}]", &f);
                            },
                            Ok(sim_state) => {
                                self.clear_sim();
                                self.simulation_name = sim_state.simulation_name.to_owned();
                                self.sim_state.sim_name = sim_state.simulation_name.to_owned();
                                self.sim_state.sim_time = sim_state.sim_time;
                                self.plot_x = sim_state.sim_time as i32 / 100;
                                self.last_autosave = sim_state.last_autosave;
                                self.world_size = sim_state.world_size.to_vec2();
                                let mut settings = sim_state.settings.to_owned();
                                settings.world_h = sim_state.world_size.y as i32;
                                settings.world_w = sim_state.world_size.x as i32;
                                set_settings(settings);
                                self.terrain = Terrain::from_serialized_terrain(&sim_state.terrain);
                                for agent_sketch in sim_state.agents.iter() {
                                    let agent = Agent::from_sketch(agent_sketch.clone(), &mut self.physics);
                                    self.agents.add_agent(agent);
                                }
                                self.ranking = sim_state.ranking.to_owned();
                            },
                        }
                    }
                }
            }
        }
    } 

    fn load_agent(&mut self, file_name: &str) {
        let f = format!("saves/agents/{}", file_name);
        let path = Path::new(&f);
        match fs::read_to_string(path) {
            Err(_) => { println!("ERROR: can't load saved agent"); },
            Ok(save) => {
                match serde_json::from_str::<AgentSketch>(&save) {
                    Err(_) => println!("ERROR: can't deserialize saved agent"),
                    Ok(agent_sketch) => {
                        let mut agent = Agent::from_sketch(agent_sketch.clone(), &mut self.physics);
                        let settings = get_settings();
                        agent.pos = random_position(settings.world_w as f32, settings.world_h as f32);
                        self.agents.add_agent(agent);
                    },
                }
            }
        }
    }

    fn load_encoded_agent(&mut self, file_name: &str) {
        let f = format!("saves/agents/{}", file_name);
        let path = Path::new(&f);
        match fs::read_to_string(path) {
            Err(_) => { println!("ERROR: can't load saved agent"); },
            Ok(save) => {
                match BASE64_STANDARD.decode(save.clone().into_bytes()) {
                    Err(_) => println!("ERROR: can't decode base64 of saved agent"),
                    Ok(decoded) => {
                        let save = String::from_utf8(decoded).expect("error during decode Vec<u8> to String");
                        match serde_json::from_str::<AgentSketch>(&save) {
                            Ok(agent_save) => {
                                let mut agent = Agent::from_sketch(agent_save.clone(), &mut self.physics);
                                let settings = get_settings();
                                agent.pos = random_position(settings.world_w as f32, settings.world_h as f32);
                                self.agents.add_agent(agent);
                            },
                            Err(_) => {

                            },
                        }
                    },
                    Err(_) => {
                        println!("WARNING: failed to parse save as json; treating it as raw binary");
                    },
                }
            }
        }
    }

    fn delete_agent(&mut self, file_name: &str) {
        let f = format!("saves/agents/{}", file_name);
        let path = Path::new(&f);
        match fs::remove_file(path) {
            Err(_) => {
                println!("error during removing agent");
            },
            Ok(_) => {},
        }
    }



    fn save_encoded_agent(&self, handle: RigidBodyHandle) {
        match self.agents.get(handle) {
            Some(agent) => {
                let agent_sketch = agent.get_sketch();
                let serialized = serde_json::to_string(&agent_sketch);
                match serialized {
                    Ok(serialized_agent) => {
                        let encoded = BASE64_STANDARD.encode(serialized_agent.as_bytes());
                        let path_str = format!("saves/agents/{}-{}.agent", agent.specie.to_uppercase(), agent.generation);
                        let path = Path::new(&path_str);
                        match fs::write(path, encoded.clone()) {
                            Ok(_) => {},
                            Err(e) => {
                                eprintln!("Couldn't write encoded agent: {}", e);
                            },
                        }
                    },
                    Err(_) => {
                        eprintln!("Failed to serialize agent");
                    },
                }
            },
            None => {
                warn!("WARN: agent not selected");
            },
        }
    }

    fn save_agent_sketch(&self, handle: RigidBodyHandle) {
        match self.agents.get(handle) {
            Some(agent) => {
                let agent_sketch = agent.get_sketch();
                let s = serde_json::to_string_pretty(&agent_sketch);
                match s {
                    Ok(js) => {
                        let path_str = format!("saves/agents/{}-{}.json", agent.specie.to_uppercase(), agent.generation);
                        let path = Path::new(&path_str);
                        match fs::write(path, js.clone()) {
                            Ok(_) => {},
                            Err(_) => println!("ERROR: can't save agent"),
                        }
                    },
                    Err(_) => println!("ERROR: can't serialize agent sketch"),
                }
            },
            None => println!("WARN: agent not selected"),
        }
    }

    pub fn input(&mut self) {
        self.mouse_input();
        self.keyboard_input();
        control_camera(&mut self.camera);
    }

    fn keyboard_input(&mut self) {
        if is_key_pressed(KeyCode::Tab) {
            self.random_selection();
        }
    }

    fn mouse_input(&mut self) {
        if is_mouse_button_released(MouseButton::Left) {
            if !self.ui.pointer_over {
                self.selected = None;
                let (mouse_posx, mouse_posy) = mouse_position();
                let mouse_pos = Vec2::new(mouse_posx, mouse_posy);
                let rel_coords = self.camera.screen_to_world(mouse_pos);
                for (id, agent) in self.agents.get_iter() {
                    if contact_mouse(rel_coords, agent.pos, agent.size) {
                        self.selected = Some(*id);
                        break;
                    }
                }
                if self.selected.is_some() { return; }
                for (id, res) in self.resources.get_iter() {
                    if contact_mouse(rel_coords, res.pos, res.size) {
                        self.selected = Some(*id);
                        break;
                    }
                }
            }
        }
    }

    fn check_settings(&mut self) {
        let settings = get_settings();
        if settings.follow_mode && self.selected.is_some() {
            match self.selected {
                None => {},
                Some(sel) => {
                    match self.agents.get(sel) {
                        None => {},
                        Some(agent) => {
                            let pos = agent.pos;
                            self.camera.target = pos;
                        },
                    }
                }
            }
        } else if settings.follow_mode && self.selected.is_none() {
            self.random_selection();
        }
        if settings.water_lvl != self.terrain.water_level() {
            self.terrain.set_water_level(settings.water_lvl);
        }

    }

    fn update_sim_state(&mut self) {
        self.sim_state.fps = self.monitor.fps();
        self.sim_state.dt = self.monitor.dt();
        self.sim_state.sim_time += get_frame_time() as f64;
        let (mouse_x, mouse_y) = mouse_position();
        self.mouse_state.pos = Vec2::new(mouse_x, mouse_y);
        self.sim_state.agents_num = self.agents.agents.len() as i32;
        self.sim_state.sources_num = self.resources.resources.len() as i32;
        self.sim_state.physics_num = self.physics.get_bodies_num() as i32;
        (self.sim_state.rigid_num, self.sim_state.colliders_num) = self.physics.get_bodies_and_colliders_num();
        let kin_eng = 0.0;
        let total_mass = 0.0;
        self.sim_state.total_eng = kin_eng;
        self.sim_state.total_mass = total_mass;
        let l = self.sim_state.lifetime.len() as i32;
        let x = self.sim_state.sim_time as i32/100;
        let next = x-self.plot_x;
        if l < next {
            let l2 = self.lifetimes.len() as f32;
            let avg: f32 = self.lifetimes.iter().sum::<f32>()/l2;
            let sizes: f32 = self.sizes.iter().sum::<f32>()/l2;
            let powers: f32 = self.powers.iter().sum::<f32>()/l2;
            let speeds: f32 = self.speeds.iter().sum::<f32>()/l2;
            let eyes: f32 = self.eyes.iter().sum::<f32>()/l2;
            let shells: f32 = self.shells.iter().sum::<f32>()/l2;
            let mutations: f32 = self.mutations.iter().sum::<f32>()/l2;
            self.powers.clear();
            self.speeds.clear();
            self.eyes.clear();
            self.mutations.clear();
            self.lifetimes.clear();
            self.sizes.clear();
            self.shells.clear();
            self.sim_state.lifetime.push([(next-1) as f64, avg as f64]);
            self.sim_state.sizes.push([(next-1) as f64, sizes as f64]);
            self.sim_state.powers.push([(next-1) as f64, powers as f64]);
            self.sim_state.speeds.push([(next-1) as f64, speeds as f64]);
            self.sim_state.eyes.push([(next-1) as f64, eyes as f64]);
            self.sim_state.shells.push([(next-1) as f64, shells as f64]);
            self.sim_state.mutations.push([(next-1) as f64, mutations as f64]);
            self.sim_state.stats.add_data("New Creatures", (next-1, self.borns[0] as f64));
            self.sim_state.stats.add_data("Born Creatures", (next-1, self.borns[1] as f64));
            self.sim_state.stats.add_data("Rank Creatures", (next-1, self.borns[2] as f64));
            self.sim_state.stats.add_data("Zero Creatures", (next-1, self.borns[3] as f64));
            self.sim_state.stats.add_data("Deaths", (next-1, self.deaths[0] as f64));
            self.sim_state.stats.add_data("Kills", (next-1, self.deaths[1] as f64));
            self.borns = [0, 0, 0, 0];
            self.deaths = [0, 0];
        }
        if (self.sim_state.sim_time-self.last_autosave).round() >= 1000.0 {
            self.last_autosave = self.sim_state.sim_time.round();
            self.save_sim();
        } 
    }

    fn check_agents_num(&mut self) {
        let settings = get_settings();
        let dt = get_frame_time();
        if self.sim_state.agents_num < (settings.agent_min_num as i32) {
            self.agent_from_zero();
            self.agent_from_sketch();
        }
        if self.population_timer.update(dt) {
            if random_unit_unsigned() < settings.new_one_probability  {
                self.agent_from_zero();
            }
        }
        if self.population_timer.update(dt) {
            if random_unit_unsigned() < settings.new_one_probability  {
                self.agent_from_sketch();
            }
        }
    }

    fn agent_from_zero(&mut self) {
        self.agents.add_many_agents(1, &mut self.physics);
        self.borns[0] += 1;
        self.borns[3] += 1;
    }

    fn agent_from_sketch(&mut self) {
        if self.ranking.is_empty() {
            return;
        }
        let l = self.ranking.len();
        let r = rand::gen_range(0, l);
        let agent_sketch = self.ranking.get_mut(r).unwrap();
        let s = agent_sketch.to_owned();
        let agent = Agent::from_sketch(s, &mut self.physics);
        agent_sketch.points -= agent_sketch.points*0.5;
        agent_sketch.points = agent_sketch.points.round();
        self.agents.add_agent(agent);
        self.borns[0] += 1;
        self.borns[2] += 1;
    }

    fn calc_selection_time(&mut self) {
        self.select_phase += self.sim_state.dt * 4.0;
        self.select_phase = self.select_phase % (2.0 * PI as f32);
    }

    pub fn process_ui(&mut self) {
        let selected_agent = match self.selected {
            Some(selected) => {
                self.agents.get(selected)
            },
            None => None,
        };
        let selected_resource = match self.selected {
            Some(selected) => {
                self.resources.get(selected)
            },
            None => None,
        };
        self.ui.ui_process(&self.sim_state, &mut self.signals, &self.camera, selected_agent, selected_resource, &self.ranking);
    }

    pub fn draw_ui(&self) {
        self.ui.ui_draw();
    }

    pub fn is_running(&self) -> bool {
        return self.running;
    }

    fn random_selection(&mut self) {
        let n = self.agents.count();
        let r = rand::gen_range(0, n);
        let keys: Vec<&RigidBodyHandle> = self.agents.agents.keys().collect();
        self.selected = Some(*keys[r]);
    }



}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SimulationSave {
    pub simulation_name: String,
    pub world_size: MyPos2,
    pub sim_time: f64,
    last_autosave: f64,
    pub agents: Vec<AgentSketch>,
    pub ranking: Vec<AgentSketch>,
    settings: Settings,
    terrain: SerializedTerrain,
}

impl SimulationSave {
    
    pub fn from_sim(sim: &Simulation) -> Self {
        let mut agents: Vec<AgentSketch> = vec![];
        let mut ranking: Vec<AgentSketch> = vec![];
        for (_, agent) in sim.agents.get_iter() {
            let sketch = agent.get_sketch();
            agents.push(sketch);
        }
        for sketch in sim.ranking.iter() {
            let sketch2 = sketch.to_owned();
            ranking.push(sketch2);
        }
        let settings = get_settings();
        Self { 
            simulation_name: sim.simulation_name.to_owned(), 
            world_size: MyPos2::from_vec(&sim.world_size), 
            sim_time: sim.sim_state.sim_time.round(), 
            agents: agents.to_owned(), 
            ranking: ranking.to_owned(),
            last_autosave: sim.sim_state.sim_time.round(),
            settings: settings.to_owned(),
            terrain: SerializedTerrain::new(&sim.terrain),

        }
    }

}