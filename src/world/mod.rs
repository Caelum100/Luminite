//! Stores data of objects and entities in the world.
use super::*;
use render::RenderBackend;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

pub struct World<B: RenderBackend> {
    objects: HashMap<u32, Object<B>>,
    object_id_counter: u32,
}

impl<B: RenderBackend> World<B> {
    pub fn add_object(&mut self, object: Object<B>) {
        self.objects.insert(object.id, object);
    }

    pub fn get_objs<'a>(&'a self) -> &'a HashMap<u32, Object<B>> {
        &self.objects
    }

    pub fn get_objs_mut<'a>(&'a mut self) -> &'a mut HashMap<u32, Object<B>> {
        &mut self.objects
    }

    pub fn get_obj(&mut self, id: u32) -> Option<&Object<B>> {
        self.objects.get(&id)
    }

    pub fn remove_obj(&mut self, id: u32) -> Option<Object<B>> {
        self.objects.remove(&id)
    }

    /// Creates a new world with no objects.
    pub fn new() -> World<B> {
        World {
            objects: HashMap::new(),
            object_id_counter: 0,
        }
    }

    pub fn alloc_obj_id(&mut self) -> u32 {
        let result = self.object_id_counter;
        self.object_id_counter += 1;
        result
    }

    pub fn tick(&mut self) {
        for object in self.objects.values_mut() {
            //object.location.yaw += 1.0;
        }
    }
}

/// An object in the world
pub struct Object<B: RenderBackend> {
    /// Unique object ID
    pub id: u32,
    /// The location in world space of the object
    pub location: Location,
    /// The render data associated with this object
    pub render: B::ObjectRender,
}

impl<B: RenderBackend> std::fmt::Debug for Object<B> {
    fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> Result<(), std::fmt::Error> {
        write!(f, "Object {{location: {:?}}}", self.location)?;
        Ok(())
    }
}

impl<B: RenderBackend> Hash for Object<B> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.id);
    }
}

impl<B: RenderBackend> PartialEq for Object<B> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<B: RenderBackend> Eq for Object<B> {}

impl<B: RenderBackend> Object<B> {
    pub fn new(world: &mut World<B>, render: B::ObjectRender, location: Location) -> Object<B> {
        Object {
            id: world.alloc_obj_id(),
            render,
            location,
        }
    }
}

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

impl Location {
    pub fn to_vec(&self) -> Vec3 {
        vec3(self.x as f32, self.y as f32, self.z as f32)
    }
}

impl Location {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Location {
            x,
            y,
            z,
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    pub fn with_rot(mut self, pitch: f32, yaw: f32) -> Self {
        self.pitch = pitch;
        self.yaw = yaw;
        self
    }
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
