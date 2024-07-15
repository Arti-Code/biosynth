//#![allow(unused)]

use crate::agent::*;
use crate::camera::*;
use crate::net_draw::draw_network;
use crate::plant::*;
use crate::timer::Timer;
use crate::ui::*;
use crate::util::*;
use crate::collector::*;
use crate::terrain::*;
use crate::Statistics;
use macroquad::camera::Camera2D;
use macroquad::prelude::*;
use rapier2d::prelude::RigidBodyHandle;
use std::collections::HashMap;
use std::f32::consts::PI;
use serde_json;
use std::fs;
use std::path::Path;
use base64::prelude::*;
use crate::monit::PerformanceMonitor;
//use crate::stats::Stats;
use crate::settings::*;
use crate::signals::*;
use crate::sketch::*;
use crate::phyx::physics::Physics;
use crate::ranking::Ranking;


//#[derive(Debug)]
pub struct Simulation {
    pub simulation_name: String,
    pub world_size: Vec2,
    pub font: Font,
    pub physics: Physics,
    pub camera: Camera2D,
    pub running: bool,
    user_action: UserAction,
    last_autosave: f64,
    pub ui: UISystem,
    pub sim_state: SimState,
    pub signals: Signals,
    select_phase: f32,
    pub selected: Option<RigidBodyHandle>,
    pub mouse_state: MouseState,
    pub agents: AgentBox,
    pub plants: PlantBox,
    pub ranking: Ranking,
    population_timer: Timer,
    pub terrain: Terrain,
    coord_timer: Timer,
    monitor: PerformanceMonitor,
    lifetimes: Vec<f32>,
    sizes: Vec<f32>,
    eyes: Vec<f32>,
    speeds: Vec<f32>,
    powers: Vec<f32>,
    mutations: Vec<f32>,
    shells: Vec<f32>,
    plot_x: i32,
    borns: [i32; 4],
    deaths: [i32; 2],
    points: Vec<f32>,
    nodes: Vec<i32>,
    links: Vec<i32>,
    population_agents: Vec<i32>,
    population_plants: Vec<i32>,
    stats_timer: Timer,
    terrain_timer: Timer,
    statistics: Statistics,
    n: usize,
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
            user_action: UserAction::new(),
            ui: UISystem::new(),
            sim_state: SimState::new(),
            signals: Signals::new(),
            selected: None,
            select_phase: 0.0,
            mouse_state: MouseState { pos: Vec2::NAN },
            agents: AgentBox::new(),
            plants: PlantBox::new(),
            //ranking: vec![],
            ranking: Ranking::new(settings.ranking_size, 20, 10),
            last_autosave: 0.0,
            population_timer: Timer::new(1.0, true, true, false),
            terrain: Terrain::new(0.0, 0.0, settings.grid_size as f32),
            coord_timer: Timer::new(0.25, true, true, true),
            terrain_timer: Timer::new(0.1, true, true, false),
            monitor: PerformanceMonitor::new(1.0),
            lifetimes: vec![],
            //lifetimes: vec![],
            sizes: vec![],
            eyes: vec![],
            speeds: vec![],
            powers: vec![],
            mutations: vec![],
            shells: vec![],
            points: vec![],
            nodes: vec![],
            links: vec![],
            plot_x: 0,
            borns: [0, 0, 0, 0],
            deaths: [0, 0],
            population_agents: vec![],
            population_plants: vec![],
            stats_timer: Timer::new(5.0, true, true, false),
            statistics: Statistics::new(settings.stats_limit),
            n: 0,
        }
    }

    fn init_stats(&mut self) {
        self.statistics = Statistics::new(get_settings().stats_limit);
        self.borns = [0, 0, 0, 0];
        self.statistics.add_data_type("borns");
        self.statistics.add_data_type("deaths");
        self.statistics.add_data_type("kills");
        self.statistics.add_data_type("points");
        self.statistics.add_data_type("lifetimes");
        self.statistics.add_data_type("sizes");
        self.statistics.add_data_type("eyes");
        self.statistics.add_data_type("speeds");
        self.statistics.add_data_type("powers");
        self.statistics.add_data_type("mutations");
        self.statistics.add_data_type("shells");
        self.statistics.add_data_type("nodes");
        self.statistics.add_data_type("links");
        self.statistics.add_data_type("agents");
        self.statistics.add_data_type("plants");
    }

    fn reset_sim(&mut self, sim_name: Option<&str>) {
        /* self.simulation_name = match sim_name {
            Some(name) => name.to_string(),
            None => format!("Simulation{}", rand::gen_range(u8::MIN, u8::MAX)),
        };
        let settings = get_settings();
        self.world_size = Vec2::new(settings.world_w as f32, settings.world_h as f32);
        self.physics = Physics::new();
        self.terrain = Terrain::new(settings.world_w as f32, settings.world_h as f32, settings.grid_size as f32, settings.water_lvl);
        self.agents.agents.clear();
        self.plants.plants.clear();
        self.ranking = Ranking::new(settings.ranking_size, 20, 10);
        //self.sim_time = 0.0;
        self.sim_state = SimState::new();
        self.sim_state.sim_name = String::from(&self.simulation_name);
        self.signals = Signals::new();
        self.selected = None;
        self.select_phase = 0.0;
        self.mouse_state = MouseState { pos: Vec2::NAN };
        self.running = true; */
        self.clear_sim(sim_name);
        self.init();
    }

    fn clear_sim(&mut self, sim_name: Option<&str>) {
        self.simulation_name = match sim_name {
            Some(name) => name.to_string(),
            None => format!("Simulation{}", rand::gen_range(u8::MIN, u8::MAX)),
        };
        let settings = get_settings();
        self.world_size = Vec2::new(settings.world_w as f32, settings.world_h as f32);
        self.physics = Physics::new();
        self.agents = AgentBox::new();
        self.plants = PlantBox::new();
        self.ranking = Ranking::new(settings.ranking_size, 20, 10);
        //self.sim_time = 0.0;
        self.sim_state = SimState::new();
        self.sim_state.sim_name = String::from(&self.simulation_name);
        self.signals = Signals::new();
        self.selected = None;
        self.select_phase = 0.0;
        self.mouse_state = MouseState { pos: Vec2::NAN };
        self.running = true;
    }

    pub fn init(&mut self) {
        let settings = get_settings();
        self.terrain = Terrain::new(settings.world_w as f32, settings.world_h as f32, settings.grid_size as f32);
        let agents_num = settings.agent_init_num;
        self.agents.add_many_agents(agents_num as usize, &mut self.physics);
        self.plants.add_many_plants(settings.plant_init_num as usize, &mut self.physics);
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
                self.points.push(agent.points);
                let (n, l) = agent.get_nodes_links_num();
                self.nodes.push(n);
                self.links.push(l);

                let mut sketch = agent.get_sketch();
                sketch.points = (sketch.points).round();
                self.ranking.add_agent(sketch);
                self.physics.remove_object(agent.rbh);
                self.deaths[0] += 1;
            }
        }
        self.agents.agents.retain(|_, agent| agent.alife == true);
        self.update_coordinates();
    }

    fn update_rank(&mut self) {
        self.ranking.update();
    }

    fn update_plants(&mut self) {
        let settings = get_settings();
        let mut new_plants: Vec<Plant> = vec![];
        let num = self.plants.count() as i32;
        for (_, plant) in self.plants.get_iter_mut() {
            match plant.update_cloning(num, &mut self.physics) {
                None => {},
                Some(new_plant) => {
                    new_plants.push(new_plant);
                }
            }
            plant.update(&mut self.physics);
            if !plant.is_alive() {
                self.physics.remove_object(plant.get_body_handle());
            }
        }
        self.plants.plants.retain(|_, p| p.is_alive() == true);
        for plant in new_plants.iter() {
            self.plants.add_plant(plant.to_owned())
        }
        if self.plants.count() < settings.plant_min_num {
            self.plants.add_many_plants(2, &mut self.physics);
        }
    }

    fn update_coordinates(&mut self) {
        if self.coord_timer.update(dt()*sim_speed()) {
            let mut coords: Vec<[i32; 2]> = vec![];
            for (_, agent) in self.agents.get_iter_mut() {
                let coordinates = self.terrain.pos_to_coord(&agent.pos);
                coords.push(coordinates);
                match self.terrain.get_cell(coordinates[0] as usize, coordinates[1] as usize) {
                    Some(cell) => {
                        agent.set_water_tile(cell.get_water());
                    },
                    None => {},
                }
            }
            self.terrain.set_occupied(coords);
        }
        let (mouse_x, mouse_y) = mouse_position();
        let cursor = self.camera.screen_to_world(vec2(mouse_x, mouse_y));
        self.terrain.set_cursor_vec2(cursor);
        
    }

    fn update_terrain(&mut self) {
        if self.sim_state.update_terrain {
            if self.terrain_timer.update(dt()) {
                self.terrain.update();
                //dbg!(self.terrain.update());
            }
        }
    }

    pub fn update(&mut self) {
        self.check_signals();
        self.check_settings();
        self.update_sim_state();
        self.update_terrain();
        self.check_agents_num();
        self.update_plants();
        self.calc_selection_time();
        self.attacks();
        self.eat();
        self.update_agents();
        self.update_rank();
        let (i, _, _) = self.agents.populate(&mut self.physics, self.sim_state.sim_time);
        self.borns[0] += i;
        self.borns[1] += i;
        self.monitor.monitor();
        self.physics.step();
    }

    fn attacks(&mut self) {
        let settings = get_settings();
        let dt = dt()*sim_speed();
        let mut hits: HashMap<RigidBodyHandle, (f32, RigidBodyHandle)> = HashMap::new();
        for (id, agent) in self.agents.get_iter() {
            if !agent.attacking { continue; }
            let attacks = agent.attack();
            for tg in attacks.iter() {
                self.agents.agents.get(tg).inspect(|target| {
                    let pow1 = 0.25*agent.size + agent.power as f32;
                    let pow2 = 0.25*target.size + target.power as f32;
                    let power1 = pow1 + pow1*random_unit();
                    let power2 = pow2 + pow2*random_unit();
                    if power1 > power2 {
                        let mut a = agent.power as f32;
                        a = a + a*random_unit();
                        let d = target.shell as f32;
                        let mut dmg = (a-d) * dt * settings.damage;
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
                });
            }
        }
        let mut killers: Vec<RigidBodyHandle> = vec![];
        for (id1, (dmg, id2)) in hits.iter() {
            let agent1 = self.agents.agents.get_mut(id1).unwrap();
            let damage = *dmg;
            if damage >= 0.0 {
                let hp = damage * settings.atk_to_eng;
                agent1.add_energy(hp);
                agent1.points += hp*0.015;
            } else {
                let dmg = damage.abs() * settings.dmg_to_hp;
                agent1.get_hit(dmg);
                agent1.pain = 1.0;
                if agent1.is_death() { killers.push(*id2); }
            }
        }

        for killer_rbh in killers.iter() {
            let killer = self.agents.agents.get_mut(killer_rbh).unwrap();
            killer.points += 30.0;
            killer.kills += 1;
            self.deaths[0] += 1;
            self.deaths[1] += 1;
        }
    }

    fn eat(&mut self) {
        let settings = get_settings();
        let dt = dt()*sim_speed();
        let mut hits: HashMap<RigidBodyHandle, f32> = HashMap::new();
        for (id, agent) in self.agents.get_iter() {
            if agent.eating && !agent.attacking {
                let attacks = agent.eat();
                for tg in attacks.iter() {
                    self.plants.plants.get(tg).inspect(|_target| {
                        let power1 = agent.size/4.0 + 12.0;
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
                    });
                }
            }
        }
        for (id, dmg) in hits.iter() {
            if *dmg > 0.0 {
                let eat = *dmg;
                match self.agents.agents.get_mut(id) {
                    Some(agent) => {

                        agent.add_energy(eat);
                        if eat > 0.0 {
                            agent.points += eat*0.01;
                        }
                    },
                    None => {
                    }
                }
            } else {
                match self.plants.plants.get_mut(id) {
                    None => {
                    },
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
        clear_background(color_u8!(0,0,0,255));
        draw_rectangle_lines(0.0, 0.0, self.world_size.x, self.world_size.y, 3.0, WHITE);
        self.draw_terrain();
        self.draw_plants();
        //self.draw_grid();
        self.draw_agents();
        if get_settings().show_network {
            match self.selected {
                Some(selected) => {
                    match self.agents.get(selected) {
                        Some(selected_agent) => {
                            let phase = self.sim_state.sim_time % 1.0;
                            draw_network(selected_agent, phase as f32, self.camera.target, self.camera.zoom);
                        },
                        None => {},
                    }
                },
                None => {},
            }
        }
    }

    pub fn debug_physic(&mut self) {
        if get_settings().debug {
            self.physics.debug_draw();
        }
    }

    pub fn draw_terrain(&self) {
        let settings = get_settings();
        self.terrain.draw(settings.show_cells, settings.terrain_edit);
    }

    fn draw_plants(&self) {
        let settings = get_settings();
        for (_, res) in self.plants.get_iter() {
            res.draw(settings.show_plant_rad);
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
            agent.draw(draw_field_of_view, &self.font, &self.physics);
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
            let rare = get_settings().rare_specie_mod;
            //let r = ((rare * n as i32) as f32).log2() as i32;
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
            //let mut settings = get_settings();
            //settings.pause = true;
            self.save_sim();
        }
        if sign.load_sim_name.is_some() {
            match sign.load_sim_name.clone() {
                None => {},
                Some(name) => {
                    let sim_name = name.to_owned();
                    let mut signals = get_signals();
                    signals.load_sim_name = None;
                    set_signals(signals);
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
                    set_signals(signals);
                },
            }
        }
        if sign3.load_agent_name.is_some() {
            match sign3.load_agent_name {
                Some(agent_file_name) => {
                    self.load_encoded_agent(&agent_file_name);
                    let mut signals = get_signals();
                    signals.load_agent_name = None;
                    set_signals(signals);
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
                    set_signals(signals);
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
            self.terrain = Terrain::new(settings.world_w as f32, settings.world_h as f32, settings.grid_size as f32);
            let mut signals = get_signals();
            signals.resize_world = None;
            set_signals(signals);
        }
        if get_signals().export_settings {
            let mut signals = get_signals();
            signals.export_settings = false;
            set_signals(signals);
            self.export_settings();
        }

        if get_signals().import_settings {
            let mut signals = get_signals();
            signals.import_settings = false;
            set_signals(signals);
            self.import_settings();
        }

        if get_signals().update_terrain {
            let mut signals = get_signals();
            signals.update_terrain = false;
            set_signals(signals);
            self.sim_state.update_terrain = !self.sim_state.update_terrain;
        }
    }

    fn export_settings(&self) {
        let export_set = get_settings();
        let p = format!("saves/settings/");
        let encoded = BASE64_STANDARD.encode(serde_json::to_string(&export_set).unwrap().as_bytes());
        match fs::DirBuilder::new().recursive(true).create(p) {
            Ok(_) => {
                let p = format!("saves/settings/{}.set", self.simulation_name.to_lowercase());
                match fs::write(p.clone(), &encoded) {
                    Ok(_) => {
                        println!("Settings saved as {}.set", self.simulation_name.to_lowercase());
                    },
                    Err(e) => {
                        println!("{}", e);
                        println!("{}", p.clone());
                    },
                }
            },
            Err(e) => {
                println!("{}", e);
            },
        };
    }

    fn import_settings(&mut self) {

    }

    fn save_sim(&self) {
        let data = SimulationSketch::from_sim(self);
        let p = format!("saves/simulations/{}/", self.simulation_name.to_lowercase());
        match serde_json::to_string(&data) {
            Ok(serial) => {
                match fs::DirBuilder::new().recursive(true).create(p) {
                    Ok(_) => {
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
                        let save = String::from_utf8(decoded).expect("error during converting from utf8");
                        match serde_json::from_str::<SimulationSketch>(&save) {
                            Err(_) => {
                                println!("can't deserialize saved sim... [{}]", &f);
                            },
                            Ok(sim_sketch) => {
                                self.reset_sim(Some(sim_sketch.simulation_name.as_str()));
                                //self.simulation_name = sim_sketch.simulation_name.to_owned();
                                self.sim_state.sim_name = sim_sketch.simulation_name.to_owned();
                                self.sim_state.sim_time = sim_sketch.sim_time;
                                self.plot_x = sim_sketch.sim_time as i32 / 100;
                                self.last_autosave = sim_sketch.last_autosave;
                                self.world_size = sim_sketch.world_size.to_vec2();
                                let mut settings = sim_sketch.settings.to_owned();
                                settings.world_h = sim_sketch.world_size.y as i32;
                                settings.world_w = sim_sketch.world_size.x as i32;
                                set_settings(settings);
                                self.terrain = Terrain::from_serialized_terrain(&sim_sketch.terrain);
                                for agent_sketch in sim_sketch.agents.iter() {
                                    let agent = Agent::from_sketch(agent_sketch.clone(), &mut self.physics, self.sim_state.sim_time);
                                    self.agents.add_agent(agent);
                                }
                                let settings = get_settings();
                                self.plants.add_many_plants(settings.plant_init_num, &mut self.physics);
                                self.ranking.general = sim_sketch.ranking.to_owned();
                                self.ranking.school =  sim_sketch.school.to_owned();
                            },
                        }
                    }
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
                                let mut agent = Agent::from_sketch(agent_save.clone(), &mut self.physics, self.sim_state.sim_time);
                                let settings = get_settings();
                                agent.pos = random_position(settings.world_w as f32, settings.world_h as f32);
                                self.agents.add_agent(agent);
                            },
                            Err(_) => {

                            },
                        }
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

    pub fn input(&mut self) {
        self.mouse_input();
        self.keyboard_input();
        control_camera(&mut self.camera);
    }

    fn keyboard_input(&mut self) {
        if is_key_pressed(KeyCode::Tab) {
            match get_settings().select_mode {
                SelectMode::RANDOM => {
                    self.random_selection();
                },
                SelectMode::POINTS => {
                    self.points_selection();
                },
                SelectMode::LIFETIME => {
                    self.lifetime_selection();
                },
                _ => {
                    self.random_selection();
                }
            }
        }
        if is_key_pressed(KeyCode::Kp6) {
            let mut n = self.n + 1;
            n = clamp(n, 0, self.agents.count()-1);
            self.select_n(n);
        }
        if is_key_pressed(KeyCode::Kp4) {
            let mut n = self.n - 1;
            n = clamp(n, 0, self.agents.count()-1);
            self.select_n(n);
        }
        if is_key_pressed(KeyCode::Kp5) {
            self.select_n(0);
        }
    }

    fn select_n(&mut self, n: usize) {
        self.n = n;
        match self.agents.get_iter().nth(self.n) {
            Some((id, _)) => {
                self.selected = Some(*id);
            },
            None => {
            },
        }
    }

    fn mouse_input(&mut self) {
        match self.user_action {
            UserAction::Idle => {
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
                        for (id, plant) in self.plants.get_iter() {
                            if contact_mouse(rel_coords, plant.pos, plant.size) {
                                self.selected = Some(*id);
                                break;
                            }
                        }
                    }
                }
            },
            UserAction::WaterAdd => {
                if self.ui.pointer_over {

                } else if is_mouse_button_released(MouseButton::Left) {
                    self.terrain.add_water_at_cursor(50);
                } else if is_mouse_button_released(MouseButton::Right) {
                    self.terrain.add_water_at_cursor(-50);
                }
            },
            UserAction::TerrainAdd => {
                if self.ui.pointer_over {
                } else if is_mouse_button_released(MouseButton::Left) {
                    self.terrain.add_terrain_at_cursor(10);
                } else if is_mouse_button_released(MouseButton::Right) {
                    self.terrain.add_terrain_at_cursor(-10);
                }
            },
            _ => {},
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
        } else if self.selected.is_some() {
            if !self.agents.agents.contains_key(&self.selected.unwrap()) {
                self.points_selection();
            }
        }
    }

    fn update_sim_state(&mut self) {
        if self.stats_timer.update(dt()) {
            self.population_agents.push(self.agents.count() as i32);
            self.population_plants.push(self.plants.count() as i32);
        }
        self.sim_state.fps = self.monitor.fps();
        self.sim_state.dt = self.monitor.dt();
        self.sim_state.sim_time += (dt()*sim_speed()) as f64;
        let (mouse_x, mouse_y) = mouse_position();
        self.mouse_state.pos = Vec2::new(mouse_x, mouse_y);
        self.sim_state.agents_num = self.agents.agents.len() as i32;
        self.sim_state.sources_num = self.plants.plants.len() as i32;
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
            let points: f32 = self.points.iter().sum::<f32>()/self.points.len() as f32;
            let nodes: f32 = self.nodes.iter().sum::<i32>() as f32/self.nodes.len() as f32;
            let links: f32 = self.links.iter().sum::<i32>() as f32/self.links.len() as f32;
            let pop_agents: f32 = self.population_agents.iter().sum::<i32>() as f32/self.population_agents.len() as f32;
            let pop_plants: f32 = self.population_plants.iter().sum::<i32>() as f32/self.population_plants.len() as f32;
            self.population_agents.clear();
            self.population_plants.clear();
            self.points.clear();
            self.powers.clear();
            self.speeds.clear();
            self.eyes.clear();
            self.mutations.clear();
            self.lifetimes.clear();
            self.sizes.clear();
            self.shells.clear();
            self.nodes.clear();
            self.links.clear();
            self.sim_state.lifetime.push([(next-1) as f64, avg as f64]);
            self.statistics.add_data("lifetimes", (next-1, avg as f64));
            self.statistics.add_data("borns", (next-1, self.borns[1] as f64));
            self.statistics.add_data("deaths", (next-1, self.deaths[0] as f64));
            self.statistics.add_data("kills", (next-1, self.deaths[1] as f64));
            self.statistics.add_data("points", (next-1, points as f64));
            self.statistics.add_data("sizes", (next-1, sizes as f64));
            self.statistics.add_data("eyes", (next-1, eyes as f64));
            self.statistics.add_data("speeds", (next-1, speeds as f64));
            self.statistics.add_data("powers", (next-1, powers as f64));
            self.statistics.add_data("mutations", (next-1, mutations as f64));
            self.statistics.add_data("shells", (next-1, shells as f64));
            self.statistics.add_data("nodes", (next-1, nodes as f64));
            self.statistics.add_data("links", (next-1, links as f64));
            self.statistics.add_data("agents", (next-1, pop_agents as f64));
            self.statistics.add_data("plants", (next-1, pop_plants as f64));
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
        let dt = dt()*sim_speed();
        if self.sim_state.agents_num < (settings.agent_min_num as i32) {
            self.agent_from_zero();
            self.agent_from_sketch();
        }
        if self.population_timer.update(dt) {
            if random_unit_unsigned() < settings.new_one_probability  {
                self.agent_from_zero();
            }
            if random_unit_unsigned() < settings.new_one_probability  {
                self.agent_from_sketch();
            }
        }
    }

    fn agent_from_zero(&mut self) {
        _ = self.agents.add_many_agents(1, &mut self.physics);
        self.borns[0] += 1;
        self.borns[3] += 1;
    }

    fn agent_from_sketch(&mut self) {
        match self.ranking.get_random_agent() {
            Some(sketch) => {
                let s = sketch.to_owned();
                let agent = Agent::from_sketch(s, &mut self.physics, self.sim_state.sim_time);
                _ = self.agents.add_agent(agent);
                self.borns[0] += 1;
                self.borns[2] += 1;
            },
            None => {},
        }
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
        let selected_plant = match self.selected {
            Some(selected) => {
                self.plants.get(selected)
            },
            None => None,
        };
        self.ui.ui_process(
            &self.sim_state, 
            &mut self.signals, 
            &self.camera, 
            selected_agent, 
            selected_plant, 
            &self.ranking, 
            &self.statistics,
            &mut self.user_action
        );
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

    fn points_selection(&mut self) {
        let mut selected: Option<RigidBodyHandle> = None;
        let mut points = 0.0;
        for (handle, agent) in self.agents.get_iter() {
            if agent.points > points {
                selected = Some(*handle);
                points = agent.points;
            }
        }
        self.selected = selected;
    }
  
    fn lifetime_selection(&mut self) {
        let mut selected: Option<RigidBodyHandle> = None;
        let mut lifetime = 0.0;
        for (handle, agent) in self.agents.get_iter() {
            if agent.lifetime > lifetime {
                selected = Some(*handle);
                lifetime = agent.points;
            }
        }
        self.selected = selected;
    }

}

