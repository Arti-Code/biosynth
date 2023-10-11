#![allow(unused)]

use egui_macroquad::egui::InputState;
use macroquad::prelude::*;
use macroquad::rand::*;
use serde::ser::SerializeStruct;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use serde_json::{self, *};
use std::fs;
use crate::util::*;
use crate::neuro::*;

pub struct Neuron {
    pub id: u64,
    pub pos: Vec2,
    pub val: f32,
}

impl Neuron {
    pub fn new() -> Neuron {
        Self { 
            id: generate_id(), 
            pos: rand_position_rel(),
            val: 0.0,
        }
    }
}