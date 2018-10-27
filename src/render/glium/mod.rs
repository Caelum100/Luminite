//! OpenGL rendering backend using glium.
pub mod asset_load;

use super::*;
use glium;
use glium::glutin;
use glium::Surface;

pub enum _RenderBackend {}

/// So we can use `implement_vertex!`, we have to
/// use a secondary struct we can transmute to
#[derive(Copy, Clone)]
struct _Vertex {
    a_position: (f32, f32, f32),
    a_normal: (f32, f32, f32),
}

implement_vertex!(_Vertex, a_position, a_normal);

impl RenderBackend for _RenderBackend {
    type ObjectRender = ObjectRender;
    type RenderContext = RenderContext;

    fn upload_model(ctx: &mut RenderContext, models: Vec<tobj::Model>) {
        asset_load::upload_model(ctx, models);
    }

    /// Creates a `RenderContext` with the specified
    /// window title and dimensions
    fn create_context(title: &str, dimensions: (u32, u32)) -> RenderContext {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(dimensions.into());
        let context = glutin::ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(window, context, &events_loop).unwrap();
        let program = compile_program(&display);

        RenderContext {
            display,
            models: Vec::new(),
            program,
            events_loop,
        }
    }

    fn create_obj_render(
        model_index: usize,
        _shader_index: usize, // Not yet used (see #7)
        _ctx: &mut RenderContext,
    ) -> ObjectRender {
        ObjectRender { model_index }
    }
}

/// Holds state data for OpenGL
pub struct RenderContext {
    pub display: glium::Display,
    pub models: Vec<ModelBuffer>,
    pub program: glium::Program,
    pub events_loop: glutin::EventsLoop,
}

pub fn render(ctx: &mut RenderContext, world: &mut World<_RenderBackend>) {
    let mut frame = ctx.display.draw();
    frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
    for object in world.get_objs().values() {
        render_obj(ctx, &mut frame, object);
    }
    frame.finish().unwrap();
}

fn render_obj(ctx: &RenderContext, frame: &mut glium::Frame, object: &Object<_RenderBackend>) {
    let (matrix, modelview) = mvp_matrix(object);
    let (matrix, modelview) = unsafe {
        let matrix = std::mem::transmute::<_, [[f32; 4]; 4]>(matrix);
        let modelview = std::mem::transmute::<_, [[f32; 4]; 4]>(modelview);
        (matrix, modelview)
    };
    let uniforms = uniform! {
        matrix: matrix,
        modelview: modelview
    };

    let draw_params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let model = &ctx.models[object.render.model_index];
    frame
        .draw(
            &model.vertices,
            &model.indices,
            &ctx.program,
            &uniforms,
            &draw_params,
        )
        .unwrap();
}

/// Vertex and index buffers
/// for a model
pub struct ModelBuffer {
    vertices: glium::VertexBuffer<_Vertex>,
    indices: glium::IndexBuffer<u32>,
}

/// Holds render data associated with an object
pub struct ObjectRender {
    /// The index into the model vector
    model_index: usize,
}

fn compile_program(display: &glium::Display) -> glium::Program {
    // Load from GLSL instead of compiled SPIR-V for now
    glium::Program::from_source(
        display,
        include_str!("../../shaders/model.glium.vert"),
        include_str!("../../shaders/model.glium.frag"),
        None,
    ).unwrap()
}
