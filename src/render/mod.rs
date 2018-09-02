//! Code shared between all rendering backends
#[cfg(not(feature = "gl"))]
pub mod gfx;
#[cfg(feature = "gl")]
pub mod glium;

#[cfg(not(feature = "gl"))]
pub use self::gfx::*;
#[cfg(feature = "gl")]
pub use self::glium::*;

use super::*;
use std::path::Path;

/// A render backend.
pub trait RenderBackend {
    /// Render data associated with an object
    type ObjectRender;
    type RenderContext;

    fn upload_model(ctx: &mut Self::RenderContext, models: Vec<tobj::Model>);
}

/// A three-dimensional vertex
/// with a position and normal.
#[derive(Clone, Copy)]
pub struct Vertex {
    pub a_position: Vec3,
    pub a_normal: Vec3,
}

impl Vertex {
    /// Produces a vertex with the specified
    /// positions and normals.
    pub fn new(x: f32, y: f32, z: f32, nx: f32, ny: f32, nz: f32) -> Self {
        Vertex {
            a_position: vec3(x, y, z),
            a_normal: vec3(nx, ny, nz),
        }
    }
}

/// Uniform
#[derive(Clone, Copy)]
struct MatrixBlock {
    /// The full MVP matrix
    matrix: Mat4,
    modelview: Mat4,
}

/// Produces a model-view-projection matrix
/// for the specified object.
fn mvp_matrix<B: RenderBackend>(object: &Object<B>) -> (Mat4, Mat4) {
    use glm::ext::*;
    let translation = translate(&num::one(), object.location.to_vec());

    let rotation: Mat4 = rotate(
        &num::one(),
        radians(object.location.yaw),
        vec3(0.0, 1.0, 0.0),
    )
        * rotate(
            &num::one(),
            radians(object.location.pitch),
            vec3(1.0, 0.0, 0.0),
        );

    let scale: Mat4 = num::one();
    let model = translation * rotation * scale;

    // TODO moving camera
    let view = look_at(
        vec3(4.0, 3.0, 3.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
    );

    // TODO view distance, custom aspect ratio
    let projection = perspective(45.0f32, 4.0 / 3.0, 0.1, 128.0);
    (projection * view * model, view * model)
}

/// Uploads vertex buffer data for models to the GPU,
/// adding the memory and buffers to the `RenderContext`'s
/// list of models
pub fn upload_models<B: RenderBackend>(ctx: &mut B::RenderContext) {
    load_and_upload_model::<B>(ctx, Path::new("assets/models/cube.obj"));
    load_and_upload_model::<B>(ctx, Path::new("assets/models/sword.obj"))
}

fn load_and_upload_model<B: RenderBackend>(ctx: &mut B::RenderContext, path: &Path) {
    let (models, _) = tobj::load_obj(path).expect("failed to load model file");
    B::upload_model(ctx, models);
}

/// Combines all models into one vector of vertices and indices.
pub fn combine_models(mut models: Vec<tobj::Model>) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for model in models.iter_mut() {
        let mut mesh = &mut model.mesh;
        vertices.append(&mut positions_to_vertices(&mesh.positions, &mesh.normals));
        indices.append(&mut mesh.indices);
    }
    (vertices, indices)
}

/// Converts vectors of floats to vectors
/// of vertices. The length of the `positions`
/// array must be a multiple of three.
pub fn positions_to_vertices(positions: &Vec<f32>, normals: &Vec<f32>) -> Vec<Vertex> {
    if positions.len() % 3 != 0 {
        panic!("Length of position array must be a multiple of three");
    }

    let mut result = Vec::new();
    for index in 0..positions.len() {
        if index % 3 != 0 {
            continue;
        }
        result.push(Vertex::new(
            positions[index],
            positions[index + 1],
            positions[index + 2],
            normals[index],
            normals[index + 1],
            normals[index + 2],
        ));
    }

    result
}
