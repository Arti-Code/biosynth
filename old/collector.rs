use std::collections::HashMap;
use macroquad::math::Vec2;

pub trait Object
    where
    key: u64,
    size: f32
{}

pub trait PhysicsObject where Self: Object {
    fn new() -> Self;
}

pub struct Element {
    pub pos: Vec2,
}

impl PhysicsObject for Element {
    fn new() -> Self {

        Self {
            pos: Vec2::ZERO,
        }
    }
}

pub struct Collector<P>
    where P: PhysicsObject,
{
    pub elements: HashMap<u64, P>,
}

impl Collector<P> {
    pub fn new<P>() -> Self<P> {
        Self {
            elements: HashMap::new(),
        }
    }

    pub fn add_many_elements<P>(&mut self, elements_num: usize, physics_world: &mut World) {
        for _ in 0..elements_num {
            let t = rand::gen_range(0, 5);
            let element = P::new();
            _ = self.add_element(element, physics_world);
        }
    }

    pub fn add_element<P>(&mut self, mut element: &impl PhysicsObject, physics_world: &mut World) -> u64 {
=        let key = element.key;
        //let handle = physics_world.add_poly_body(key,&element.pos, element.points2.clone());
        let handle = physics_world.add_particle(&element.pos, element.size);
        element.physics_handle = Some(handle);
        self.elements.insert(key, element);
        return key;
    }

    pub fn get_type(&self, p_type: u8) -> &ParticleType {
        let particle_type = self.particle_types.get_type(p_type);
        return particle_type;
    }

    pub fn get(&self, id: u64) -> Option<&Particle> {
        return self.elements.get(&id);
    }

    pub fn remove(&mut self, id: u64) {
        self.elements.remove(&id);
    }

    pub fn get_iter(&self) -> Iter<u64, Particle> {
        return self.elements.iter();
    }

    pub fn get_iter_mut(&mut self) -> IterMut<u64, Particle> {
        return self.elements.iter_mut();
    }

    pub fn count(&self) -> usize {
        return self.elements.len();
    }
}
