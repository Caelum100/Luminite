//! Module for loading assets from files.
use super::*;

pub fn upload_model(ctx: &mut RenderContext<back::Backend>, models: Vec<tobj::Model>) {
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
