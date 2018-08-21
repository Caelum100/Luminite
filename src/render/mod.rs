use gfx_hal::{
    command::{ClearColor, ClearValue},
    format::{Aspects, ChannelType, Format, Swizzle},
    image::{Access, Layout, SubresourceRange, ViewKind},
    pass::{
        Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDependency,
        SubpassDesc, SubpassRef,
    },
    pool::{CommandPoolCreateFlags, CommandPool},
    pso::{
        BlendState, ColorBlendDesc, ColorMask, EntryPoint, GraphicsPipelineDesc, GraphicsShaderSet,
        PipelineStage, Rasterizer, Rect, Viewport,
    },
    queue::Submission,
    Backbuffer, Device, FrameSync, Graphics, Instance, Primitive, Surface, SwapImageIndex,
    Swapchain, SwapchainConfig, Backend
};
use super::*;
use super::back;
use super::winit;

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
    /// The physical device corresponding to the logical device
    pub physical_device: B::PhysicalDevice,
    /// The events loop associated with the window
    pub events_loop: winit::EventsLoop,
    /// The window the game is open in
    pub window: winit::Window,
    /// The surface for rendering to
    pub surface: B::Surface,
    /// The command queue group for submitting commands to the GPU
    pub queue_group: B::QueueFamily,
    /// The command pool for submitting commands to the GPU
    pub command_pool: B::CommandPool,
    /// The color format used to represent the surface
    pub surface_color_format: Format,
    /// The current render pass (changed upon window resize)
    pub render_pass: B::RenderPass,
    /// The default graphics pipeline, which includes vertex and fragment shaders
    pub pipeline: B::GraphicsPipeline,
    /// The swapchain
    pub swapchain: B::Swapchain,
    /// The backbuffer for the swapchain
    pub backbuffer: Backbuffer<B>,
    /// Image views
    pub frame_views: Vec<B::ImageView>,
    /// Frame buffers
    pub frame_buffers: Vec<B::Framebuffer>,
    /// Semaphore to wait before drawing to the frame
    pub frame_semaphore: B::Semaphore,
    /// Fence to wait for draw calls to finish
    pub frame_fence: B::Fence,
}

impl <B: Backend> RenderContext<B> {
    /// Initiates a new RenderContext using default options
    /// and the specified shaders.
    pub fn init(vertex_shader: &[u8], fragment_shader: &[u8]) /*-> RenderContext<back::Backend>*/ {
        // TODO
    }
}