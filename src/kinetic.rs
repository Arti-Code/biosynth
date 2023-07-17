use std::collections::HashMap;

//use glam;
use macroquad::math::Vec2;
use nalgebra::*;
use rapier2d::geometry::*;
use rapier2d::parry::query::contact;
//use rapier2d::parry::query::TrackedContact;
use rapier2d::parry::query::*;
use rapier2d::prelude::*;
//use parry2d::query::contact;
//use parry2d::shape::*;
//use parry2d::{query::Contact};
//use rapier2d::prelude::*;

//use crate::agent::Agent;

pub fn make_isometry(posx: f32, posy: f32, rotation: f32) -> nalgebra::Isometry2<f32> {
    let iso = Isometry2::new(Vector2::new(posx, posy), rotation);
    return iso;
}

/* pub fn contact_circles(
    pos1: Vec2,
    rot1: f32,
    rad1: f32,
    pos2: Vec2,
    rot2: f32,
    rad2: f32,
) -> Option<Contact> {
    let v1 = glam::Vec2::new(pos1.x, pos1.y);
    let v2 = glam::Vec2::new(pos2.x, pos2.y);
    let pos1 = make_isometry(v1.x, v1.y, rot1);
    let pos2 = make_isometry(v2.x, v2.y, rot2);
    let ball1 = Ball::new(rad1);
    let ball2 = Ball::new(rad2);
    let contact = contact(&pos1, &ball1, &pos2, &ball2, 0.0).unwrap();
    return contact;
} */

pub fn contact_mouse(mouse_pos: Vec2, target_pos: Vec2, target_rad: f32) -> bool {
    let v1 = glam::Vec2::new(mouse_pos.x, mouse_pos.y);
    let v2 = glam::Vec2::new(target_pos.x, target_pos.y);
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

/* pub struct Detection {
    pub distance: f32,
    pub angle: f32,
    pub pos: Vec2,
    pub target_type: ObjectType,
}

impl Detection {
    pub fn new(distance: f32, angle: f32, pos: Vec2) -> Self {
        Self {
            distance,
            angle,
            pos,
            target_type: ObjectType::Agent,
        }
    }
    pub fn new_empty() -> Self {
        Self {
            distance: f32::NAN,
            angle: f32::NAN,
            pos: Vec2::NAN,
            target_type: ObjectType::Empty,
        }
    }
    pub fn add_closer(&mut self, distance: f32, angle: f32, pos: Vec2) {
        if self.is_empty() || self.distance > distance {
            self.distance = distance;
            self.angle = angle;
            self.pos = pos;
            self.target_type = ObjectType::Agent;
        }
    }
    pub fn is_empty(&self) -> bool {
        if self.angle.is_nan() || self.distance.is_nan() {
            return true;
        }
        return false;
    }
} */

/* pub struct Detections {
    pub detections: HashMap<ObjectType, Detection>
}

impl Detections {
    pub fn new_empty() -> Self {
        Self {
            detections: HashMap::new(),
        }
    }
    pub fn add_closer(&mut self, distance: f32, angle: f32, pos: Vec2) {
        if self.is_empty() || self.distance > distance {
            self.distance = distance;
            self.angle = angle;
            self.pos = pos;
            self.target_type = ObjectType::Agent;
        }
    }
    pub fn is_empty(&self) -> bool {
        if self.angle.is_nan() || self.distance.is_nan() {
            return true;
        }
        return false;
    }
} */

/* pub struct DetectionsMap {
    pub detections: HashMap<u64, Detection>,
    pub sources: HashMap<u64, Detection>,
}

impl DetectionsMap {
    pub fn new() -> Self {
        Self {
            detections: HashMap::new(),
            sources: HashMap::new(),
        }
    }
    pub fn add_detection(&mut self, id: u64, detection: Detection) {
        let old_detection = self.detections.get(&id);
        let actual = match old_detection {
            Some(actual_detection) if actual_detection.distance > detection.distance => detection,
            Some(actual_detection) if actual_detection.distance <= detection.distance => {
                Detection::new(
                    actual_detection.distance,
                    actual_detection.angle,
                    actual_detection.pos,
                )
            }
            Some(_) => Detection::new(f32::NAN, f32::NAN, Vec2::NAN),
            None => detection,
        };
        self.detections.insert(id, actual);
    }
    pub fn clear(&mut self) {
        self.detections.clear();
    }
    pub fn remove_detection(&mut self, id: u64) {
        _ = self.detections.remove(&id);
    }
    pub fn get_detection(&mut self, id: u64) -> Option<&Detection> {
        return self.detections.get(&id);
    }
}

//      **********************************************
//      **                 CONTACTS                 **
//      **********************************************

pub struct Hit {
    pub normal: macroquad::math::Vec2,
    pub overlap: f32,
    pub target_type: ObjectType,
    pub target_id: u64,
}

pub struct CollisionsMap {
    contacts: HashMap<u64, Hit>,
}

impl CollisionsMap {
    pub fn new() -> Self {
        Self {
            contacts: HashMap::new(),
        }
    }
    pub fn add_collision(&mut self, id: u64, hit: Hit) {
        self.contacts.insert(id, hit);
    }
    pub fn clear(&mut self) {
        self.contacts.clear();
    }
    pub fn remove_collision(&mut self, id: u64) {
        _ = self.contacts.remove(&id);
    }
    pub fn get_collision(&mut self, id: u64) -> Option<&Hit> {
        return self.contacts.get(&id);
    }
}

pub struct Collisions {
    collisions: Vec<Hit>,
}

pub struct CollisionsList {
    collisions: HashMap<u32, Collisions>,
}

//      **********************************************
//      **               OBJECT TYPE                **
//      **********************************************

#[derive(PartialEq, Eq)]
pub enum ObjectType {
    Empty,
    Agent,
    Source,
    Obstacle,
}
 */
