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
    }, pso::{DescriptorSetLayoutBinding},
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

pub fn create_context() -> RenderContext<back::Backend> {
    let builder = RenderBuilder::new()
        .with_title("Luminite: Light of Life in Darkness")
        .with_vertex_shader(include_bytes!("../../assets/shaders/model.vert.spv"))
        .with_fragment_shader(include_bytes!("../../assets/shaders/model.frag.spv"));

    builder.build()
}

