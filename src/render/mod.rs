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
    // TODO maybe use builder pattern? This is messy
    pub fn init(vertex_shader: &[u8], fragment_shader: &[u8], title: &str)
                -> RenderContext<back::Backend> {
        let events_loop = winit::EventsLoop::new();
        let window = winit::WindowBuilder::new()
            .with_title(title)
            .with_dimensions((720, 480).into())
            .build(&events_loop)
            .unwrap();

        let instance = back::Instance::create(title, 1);
        let mut surface = instance.create_surface(&window);
        let mut adapter = instance
            .enumerate_adapters()
            .remove(0); // Just take the first device for now
        let (device, queue_group) = adapter
            .open_with::<_, Graphics>(1, |family| surface.supports_queue_family(family))
            .unwrap();

        let max_buffers = 16;
        let command_pool = device.create_command_pool_typed(
            &queue_group,
            CommandPoolCreateFlags::empty(),
            max_buffers,
        );

        let physical_device = &adapter.physical_device;
        let (caps, formats, _) =
            surface.compatibility(physical_device);

        let surface_color_format = {
            // Pick color format
            match formats {
                Some(choices) => choices
                    .into_iter()
                    .find(|format| format.base_format().1 == ChannelType::Srgb)
                    .unwrap(),
                None => Format::Rgba8Srgb,
            }
        };

        let render_pass = {
            let color_attachment = Attachment {
                format: Some(surface_color_format),
                samples: 1,
                ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
                stencil_ops: AttachmentOps::DONT_CARE,
                layouts: Layout::Undefined..Layout::Present,
            };

            // Single subpass for now
            let subpass = SubpassDesc {
                colors: &[(0, Layout::ColorAttachmentOptimal)],
                depth_stencil: None,
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };

            let dependency = SubpassDependency {
                passes: SubpassRef::External..SubpassRef::Pass(0),
                stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT..PipelineStage::COLOR_ATTACHMENT_OUTPUT,
                accesses: Access::empty()
                    ..(Access::COLOR_ATTACHMENT_READ | Access::COLOR_ATTACHMENT_WRITE),
            };

            device.create_render_pass(
                &[color_attachment],
                &[subpass],
                &[dependency],
            )
        };

        // No uniforms just yet
        let pipeline_layout = device.create_pipeline_layout(
            &[],
            &[]);

        let vertex_shader_mod =
            create_shader::<back::Backend>(vertex_shader, &device);
        let fragment_shader_mod =
            create_shader::<back::Backend>(fragment_shader, &device);

        let pipeline = {
            let vs_entry = EntryPoint::<back::Backend> {
                entry: "main",
                module: &vertex_shader_mod,
                specialization: &[],
            };

            let fs_entry = EntryPoint::<back::Backend> {
                entry: "main",
                module: &fragment_shader_mod,
                specialization: &[],
            };

            let shader_entries = GraphicsShaderSet {
                vertex: vs_entry,
                hull: None,
                domain: None,
                geometry: None,
                fragment: Some(fs_entry),
            };

            let subpass = Subpass {
                index: 0,
                main_pass: &render_pass,
            };

            let mut pipeline_desc = GraphicsPipelineDesc::new(
                shader_entries,
                Primitive::TriangleList,
                Rasterizer::FILL,
                &pipeline_layout,
                subpass,
            );

            pipeline_desc
                .blender
                .targets
                .push(ColorBlendDesc(ColorMask::ALL, BlendState::ALPHA));

            device
                .create_graphics_pipeline(&pipeline_desc, None)
                .unwrap()
        };

        // Swapchain
        let swapchain_config = SwapchainConfig::from_caps(
            &caps,
            surface_color_format);
        let extent = swapchain_config.extent.to_extent();

        let (swapchain, backbuffer) =
            device.create_swapchain(&mut surface, swapchain_config, None);

        // Create image views and frame buffers
        let (image_views, frame_buffers) = match backbuffer {
            Backbuffer::Images(images) => {
                let color_range = SubresourceRange {
                    aspects: Aspects::COLOR,
                    levels: 0..1,
                    layers: 0..1,
                };

                let image_views = images
                    .iter()
                    .map(|image| {
                        device
                            .create_image_view(
                                image,
                                ViewKind::D2,
                                surface_color_format,
                                Swizzle::NO,
                                color_range.clone(),
                            )
                            .unwrap()
                    })
                    .collect::<Vec<_>>();

                let _frame_buffers = image_views
                    .iter()
                    .map(|image_view| {
                        device
                            .create_framebuffer(&render_pass, vec![image_view], extent)
                            .unwrap()
                    })
                    .collect();

                (image_views, _frame_buffers)
            }

            // For OpenGL backend
            Backbuffer::Framebuffer(fbo) => (vec![], vec![fbo]),
        };

        let frame_semaphore = device.create_semaphore();
        let frame_fence = device.create_fence(false);

        RenderContext {
            instance,
            device,
            events_loop,
            window,
            surface,
            queue_group,
            command_pool,
            render_pass,
            pipeline,
            swapchain,
            image_views,
            frame_buffers,
            frame_semaphore,
            frame_fence,
        }
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