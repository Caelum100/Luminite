use super::*;

pub struct BufferMem<B: Backend> {
    pub buffer: B::Buffer,
    pub memory: B::Memory,
    pub element_count: usize,
}

impl<B: Backend> BufferMem<B> {
    /// Creates a BufferMem with element count 1
    /// and the specified buffer and memory.
    pub fn new(buffer: B::Buffer, memory: B::Memory) -> BufferMem<B> {
        BufferMem {
            buffer,
            memory,
            element_count: 1,
        }
    }
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
    pub pipeline_layout: B::PipelineLayout,
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
    pub extent: Extent,
    /// A vector containing all models uploaded to the GPU.
    /// The `model_index` property of objects is an index
    /// into this vector.
    pub models: Vec<ModelBuffer<B>>,
    pub memory_types: Vec<MemoryType>,
    pub set_layout: B::DescriptorSetLayout,
}

pub struct ModelBuffer<B: Backend> {
    pub vertices: BufferMem<B>,
    pub indices: BufferMem<B>,
}

pub struct UniformBuffer<B: Backend> {
    pub buffer: BufferMem<B>,
    pub desc_set: B::DescriptorSet,
    pub desc_pool: B::DescriptorPool,
}
