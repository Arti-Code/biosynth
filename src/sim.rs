#![allow(unused)]

use crate::agent::*;
use crate::being::LifesBox;
use crate::camera::*;
use crate::consts::*;
use crate::plant::*;
use crate::ui::*;
use crate::util::{Signals, contact_mouse};
use crate::world::*;
use crate::being::Being;
use macroquad::camera::Camera2D;
use macroquad::prelude::*;
use std::f32::consts::PI;


pub static mut SIM_PARAMS: SimConfig = SimConfig {
    agent_min_num: AGENTS_NUM_MIN,
    agents_init_num: AGENTS_NUM,
    plant_init_num: PLANTS_NUM,
    plant_min_num: PLANTS_MIN_NUM,
    agent_rotation: AGENT_ROTATION,
    agent_speed: AGENT_SPEED,
    agent_vision_range: AGENT_VISION_RANGE,
    lifes_min_num: LIFE_NUM_MIN as usize,
};

pub struct Simulation {
    pub simulation_name: String,
    pub world_size: Vec2,
    pub font: Font,
    pub world: World,
    pub camera: Camera2D,
    pub running: bool,
    pub sim_time: f64,
    config: SimConfig,
    pub ui: UISystem,
    pub sim_state: SimState,
    pub signals: Signals,
    select_phase: f32,
    pub selected: u64,
    pub mouse_state: MouseState,
    pub agents: AgentsBox,
    pub plants: PlantsBox,
    pub lifes: LifesBox,
}

impl Simulation {
    pub fn new(configuration: SimConfig, font: Font) -> Self {
        Self {
            simulation_name: String::new(),
            world_size: Vec2 {
                x: WORLD_W,
                y: WORLD_H,
            },
            font: font,
            world: World::new(),
            camera: create_camera(),
            running: false,
            sim_time: 0.0,
            config: configuration,
            ui: UISystem::new(),
            sim_state: SimState::new(),
            signals: Signals::new(),
            selected: 0,
            select_phase: 0.0,
            mouse_state: MouseState { pos: Vec2::NAN },
            agents: AgentsBox::new(),
            plants: PlantsBox::new(),
            lifes: LifesBox::new(),
        }
    }

    fn reset_sim(&mut self, sim_name: Option<&str>) {
        self.simulation_name = match sim_name {
            Some(name) => name.to_string(),
            None => String::new(),
        };
        self.world = World::new();
        self.agents.agents.clear();
        self.plants.plants.clear();
        self.lifes.plants.clear();
        //self.particles.elements.clear();
        self.sim_time = 0.0;
        self.sim_state = SimState::new();
        self.sim_state.sim_name = String::from(&self.simulation_name);
        self.signals = Signals::new();
        self.selected = 0;
        self.select_phase = 0.0;
        self.mouse_state = MouseState { pos: Vec2::NAN };
        self.running = true;
        self.init();
        //self.build_wall();
    }

    pub fn init(&mut self) {
        let agents_num = self.config.agents_init_num;
        let lifes_num = self.config.lifes_min_num;
        self.agents.add_many_agents(agents_num as usize, &mut self.world);
        self.plants.add_many_plants(PLANTS_NUM, &mut self.world);
        //self.lifes.add_many_plants(lifes_num, &mut self.world);
        //self.sources.add_many(48);
    }

    pub fn autorun_new_sim(&mut self) {
        self.signals.new_sim = true;
        self.signals.new_sim_name = "BioSynth".to_string();
    }

    fn update_agents(&mut self) {
        let dt = self.sim_state.dt;
        for (_, agent) in self.agents.get_iter_mut() {
            //let uid = *id;
            if !agent.update(dt, &mut self.world) {
                match agent.physics_handle {
                    Some(handle) => {
                        self.world.remove_physics_object(handle);
                    }
                    None => {}
                }
            };
        }
        self.agents.agents.retain(|_, agent| agent.alife == true);
    }

    fn update_plants(&mut self) {
        let dt = self.sim_state.dt;
        for (_, plant) in self.plants.get_iter_mut() {
            //let uid = *id;
            if !plant.update(dt, &mut self.world) {
                match plant.physics_handle {
                    Some(handle) => {
                        self.world.remove_physics_object(handle);
                    }
                    None => {}
                }
            };
        }
        self.plants.plants.retain(|_, plant| plant.alife == true);        
    }

    fn update_lifes(&mut self) {
        let dt = self.sim_state.dt;
        for (_, life) in self.lifes.get_iter_mut() {
            //let uid = *id;
            if !life.update(dt, &mut self.world) {
                match life.physics_handle {
                    Some(handle) => {
                        self.world.remove_physics_object(handle);
                    }
                    None => {}
                }
            };
        }
        self.lifes.plants.retain(|_, plant| plant.alife == true);        
    }

    pub fn update(&mut self) {
        self.signals_check();
        self.update_sim_state();
        self.check_agents_num();
        self.calc_selection_time();
        self.update_agents();
        self.update_plants();
        self.update_lifes();
        //self.sim_state.contacts_info = self.world.get_contacts_info();
        self.world.step_physics();
    }

    pub fn draw(&self) {
        //set_default_camera();
        set_camera(&self.camera);
        clear_background(BLACK);
        draw_rectangle_lines(0.0, 0.0, self.world_size.x, self.world_size.y, 3.0, WHITE);
        self.draw_grid(50);
        self.draw_agents();
        self.draw_plants();
        self.draw_lifes();
    }

    fn draw_agents(&self) {
        for (id, agent) in self.agents.get_iter() {
            let mut draw_field_of_view: bool = false;
            if *id == self.selected {
                draw_field_of_view = true;
            };
            agent.draw(draw_field_of_view, &self.font);
        }
        match self.agents.get(self.selected) {
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
            }
            None => {}
        };
    }

    fn draw_plants(&self) {
        for (id, plant) in self.plants.get_iter() {
            let mut marked: bool = false;
            if *id == self.selected {
                marked = true;
            };
            plant.draw(marked, &self.font);
        }
        match self.plants.get(self.selected) {
            Some(selected_plant) => {
                let pos = Vec2::new(selected_plant.pos.x, selected_plant.pos.y);
                let s = selected_plant.size;
                draw_circle_lines(pos.x, pos.y, 2.0 * (s as f32) + (self.select_phase.sin() * (s as f32) * 0.5), 1.0, ORANGE);
            }
            None => {}
        };
    }

    fn draw_lifes(&self) {
        for (id, life) in self.lifes.get_iter() {
            let mut marked: bool = false;
            if *id == self.selected {
                marked = true;
            };
            life.draw(marked, &self.font);
        }
    }

    fn draw_grid(&self, cell_size: u32) {
        let w = self.world_size.x;
        let h = self.world_size.y;
        let col_num = (w / cell_size as f32).floor() as u32;
        let row_num = (h / cell_size as f32).floor() as u32;
        //draw_grid(100, 20.0, GRAY, DARKGRAY);
        for x in 0..col_num + 1 {
            for y in 0..row_num + 1 {
                draw_circle((x * cell_size) as f32, (y * cell_size) as f32, 1.0, GRAY);
            }
        }
    }

    pub fn signals_check(&mut self) {
        if self.signals.spawn_agent {
            let agent = Agent::new();
            self.agents.add_agent(agent, &mut self.world);
            self.signals.spawn_agent = false;
        }
        if self.signals.spawn_plant {
            let plant: Plant = Plant::new();
            self.plants.add_plant(plant, &mut self.world);
            self.signals.spawn_plant = false;
        }
        if self.signals.new_sim {
            self.signals.new_sim = false;
            self.reset_sim(Some(&self.signals.new_sim_name.to_owned()));
        }
    }

    pub fn input(&mut self) {
        self.mouse_input();
        control_camera(&mut self.camera);
    }

    fn mouse_input(&mut self) {
        if is_mouse_button_released(MouseButton::Left) {
            if !self.ui.pointer_over {
                self.selected = 0;
                let (mouse_posx, mouse_posy) = mouse_position();
                let mouse_pos = Vec2::new(mouse_posx, mouse_posy);
                let rel_coords = self.camera.screen_to_world(mouse_pos);
                for (id, agent) in self.agents.get_iter() {
                    if contact_mouse(rel_coords, agent.pos, agent.size) {
                        self.selected = *id;
                        break;
                    }
                }
            }
        }
    }

    fn update_sim_state(&mut self) {
        self.sim_state.fps = get_fps();
        self.sim_state.dt = get_frame_time();
        self.sim_state.sim_time += self.sim_state.dt as f64;
        let (mouse_x, mouse_y) = mouse_position();
        self.mouse_state.pos = Vec2::new(mouse_x, mouse_y);
        self.sim_state.agents_num = self.agents.agents.len() as i32;
        self.sim_state.plants_num = self.plants.plants.len() as i32;
        self.sim_state.lifes_num = self.lifes.plants.len() as i32;
        self.sim_state.physics_num = self.world.get_physics_obj_num() as i32;
        let mut kin_eng = 0.0;
        let mut total_mass = 0.0;
        for (_, rb) in self.world.rigid_bodies.iter() {
            kin_eng += rb.kinetic_energy();
            total_mass += rb.mass();
        }
        self.sim_state.total_eng = kin_eng;
        self.sim_state.total_mass = total_mass;
    }

    fn check_agents_num(&mut self) {
        unsafe {
            if self.sim_state.agents_num < (SIM_PARAMS.agent_min_num as i32) {
                let agent = Agent::new();
                self.agents.add_agent(agent, &mut self.world);
            }
            if self.sim_state.lifes_num < (SIM_PARAMS.lifes_min_num as i32) {
                self.lifes.add_many_plants(1, &mut self.world);
            }
        }
        if self.sim_state.plants_num < (self.config.plant_min_num as i32) {
            self.plants.add_many_plants(1, &mut self.world);
        }
        
    }

    fn calc_selection_time(&mut self) {
        self.select_phase += self.sim_state.dt * 4.0;
        self.select_phase = self.select_phase % (2.0 * PI as f32);
    }

    pub fn process_ui(&mut self) {
        let marked_agent = self.agents.get(self.selected);
        self.ui.ui_process(&self.sim_state, &mut self.signals, &self.camera, marked_agent);
    }

    pub fn draw_ui(&self) {
        self.ui.ui_draw();
    }

    pub fn is_running(&self) -> bool {
        return self.running;
    }
}

//?         [[[SIM_CONFIG]]]
#[derive(Clone, Copy)]
pub struct SimConfig {
    pub agents_init_num: usize,
    pub plant_min_num: usize,
    pub plant_init_num: usize,
    pub agent_min_num: usize,
    pub agent_speed: f32,
    pub agent_vision_range: f32,
    pub agent_rotation: f32,
    pub lifes_min_num: usize,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            agents_init_num: AGENTS_NUM,
            agent_min_num: AGENTS_NUM_MIN,
            plant_init_num: PLANTS_NUM,
            plant_min_num: PLANTS_MIN_NUM,
            agent_speed: AGENT_SPEED,
            agent_rotation: AGENT_ROTATION,
            agent_vision_range: AGENT_VISION_RANGE,
            lifes_min_num: LIFE_NUM_MIN as usize,
        }
    }
}

/* impl SimConfig {
    pub fn new(
        agents_num: usize,
        agents_min_num: usize,
        agent_speed: f32,
        agent_turn: f32,
        vision_range: f32,
        sources_num: usize,
        sources_min_num: usize,
    ) -> Self {
        Self {
            agents_init_num: agents_num,
            agent_min_num: agents_min_num,
            agent_speed: agent_speed,
            agent_rotation: agent_turn,
            agent_vision_range: vision_range,
            sources_init_num: sources_num,
            sources_min_num: sources_min_num,
        }
    }
} */

//?         [[[SIM_STATE]]]
pub struct SimState {
    pub sim_name: String,
    pub agents_num: i32,
    pub plants_num: i32,
    pub lifes_num: i32,
    pub physics_num: i32,
    pub total_mass: f32,
    pub total_eng: f32,
    pub sim_time: f64,
    pub fps: i32,
    pub dt: f32,
    pub total_kin_eng: f32,
    pub contacts_info: (i32, i32),
}

impl SimState {
    pub fn new() -> Self {
        Self {
            sim_name: String::new(),
            agents_num: AGENTS_NUM as i32,
            plants_num: PLANTS_NUM as i32,
            lifes_num: 0,
            physics_num: 0,
            total_mass: 0.0,
            total_eng: 0.0,
            sim_time: 0.0,
            fps: 0,
            dt: 0.0,
            total_kin_eng: 0.0,
            contacts_info: (0, 0),
        }
    }
}
//?         [[[MOUSESTATE]]]
pub struct MouseState {
    pub pos: Vec2,
}
