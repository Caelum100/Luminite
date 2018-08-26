//! Stores data of objects and entities in the world.
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

pub struct World {
    objects: HashMap<u32, Object>,
}

impl World {
    pub fn add_object(&mut self, object: Object) {
        self.objects.insert(object.id, object);
    }

    pub fn get_objs<'a>(&'a self) -> &'a HashMap<u32, Object> {
        &self.objects
    }

    pub fn get_obj(&mut self, id: u32) -> Option<&Object> {
        self.objects.get(&id)
    }

    pub fn remove_obj(&mut self, id: u32) -> Option<Object> {
        self.objects.remove(&id)
    }

    /// Creates a new world with no objects.
    pub fn new() -> World {
        World {
            objects: HashMap::new(),
        }
    }
}

/// An object in the world
pub struct Object {
    /// Unique object ID
    pub id: u32,
    /// The index into the model vector
    pub model_index: usize,
    /// The location in world space of the object
    pub location: Location,
}

impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.id);
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Object {}

/// A three-dimensional location in world space
/// using Euler angles (pitch and yaw).
#[derive(PartialEq, Debug, Clone)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f32,
    pub yaw: f32,
}

impl Hash for Location {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.x.to_bits());
        state.write_u64(self.y.to_bits());
        state.write_u64(self.z.to_bits());
        state.write_u32(self.pitch.to_bits());
        state.write_u32(self.yaw.to_bits());
    }
}
