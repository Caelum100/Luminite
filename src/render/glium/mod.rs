//! OpenGL rendering backend using glium.
extern crate spirv_cross as spv;
use self::spv::{glsl, spirv};
use super::*;
use glium;
use glium::glutin;

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
        unimplemented!()
    }
}

/// Holds state data for OpenGL
pub struct RenderContext {
    display: glium::Display,
    models: Vec<ModelBuffer>,
    program: glium::Program,
    events_loop: glutin::EventsLoop,
}

/// Creates a `RenderContext` with the specified
/// window title and dimensions
pub fn create_context(title: &str, dimensions: (u32, u32)) -> RenderContext {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title(title)
        .with_dimensions(dimensions.into());
    let context = glutin::ContextBuilder::new()
        .with_depth_buffer(24)
        .with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let program = compile_program(&display);

    RenderContext {
        display,
        models: Vec::new(),
        program,
        events_loop,
    }
}

pub fn render(ctx: &mut RenderContext, world: &mut World<_RenderBackend>) {
    let mut frame = ctx.display.draw();
    frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
    for object in world.get_objs().values() {
        render_obj(ctx, &mut frame, object);
    }
    frame.finish();
}

fn render_obj(ctx: &RenderContext, frame: &mut glium::Frame, object: &Object<_RenderBackend>) {
    let (matrix, modelview) = mvp_matrix(object);
    let uniforms = unsafe {
        let matrix = std::mem::transmute::<_, [[f32; 4]; 4]>(matrix);
        let modelview = std::mem::transmute::<_, [[f32; 4]; 4]>(modelview);
        uniform!(matrix, modelview,)
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

pub fn create_obj_render(
    model_index: usize,
    _shader_index: usize, // Not yet used (see #7)
    ctx: &RenderContext,
) -> ObjectRender {
    ObjectRender { model_index }
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
    let vertex_mod =
        spirv::Module::from_words(include_bytes!("../../../assets/shaders/model.vert.spv"));
    let vertex_glsl = spirv::Ast::<glsl::Target>::parse(&vertex_mod)
        .unwrap()
        .compile()
        .unwrap();
    debug!("Loaded GLSL shader from SPIR-V bytecode: {}", glsl);
    let frag_mod =
        spirv::Module::from_words(include_bytes!("../../../assets/shaders/model.frag.spv"));
    let frag_glsl = spirv::Ast::<glsl::Target>::parse(&frag_mod)
        .unwrap()
        .compile()
        .unwrap();
    debug!("Loaded GLSL shader from SPIR-V bytecode: {}", glsl);

    glium::Program::from_source(&display, vertex_glsl.as_str(), frag_glsl.as_str(), None).unwrap()
}

fn create_vertex(vertex: Vertex) -> _Vertex {
    unsafe { std::mem::transmute(vertex) }
}
