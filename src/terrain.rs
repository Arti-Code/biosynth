#![allow(unused)]


use std::collections::HashMap;
use std::f32::consts::PI;
use crate::neuro::*;
use crate::timer::*;
use crate::util::*;
use crate::physics::*;
use crate::globals::*;
use macroquad::{color, prelude::*};
use macroquad::rand::*;
use rapier2d::geometry::*;
use rapier2d::na::Vector2;
use rapier2d::prelude::{RigidBody, RigidBodyHandle};
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use noise::{*, utils::{NoiseMap, PlaneMapBuilder, NoiseMapBuilder}};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cell {
    alt: u8,
}

impl Cell {

    pub fn new(altitude: f32) -> Self {
        Self {
            alt: clamp(altitude, 0.0, 10.0) as u8,
        }
    }

    pub fn set_cell(&mut self, altitude: f32) {
        self.alt = clamp(altitude, 0.0, 10.0) as u8;
    }

    pub fn get_altitude(&self) -> f32 {
        return self.alt as f32;
    }

    pub fn get_color(&self) -> Color {
        let alt = self.alt as f32;
        let c = (alt * 10.0 + 50.0) as u8;
        let mut color = color_u8!(c, c, c, 255);
        return color;
    }

}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Terrain {
    pub cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    cell_size: f32,
    occupied: Vec<[i32; 2]>
}

impl Terrain {

    pub fn new(w: f32, h: f32, s: f32) -> Self {
        let row_num = (h / s) as usize;
        let col_num = (w / s) as usize;
        let map = Self::generate_noise_map(col_num, row_num);
        let mut cells: Vec<Vec<Cell>> = vec![];
        for r in 0..row_num {
            let mut row: Vec<Cell> = vec![];
            for c in 0..col_num {
                let mut v = map.get_value(c, r) as f32;
                v = v + 0.4;
                v = clamp(v, 0.0, 1.0);
                let cell = Cell::new(v*10.0);
                row.push(cell);
            }
            cells.push(row);
        }
        Self {cells, width: col_num, height: row_num, cell_size: s, occupied: vec![]}
    }

    pub fn from_serialized_terrain(serialized: &SerializedTerrain) -> Self {
        Self { 
            cells: serialized.cells.to_vec(), 
            width: serialized.columns_num,
            height: serialized.rows_num,
            cell_size: serialized.cell_size,
            occupied: vec![],
        }
    }

    pub fn update(&mut self) {
        for r in 0..self.cells.len() {
            for c in 0..self.cells[r].len() {
                let cell = &mut self.cells[r][c];
                
            }
        }
    }

    pub fn get_altitude(&self, x: usize, y: usize) -> f32 {
        return self.cells[y][x].get_altitude();
    }

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        return self.cells[y][x].get_color();
    }

    pub fn draw(&self) {
        for r in 0..self.cells.len() {
            for c in 0..self.cells[r].len() {
                let mut color = self.get_color(c, r);
                color.a = 0.35;
                let x0 = c as f32 * self.cell_size;
                let y0 = r as f32 * self.cell_size;
                draw_rectangle(x0, y0, self.cell_size, self.cell_size, color);
            }
        }
        for occupied in self.occupied.iter() {
            let x0 = occupied[0] as f32 * self.cell_size;
            let y0 = occupied[1] as f32 * self.cell_size;
            draw_rectangle_lines(x0, y0, self.cell_size, self.cell_size, 2.0, color_u8!(255, 0, 0, 255));
        }
    }

    pub fn set_occupied(&mut self, coord_list: Vec<[i32; 2]>) {
        self.occupied = coord_list;
    }

    pub fn pos_to_coord(&self, position: &Vec2) -> [i32; 2] {
        let x = ((position.x/self.cell_size).floor()) as i32;
        let y = ((position.y/self.cell_size).floor()) as i32;
        return [x, y];
    }

    pub fn coord_to_pos(&self, coordinates: [i32; 2]) -> Vec2 {
        let x = coordinates[0] as f32 * self.cell_size;
        let y = coordinates[1] as f32 * self.cell_size;
        return vec2(x, y);
    }

    fn generate_noise_map(w: usize, h: usize) -> NoiseMap {
        let seed = generate_seed() as u32;
        let mut basic_multi = BasicMulti::<Perlin>::new(seed);
        basic_multi.frequency = rand::gen_range(0.2, 0.8);
        basic_multi.octaves = rand::gen_range(1, 6);
        basic_multi.lacunarity = rand::gen_range(0.2, 0.8);
        basic_multi.persistence = rand::gen_range(0.2, 0.8);
        PlaneMapBuilder::<_, 2>::new(&basic_multi)
            .set_size(w, h).set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0).build()
    }

}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SerializedTerrain {
    cell_size: f32,
    columns_num: usize,
    rows_num: usize,
    cells: Vec<Vec<Cell>>,
}

impl SerializedTerrain {

    pub fn new(terrain: &Terrain) -> Self {
        let mut serialized_terrain = SerializedTerrain {
            cell_size: terrain.cell_size,
            columns_num: terrain.width,
            rows_num: terrain.height,
            cells: terrain.cells.to_vec(),
        };
        return serialized_terrain;
    }

}