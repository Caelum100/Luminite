//! Module for loading assets from files.
use super::*;
use std::path::Path;

/// Uploads vertex buffer data for models to the GPU,
/// adding the memory and buffers to the `RenderContext`'s
/// list of models
pub fn upload_models(ctx: &mut RenderContext<back::Backend>) {
    // For now, just load the models one by one
    // In the future, we may use JSON files to list
    // and load all models.
    let (models, _) = tobj::load_obj(Path::new("assets/models/cube.obj")).unwrap();
    let (vertices, indices) = combine_models(models);
    let vertices_count = vertices.len();
    let indices_count = indices.len();

    let (v_buffer, v_mem) = buffer_util::create_buffer::<back::Backend, _>(
        &ctx.device,
        &ctx.memory_types,
        Properties::CPU_VISIBLE,
        Usage::VERTEX,
        &vertices,
    );

    let (i_buffer, i_mem) = buffer_util::create_buffer::<back::Backend, _>(
        &ctx.device,
        &ctx.memory_types,
        Properties::CPU_VISIBLE,
        Usage::INDEX,
        &indices,
    );

    ctx.models.push(context::ModelBuffer {
        vertices: context::BufferMem {
            buffer: v_buffer,
            memory: v_mem,
            element_count: vertices_count,
        },
        indices: context::BufferMem {
            buffer: i_buffer,
            memory: i_mem,
            element_count: indices_count,
        },
    });
}

/// Combines all models into one vector of vertices and indices.
fn combine_models(mut models: Vec<tobj::Model>) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for model in models.iter_mut() {
        let mut mesh = &mut model.mesh;
        vertices.append(&mut positions_to_vertices(&mesh.positions));
        indices.append(&mut mesh.indices);
    }
    (vertices, indices)
}

/// Converts vectors of floats to vectors
/// of vertices. The length of the `positions`
/// array must be a multiple of three.
fn positions_to_vertices(positions: &Vec<f32>) -> Vec<Vertex> {
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
        ));
    }

    result
}
