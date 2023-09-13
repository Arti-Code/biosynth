#![allow(unused)]

use crate::unit::*;
use crate::camera::*;
use crate::consts::*;
use crate::ui::*;
use crate::util::*;
use crate::physics::*;
use crate::collector::*;
use macroquad::camera::Camera2D;
use macroquad::prelude::*;
use std::f32::consts::PI;



pub struct Simulation {
    pub simulation_name: String,
    pub world_size: Vec2,
    pub font: Font,
    pub physics: PhysicsWorld,
    pub camera: Camera2D,
    pub running: bool,
    pub sim_time: f64,
    pub settings: Settings,
    pub ui: UISystem,
    pub sim_state: SimState,
    pub signals: Signals,
    select_phase: f32,
    pub selected: u64,
    pub mouse_state: MouseState,
    pub units: UnitsBox,
}

impl Simulation {
    
    pub fn new(settings: Settings, font: Font) -> Self {
        Self {
            simulation_name: String::new(),
            world_size: Vec2 {
                x: f32::NAN,
                y: f32::NAN,
            },
            font,
            physics: PhysicsWorld::new(),
            camera: create_camera(),
            running: false,
            sim_time: 0.0,
            settings,
            ui: UISystem::new(),
            sim_state: SimState::new(),
            signals: Signals::new(),
            selected: 0,
            select_phase: 0.0,
            mouse_state: MouseState { pos: Vec2::NAN },
            units: UnitsBox::new(),
        }
    }

    fn reset_sim(&mut self, sim_name: Option<&str>) {
        self.simulation_name = match sim_name {
            Some(name) => name.to_string(),
            None => String::from("Simulation"),
        };
        self.world_size = Vec2::new(self.settings.world_w as f32, self.settings.world_h as f32);
        self.physics = PhysicsWorld::new();
        self.units.agents.clear();
        self.sim_time = 0.0;
        self.sim_state = SimState::new();
        self.sim_state.sim_name = String::from(&self.simulation_name);
        self.signals = Signals::new();
        self.selected = 0;
        self.select_phase = 0.0;
        self.mouse_state = MouseState { pos: Vec2::NAN };
        self.running = true;
        self.init();
    }

    pub fn init(&mut self) {
        let agents_num = self.settings.agent_init_num;
        self.units.add_many_agents(agents_num as usize, &mut self.physics, &self.settings);
    }

    pub fn autorun_new_sim(&mut self) {
        self.signals.new_sim = true;
        self.signals.new_sim_name = "BioSynth".to_string();
    }

    fn update_agents(&mut self) {
        let dt = self.sim_state.dt;
        for (_, agent) in self.units.get_iter_mut() {
            if !agent.update(dt, &mut self.physics) {
                self.physics.remove_physics_object(agent.physics_handle);
            }
        }
        self.units.agents.retain(|_, agent| agent.alife == true);
    }

    pub fn update(&mut self) {
        self.signals_check();
        self.update_sim_state();
        self.check_agents_num();
        self.calc_selection_time();
        self.update_agents();
        self.units.populate(&mut self.physics);
        self.physics.step_physics();
    }

    pub fn draw(&self) {
        //set_default_camera();
        set_camera(&self.camera);
        clear_background(BLACK);
        draw_rectangle_lines(0.0, 0.0, self.world_size.x, self.world_size.y, 3.0, WHITE);
        self.draw_grid(50);
        self.draw_agents();
        if self.settings.show_network { self.draw_network(); }
    }

    fn draw_network(&self) {
        match self.units.get(self.selected) {
            None => {},
            Some(agent) => {
                agent.network.draw();
            },
        }
    }

    fn draw_agents(&self) {
        for (id, agent) in self.units.get_iter() {
            let mut draw_field_of_view: bool = false;
            if *id == self.selected {
                draw_field_of_view = true;
            };
            agent.draw(draw_field_of_view, &self.font);
        }
        match self.units.get(self.selected) {
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

    fn draw_grid(&self, cell_size: u32) {
        let w = self.world_size.x;
        let h = self.world_size.y;
        let col_num = (w / cell_size as f32).floor() as u32;
        let row_num = (h / cell_size as f32).floor() as u32;
        for x in 0..col_num + 1 {
            for y in 0..row_num + 1 {
                draw_circle((x * cell_size) as f32, (y * cell_size) as f32, 1.0, GRAY);
            }
        }
    }

    pub fn signals_check(&mut self) {
        if self.signals.spawn_agent {
            self.units.add_many_agents(1, &mut self.physics, &self.settings);
            self.signals.spawn_agent = false;
        }
        if self.signals.new_sim {
            self.signals.new_sim = false;
            self.reset_sim(Some(&self.signals.new_sim_name.to_owned()));
        }
        if self.signals.new_settings {
            self.signals.new_settings = false;
            self.units.reload_settings(&self.settings)
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
                for (id, agent) in self.units.get_iter() {
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
        self.sim_state.agents_num = self.units.agents.len() as i32;
        self.sim_state.physics_num = self.physics.get_physics_obj_num() as i32;
        let mut kin_eng = 0.0;
        let mut total_mass = 0.0;
        for (_, rb) in self.physics.rigid_bodies.iter() {
            kin_eng += rb.kinetic_energy();
            total_mass += rb.mass();
        }
        self.sim_state.total_eng = kin_eng;
        self.sim_state.total_mass = total_mass;
    }

    fn check_agents_num(&mut self) {
        if self.sim_state.agents_num < (self.settings.agent_min_num as i32) {
            self.units.add_many_agents(1, &mut self.physics, &self.settings);
        }
    }

    fn calc_selection_time(&mut self) {
        self.select_phase += self.sim_state.dt * 4.0;
        self.select_phase = self.select_phase % (2.0 * PI as f32);
    }

    pub fn process_ui(&mut self) {
        let marked_agent = self.units.get(self.selected);
        self.ui.ui_process(&mut self.settings ,&self.sim_state, &mut self.signals, &self.camera, marked_agent);
    }

    pub fn draw_ui(&self) {
        self.ui.ui_draw();
    }

    pub fn is_running(&self) -> bool {
        return self.running;
    }
}

