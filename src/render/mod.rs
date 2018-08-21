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
    }, queue::{QueueGroup, Submission},
    Surface, Swapchain, SwapchainConfig, SwapImageIndex,
};
use super::*;
use super::back;
use super::winit;

mod factory;

/// A three-dimensional vertex
/// with a normal.
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
}

/// Holds all values necessary to render to the screen.
pub struct RenderContext<B: Backend> {
    /// The gfx-rs instance
    pub instance: back::Instance,
    /// The logical device selected for rendering
    pub device: B::Device,
    /// The events loop associated with the window
    pub events_loop: winit::EventsLoop,
    /// The window the game is open in
    pub window: winit::Window,
    /// The surface for rendering to
    pub surface: B::Surface,
    /// The command queue group for submitting commands to the GPU
    pub queue_group: QueueGroup<B, Graphics>,
    /// The command pool for submitting commands to the GPU
    pub command_pool: CommandPool<B, Graphics>,
    /// The current render pass (changed upon window resize)
    pub render_pass: B::RenderPass,
    /// The default graphics pipeline, which includes vertex and fragment shaders
    pub pipeline: B::GraphicsPipeline,
    /// The swapchain
    pub swapchain: B::Swapchain,
    /// Image views
    pub image_views: Vec<B::ImageView>,
    /// Frame buffers
    pub frame_buffers: Vec<B::Framebuffer>,
    /// Semaphore to wait before drawing to the frame
    pub frame_semaphore: B::Semaphore,
    /// Fence to wait for draw calls to finish
    pub frame_fence: B::Fence,
}

impl<B: Backend> RenderContext<B> {
    /// Initiates a new RenderContext using default options
    /// and the specified shaders.
    ///
    /// The shaders should be in compiled SPIR-V
    /// format.
    ///
    pub fn init(vertex_shader: &[u8], fragment_shader: &[u8], title: &str)
                -> RenderContext<back::Backend> {
        let builder: factory::RenderBuilder<back::Backend> = factory::RenderBuilder::new();
        builder.build()
    }

    /// Renders for this iteration of the main loop.
    pub fn render(&mut self) {
        // TODO
    }
}

#[inline(always)]
fn create_shader<B: Backend>(raw: &[u8], device: &B::Device) -> B::ShaderModule {
    device.create_shader_module(raw).unwrap()
}