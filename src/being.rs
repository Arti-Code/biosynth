#![allow(unused)]

use std::collections::hash_map::{Iter, IterMut};

use macroquad::prelude::*;
use crate::world::*;

pub trait Being {
    //fn new() -> Self;
    fn draw(&self, selected: bool, font: &Font);
    fn update(&mut self, dt: f32, physics: &mut World) -> bool;
}

trait Collector<T> {
    fn new() -> Self;
    fn add_many(&mut self, number: usize, physics: &mut World);
    fn add(&mut self, being: T, physics: &mut World);
    fn get(&self, id: u64) -> Option<T>;
    fn remove(&mut self, id: u64);
    fn get_iter(&self) -> Iter<u64, T>;
    fn get_iter_mut(&mut self) -> IterMut<u64, T>;
    fn count(&self) -> usize;
}