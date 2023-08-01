#![allow(unused)]

use macroquad::math::Vec2;
use nalgebra::*;
use rapier2d::geometry::*;
use rapier2d::parry::query::contact;
use rapier2d::prelude::*;

pub fn make_isometry(posx: f32, posy: f32, rotation: f32) -> nalgebra::Isometry2<f32> {
    let iso = Isometry2::new(Vector2::new(posx, posy), rotation);
    return iso;
}

pub fn contact_mouse(mouse_pos: Vec2, target_pos: Vec2, target_rad: f32) -> bool {
    let v1 = Vec2::new(mouse_pos.x, mouse_pos.y);
    let v2 = Vec2::new(target_pos.x, target_pos.y);
    let pos1 = make_isometry(v1.x, v1.y, 0.0);
    let pos2 = make_isometry(v2.x, v2.y, 0.0);
    let ball1 = Ball::new(2.0);
    let ball2 = Ball::new(target_rad);
    match contact(&pos1, &ball1, &pos2, &ball2, 0.0).unwrap() {
        Some(_) => true,
        None => false,
    }
}

//      **********************************************
//      **                   ROT                    **
//      **********************************************

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rot {
    cos: f32,
    sin: f32,
}

impl Default for Rot {
    fn default() -> Self {
        Self::ZERO
    }
}

impl Rot {
    pub const ZERO: Self = Self { cos: 1., sin: 0. };

    pub fn from_radians(radians: f32) -> Self {
        Self {
            cos: radians.cos(),
            sin: radians.sin(),
        }
    }

    pub fn from_degrees(degrees: f32) -> Self {
        let radians = degrees.to_radians();
        Self::from_radians(radians)
    }

    pub fn as_radians(&self) -> f32 {
        f32::atan2(self.sin, self.cos)
    }

    pub fn rotate(&self, vec: Vec2) -> Vec2 {
        Vec2::new(
            vec.x * self.cos - vec.y * self.sin,
            vec.x * self.sin + vec.y * self.cos,
        )
    }

    pub fn inv(self) -> Self {
        Self {
            cos: self.cos,
            sin: -self.sin,
        }
    }

    pub fn mul(self, rhs: Rot) -> Self {
        Self {
            cos: self.cos * rhs.cos - self.sin * rhs.sin,
            sin: self.sin * rhs.cos + self.cos * rhs.sin,
        }
    }
}

//      **********************************************
//      **               DETECTIONS                 **
//      **********************************************

pub enum DetectionTypes {
    Enemy,
    Food,
    Rock,
}
