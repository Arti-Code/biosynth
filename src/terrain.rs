//#![allow(unused)]


use crate::util::*;
use macroquad::prelude::*;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use noise::{
    core::perlin::perlin_2d, 
    permutationtable::PermutationTable, 
    utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder}, 
    Perlin,
};
use ::rand::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cell {
    alt: i32,
    water: i32,
}

impl Cell {

    pub fn new(altitude: f32, water: f32) -> Self {
        Self {
            alt: clamp(altitude, 0.0, 100.0) as i32,
            water: water as i32,
        }
    }

    pub fn set_cell(&mut self, altitude: f32, water: f32) {
        self.set_altitude(altitude);
        self.set_water(water);
    }

    pub fn set_altitude(&mut self, altitude: f32) {
        self.alt = clamp(altitude, 0.0, 100.0) as i32;
    }

    pub fn set_water(&mut self, water: f32) {
        self.water = clamp(water, 0.0, 100.0) as i32;
    }

    pub fn get_altitude(&self) -> i32 {
        return self.alt;
    }

    pub fn get_water(&self) -> i32 {
        return self.water;
    }

    pub fn get_color(&self, water_level: i32) -> Color {
        let dif: i32 = self.alt as i32 - water_level as i32;
        if dif < 0 {
            let mut b = 1.0 + (dif as f32 / 10.0);
            let r = clamp(b-0.75, 0.0, 1.0);
            let g = clamp(b-0.75, 0.0, 1.0);
            b = clamp(b, 0.5, 1.0);
            //let a = 0.5 - (water_level as f32 / 10.0) * 0.5;
            return Color::new(r, g, b, 1.0);
        }
        let alt = self.alt as f32;
        let c = (alt * 2.0 + 55.0) as i32;
        let color = color_u8!(c, c, c, 255);
        return color;
    }

}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Terrain {
    pub cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    cell_size: f32,
    occupied: Vec<[i32; 2]>,
    cursor: Option<[i32; 2]>,
    water_lvl: i32,
}

impl Terrain {

    pub fn new(w: f32, h: f32, s: f32, water_lvl: i32) -> Self {
        let row_num = (h/s) as usize;
        let col_num = (w/s) as usize;
        let map = Self::generate_noise_map(col_num, row_num);
        let mut cells: Vec<Vec<Cell>> = vec![];
        for c in 0..col_num {
            let mut row: Vec<Cell> = vec![];
            for r in 0..row_num {
                let mut v = map.get_value(c, r) as f32;
                v = v + 0.25;
                v = clamp(v, 0.0, 1.0);
                let cell = Cell::new(v*100.0, 0.0);
                row.push(cell);
            }
            cells.push(row);
        }
        Self {cells, width: col_num, height: row_num, cell_size: s, occupied: vec![], water_lvl, cursor: None}
    }

    pub fn from_serialized_terrain(serialized: &SerializedTerrain) -> Self {
        Self { 
            cells: serialized.cells.to_vec(), 
            width: serialized.columns_num,
            height: serialized.rows_num,
            cell_size: serialized.cell_size,
            occupied: vec![],
            water_lvl: serialized.water_lvl,
            cursor: None,
        }
    }

    pub fn update(&mut self) {
        let mut water_buf: Vec<Vec<f32>> = vec![vec![]];
        for c in 0..self.cells.len() {
            let mut col: Vec<f32> = vec![];
            for r in 0..self.cells[c].len() {
                let cell = &mut self.cells[c][r];
                col.push(cell.get_water() as f32);
            }
            water_buf.push(col);
        }
        for c in 0..self.cells.len() {
            for r in 0..self.cells[c].len() {
                let cell = &mut self.cells[c][r];
                let w0 = cell.get_water();
                let a0 = cell.get_altitude();
                for x in 0..=2 {
                    for y in 0..=2 {
                        if x == 1 && y == 1 { continue; }
                        let w1 = self.cells[c+x-1][r+y-1].get_water();
                        let a1 = self.cells[c+x-1][r+y-1].get_altitude();
                        //let sum0 = a0+w0;
                        //let sum1 = a1+w1;
                        let d =  (a0+w0)-(a1+w1)-(a0-a1);
                        let mut water0 = 0;
                        if d > 0 {
                            if thread_rng().gen_bool(d as f64/6.0 as f64) {
                                water0 += 1;
                                water_buf[c+x-1][r+y-1] += 1.0;
                            }
                        }
                        water_buf[c][r] -= water0 as f32;
                    }
                }
            }
        }
        for c in 0..self.cells.len() {
            for r in 0..self.cells[c].len() {
                self.cells[c][r].set_water(water_buf[c][r]);
            }
        }
    }
/* 
w0 = 3
a0 = 7
w1 = 5
a1 = 2

sum0 = 10
sum1 = 7
d = 10 - 7 - 5 = -2

*/
    pub fn get_altitude(&self, x: usize, y: usize) -> i32 {
        return self.cells[x][y].get_altitude();
    }

    pub fn get_color(&self, x: usize, y: usize) -> Color {
        return self.cells[x][y].get_color(self.water_lvl);
    }

    pub fn draw(&self, show_occupied: bool, edit: bool) {
        for c in 0..self.cells.len() {
            for r in 0..self.cells[c].len() {
                let mut color = self.get_color(c, r);
                if color.a == 1.0 {
                    color.a = 0.35;
                }
                let x0 = c as f32 * self.cell_size;
                let y0 = r as f32 * self.cell_size;
                draw_rectangle(x0, y0, self.cell_size, self.cell_size, color);
            }
        }
        if show_occupied {
            for occupied in self.occupied.iter() {
                let x0 = occupied[0] as f32 * self.cell_size;
                let y0 = occupied[1] as f32 * self.cell_size;
                draw_rectangle_lines(x0, y0, self.cell_size, self.cell_size, 2.0, color_u8!(255, 0, 0, 255));
            }
        }
        //println!("cursor");
        if edit {
            match self.cursor {
                None => {},
                Some(coord) => {
                    let x = coord[0] as f32 * self.cell_size;
                    let y = coord[1] as f32 * self.cell_size;
                    //println!("cursor: x{}|y{}", x as i32, y as i32);
                    draw_rectangle_lines(x, y, self.cell_size, self.cell_size, 2.0, color_u8!(0, 0, 255, 255));
                },
            }
        }
    }

    pub fn set_cursor(&mut self, x: i32, y: i32) {
        self.cursor = Some([x, y]);
    }

    pub fn set_cursor_vec2(&mut self, pos: Vec2) {
        let coord = self.pos_to_coord(&pos);
        self.cursor = Some(coord);
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
        let perlin = Perlin::new(seed);
        return PlaneMapBuilder::new(perlin)
            .set_size(w, h)
            .set_x_bounds(-6.0, 6.0)
            .set_y_bounds(-6.0, 6.0)
            .build();
    }

    pub fn water_level(&self) -> i32 {
        return self.water_lvl;
    }

    pub fn set_water_level(&mut self, new_level: i32) {
        self.water_lvl = clamp(new_level, 0, 20);
    }

    pub fn get_water_level(&self, coord: [i32; 2]) -> i32 {
        if self.width as i32 <= coord[0] || self.height as i32 <= coord[1] || coord[0] < 0 || coord[1] < 0 {
            //warn!("get_water_lvl: coord out of bounds: {:?}", coord);
            return 0;
        }
        let alt = self.cells[coord[0] as usize][coord[1] as usize].alt;
        let w = self.water_level()-alt;
        if w >= 0 {
            return w;
        } else {
            return 0;
        }
    }

}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SerializedTerrain {
    cell_size: f32,
    columns_num: usize,
    rows_num: usize,
    cells: Vec<Vec<Cell>>,
    water_lvl: i32,
}

impl SerializedTerrain {

    pub fn new(terrain: &Terrain) -> Self {
        let serialized_terrain = SerializedTerrain {
            cell_size: terrain.cell_size,
            columns_num: terrain.width,
            rows_num: terrain.height,
            cells: terrain.cells.to_vec(),
            water_lvl: terrain.water_lvl,
        };
        return serialized_terrain;
    }

}