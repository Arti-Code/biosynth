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


struct Cell {
    alt: f32,
}

impl Cell {

    pub fn new(altitude: f32) -> Self {
        Self {
            alt: clamp(altitude, 0.0, 10.0),
        }
    }

    pub fn set_cell(&mut self, altitude: f32) {
        self.alt = clamp(altitude, 0.0, 10.0);
    }

    pub fn get_altitude(&self) -> f32 {
        return self.alt;
    }

    pub fn get_color(&self) -> Color {
        let alt = self.alt;
        let c = (alt * 25.5) as u8;
        let mut color = color_u8!(c, c, c, 255);
        return color;
    }

}

pub struct Terrain {
    pub cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    cell_size: f32,
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
        Self {cells, width: col_num, height: row_num, cell_size: s}
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
                let color = self.get_color(c, r);
                let x0 = c as f32 * self.cell_size;
                let y0 = r as f32 * self.cell_size;
                draw_rectangle(x0, y0, self.cell_size, self.cell_size, color);
            }
        }
    }

    fn generate_noise_map(w: usize, h: usize) -> NoiseMap {
        let seed = generate_seed() as u32;
        let mut basic_multi = BasicMulti::<Perlin>::new(seed);
        basic_multi.frequency = 0.4;
        basic_multi.octaves = 3;
        PlaneMapBuilder::<_, 2>::new(&basic_multi)
            .set_size(w, h).set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0).build()
    }

}