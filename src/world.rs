//use crossbeam::channel::{Receiver, Sender};
//use crossbeam::*;
use crate::consts::{ASTER_SPEED, GRAV, PARTICLE_SPEED, WORLD_H, WORLD_W, FIELD_RADIUS};
use crate::util::*;
use macroquad::prelude::*;
use nalgebra::Point2;
use rapier2d::{na::Vector2, prelude::*};
use rapier2d::geometry::Ball;
use std::collections::HashMap;
use std::f32::consts::PI;
use crate::particle::ParticleTable;

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
    //event_handler: ChannelEventCollector,
    //collision_recv: Receiver<CollisionEvent>,
    pub detections: HashMap<RigidBodyHandle, (RigidBodyHandle, f32)>,
    particle_types: ParticleTable,
}

impl World {
    pub fn new() -> Self {
        //let (collision_send, collision_recv) = crossbeam::channel::unbounded();
        //let (contact_force_send, contact_force_recv) = crossbeam::channel::unbounded();
        //let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);
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
            //event_handler: event_handler,
            //collision_recv: collision_recv,
            detections: HashMap::new(),
            particle_types: ParticleTable::new_random(),
        }
    }

    fn update_intersections(&mut self) {
        self.query_pipeline.update(&self.rigid_bodies, &self.colliders);
    }

    pub fn add_circle_body(
        &mut self,
        key: u64,
        position: &Vec2,
        radius: f32,
        detection_range: Option<f32>,
    ) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), 0.0);
        let ball = RigidBodyBuilder::dynamic()
            .position(iso)
            .linear_damping(0.01)
            .angular_damping(0.01)
            .user_data(key as u128)
            .build();
        let mut collider = ColliderBuilder::ball(radius)
            .density(0.001)
            .restitution(0.5)
            .friction(0.6)
            .active_collision_types(ActiveCollisionTypes::default())
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let rb_handle = self.rigid_bodies.insert(ball);
        let coll_handle =
            self.colliders
                .insert_with_parent(collider, rb_handle, &mut self.rigid_bodies);
        if detection_range.is_some() {
            let detector = ColliderBuilder::ball(detection_range.unwrap())
                .sensor(true)
                .build();
            _ = self
                .colliders
                .insert_with_parent(detector, rb_handle, &mut self.rigid_bodies);
        }
        return rb_handle;
    }

    pub fn add_particle(&mut self, p_type: u8, position: &Vec2, radius: f32) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), 0.0);
        let particle = RigidBodyBuilder::dynamic().position(iso).linear_damping(0.0).angular_damping(0.0)
            .user_data(p_type as u128).build();
        let collider = ColliderBuilder::ball(radius * 1.1).density(1.0).restitution(0.5).friction(0.5)
            //.active_collision_types(ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_DYNAMIC)
            .active_collision_types(ActiveCollisionTypes::empty())
            .active_events(ActiveEvents::COLLISION_EVENTS | ActiveEvents::CONTACT_FORCE_EVENTS).build();
        let rb_handle = self.rigid_bodies.insert(particle);
        let coll_handle = self.colliders.insert_with_parent(collider, rb_handle, &mut self.rigid_bodies);
        //let imp = Vector2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)) * PARTICLE_SPEED;
        //let obj = self.rigid_bodies.get_mut(rb_handle).unwrap();
        //obj.set_linvel(imp, true);
        let detector = ColliderBuilder::ball(FIELD_RADIUS).sensor(true).density(0.0).build();
        _ = self.colliders.insert_with_parent(detector, rb_handle, &mut self.rigid_bodies);
        return rb_handle;
    }

    pub fn add_static_circle(&mut self, key: u64, position: &Vec2, size: f32) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), 0.0);
        let brick = RigidBodyBuilder::fixed()
            .position(iso)
            .user_data(key as u128)
            .build();
        let collider = ColliderBuilder::ball(size)
            .restitution(0.5)
            .friction(0.6)
            .active_collision_types(
                ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_FIXED,
            )
            .build();
        let rb_handle = self.rigid_bodies.insert(brick);
        let coll_handle =
            self.colliders
                .insert_with_parent(collider, rb_handle, &mut self.rigid_bodies);
        return rb_handle;
    }

    pub fn add_static_box(&mut self, key: u64, position: &Vec2, size: f32) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), 0.0);
        let brick = RigidBodyBuilder::fixed()
            .position(iso)
            .user_data(key as u128)
            .build();
        let collider = ColliderBuilder::cuboid(size, 10.0)
            .restitution(0.5)
            .friction(0.6)
            .active_collision_types(ActiveCollisionTypes::default())
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let rb_handle = self.rigid_bodies.insert(brick);
        let coll_handle =
            self.colliders
                .insert_with_parent(collider, rb_handle, &mut self.rigid_bodies);
        println!("static box");
        return rb_handle;
    }

    pub fn add_dynamic_agent(&mut self, key: u64, position: &Vec2, radius: f32, rotation: f32, detection_range: Option<f32>) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), rotation);
        let ball = RigidBodyBuilder::dynamic()
            .position(iso)
            .linear_damping(0.5)
            .angular_damping(0.7)
            .additional_mass_properties(MassProperties::from_ball(1.0, radius))
            .user_data(key as u128)
            .build();
        let mut collider = ColliderBuilder::ball(radius)
            .density(0.0)
            .active_collision_types(ActiveCollisionTypes::default())
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build();
        let rb_handle = self.rigid_bodies.insert(ball);
        let coll_handle =
            self.colliders
                .insert_with_parent(collider, rb_handle, &mut self.rigid_bodies);
        if detection_range.is_some() {
            let detector = ColliderBuilder::ball(detection_range.unwrap())
                .sensor(true)
                .density(0.0)
                .build();
            _ = self
                .colliders
                .insert_with_parent(detector, rb_handle, &mut self.rigid_bodies);
        }
        return rb_handle;
    }

    pub fn add_poly_body(&mut self, key: u64, position: &Vec2, points: Vec<Point2<f32>> ) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), 0.0);
        let poly = RigidBodyBuilder::dynamic()
            .position(iso)
            .linear_damping(0.0)
            .angular_damping(0.0)
            .can_sleep(true)
            .user_data(key as u128)
            .build();
        let collider = ColliderBuilder::convex_polyline(points)
            .unwrap()
            .restitution(0.5)
            .friction(0.6)
            .active_collision_types(
                ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_DYNAMIC,
            )
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .density(1.0)
            .build();
        let rb_handle = self.rigid_bodies.insert(poly);
        let coll_handle =
            self.colliders
                .insert_with_parent(collider, rb_handle, &mut self.rigid_bodies);
        let obj = self.rigid_bodies.get_mut(rb_handle).unwrap();
        let imp =
            Vector2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)) * ASTER_SPEED;
        //obj.apply_impulse(imp, true);
        obj.set_linvel(imp, true);
        return rb_handle;
    }

    pub fn add_jet_hull(
        &mut self,
        key: u64,
        position: &Vec2,
        points: Vec<Point2<f32>>,
    ) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), 0.0);
        let poly = RigidBodyBuilder::dynamic()
            .position(iso)
            .linear_damping(0.05)
            .angular_damping(0.5)
            .can_sleep(false)
            .user_data(key as u128)
            .build();
        let collider = ColliderBuilder::convex_polyline(points)
            .expect("cant build hull collider")
            .active_collision_types(
                ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_DYNAMIC,
            )
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .density(1.0)
            .build();
        let rb_handle = self.rigid_bodies.insert(poly);
        let coll_handle =
            self.colliders
                .insert_with_parent(collider, rb_handle, &mut self.rigid_bodies);
        let obj = self
            .rigid_bodies
            .get_mut(rb_handle)
            .expect("can't get mut hull collider");
        //obj.add
        //let imp = Vector2::new(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)) * 0.0;
        //obj.apply_impulse(imp, true);
        //obj.set_linvel(imp, true);
        return rb_handle;
    }

    pub fn remove_physics_object(&mut self, body_handle: RigidBodyHandle) {
        _ = self.rigid_bodies.remove(
            body_handle,
            &mut self.island_manager,
            &mut self.colliders,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            true,
        );
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

    fn get_body_handle_from_collider(
        &self,
        collider_handle: ColliderHandle,
    ) -> Option<RigidBodyHandle> {
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

    pub fn get_around(&mut self, agent_body_handle: RigidBodyHandle) {
        //let rbm = &mut self.rigid_bodies;
        let mut action = Vec2::ZERO;
        let rb = self.rigid_bodies.get(agent_body_handle).unwrap();
        let mass = rb.mass();
        let pos1 = matric_to_vec2(rb.position().translation);
        let dist = f32::INFINITY;
        //let collider = ColliderBuilder::ball(32.0).build();
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
            self.query_pipeline.intersections_with_shape(
                &self.rigid_bodies,
                &self.colliders,
                rb.position(),
                &rapier2d::geometry::Ball::new(FIELD_RADIUS),
                //&Ball::new(FIELD_RADIUS),
                //collider.shape(),
                filter,
                |collided| {
                    let rb2_handle = self.get_body_handle_from_collider(collided).unwrap();
                    let rb2 = self.rigid_bodies.get(rb2_handle).unwrap();
                    let mass2 = rb2.mass();
                    let act = self.particle_types.get_action(rb.user_data as u8, rb2.user_data as u8);
                    let pos2 = matric_to_vec2(rb2.position().translation);
                    let dist = pos2.distance(pos1);
                    let mut rel_dist = (FIELD_RADIUS-dist) / FIELD_RADIUS;
                    let rev = 1.0;
                    if rel_dist <= rev {
                        rel_dist = rel_dist * act;
                    } // else {
                    //    rel_dist = -((rel_dist-rev)/(1.0-rev));
                    //    //rel_dist = -((rel_dist-rev)/(1.0-rev));
                    //}
                    let dir = pos2.normalize() - pos1.normalize();
                    //rel_vec = rel_vec.normalize();
                    let f = GRAV * mass2 * rel_dist;
                    let vf = dir * f;
                    action += vf;
                    return true;
                },
            );
        }
        let rbm = self.rigid_bodies.get_mut(agent_body_handle).unwrap();
        rbm.reset_forces(true);
        rbm.add_force(Vector2::new(action.x, action.y), true);
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
            self.query_pipeline.intersections_with_shape(
                &self.rigid_bodies,
                &self.colliders,
                rb.position(),
                collider.shape(),
                filter,
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
}

pub struct PhysicsData {
    pub position: Vec2,
    pub rotation: f32,
    pub mass: f32,
    pub kin_eng: Option<f32>,
    pub force: Option<Vec2>,
}
