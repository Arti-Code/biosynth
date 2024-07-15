//#![allow(unused)]

use crate::util::*;
use macroquad::prelude::*;
use macroquad::math::clamp;
use rapier2d::na::*;
use rapier2d::prelude::*;
use std::collections::HashSet;
use crate::settings::*;
use super::physics_misc::*;
use crate::phyx::dbg::MacroRapierDebugger;

pub struct Physics {
    pub core: PhysicsCore,
}

impl Physics {
    
    pub fn new() -> Physics {
        Self { core: PhysicsCore::new(), }
    }

    pub fn step(&mut self) {
        self.core.step_physics();
    }

    pub fn remove_object(&mut self, rbh: RigidBodyHandle) {
        self.core.remove_physics_object(rbh);
    } 

    pub fn get_bodies_num(&self) -> usize {
        return self.core.get_physics_obj_num();
    }

    pub fn get_bodies_and_colliders_num(&self) -> (usize, usize) {
        return self.core.get_numbers();
    }

    pub fn add_dynamic_object(&mut self, position: &Vec2, rotation: f32, shape: SharedShape, material: PhysicsMaterial, groups: InteractionGroups, can_sleep: bool) -> RigidBodyHandle {
        self.core.add_dynamic(position, rotation, shape, material, groups, can_sleep)
    }

    pub fn add_collider(&mut self, rbh: RigidBodyHandle, rel_position: &Vec2, rotation: f32, shape: SharedShape, material: PhysicsMaterial, groups: InteractionGroups) -> ColliderHandle {
        return self.core.add_collider(rbh, rel_position, rotation, shape, material, groups);
    }

    pub fn get_object_state(&self, rbh: RigidBodyHandle) -> PhysicState {
        return self.core.get_physics_data(rbh);
    }

    pub fn get_object_position(&self, rbh: RigidBodyHandle) -> Option<Vec2> {
        return self.core.get_object_position(rbh);
    }

    pub fn get_object_size(&self, rbh: RigidBodyHandle) -> Option<f32> {
        return self.core.get_object_size(rbh);
    }

    pub fn get_closest_agent(&self, agent_body_handle: RigidBodyHandle, detection_range: f32, detection_angle: f32, direction: Vec2) -> Option<RigidBodyHandle> {
        return self.core.get_closest_agent(agent_body_handle, detection_range, detection_angle, direction);
    }

    pub fn get_closest_plant(&self, agent_body_handle: RigidBodyHandle, detection_range: f32, detection_angle: f32, direction: Vec2) -> Option<RigidBodyHandle> {
        return self.core.get_closest_plant(agent_body_handle, detection_range, detection_angle, direction);
    }

    pub fn get_contacts_set(&mut self, agent_body_handle: RigidBodyHandle, radius: f32) -> HashSet<RigidBodyHandle> {
        return self.core.get_contacts_set(agent_body_handle, radius);
    }

    pub fn get_contacted_agent_set(&mut self, agent_body_handle: RigidBodyHandle, radius: f32) -> HashSet<RigidBodyHandle> {
        return self.core.get_contacted_agent_set(agent_body_handle, radius);
    }

    pub fn get_contacted_plant_set(&mut self, agent_body_handle: RigidBodyHandle, radius: f32) -> HashSet<RigidBodyHandle> {
        return self.core.get_contacted_plant_set(agent_body_handle, radius);
    }

    pub fn get_object(&mut self, rbh: RigidBodyHandle) -> Option<&RigidBody> {
        return self.core.rigid_bodies.get(rbh);
    }

    pub fn get_object_mut(&mut self, rbh: RigidBodyHandle) -> Option<&mut RigidBody> {
        return self.core.rigid_bodies.get_mut(rbh);
    }

    pub fn get_objects_iter(&self) -> impl Iterator<Item = (RigidBodyHandle, &RigidBody)> {
        return self.core.rigid_bodies.iter();
    }

    pub fn get_objects_iter_mut(&mut self) -> impl Iterator<Item = (RigidBodyHandle, &mut RigidBody)> {
        return self.core.rigid_bodies.iter_mut();
    }

    pub fn count_near_plants(&self, rbh: RigidBodyHandle, detection_range: f32) -> usize {
        return self.core.count_near_plants(rbh, detection_range);
    }    

    pub fn get_first_collider_mut(&mut self, rbh: RigidBodyHandle) -> &mut Collider {
        let rb = self.core.rigid_bodies.get(rbh).unwrap();
        let ch = rb.colliders().first().unwrap();
        let c = self.core.colliders.get_mut(*ch).unwrap();
        return c;
    }

    pub fn debug_draw(&mut self) {
        self.core.debug_draw();
    }

}


// struct PhysicsCore{...}
pub struct PhysicsCore {
    pub attract_num: u32,
    pub rigid_bodies: RigidBodySet,
    pub colliders: ColliderSet,
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhaseMultiSap,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    debug_render_pipeline: DebugRenderPipeline,
    debug_renderer: MacroRapierDebugger,
    query_pipeline: QueryPipeline,
    physics_hooks: (),
    event_handler: (),
}

impl PhysicsCore {

    pub fn new() -> Self {

        let params = IntegrationParameters {
            dt: 1./60.,
            ..Default::default()
        };

        let dbg_cfg = DebugRenderStyle {
            collider_dynamic_color: [0.83, 1.0, 0.5, 1.0],
            collider_kinematic_color: [0.33, 1.0, 0.5, 1.0],
            impulse_joint_anchor_color: [0.5, 1.0, 0.5, 1.0],
            impulse_joint_separation_color: [0.66, 1.0, 0.5, 1.0],
            ..Default::default()
        };
        let dbg_mode = 
            DebugRenderMode::COLLIDER_SHAPES | 
            DebugRenderMode::IMPULSE_JOINTS | 
            DebugRenderMode::JOINTS;
            DebugRenderMode::SOLVER_CONTACTS;

        Self {
            attract_num: 0,
            rigid_bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            gravity: Vector2::new(0.0, 0.0),
            integration_parameters: params,
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhaseMultiSap::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            debug_render_pipeline: DebugRenderPipeline::new(dbg_cfg, dbg_mode),
            debug_renderer: MacroRapierDebugger,
            query_pipeline: QueryPipeline::new(),
            physics_hooks: (),
            event_handler: (),
        }
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
    }

    pub fn remove_physics_object(&mut self, body_handle: RigidBodyHandle) {
        _ = self.rigid_bodies.remove(body_handle, &mut self.island_manager, &mut self.colliders, &mut self.impulse_joint_set, &mut self.multibody_joint_set, true);
        //self.remove_body_key_relation(&body_handle);
    }

    pub fn get_physics_obj_num(&self) -> usize {
        let body_num = self.rigid_bodies.len();
        return body_num;
    }

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

    fn add_dynamic_rigidbody(&mut self, position: &Vec2, rotation: f32, linear_damping: f32, angular_damping: f32, can_sleep: bool) -> RigidBodyHandle {
        let pos = make_isometry(position.x, position.y, rotation);

        let dynamic_body = RigidBodyBuilder::dynamic().position(pos)
            .linear_damping(linear_damping).angular_damping(angular_damping).can_sleep(can_sleep).build();
        return self.rigid_bodies.insert(dynamic_body);
    }

    pub fn add_collider(&mut self, body_handle: RigidBodyHandle, rel_position: &Vec2, rotation: f32, shape: SharedShape, physics_props: PhysicsMaterial, groups: InteractionGroups) -> ColliderHandle {
        let iso = make_isometry(rel_position.x, rel_position.y, rotation);
        //let iso2 = Vector2::new(rel_position.x, rel_position.y);
        let r = UnitComplex::from_angle(rotation);
        Isometry2::from_parts(Translation2::new(rel_position.x, rel_position.y), r);
        let collider = match shape.shape_type() {
            ShapeType::Ball => {
                //let radius = shape.0.as_ball().unwrap().radius;
                ColliderBuilder::new(shape).position(iso).density(physics_props.density).friction(physics_props.friction).restitution(physics_props.restitution)
                    .active_collision_types(ActiveCollisionTypes::DYNAMIC_DYNAMIC).active_events(ActiveEvents::COLLISION_EVENTS).collision_groups(groups)
                    .build()
            },
            ShapeType::ConvexPolygon => {
                ColliderBuilder::new(shape).density(physics_props.density).friction(physics_props.friction).restitution(physics_props.restitution)
                .active_collision_types(ActiveCollisionTypes::default()).active_events(ActiveEvents::COLLISION_EVENTS).build()
            },
            _ => {
                ColliderBuilder::ball(5.0).position(iso).build()
            },
        };
        return self.colliders.insert_with_parent(collider, body_handle, &mut self.rigid_bodies);
    }

    pub fn add_dynamic(&mut self, position: &Vec2, rotation: f32, shape: SharedShape, physics_props: PhysicsMaterial, groups: InteractionGroups, can_sleep: bool) -> RigidBodyHandle {
        let rbh = self.add_dynamic_rigidbody(position, rotation, physics_props.linear_damping, physics_props.angular_damping, can_sleep);
        let _ch = self.add_collider(rbh, &Vec2::ZERO, 0.0, shape, physics_props, groups);
        return rbh;
    }

    pub fn get_physics_data(&self, handle: RigidBodyHandle) -> PhysicState {
        let settings = get_settings();
        if let Some(rb) = self.rigid_bodies.get(handle) {
            let iso = rb.position();
            let (pos, rot) = iso_to_vec2_rot(iso);
            let force = Vec2::new(rb.user_force().data.0[0][0], rb.user_force().data.0[0][1]);
            let data = PhysicState {
                position: pos,
                rotation: rot,
                mass: rb.mass(),
                kin_eng: Some(rb.kinetic_energy()),
                force: Some(force),
            };
            return data;
        } else {
            return PhysicState {
                position: Vec2::new((settings.world_w / 2) as f32, (settings.world_h / 2) as f32 ),
                rotation: 0.0,
                mass: 0.0,
                kin_eng: Some(0.0),
                force: None,
            };
        }
    }

    pub fn get_numbers(&self) -> (usize, usize) {
        let r = self.rigid_bodies.len();
        let c = self.colliders.len();
        return (r,c);
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

    pub fn get_object_size(&self, handle: RigidBodyHandle) -> Option<f32> {
        let rb = self.rigid_bodies.get(handle);
        match rb {
            Some(body) => {
                match body.colliders().first() {
                    Some(collider_handle) => {
                        match self.colliders.get(*collider_handle) {
                            Some(collider) => {
                                return Some(collider.shape().as_ball().unwrap().radius);
                            },
                            None => {
                                return None;
                            },
                        }
                    },
                    None => { 
                      return None; 
                    },
                }
            },
            None => {
                return None;
            },
        }
    }

    pub fn get_contacts_set(&mut self, agent_body_handle: RigidBodyHandle, radius: f32) -> HashSet<RigidBodyHandle> {
        let mut contacts: HashSet<RigidBodyHandle> = HashSet::new();
        let rb = self.rigid_bodies.get(agent_body_handle).unwrap();
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_DYNAMIC | QueryFilterFlags::EXCLUDE_SENSORS,
            groups: None,
            exclude_rigid_body: Some(agent_body_handle),
            ..Default::default()
        };
        for c in rb.colliders() {
            let collider = self.colliders.get(*c).unwrap();
            if collider.is_sensor() {
                continue;
            }
            self.query_pipeline.intersections_with_shape(&self.rigid_bodies, &self.colliders, rb.position(), &rapier2d::geometry::Ball::new(radius), filter,
                |collided| {
                    let rb2_handle = self.get_body_handle_from_collider(collided).unwrap();
                    contacts.insert(rb2_handle);
                    return true;
                },
            );
        }
        return contacts;
    }

    pub fn get_contacted_agent_set(&mut self, agent_body_handle: RigidBodyHandle, radius: f32) -> HashSet<RigidBodyHandle> {
        let mut contacts: HashSet<RigidBodyHandle> = HashSet::new();
        let rb = self.rigid_bodies.get(agent_body_handle).unwrap();
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_DYNAMIC | QueryFilterFlags::EXCLUDE_SENSORS,
            groups: Some(InteractionGroups::new(Group::GROUP_1, Group::GROUP_1)),
            exclude_rigid_body: Some(agent_body_handle),
            ..Default::default()
        };
        for c in rb.colliders() {
            let collider = self.colliders.get(*c).unwrap();
            if collider.is_sensor() {
                continue;
            }
            self.query_pipeline.intersections_with_shape(&self.rigid_bodies, &self.colliders, rb.position(), &rapier2d::geometry::Ball::new(radius), filter,
                |collided| {
                    let rb2_handle = self.get_body_handle_from_collider(collided).unwrap();
                    contacts.insert(rb2_handle);
                    return true;
                },
            );
        }
        return contacts;
    }

    pub fn get_contacted_plant_set(&mut self, agent_body_handle: RigidBodyHandle, radius: f32) -> HashSet<RigidBodyHandle> {
        let mut contacts: HashSet<RigidBodyHandle> = HashSet::new();
        let rb = self.rigid_bodies.get(agent_body_handle).unwrap();
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_DYNAMIC | QueryFilterFlags::EXCLUDE_SENSORS,
            groups: Some(InteractionGroups::new(Group::GROUP_2, Group::GROUP_2)),
            exclude_rigid_body: Some(agent_body_handle),
            ..Default::default()
        };
        for c in rb.colliders() {
            let collider = self.colliders.get(*c).unwrap();
            if collider.is_sensor() {
                continue;
            }
            self.query_pipeline.intersections_with_shape(&self.rigid_bodies, &self.colliders, rb.position(), &rapier2d::geometry::Ball::new(radius), filter,
                |collided| {
                    let rb2_handle = self.get_body_handle_from_collider(collided).unwrap();
                    contacts.insert(rb2_handle);
                    return true;
                },
            );
        }
        return contacts;
    }

    pub fn get_closest_agent(&self, agent_body_handle: RigidBodyHandle, detection_range: f32, detection_angle: f32, direction: Vec2) -> Option<RigidBodyHandle> {
        let mut small_vision = get_settings().peripheral_vision*get_settings().agent_vision_range;
        small_vision = clamp(small_vision, 0.0, detection_range);
        let rb = self.rigid_bodies.get(agent_body_handle).unwrap();
        let pos1 = matrix_to_vec2(rb.position().translation);
        let mut dist = f32::INFINITY;
        let mut target: RigidBodyHandle = RigidBodyHandle::invalid();
        let detector = ColliderBuilder::ball(detection_range).sensor(true).density(0.0).build();
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_DYNAMIC | QueryFilterFlags::EXCLUDE_SENSORS,
            groups: Some(InteractionGroups::new(Group::GROUP_1, Group::GROUP_1)),
            exclude_collider: None,
            exclude_rigid_body: Some(agent_body_handle),
            ..Default::default()
        };
        self.query_pipeline.intersections_with_shape(&self.rigid_bodies, &self.colliders, rb.position(), detector.shape(), filter,
            |collided| {
                let rb2_handle = self.get_body_handle_from_collider(collided).unwrap();
                let rb2 = self.rigid_bodies.get(rb2_handle).unwrap();
                let pos2 = matrix_to_vec2(rb2.position().translation);
                let new_dist = pos1.distance(pos2);
                let local_pos = pos2 - pos1;
                let ang = direction.angle_between((local_pos).normalize_or_zero());
                if new_dist <= small_vision && new_dist < dist {
                    dist = new_dist;
                    target = rb2_handle;
                } else if new_dist < dist && ang.abs() <= detection_angle/2.0 {
                    dist = new_dist;
                    target = rb2_handle;
                }
                return true;
            },
        );
        if dist < f32::INFINITY {
            return Some(target);
        } else {
            return None;
        }
    }

    pub fn get_closest_plant(&self, agent_body_handle: RigidBodyHandle, detection_range: f32, detection_angle: f32, direction: Vec2) -> Option<RigidBodyHandle> {
        let rb = self.rigid_bodies.get(agent_body_handle).unwrap();
        let pos1 = matrix_to_vec2(rb.position().translation);
        let mut dist = f32::INFINITY;
        let mut target: RigidBodyHandle = RigidBodyHandle::invalid();
        let detector = ColliderBuilder::ball(detection_range).sensor(true).density(0.0).build();
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_DYNAMIC | QueryFilterFlags::EXCLUDE_SENSORS,
            groups: Some(InteractionGroups::new(Group::GROUP_2, Group::GROUP_2)),
            exclude_collider: None,
            exclude_rigid_body: Some(agent_body_handle),
            ..Default::default()
        };
        self.query_pipeline.intersections_with_shape(&self.rigid_bodies, &self.colliders, rb.position(), detector.shape(), filter,
            |collided| {
                let rb2_handle = self.get_body_handle_from_collider(collided).unwrap();
                let rb2 = self.rigid_bodies.get(rb2_handle).unwrap();
                let pos2 = matrix_to_vec2(rb2.position().translation);
                let new_dist = pos1.distance(pos2);
                let local_pos = pos2 - pos1;
                let ang = direction.angle_between((local_pos).normalize_or_zero());
                if new_dist <= detection_range*0.1 && new_dist < dist {
                    dist = new_dist;
                    target = rb2_handle;
                } else if new_dist < dist && ang.abs() <= detection_angle/2.0 {
                    dist = new_dist;
                    target = rb2_handle;
                }
                return true;
            },
        );
        if dist < f32::INFINITY {
            return Some(target);
        } else {
            return None;
        }
    }

    pub fn count_near_plants(&self, rbh: RigidBodyHandle, detection_range: f32) -> usize {
        let rb = self.rigid_bodies.get(rbh).unwrap();
        //let pos1 = matrix_to_vec2(rb.position().translation);
        //let mut dist = f32::INFINITY;
        //let mut target: RigidBodyHandle = RigidBodyHandle::invalid();
        let detector = ColliderBuilder::ball(detection_range).sensor(true).density(0.0).build();
        let filter = QueryFilter {
            flags: QueryFilterFlags::ONLY_DYNAMIC | QueryFilterFlags::EXCLUDE_SENSORS,
            groups: Some(InteractionGroups::new(Group::GROUP_2, Group::GROUP_2)),
            exclude_collider: None,
            exclude_rigid_body: Some(rbh),
            ..Default::default()
        };
        let mut n: usize = 0;
        self.query_pipeline.intersections_with_shape(&self.rigid_bodies, &self.colliders, rb.position(), detector.shape(), filter,
         |_| {
            n += 1;
            return true;
        });
        return n;
    }

    pub fn debug_draw(&mut self) {
        self.debug_render_pipeline.render(
            &mut self.debug_renderer, 
            &self.rigid_bodies, 
            &self.colliders, 
            &self.impulse_joint_set, 
            &self.multibody_joint_set, 
            &self.narrow_phase
        );
    }

}

