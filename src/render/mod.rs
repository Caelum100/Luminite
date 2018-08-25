use gfx_hal::{
    Backbuffer,
    Backend,
    command::{ClearColor, ClearValue},
    Device,
    format::{Aspects, ChannelType, Format, Swizzle},
    FrameSync,
    Graphics,
    image::{Access, Layout, SubresourceRange, ViewKind}, Instance, pass::{
        Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDependency,
        SubpassDesc, SubpassRef,
    }, pool::{CommandPool, CommandPoolCreateFlags}, Primitive, pso::{
        BlendState, ColorBlendDesc, ColorMask, EntryPoint, GraphicsPipelineDesc, GraphicsShaderSet,
        PipelineStage, Rasterizer, Rect, Viewport,
    }, pso::{DescriptorSetLayoutBinding, DescriptorType, ShaderStageFlags},
    Surface, Swapchain, SwapchainConfig, SwapImageIndex, QueueGroup
};
use self::context::RenderContext;
use self::factory::RenderBuilder;
use super::*;
use super::back;
use super::winit;

pub mod factory;
pub mod context;


/// A three-dimensional vertex
/// with a normal.
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
}

/// Uniform
struct MatrixBlock {
    /// The full MVP matrix
    matrix: Mat4,
    /// The model and view matrices multiplied together
    modelview: Mat4,
}

pub fn create_context() -> RenderContext<back::Backend> {
    let pipeline_layout = [
        DescriptorSetLayoutBinding {
            binding: 0,
            ty: DescriptorType::UniformBuffer,
            count: 1,
            stage_flags: ShaderStageFlags::VERTEX,
            immutable_samplers: false,
        }
    ];
    let builder = RenderBuilder::new()
        .with_title("Luminite")
        .with_vertex_shader(include_bytes!("../../assets/shaders/model.vert.spv"))
        .with_fragment_shader(include_bytes!("../../assets/shaders/model.frag.spv"));

    builder.build()
}

pub fn render(ctx: &mut RenderContext<back::Backend>) {
    let device = &mut ctx.device;
    let swapchain = &mut ctx.swapchain;
    let image_views = &mut ctx.image_views;
    let frame_buffers = &mut ctx.frame_buffers;
}