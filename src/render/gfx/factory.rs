//! Includes factory functions for building RenderContexts.
use super::*;
use gfx_hal::{Instance, PhysicalDevice, Surface};

/// Struct used to build RenderContexts
/// in a clean manner
pub struct RenderBuilder<'a, B: Backend> {
    /// The gfx-rs instance
    instance: Option<back::Instance>,
    /// The logical device selected for rendering
    device: Option<B::Device>,
    /// The events loop associated with the window
    events_loop: Option<winit::EventsLoop>,
    /// The window the game is open in
    window: Option<winit::Window>,
    /// The surface for rendering to
    surface: Option<B::Surface>,
    /// The command queue group for submitting commands to the GPU
    queue_group: Option<QueueGroup<B, Graphics>>,
    /// The command pool for submitting commands to the GPU
    command_pool: Option<CommandPool<B, Graphics>>,
    /// The current render pass (changed upon window resize)
    render_pass: Option<B::RenderPass>,
    /// Raw vertex shader
    // TODO multiple pipelines/shaders
    vertex_shader: &'a [u8],
    /// Raw fragment shader
    fragment_shader: &'a [u8],
    /// Title of window
    title: &'a str,
    /// Dimensions of window
    dimensions: (u32, u32),
    pipeline_layout: &'a [DescriptorSetLayoutBinding],
    /// Surface's color format
    surface_color_format: Option<Format>,
    adapter: Option<gfx_hal::Adapter<B>>,
    caps: Option<gfx_hal::SurfaceCapabilities>,
    vertex_desc: Option<VertexBufferDesc>,
    attr_descs: Vec<AttributeDesc>,
    memory_types: Vec<MemoryType>,
    depth_format: Format,
}

impl<'a, B: Backend> Default for RenderBuilder<'a, B> {
    fn default() -> Self {
        RenderBuilder {
            instance: None,
            device: None,
            events_loop: None,
            window: None,
            surface: None,
            queue_group: None,
            command_pool: None,
            render_pass: None,
            // TODO allow for more shaders
            vertex_shader: &[],
            fragment_shader: &[],
            title: "",
            dimensions: (720, 480),
            surface_color_format: None,
            adapter: None,
            caps: None,
            pipeline_layout: &[],
            vertex_desc: None,
            attr_descs: vec![],
            memory_types: vec![],
            depth_format: Format::D32FloatS8Uint,
        }
    }
}

impl<'a> RenderBuilder<'a, back::Backend> {
    /// Creates a new RenderBuilder.
    pub fn new() -> RenderBuilder<'a, back::Backend> {
        RenderBuilder {
            ..Default::default()
        }
    }

    pub fn with_title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.dimensions = (width, height);
        self
    }

    pub fn with_vertex_shader(mut self, vertex_shader: &'a [u8]) -> Self {
        self.vertex_shader = vertex_shader;
        self
    }

    pub fn with_fragment_shader(mut self, fragment_shader: &'a [u8]) -> Self {
        self.fragment_shader = fragment_shader;
        self
    }

    pub fn with_pipeline(mut self, desc: &'a [DescriptorSetLayoutBinding]) -> Self {
        self.pipeline_layout = desc;
        self
    }

    pub fn with_vertex_attr(
        mut self,
        vertex_desc: VertexBufferDesc,
        attr_descs: Vec<AttributeDesc>,
    ) -> Self {
        self.vertex_desc = Some(vertex_desc);
        self.attr_descs = attr_descs;
        self
    }

    /// Builds a RenderContext, initializing all values and
    /// consuming the RenderBuilder in the process.
    pub fn build(mut self) -> RenderContext<back::Backend> {
        self.build_instance();
        self.build_window_and_events_loop();
        self.build_device_and_queue_group_and_surface();
        self.build_command_pool();
        self.build_render_pass();
        self.finish()
    }

    fn build_instance(&mut self) {
        self.instance = Some(back::Instance::create(self.title, 1));
    }

    fn build_device_and_queue_group_and_surface(&mut self) {
        self.surface = Some(
            self.instance
                .as_ref()
                .unwrap()
                .create_surface(self.window.as_ref().unwrap()),
        );

        let (device, queue_group) = {
            let mut adapter = self
                .instance
                .as_mut()
                .unwrap()
                .enumerate_adapters()
                .remove(0);
            let surface = self.surface.as_mut().unwrap();
            let (device, queue_group) = adapter
                .open_with::<_, Graphics>(1, |family| surface.supports_queue_family(family))
                .unwrap();
            self.adapter = Some(adapter);
            (device, queue_group)
        };
        let physical_device = &self.adapter.as_mut().unwrap().physical_device;
        let (caps, formats, _) = self
            .surface
            .as_mut()
            .unwrap()
            .compatibility(physical_device);
        self.caps = Some(caps);

        self.memory_types = physical_device.memory_properties().memory_types;

        self.surface_color_format = {
            // Pick color format
            match formats {
                Some(choices) => Some(
                    choices
                        .into_iter()
                        .find(|format| format.base_format().1 == ChannelType::Srgb)
                        .unwrap(),
                ),
                None => Some(Format::Rgba8Srgb),
            }
        };

        self.device = Some(device);
        self.queue_group = Some(queue_group);
    }

    fn build_window_and_events_loop(&mut self) {
        self.events_loop = Some(winit::EventsLoop::new());
        self.window = Some(
            winit::WindowBuilder::new()
                .with_title(self.title)
                .with_dimensions(self.dimensions.into())
                .build(self.events_loop.as_ref().unwrap())
                .unwrap(),
        );
    }

    fn build_command_pool(&mut self) {
        let max_buffers = 16;
        self.command_pool = Some(self.device.as_mut().unwrap().create_command_pool_typed(
            &self.queue_group.as_mut().unwrap(),
            CommandPoolCreateFlags::empty(),
            max_buffers,
        ));
    }

    fn build_render_pass(&mut self) {
        let render_pass = {
            let color_attachment = Attachment {
                format: Some(self.surface_color_format.unwrap().clone()),
                samples: 1,
                ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
                stencil_ops: AttachmentOps::DONT_CARE,
                layouts: Layout::Undefined..Layout::Present,
            };

            let depth_attachment = Attachment {
                format: Some(self.depth_format),
                samples: 1,
                ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::DontCare),
                stencil_ops: AttachmentOps::DONT_CARE,
                layouts: Layout::Undefined..Layout::DepthStencilAttachmentOptimal,
            };

            // Single subpass for now
            let subpass = SubpassDesc {
                colors: &[(0, Layout::ColorAttachmentOptimal)],
                depth_stencil: Some(&(1, Layout::DepthStencilAttachmentOptimal)),
                inputs: &[],
                resolves: &[],
                preserves: &[],
            };

            let dependency = SubpassDependency {
                passes: SubpassRef::External..SubpassRef::Pass(0),
                stages: PipelineStage::COLOR_ATTACHMENT_OUTPUT
                    ..PipelineStage::COLOR_ATTACHMENT_OUTPUT,
                accesses: Access::empty()
                    ..(Access::COLOR_ATTACHMENT_READ | Access::COLOR_ATTACHMENT_WRITE),
            };

            self.device.as_mut().unwrap().create_render_pass(
                &[color_attachment, depth_attachment],
                &[subpass],
                &[dependency],
            )
        };
        self.render_pass = Some(render_pass);
    }

    fn finish(mut self) -> RenderContext<back::Backend> {
        let set_layout = self
            .device
            .as_ref()
            .unwrap()
            .create_descriptor_set_layout(self.pipeline_layout, &[]);

        let pipeline_layout = self
            .device
            .as_ref()
            .unwrap()
            .create_pipeline_layout(vec![&set_layout], &[]);

        let vertex_shader_mod =
            create_shader::<back::Backend>(self.vertex_shader, self.device.as_ref().unwrap());
        let fragment_shader_mod =
            create_shader::<back::Backend>(self.fragment_shader, self.device.as_ref().unwrap());

        let pipeline = {
            let vs_entry = EntryPoint::<back::Backend> {
                entry: "main",
                module: &vertex_shader_mod,
                specialization: Specialization {
                    constants: &[],
                    data: &[],
                },
            };

            let fs_entry = EntryPoint::<back::Backend> {
                entry: "main",
                module: &fragment_shader_mod,
                specialization: Specialization {
                    constants: &[],
                    data: &[],
                },
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
                main_pass: self.render_pass.as_ref().unwrap(),
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

            // Vertex buffers
            if let Some(vertex_desc) = self.vertex_desc {
                pipeline_desc.vertex_buffers.push(vertex_desc);

                for attr_desc in self.attr_descs.clone() {
                    pipeline_desc.attributes.push(attr_desc);
                }
            }

            pipeline_desc.depth_stencil = DepthStencilDesc {
                depth: DepthTest::On {
                    fun: Comparison::Less,
                    write: true,
                },
                depth_bounds: false,
                stencil: StencilTest::default(),
            };

            self.device
                .as_ref()
                .unwrap()
                .create_graphics_pipeline(&pipeline_desc, None)
                .unwrap()
        };

        self.device
            .as_ref()
            .unwrap()
            .destroy_shader_module(vertex_shader_mod);
        self.device
            .as_ref()
            .unwrap()
            .destroy_shader_module(fragment_shader_mod);

        // Swapchain
        let swapchain_config = SwapchainConfig::from_caps(
            self.caps.as_ref().unwrap(),
            self.surface_color_format.unwrap(),
        );
        let extent = swapchain_config.extent.to_extent();

        let surface_color_format = self.surface_color_format.unwrap();
        let (swapchain, backbuffer) = self.device.as_ref().unwrap().create_swapchain(
            self.surface.as_mut().unwrap(),
            swapchain_config,
            None,
        );
        let depth_format = self.depth_format;

        // Depth testing
        let (depth_image, depth_image_memory, depth_image_view) = {
            let ty = image::Kind::D2(extent.width as Size, extent.height as Size, 1, 1);

            let unbound_depth_image = self
                .device
                .as_ref()
                .unwrap()
                .create_image(
                    ty,
                    1,
                    depth_format,
                    image::Tiling::Optimal,
                    image::Usage::DEPTH_STENCIL_ATTACHMENT,
                    image::ViewCapabilities::empty(),
                )
                .unwrap();

            let image_reqs = self
                .device
                .as_ref()
                .unwrap()
                .get_image_requirements(&unbound_depth_image);

            let device_type = self
                .memory_types
                .iter()
                .enumerate()
                .position(|(id, memory_type)| {
                    image_reqs.type_mask & (1 << id) != 0
                        && memory_type.properties.contains(Properties::DEVICE_LOCAL)
                })
                .unwrap()
                .into();

            let depth_image_memory = self
                .device
                .as_ref()
                .unwrap()
                .allocate_memory(device_type, image_reqs.size)
                .unwrap();

            let depth_image = self
                .device
                .as_ref()
                .unwrap()
                .bind_image_memory(&depth_image_memory, 0, unbound_depth_image)
                .unwrap();

            let depth_image_view = self
                .device
                .as_ref()
                .unwrap()
                .create_image_view(
                    &depth_image,
                    image::ViewKind::D2,
                    depth_format,
                    Swizzle::NO,
                    image::SubresourceRange {
                        aspects: Aspects::DEPTH | Aspects::STENCIL,
                        levels: 0..1,
                        layers: 0..1,
                    },
                )
                .unwrap();

            (depth_image, depth_image_memory, depth_image_view)
        };

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
                        self.device
                            .as_ref()
                            .unwrap()
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
                        self.device
                            .as_ref()
                            .unwrap()
                            .create_framebuffer(
                                self.render_pass.as_ref().unwrap(),
                                vec![image_view, &depth_image_view],
                                extent,
                            )
                            .unwrap()
                    })
                    .collect();

                (image_views, _frame_buffers)
            }

            // For OpenGL backend
            Backbuffer::Framebuffer(fbo) => (vec![], vec![fbo]),
        };

        let frame_semaphore = self.device.as_ref().unwrap().create_semaphore();
        let frame_fence = self.device.as_ref().unwrap().create_fence(false);

        RenderContext {
            instance: self.instance.unwrap(),
            device: self.device.unwrap(),
            events_loop: self.events_loop.unwrap(),
            window: self.window.unwrap(),
            surface: self.surface.unwrap(),
            queue_group: self.queue_group.unwrap(),
            command_pool: self.command_pool.unwrap(),
            render_pass: self.render_pass.unwrap(),
            pipeline,
            pipeline_layout,
            swapchain,
            image_views,
            frame_buffers,
            frame_semaphore,
            frame_fence,
            extent,
            models: Vec::new(),
            memory_types: self.memory_types,
            set_layout,
            depth_image,
            depth_image_view,
            depth_image_memory,
        }
    }
}

#[inline(always)]
fn create_shader<B: Backend>(raw: &[u8], device: &B::Device) -> B::ShaderModule {
    device.create_shader_module(raw).unwrap()
}
