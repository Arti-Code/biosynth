#![allow(unused)]

use crate::consts::*;
use crate::util::*;
use macroquad::prelude::*;
use rapier2d::{na::Vector2, prelude::*};
use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;

pub struct World {
    pub attract_num: u32,
    pub rigid_bodies: RigidBodySet,
    pub colliders: ColliderSet,
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    query_pipeline: QueryPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: (),
    collision_recv: (),
    pub detections: HashMap<RigidBodyHandle, (RigidBodyHandle, f32)>,
    pub contacts: Contacts2,
}

impl World {

    pub fn new() -> Self {
        Self {
            attract_num: 0,
            rigid_bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            gravity: Vector2::new(0.0, 0.0),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            query_pipeline: QueryPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: (),
            event_handler: (),
            collision_recv: (),
            detections: HashMap::new(),
            //particle_types: ParticleTable::new_random(),
            contacts: Contacts2::new(),
        }
    }

    fn update_intersections(&mut self) {
        self.query_pipeline.update(&self.rigid_bodies, &self.colliders);
    }

    pub fn add_dynamic_agent(&mut self, key: u64, position: &Vec2, radius: f32, rotation: f32, detection_range: Option<f32>) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), rotation);
        let ball = RigidBodyBuilder::dynamic().position(iso).linear_damping(1.0).angular_damping(1.0)
            //.additional_mass_properties(MassProperties::from_ball(1.0, radius))
            .user_data(key as u128).build();
        let collider = ColliderBuilder::ball(radius).density(1.0).restitution(0.2).friction(0.8)
            .active_collision_types(ActiveCollisionTypes::default()).active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let rb_handle = self.rigid_bodies.insert(ball);
        _ = self.colliders.insert_with_parent(collider, rb_handle, &mut self.rigid_bodies);
        if detection_range.is_some() {
            let detector = ColliderBuilder::ball(detection_range.unwrap())
                .sensor(true).density(0.0).build();
            _ = self.colliders.insert_with_parent(detector, rb_handle, &mut self.rigid_bodies);
        }
        return rb_handle;
    }

    pub fn add_complex_agent(&mut self, key: u64, position: &Vec2, points: Vec<Vec2>, rotation: f32, detection_range: Option<f32>) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), rotation);
        let rb = RigidBodyBuilder::dynamic().position(iso).linear_damping(1.0).angular_damping(1.0)
            .user_data(key as u128).build();
        let complex = ColliderBuilder::polyline(vec2_to_point2_collection(&points), None).density(0.1)
            .active_collision_types(ActiveCollisionTypes::default()).active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let rb_handle = self.rigid_bodies.insert(rb);
        _ = self.colliders.insert_with_parent(complex, rb_handle, &mut self.rigid_bodies);
        if detection_range.is_some() {
            let detector = ColliderBuilder::ball(detection_range.unwrap())
                .sensor(true).density(0.0).build();
            _ = self.colliders.insert_with_parent(detector, rb_handle, &mut self.rigid_bodies);
        }
        return rb_handle;
    }

    pub fn remove_physics_object(&mut self, body_handle: RigidBodyHandle) {
        _ = self.rigid_bodies.remove(body_handle, &mut self.island_manager, &mut self.colliders, &mut self.impulse_joint_set, &mut self.multibody_joint_set, true);
    }

    pub fn get_physics_obj_num(&self) -> usize {
        let body_num = self.rigid_bodies.len();
        return body_num;
    }

    pub fn step_physics(&mut self) {
        self.attract_num = 0;
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_bodies,
            &mut self.colliders,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &self.physics_hooks,
            &self.event_handler,
        );
        self.update_intersections();
        //self.receive_collisions_evts();
        
        //if random_unit() <= 0.05 {
        //    println!("ATTRACTIONS: {}", self.attract_num);
        //}
    }

    fn iso_to_vec2_rot(&self, isometry: &Isometry<Real>) -> (Vec2, f32) {
        let pos = Vec2::new(isometry.translation.x, isometry.translation.y);
        let rot = isometry.rotation.angle() + PI;
        return (pos, rot);
    }

    pub fn get_physics_data(&self, handle: RigidBodyHandle) -> PhysicsData {
        if let Some(rb) = self.rigid_bodies.get(handle) {
            //.expect("handle to non-existent rigid body");
            let iso = rb.position();
            let (pos, rot) = self.iso_to_vec2_rot(iso);
            let force = Vec2::new(rb.user_force().data.0[0][0], rb.user_force().data.0[0][1]);
            let data = PhysicsData {
                position: pos,
                rotation: rot,
                mass: rb.mass(),
                kin_eng: Some(rb.kinetic_energy()),
                force: Some(force),
            };
            return data;
        } else {
            return PhysicsData {
                position: Vec2::new(WORLD_W / 2., WORLD_H / 2.),
                rotation: 0.0,
                mass: 0.0,
                kin_eng: Some(0.0),
                force: None,
            };
        }
    }

    pub fn get_object_position(&self, handle: RigidBodyHandle) -> Option<Vec2> {
        let rb = self.rigid_bodies.get(handle);
        match rb {
            Some(body) => {
                let pos = Vec2::new(body.position().translation.x, body.position().translation.y);
                return Some(pos);
            }
            None => {
                return None;
            }
        }
    }

    /*  pub fn get_contacts(&self, agent_body_handle: RigidBodyHandle) -> Option<RigidBodyHandle> {
        let target = self.detections.get(&agent_body_handle);
        match target {
            Some((tg, dst)) => {
                return Some(*tg);
            },
            None => {
                return None;
            }
        }
    } */

    fn get_body_handle_from_collider(&self, collider_handle: ColliderHandle) -> Option<RigidBodyHandle> {
        let collider: &Collider;
        match self.colliders.get(collider_handle) {
            Some(col) => {
                collider = col;
            }
            None => {
                return None;
            }
        };
        match collider.parent() {
            Some(rbh) => {
                return Some(rbh);
            }
            None => {
                return None;
            }
        }
    }

    pub fn get_contacts_set(&mut self, agent_body_handle: RigidBodyHandle, radius: f32) -> HashSet<RigidBodyHandle> {
        let mut contacts: HashSet<RigidBodyHandle> = HashSet::new();
        let rb = self.rigid_bodies.get(agent_body_handle).unwrap();
        let pos1 = matric_to_vec2(rb.position().translation);
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_DYNAMIC | QueryFilterFlags::EXCLUDE_SENSORS,
            groups: None,
            exclude_rigid_body: Some(agent_body_handle),
            ..Default::default()
        };
        for c in rb.colliders() {
            let collider = self.colliders.get(*c).unwrap();
            if !collider.is_sensor() {
                continue;
            }
            self.query_pipeline.intersections_with_shape(&self.rigid_bodies, &self.colliders, rb.position(), &rapier2d::geometry::Ball::new(radius), filter,
                |collided| {
                    let rb2_handle = self.get_body_handle_from_collider(collided).unwrap();
                    contacts.insert(rb2_handle);
                    let rb2 = self.rigid_bodies.get(rb2_handle).unwrap();
                    let mass2 = rb2.mass();
                    let pos2 = matric_to_vec2(rb2.position().translation);
                    let dist = pos2.distance(pos1);
                    let dir = pos2.normalize() - pos1.normalize();
                    return true;
                },
            );
        }
        return contacts;
    }

    pub fn get_closesd_agent(&self, agent_body_handle: RigidBodyHandle) -> Option<RigidBodyHandle> {
        let rb = self.rigid_bodies.get(agent_body_handle).unwrap();
        let pos1 = matric_to_vec2(rb.position().translation);
        let mut dist = f32::INFINITY;
        let mut target: RigidBodyHandle = RigidBodyHandle::invalid();
        for c in rb.colliders() {
            let collider = self.colliders.get(*c).unwrap();
            if !collider.is_sensor() {
                continue;
            }
            let filter = QueryFilter {
                flags: QueryFilterFlags::ONLY_DYNAMIC | QueryFilterFlags::EXCLUDE_SENSORS,
                groups: None,
                exclude_collider: Some(*c),
                exclude_rigid_body: Some(agent_body_handle),
                ..Default::default()
            };
            self.query_pipeline.intersections_with_shape(&self.rigid_bodies, &self.colliders, rb.position(), collider.shape(), filter,
             |collided| {
                    let rb2_handle = self.get_body_handle_from_collider(collided).unwrap();
                    let rb2 = self.rigid_bodies.get(rb2_handle).unwrap();
                    let pos2 = matric_to_vec2(rb2.position().translation);

                    let new_dist = pos1.distance(pos2);
                    if new_dist < dist {
                        dist = new_dist;
                        target = rb2_handle;
                    }
                    return true;
                },
            );
        }
        if dist < f32::INFINITY {
            return Some(target);
        } else {
            return None;
        }
    }

    pub fn get_contacts_info(&self) -> (i32, i32) {
        return self.contacts.count();
    }
}

pub struct PhysicsData {
    pub position: Vec2,
    pub rotation: f32,
    pub mass: f32,
    pub kin_eng: Option<f32>,
    pub force: Option<Vec2>,
}


pub struct Contacts {
    contacts: HashMap<u128, Vec<u128>>,
}

impl Contacts {
    pub fn new() -> Self {
        Self {
            contacts: HashMap::new(),
        }
    }

    pub fn clean(&mut self) {
        self.contacts.clear();
    }

    pub fn get(&self, key: u128) -> Option<&Vec<u128>> {
        let list = self.contacts.get(&key);
        return list;
    }

    pub fn insert(&mut self, key: u128, contact_id: u128) {
        if let Some(c) = self.contacts.get_mut(&key) {
            c.push(contact_id);
        } else {
            self.contacts.insert(key, vec![contact_id]);
        }
    }

    pub fn del(&mut self, key: u128, contact_id: u128) {
        if let Some(c) = self.contacts.get_mut(&key) {
            if c.contains(&contact_id) {
                c.retain(|&v| v != contact_id);
            }
        }
    }

    pub fn count(&self) -> (i32, i32) {
        let mut key_num = 0;
        let mut contact_num = 0;
        for (key, list) in self.contacts.iter() {
            key_num += 1;
            contact_num += list.len() as i32;
        }
        return (key_num, contact_num);
    }
}

pub struct Contacts2 {
    contacts: HashMap<ColliderHandle, HashSet<ColliderHandle>>,
}

impl Contacts2 {
    pub fn new() -> Self {
        Self {
            contacts: HashMap::new(),
        }
    }

    pub fn clean(&mut self) {
        self.contacts.clear();
    }

    pub fn get(&self, key: ColliderHandle) -> Option<&HashSet<ColliderHandle>> {
        let list = self.contacts.get(&key);
        return list;
    }

    pub fn insert(&mut self, key: ColliderHandle, contact_id: ColliderHandle) {
        if let Some(c) = self.contacts.get_mut(&key) {
            c.insert(contact_id);
        } else {
            let mut set = HashSet::new();
            set.insert(contact_id);
            self.contacts.insert(key, set);
        }
    }

    pub fn del(&mut self, key: ColliderHandle, contact_id: &ColliderHandle) {
        if let Some(c) = self.contacts.get_mut(&key) {
            if c.contains(&contact_id) {
                c.remove(contact_id);
            }
        }
        self.contacts.remove(&key);
    }

    pub fn count(&self) -> (i32, i32) {
        let mut key_num = 0;
        let mut contact_num = 0;
        for (key, list) in self.contacts.iter() {
            key_num += 1;
            contact_num += list.len() as i32;
        }
        return (key_num, contact_num);
    }
}