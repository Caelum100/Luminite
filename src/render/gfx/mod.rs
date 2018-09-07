//! gfx-rs render backend
//! for Luminite

use self::context::RenderContext;
use self::factory::RenderBuilder;
use super::back;
use super::winit;
use super::*;
use gfx_hal::{
    buffer::{IndexBufferView, Usage},
    command::{ClearColor, ClearDepthStencil, ClearValue, Primary, RenderPassInlineEncoder},
    format::{Aspects, ChannelType, Format, Swizzle},
    image,
    image::{Access, Extent, Layout, Size, SubresourceRange, ViewKind},
    memory::Properties,
    pass::{
        Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDependency,
        SubpassDesc, SubpassRef,
    },
    pool::{CommandPool, CommandPoolCreateFlags},
    pso::{
        AttributeDesc, Comparison, DepthStencilDesc, DepthTest, Descriptor, DescriptorRangeDesc,
        DescriptorSetLayoutBinding, DescriptorSetWrite, DescriptorType, Element, ShaderStageFlags,
        StencilTest, VertexBufferDesc,
    },
    pso::{
        BlendState, ColorBlendDesc, ColorMask, EntryPoint, GraphicsPipelineDesc, GraphicsShaderSet,
        PipelineStage, Rasterizer, Rect, Viewport,
    },
    Backbuffer, Backend, DescriptorPool, Device, FrameSync, Graphics, MemoryType, Primitive,
    QueueGroup, Submission, SwapImageIndex, Swapchain, SwapchainConfig,
};

use self::asset_load::upload_model;
pub use self::context::{BufferMem, UniformBuffer};
use gfx_hal::IndexType;
use std::borrow::Borrow;

pub mod asset_load;
pub mod buffer_util;
pub mod context;
pub mod factory;

pub enum _RenderBackend {}
impl RenderBackend for _RenderBackend {
    type ObjectRender = ObjectRender<back::Backend>;
    type RenderContext = RenderContext<back::Backend>;

    fn upload_model(ctx: &mut Self::RenderContext, models: Vec<tobj::Model>) {
        upload_model(ctx, models);
    }

    fn create_context(title: &str, dimensions: (u32, u32)) -> Self::RenderContext {
        _create_context(title, dimensions)
    }

    fn create_obj_render(
        model_index: usize,
        shader_index: usize,
        render: &mut <Self as RenderBackend>::RenderContext,
    ) -> Self::ObjectRender {
        _create_obj_render(model_index, shader_index, render)
    }
}

/// Uniform
#[derive(Clone, Copy)]
// Although the compiler seems
// to think the fields here are never
// used, they are.
#[allow(dead_code)]
struct MatrixBlock {
    /// The full MVP matrix
    matrix: Mat4,
    modelview: Mat4,
}

/// Render data associated with an object
pub struct ObjectRender<B: Backend> {
    pub model_index: usize,
    pub uniform: UniformBuffer<B>,
    pub shader_index: usize,
}

pub fn _create_context(title: &str, dimensions: (u32, u32)) -> RenderContext<back::Backend> {
    let pipeline_layout = [DescriptorSetLayoutBinding {
        binding: 0,
        ty: DescriptorType::UniformBuffer,
        count: 1,
        stage_flags: ShaderStageFlags::VERTEX,
        immutable_samplers: false,
    }];

    let vertex_desc = VertexBufferDesc {
        binding: 0,
        stride: std::mem::size_of::<Vertex>() as u32,
        rate: 0,
    };

    let position_attr = AttributeDesc {
        location: 0,
        binding: 0,
        element: Element {
            format: Format::Rgb32Float,
            offset: 0,
        },
    };

    let normal_attr = AttributeDesc {
        location: 1,
        binding: 0,
        element: Element {
            format: Format::Rgb32Float,
            offset: 12,
        },
    };

    let builder = RenderBuilder::new()
        .with_title(title)
        .with_vertex_shader(include_bytes!("../../../assets/shaders/model.vert.spv"))
        .with_fragment_shader(include_bytes!("../../../assets/shaders/model.frag.spv"))
        .with_pipeline(&pipeline_layout)
        .with_vertex_attr(vertex_desc, vec![position_attr, normal_attr])
        .with_dimensions(dimensions.0, dimensions.1);

    let mut ctx = builder.build();
    upload_models::<_RenderBackend>(&mut ctx);
    ctx
}

pub fn render(ctx: &mut RenderContext<back::Backend>, world: &mut World<_RenderBackend>) {
    let device = &ctx.device;
    let frame_buffers = &ctx.frame_buffers;
    let (frame_fence, frame_semaphore) = (&ctx.frame_fence, &ctx.frame_semaphore);

    device.reset_fence(&frame_fence);
    ctx.command_pool.reset();

    let frame_index: SwapImageIndex = ctx
        .swapchain
        .acquire_image(!0, FrameSync::Semaphore(frame_semaphore))
        .unwrap();

    let finished_command_buffer = {
        let mut command_buffer = ctx.command_pool.acquire_command_buffer(false);

        let viewport = viewport(&ctx.extent);
        command_buffer.set_viewports(0, &[viewport.clone()]);
        command_buffer.set_scissors(0, &[viewport.rect]);

        {
            let mut encoder = command_buffer.begin_render_pass_inline(
                &ctx.render_pass,
                &frame_buffers[frame_index as usize],
                viewport.rect,
                &[
                    ClearValue::Color(ClearColor::Float([0.0, 0.0, 0.0, 1.0])),
                    ClearValue::DepthStencil(ClearDepthStencil(1.0, 0)),
                ],
            );

            // Draw each object in the world
            // TODO distance checks, instanced rendering
            for object in world.get_objs_mut().values_mut() {
                render_obj(
                    object,
                    &mut encoder,
                    &ctx.device,
                    &ctx.models,
                    &ctx.pipeline,
                    &ctx.pipeline_layout,
                );
            }
        }

        command_buffer.finish()
    };

    let submission = Submission::new()
        .wait_on(&[(frame_semaphore.borrow(), PipelineStage::BOTTOM_OF_PIPE)])
        .submit(vec![finished_command_buffer]);

    ctx.queue_group.queues[0].submit(submission, Some(&frame_fence));

    device.wait_for_fence(&frame_fence, !0);

    ctx.swapchain
        .present(&mut ctx.queue_group.queues[0], frame_index, &[])
        .unwrap();
}

/// Renders the object
/// using its model buffer
/// and uniform
fn render_obj(
    object: &mut world::Object<_RenderBackend>,
    encoder: &mut RenderPassInlineEncoder<back::Backend, Primary>,
    device: &<back::Backend as Backend>::Device,
    models: &Vec<context::ModelBuffer<back::Backend>>,
    pipeline: &<back::Backend as Backend>::GraphicsPipeline,
    pipeline_layout: &<back::Backend as Backend>::PipelineLayout,
) {
    let model_buffer = &models[object.render.model_index];
    encoder.bind_vertex_buffers(0, vec![(&model_buffer.vertices.buffer, 0)]);
    encoder.bind_graphics_pipeline(pipeline);
    encoder.bind_graphics_descriptor_sets(
        pipeline_layout,
        0,
        vec![&object.render.uniform.desc_set],
        &[],
    );

    let index_buffer_view = IndexBufferView {
        buffer: &model_buffer.indices.buffer,
        offset: 0,
        index_type: IndexType::U32,
    };
    encoder.bind_index_buffer(index_buffer_view);

    let (matrix, modelview) = mvp_matrix(object);

    buffer_util::fill_buffer::<back::Backend, MatrixBlock>(
        device,
        &mut object.render.uniform.buffer.memory,
        &[MatrixBlock { matrix, modelview }],
    );

    encoder.draw_indexed(0..(model_buffer.indices.element_count as u32), 0, 0..1);
}

/// Destroys the RenderContext.
pub fn destroy(ctx: RenderContext<back::Backend>) {
    let device = ctx.device;

    device.destroy_graphics_pipeline(ctx.pipeline);
    device.destroy_pipeline_layout(ctx.pipeline_layout);

    for framebuffer in ctx.frame_buffers {
        device.destroy_framebuffer(framebuffer);
    }

    for image_view in ctx.image_views {
        device.destroy_image_view(image_view);
    }

    device.destroy_render_pass(ctx.render_pass);
    device.destroy_swapchain(ctx.swapchain);

    device.destroy_command_pool(ctx.command_pool.into_raw());
    device.destroy_fence(ctx.frame_fence);
    device.destroy_semaphore(ctx.frame_semaphore);

    // TODO - descriptors
}

fn viewport(extent: &Extent) -> Viewport {
    Viewport {
        rect: Rect {
            x: 0,
            y: 0,
            w: extent.width as i16,
            h: extent.height as i16,
        },
        depth: 0.0..1.0,
    }
}

/// Creates a descriptor set and pool and uniform buffer/memory
/// for the object. The model_index is the index into the RenderContext's
/// model vector and the shader_index is not used yet (GitHub issue #7)
pub fn _create_obj_render<B: Backend>(
    model_index: usize,
    shader_index: usize,
    ctx: &mut RenderContext<B>,
) -> ObjectRender<B> {
    // Allocate uniform buffer and init descriptor set
    let mut desc_pool = ctx.device.create_descriptor_pool(
        1,
        &[DescriptorRangeDesc {
            ty: DescriptorType::UniformBuffer,
            count: 1,
        }],
    );
    let desc_set = desc_pool.allocate_set(&ctx.set_layout).unwrap();

    let (buffer, memory) = buffer_util::empty_buffer::<B, MatrixBlock>(
        &ctx.device,
        &ctx.memory_types,
        Properties::CPU_VISIBLE,
        Usage::UNIFORM,
        1,
    );

    ctx.device.write_descriptor_sets(vec![DescriptorSetWrite {
        set: &desc_set,
        binding: 0,
        array_offset: 0,
        descriptors: Some(Descriptor::Buffer(&buffer, None..None)),
    }]);

    ObjectRender {
        model_index,
        shader_index,
        uniform: UniformBuffer {
            buffer: BufferMem::new(buffer, memory),
            desc_set,
            desc_pool,
        },
    }
}
