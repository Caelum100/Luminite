//! Stores data of objects and entities in the world.
use super::*;
use render::RenderBackend;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

static mut OBJECT_GLOBAL_ID: u64 = 0;

pub struct World<B: RenderBackend> {
    objects: HashMap<u64, Object<B>>,
}

impl<B: RenderBackend> World<B> {
    pub fn add_obj(&mut self, object: Object<B>) {
        let id = object.global_id;
        self.objects.insert(id, object);
    }

    pub fn add_objs(&mut self, objects: Vec<Object<B>>) {
        for obj in objects {
            let id = obj.global_id;
            self.objects.insert(id, obj);
        }
    }

    pub fn get_objs<'a>(&'a self) -> &'a HashMap<u64, Object<B>> {
        &self.objects
    }

    pub fn get_objs_mut<'a>(&'a mut self) -> &'a mut HashMap<u64, Object<B>> {
        &mut self.objects
    }

    pub fn get_obj(&mut self, id: u64) -> Option<&Object<B>> {
        self.objects.get(&id)
    }

    pub fn remove_obj(&mut self, id: u64) -> Option<Object<B>> {
        self.objects.remove(&id)
    }

    /// Creates a new world with no objects.
    pub fn new() -> World<B> {
        World {
            objects: HashMap::new(),
        }
    }

    pub fn tick(&mut self) {}
}

/// An object in the world
pub struct Object<B: RenderBackend> {
    /// The GLOBAL ID for the object. This
    /// is used for things like hashing and equal.
    pub global_id: u64,
    /// The location in world space of the object
    pub location: Location,
    /// The render data associated with this object
    pub render: B::ObjectRender,
    // The descriptor for the object - this is loaded from
    // JSON files in assets/objects
    // TODO
}

impl<B: RenderBackend> std::fmt::Debug for Object<B> {
    fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> Result<(), std::fmt::Error> {
        write!(f, "Object {{location: {:?}}}", self.location)?;
        Ok(())
    }
}

impl<B: RenderBackend> Hash for Object<B> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.global_id);
    }
}

impl<B: RenderBackend> PartialEq for Object<B> {
    fn eq(&self, other: &Self) -> bool {
        self.global_id == other.global_id
    }
}

impl<B: RenderBackend> Eq for Object<B> {}

impl<B: RenderBackend> Object<B> {
    pub fn new(render: B::ObjectRender, location: Location) -> Object<B> {
        Object {
            render,
            location,
            global_id: unsafe {
                OBJECT_GLOBAL_ID += 1;
                OBJECT_GLOBAL_ID
            },
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
