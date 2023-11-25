#![allow(unused)]

use crate::agent::*;
use crate::camera::*;
use crate::neuro::MyPos2;
use crate::timer::Timer;
use crate::ui::*;
use crate::util::*;
use crate::physics::*;
use crate::collector::*;
use crate::globals::*;
use macroquad::camera::Camera2D;
use macroquad::prelude::*;
use macroquad::experimental::collections::storage;
use rapier2d::prelude::RigidBodyHandle;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fmt::format;
use std::fs::DirEntry;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;
use std::path::Path;



pub struct Simulation {
    pub simulation_name: String,
    pub world_size: Vec2,
    pub font: Font,
    pub physics: PhysicsWorld,
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
}

impl Simulation {
    
    pub fn new(font: Font) -> Self {
        Self {
            simulation_name: format!("Simulation{}", rand::gen_range(u8::MIN, u8::MAX)),
            world_size: Vec2 {
                x: f32::NAN,
                y: f32::NAN,
            },
            font,
            physics: PhysicsWorld::new(),
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
        }
    }

    fn reset_sim(&mut self, sim_name: Option<&str>) {
        self.simulation_name = match sim_name {
            Some(name) => name.to_string(),
            None => format!("Simulation{}", rand::gen_range(u8::MIN, u8::MAX)),
        };
        let settings = get_settings();
        self.world_size = Vec2::new(settings.world_w as f32, settings.world_h as f32);
        self.physics = PhysicsWorld::new();
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
        self.physics = PhysicsWorld::new();
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
    }

    pub fn autorun_new_sim(&mut self) {
        self.signals.new_sim = true;
        self.signals.new_sim_name = "BioSynth".to_string();
    }

    fn update_agents(&mut self) {
        for (_, agent) in self.agents.get_iter_mut() {
            if !agent.update(&mut self.physics) {
                let sketch = agent.get_sketch();
                self.ranking.push(sketch);
                self.physics.remove_physics_object(agent.physics_handle);
            }
        }
        self.agents.agents.retain(|_, agent| agent.alife == true);
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
        let min_points = 0.0;
    }

    fn update_res(&mut self) {
        for (_, res) in self.resources.get_iter_mut() {
            res.update(&mut self.physics);
            if !res.alife {
                self.physics.remove_physics_object(res.physics_handle);
            }
        }
        self.resources.resources.retain(|_, res| res.alife == true);
        let dt = get_frame_time();
        let settings = get_settings();
        let res_num = settings.res_num;
        let res_prob = (res_num*dt)/(self.resources.count() as f32+1.0);
        if rand::gen_range(0.0, 1.0) < res_prob {
            self.resources.add_many_resources(1, &mut self.physics);
        }
    }

    pub fn update(&mut self) {
        self.signals_check();
        self.update_sim_state();
        self.check_agents_num();
        self.update_res();
        self.calc_selection_time();
        self.attacks();
        self.eat();
        self.update_agents();
        self.update_rank();
        self.agents.populate(&mut self.physics);
        self.physics.step_physics();
    }

    fn attacks(&mut self) {
        let settings = get_settings();
        let dt = get_frame_time();
        //let temp_units = self.units.agents.
        let mut hits: HashMap<RigidBodyHandle, f32> = HashMap::new();
        for (id, agent) in self.agents.get_iter() {
            if !agent.attacking { continue; }
            let attacks = agent.attack();
            for tg in attacks.iter() {
                if let Some(mut target) = self.agents.agents.get(tg) {
                    let power1 = agent.size + agent.size*random_unit();
                    let power2 = target.size + target.size*random_unit();
                    if power1 > power2 {
                        let mut dmg = (power1 - power2) * dt * settings.damage;
                        if hits.contains_key(id) {
                            let old_dmg = *hits.get_mut(id).unwrap();
                            dmg = dmg + old_dmg;
                            hits.insert(*id, dmg);
                        } else {
                            hits.insert(*id, dmg);
                        }
                        if hits.contains_key(tg) {
                            let old_dmg = *hits.get_mut(tg).unwrap();
                            let hit = -dmg + old_dmg;
                            hits.insert(*tg, hit);
                        } else {
                            let hit = -dmg;
                            hits.insert(*tg, hit);
                        }
                    }
                }
            }
        }
        for (id, dmg) in hits.iter() {
            let mut agent = self.agents.agents.get_mut(id).unwrap();
            let mut damage = *dmg;
            if damage >= 0.0 {
                let hp = damage * settings.atk_to_eng;
                agent.add_energy(hp);
                agent.points += hp*0.1;
            } else {
                agent.add_energy(damage);
                agent.pain = true;
            }
        }
    }

    fn eat(&mut self) {
        let settings = get_settings();
        //let eat_to_eng = settings.eat_to_eng;
        let dt = get_frame_time();
        //let temp_units = self.units.agents.
        let mut hits: HashMap<RigidBodyHandle, f32> = HashMap::new();
        for (id, agent) in self.agents.get_iter() {
            let attacks = agent.eat();
            for tg in attacks.iter() {
                if let Some(mut target) = self.resources.resources.get(tg) {
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
        for (id, dmg) in hits.iter() {
            if *dmg > 0.0 {
                let eat = *dmg;
                let mut agent = self.agents.agents.get_mut(id).unwrap();
                agent.add_energy(eat);
                if eat > 0.0 {
                    agent.points += eat*0.1;
                }
            } else {
                let mut source = self.resources.resources.get_mut(id).unwrap();
                let damage = *dmg;
                source.drain_eng(damage.abs());
            }
        }
    }

    pub fn draw(&self) {
        //set_default_camera();
        set_camera(&self.camera);
        clear_background(color_u8!(35,35,35,255));
        draw_rectangle_lines(0.0, 0.0, self.world_size.x, self.world_size.y, 3.0, WHITE);
        self.draw_grid();
        self.draw_agents();
        self.draw_res();
    }

    fn draw_res(&self) {
        for (id, res) in self.resources.get_iter() {
            res.draw();
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

    pub fn signals_check(&mut self) {
        let mut sign = get_signals();
        if self.signals.spawn_agent {
            self.agents.add_many_agents(1, &mut self.physics);
            self.signals.spawn_agent = false;
        }
        if self.signals.new_sim {
            self.signals.new_sim = false;
            self.reset_sim(Some(&self.signals.new_sim_name.to_owned()));
            //self.app_state = AppState::RESTART;
        }
        if self.signals.new_settings {
            self.signals.new_settings = false;
        }
        if self.signals.save_selected {
            self.signals.save_selected = false;
            match self.selected {
                Some(handle) => {
                    self.save_agent_sketch(handle);
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
                    sign.load_sim_name = None;
                    init_global_signals(sign);
                    self.load_sim(&sim_name);
                },
            }
        }
    }

    fn save_sim(&self) {
        let data = SimulationSave::from_sim(self);
        let s = serde_json::to_string_pretty(&data);
        let p = format!("saves/simulations/{}/", self.simulation_name);
        match s {
            Ok(save) => {
                match fs::DirBuilder::new().recursive(true).create(p) {
                    Ok(_) => {
                        let f = format!("saves/simulations/{}/last.json", self.simulation_name);
                        match fs::write(f, save) {
                            Ok(_) => {println!("SAVED");},
                            Err(_) => println!("ERROR"),
                        }
                    },
                    Err(_) => {
                        let f = format!("saves/simulations/{}/last.json", self.simulation_name);
                        match fs::write(f, save) {
                            Ok(_) => {println!("SAVED EXIST");},
                            Err(_) => println!("ERROR EXIST"),
                        }
                    },
                }
            },
            Err(_) => {
                warn!("ERROR PATH");
            },
        }
    }

    fn load_sim(&mut self, sim_name: &str) {
        let f = format!("saves/simulations/{}/last.json", sim_name);
        let path = Path::new(&f);
        match fs::read_to_string(path) {
            Err(_) => {},
            Ok(save) => {
                match serde_json::from_str::<SimulationSave>(&save) {
                    Err(_) => {
                        println!("error during deserialization of saved sim...");
                    },
                    Ok(sim_state) => {
                        self.clear_sim();
                        for agent_sketch in sim_state.agents.iter() {
                            let agent = Agent::from_sketch(agent_sketch.clone(), &mut self.physics);
                            self.agents.add_agent(agent);
                        }
                        self.ranking = sim_state.ranking.to_owned();
                        self.sim_state.sim_time = sim_state.sim_time;
                        self.last_autosave = sim_state.last_autosave;
                        self.simulation_name = sim_state.simulation_name.to_owned();
                        self.sim_state.sim_name = sim_state.simulation_name.to_owned();
                        self.world_size = sim_state.world_size.to_vec2();
                        let mut settings = sim_state.settings.to_owned();
                        settings.world_h = sim_state.world_size.y as i32;
                        settings.world_w = sim_state.world_size.x as i32;
                        init_global_settings(settings);
                    },
                }
            }
        }
    }

/*     fn clean_sim(&mut self) {
        for (_, agent) in self.agents.get_iter_mut() {
            self.physics.remove_physics_object(agent.physics_handle);
        }
        self.agents.agents.clear();

        for (_, source) in self.resources.get_iter_mut() {
            self.physics.remove_physics_object(source.physics_handle);
        }
        self.resources.resources.clear();

        self.ranking.clear();

        //self.physics.rigid_bodies.
    } */

    fn save_agent_sketch(&self, handle: RigidBodyHandle) {
        match self.agents.get(handle) {
            Some(agent) => {
                let agent_sketch = agent.get_sketch();
                let s = serde_json::to_string_pretty(&agent_sketch);
                match s {
                    Ok(js) => {
                        let path_str = format!("saves/agents/agent{}.json", agent.key);
                        let path = Path::new(&path_str);
                        match fs::write(path, js.clone()) {
                            Ok(_) => {},
                            Err(_) => println!("ERROR: not saved"),
                        }
                    },
                    Err(_) => {},
                }
            },
            None => {},
        }
    }

    pub fn input(&mut self) {
        self.mouse_input();
        control_camera(&mut self.camera);
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
            }
        }
    }

    fn update_sim_state(&mut self) {
        self.sim_state.fps = get_fps();
        self.sim_state.dt = get_frame_time();
        self.sim_state.sim_time += get_frame_time() as f64;
        let (mouse_x, mouse_y) = mouse_position();
        self.mouse_state.pos = Vec2::new(mouse_x, mouse_y);
        self.sim_state.agents_num = self.agents.agents.len() as i32;
        self.sim_state.sources_num = self.resources.resources.len() as i32;
        self.sim_state.physics_num = self.physics.get_physics_obj_num() as i32;
        (self.sim_state.rigid_num, self.sim_state.colliders_num) = self.physics.get_numbers();
        let mut kin_eng = 0.0;
        let mut total_mass = 0.0;
        for (_, rb) in self.physics.rigid_bodies.iter() {
            kin_eng += rb.kinetic_energy();
            total_mass += rb.mass();
        }
        self.sim_state.total_eng = kin_eng;
        self.sim_state.total_mass = total_mass;
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
                self.agent_from_sketch();
            }
        }
    }

    fn agent_from_zero(&mut self) {
        self.agents.add_many_agents(1, &mut self.physics);
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
        agent_sketch.points -= agent_sketch.points*0.35;
        agent_sketch.points.round();
        self.agents.add_agent(agent);
    }

    fn calc_selection_time(&mut self) {
        self.select_phase += self.sim_state.dt * 4.0;
        self.select_phase = self.select_phase % (2.0 * PI as f32);
    }

    pub fn process_ui(&mut self) {
        let selected = match self.selected {
            Some(selected) => {
                self.agents.get(selected)
            },
            None => None,
        };
        self.ui.ui_process(&self.sim_state, &mut self.signals, &self.camera, selected, &self.ranking);
    }

    pub fn draw_ui(&self) {
        self.ui.ui_draw();
    }

    pub fn is_running(&self) -> bool {
        return self.running;
    }

    pub fn to_serialize(&self) {

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
        }
    }

}