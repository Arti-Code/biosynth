//#![allow(unused)]

use std::i32::*;
use crate::{get_settings, util::*};
use macroquad::prelude::*;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use noise::{
    core::perlin::perlin_2d, permutationtable::PermutationTable, utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder}, Fbm, NoiseFn, OpenSimplex, Perlin
};
use ::rand::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cell {
    alt: i32,
    water: i32,
}

impl Cell {

    pub fn new(altitude: i32, water: i32) -> Self {
        Self {
            alt: clamp(altitude, 0, 100),
            water: water,
        }
    }

    pub fn set_cell(&mut self, altitude: i32, water: i32) {
        self.set_altitude(altitude);
        self.set_water(water);
    }

    pub fn set_altitude(&mut self, altitude: i32) {
        self.alt = clamp(altitude, 0, 100);
    }

    pub fn set_water(&mut self, water: i32) {
        self.water = clamp(water, 0, 100);
    }

    pub fn get_altitude(&self) -> i32 {
        return self.alt;
    }

    pub fn get_water(&self) -> i32 {
        return self.water;
    }

/*     pub fn get_color(&self, _water_level: i32) -> Color {
        if self.water >= 10 {
            let mut b = (self.water as f32 / 100.0)*3.0;
            let r = clamp(b-0.75, 0.0, 1.0);
            let g = clamp(b-0.75, 0.0, 1.0);
            b = clamp(b*3.0, 0.5, 1.0);
            return Color::new(r, g, b, 1.0);
        } else {
            let alt = self.alt as f32;
            let c = (alt * 2.0 + 55.0) as i32;
            let color = color_u8!(c, c, c, 255);
            return color;
        }
    } */

    pub fn get_colors(&self) -> (Color, Option<Color>) {
        let alt = self.alt as f32 / 100.0;
        let c0 = alt*0.8 + 0.2;;
        let terrain = Color::new(c0, c0, c0, 0.8);
        if self.water == 0 {
            return (terrain, None);
        } else {
            let mut a = self.water as f32 / 100.0;
            //let b = 1.0;
            a = clamp(a+0.4, 0.0, 1.0);
            //let r = clamp(b-0.75, 0.0, 1.0);
            //let g = clamp(b-0.75, 0.0, 1.0);
            let water = Some(Color::new(0.0, 0.0, 1.0, a));
            return (terrain, water);
        }
        
    }

}


#[derive(Clone, Debug)]
pub struct Terrain {
    pub cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    cell_size: f32,
    occupied: Vec<[i32; 2]>,
    brushed: Vec<(IVec2, f32)>,
    cursor: Option<[i32; 2]>,
    brush_size: u32,
}

impl Terrain {

    pub fn new(w: f32, h: f32, s: f32) -> Self {
        let row_num = (h/s) as usize;
        let col_num = (w/s) as usize;
        let map = Self::generate_noise_map(col_num, row_num);
        let mut cells: Vec<Vec<Cell>> = Vec::new();
        for c in 0..col_num {
            let mut row: Vec<Cell> = Vec::new();
            for r in 0..row_num {
                let mut v = map.get_value(c, r) as f32;
                v = v+0.15;
                v = clamp(v*100.0, 0.0, 100.0);
                let cell = Cell::new(v as i32, 0);
                row.push(cell);
            }
            cells.push(row);
        }
        Self {
            cells, 
            width: col_num, 
            height: row_num, 
            cell_size: s, 
            occupied: Vec::new(), 
            brushed: Vec::new(), 
            cursor: None,
            brush_size: 1,
        }
    }

    pub fn from_serialized_terrain(serialized: &SerializedTerrain) -> Self {
        Self { 
            cells: serialized.cells.to_vec(), 
            width: serialized.columns_num,
            height: serialized.rows_num,
            cell_size: serialized.cell_size,
            occupied: vec![],
            brushed: Vec::new(), 
            cursor: None,
            brush_size: 1,
        }
    }

    pub fn update(&mut self) {
        let mut water_buf: Vec<Vec<i32>> = Vec::new();
        for c in 0..self.cells.len() {
            let mut col: Vec<i32> = Vec::new();
            for r in 0..self.cells[c].len() {
                match self.get_cell(c, r) {
                    Some(cell) => {
                        col.push(cell.get_water());
                    },
                    None => {
                        let msg = format!("cell not exist: (x: {} | y {})", c, r);
                        warn!("{}", msg);
                        col.push(0);
                    },
                }
            }
            water_buf.push(col);
        }
        for c in 0..self.cells.len()-1 {
            for r in 0..self.cells[c].len()-1 {
                if let Some(cell) = self.get_cell(c, r) {
                    let w0 = cell.get_water();
                    let a0 = cell.get_altitude();
                    let mut water0 = 0;
                    for x in 0_i32..=2_i32 {
                        for y in 0_i32..=2_i32 {
                            if x.abs() == y.abs() { continue; }
                            let c2 = c as i32 + x as i32 - 1;
                            let r2 = r as i32 + y as i32 - 1;
                            if c2 < 0 || r2 < 0 { continue; }
                            if water0 >= w0 { continue; }
                            if let Some(cell2) = self.get_cell(c2 as usize, r2 as usize) {
                                let w1 = cell2.get_water();
                                let a1 = cell2.get_altitude();
                                let d =  (a0+w0)-(a1+w1);

                                if d > 0 && w0 > 0 {
                                    let over = clamp(d, 0, w0);
                                    let p = clamp(over as f64/5.0 as f64, 0.0, 1.0);
                                    if thread_rng().gen_bool(p) {
                                        water0 += 1;
                                        water_buf[c2 as usize][r2 as usize] += 1;
                                    }
                                }

                            } else {
                                let msg = format!("cell not exist: (x: {} | y {})", c2, r2);
                                warn!("{}", msg);
                            }
                        }
                    }
                    water_buf[c][r] -= water0;
                }
            }
        }
        for c in 0..self.cells.len()-1 {
            for r in 0..self.cells[c].len()-1 {
                self.cells[c][r].set_water(water_buf[c][r]);
            }
        }
    }

    pub fn get_altitude(&self, x: usize, y: usize) -> i32 {
        return self.cells[x][y].get_altitude();
    }

    pub fn get_color(&self, x: usize, y: usize) -> (Color, Option<Color>) {
        return self.cells[x][y].get_colors();
    }

    pub fn draw(&self, show_occupied: bool, edit: bool) {
        for c in 0..self.cells.len() {
            for r in 0..self.cells[c].len() {
                let (terrain, water) = self.get_color(c, r);
                let x0 = c as f32 * self.cell_size;
                let y0 = r as f32 * self.cell_size;
                draw_rectangle(x0, y0, self.cell_size, self.cell_size, terrain);
                match water {
                    None => {},
                    Some(water) => {
                        draw_rectangle(x0, y0, self.cell_size, self.cell_size, water);
                    },
                }
            }
        }
        if show_occupied {
            for occupied in self.occupied.iter() {
                let x0 = occupied[0] as f32 * self.cell_size;
                let y0 = occupied[1] as f32 * self.cell_size;
                draw_rectangle_lines(x0, y0, self.cell_size, self.cell_size, 2.0, color_u8!(255, 0, 0, 255));
            }
        }
        if edit {
            match self.cursor {
                None => {},
                Some(coord) => {
                    for (cell_loc, i) in self.brushed.iter() {
                        let x = cell_loc[0] as f32 * self.cell_size;
                        let y = cell_loc[1] as f32 * self.cell_size;
                        let a = clamp((100.0+155.0*i) as u8, 0, 255) as u8;
                        draw_rectangle_lines(x, y, self.cell_size, self.cell_size, 2.0, color_u8!(0, 0, 255, a));
                    }
                },
            }
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        return match self.cells.get(x) {
            None => None,
            Some(col) => {
                match col.get(y) {
                    None => None,
                    Some(cell) => Some(cell),
                }
            },
        }
    }

    pub fn get_mut_cell(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        return match self.cells.get_mut(x) {
            None => None,
            Some(col) => {
                match col.get_mut(y) {
                    None => None,
                    Some(cell) => Some(cell),
                }
            },
        }
    }

    pub fn set_cursor(&mut self, x: i32, y: i32) {
        self.cursor = Some([x, y]);
        self.collect_cells_under_brush();
    }

    pub fn set_cursor_vec2(&mut self, pos: Vec2) {
        let coord = self.pos_to_coord(&pos);
        self.set_cursor(coord[0], coord[1]);
    }

    pub fn set_occupied(&mut self, coord_list: Vec<[i32; 2]>) {
        self.occupied = coord_list;
    }

    pub fn get_brush_size(&self) -> u32 {
        return self.brush_size;
    }

    pub fn set_brush_size(&mut self, size: u32) {
        self.brush_size = size;
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
        let mut fbm = Fbm::<Perlin>::new(seed);
        fbm.frequency = 0.6;
        fbm.octaves = 4;
        fbm.lacunarity = 0.8;
        fbm.persistence = 0.8;
        
        return PlaneMapBuilder::new(&fbm)
            .set_size(w, h)
            .set_x_bounds(-6.0, 6.0)
            .set_y_bounds(-6.0, 6.0)
            .build();
    }

/*     pub fn water_level(&self) -> i32 {
        return self.water_lvl;
    }

    pub fn set_water_level(&mut self, new_level: i32) {
        self.water_lvl = clamp(new_level, 0, 100);
    }

    pub fn get_water_level(&self, coord: [i32; 2]) -> i32 {
        if self.width as i32 <= coord[0] || self.height as i32 <= coord[1] || coord[0] < 0 || coord[1] < 0 {
            return 0;
        }
        let alt = self.cells[coord[0] as usize][coord[1] as usize].alt;
        let w = self.water_level()-alt;
        if w >= 0 {
            return w;
        } else {
            return 0;
        }
    } */

    pub fn add_water_at_cursor(&mut self, amount: i32) {
        match self.cursor {
            None => {},
            Some(_) => {
                let mut buf: Vec<(IVec2, f32)> = Vec::new();
                for v in self.brushed.iter() {
                    let (iv, f) = *v;
                    buf.push((iv, f));
                }
                for (cell_loc, intens) in buf.iter() {
                    match self.get_mut_cell(cell_loc.x as usize, cell_loc.y as usize) {
                        None => {},
                        Some(cell) => {
                            let w = cell.get_water();
                            //let a = cell.get_altitude();
                            //println!("terrain: {:?} | water {:?}", a, w);
                            let t = w + (amount as f32*intens) as i32;
                            cell.set_water(t);
                        },
                    }
                }
            },
        }
    }

    pub fn add_terrain_at_cursor(&mut self, amount: i32) {
        match self.cursor {
            None => {},
            Some(_) => {
                let mut buf: Vec<(IVec2, f32)> = Vec::new();
                for v in self.brushed.iter() {
                    let (iv, f) = *v;
                    buf.push((iv, f));
                }
                for (cell_loc, intens) in buf.iter() {
                    match self.get_mut_cell(cell_loc.x as usize, cell_loc.y as usize) {
                        None => {},
                        Some(cell) => {
                            //let w = cell.get_water();
                            let a = cell.get_altitude();
                            //println!("terrain: {:?} | water {:?}", a, w);
                            let t = a + (amount as f32*intens) as i32;
                            cell.set_altitude(t);
                        },
                    }
                }
            },
        }
    }

    fn collect_cells_under_brush(&mut self) {
        self.brushed.clear();
        self.brush_size = get_settings().brush_size as u32;
        let s = self.brush_size;
        match self.cursor {
            None => {},
            Some(coord) => {
                let pos1 = ivec2(coord[0], coord[1]);
                let half = (s/2) as i32;
                for c in 0..s {
                    for r in 0..s {
                        let x = c as i32 - half;
                        let y = r as i32 - half;
                        let pos2 = ivec2(x, y);
                        let pos = pos1 + pos2;
                        let fpos = vec2(pos.x as f32, pos.y as f32);
                        let fpos1 = vec2(pos1.x as f32, pos1.y as f32);
                        let dist = fpos1.distance(fpos);
                        if dist <= self.brush_size as f32 {
                            match self.get_cell(pos.x as usize, pos.y as usize) {
                                Some(_) => {
                                    let d = 1.0 - (dist/self.brush_size as f32);
                                    self.brushed.push((pos, d));
                                },
                                None => {},
                            }
                        }
                    }
                }
            },
        }
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
        let serialized_terrain = SerializedTerrain {
            cell_size: terrain.cell_size,
            columns_num: terrain.width,
            rows_num: terrain.height,
            cells: terrain.cells.to_vec(),
        };
        return serialized_terrain;
    }

}