use super::*;

pub fn upload_model(ctx: &mut RenderContext, models: Vec<tobj::Model>) {
    let (vertices, indices) = combine_models(models);
    let vertices = unsafe { std::mem::transmute::<_, Vec<_Vertex>>(vertices) };

    let vertices = glium::VertexBuffer::new(&ctx.display, &vertices).unwrap();

    let indices = glium::IndexBuffer::new(
        &ctx.display,
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    ).unwrap();

    ctx.models.push(ModelBuffer { vertices, indices });
}
