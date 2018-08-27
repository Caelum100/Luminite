use self::context::RenderContext;
use self::factory::RenderBuilder;
use super::back;
use super::winit;
use super::*;
use gfx_hal::{
    buffer::{IndexBufferView, Usage},
    command::{ClearColor, ClearValue, Primary, RenderPassInlineEncoder},
    format::{Aspects, ChannelType, Format, Swizzle},
    image::{Access, Extent, Layout, SubresourceRange, ViewKind},
    memory::{Barrier, Dependencies, Properties},
    pass::{
        Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDependency,
        SubpassDesc, SubpassRef,
    },
    pool::{CommandPool, CommandPoolCreateFlags},
    pso::{
        AttributeDesc, Descriptor, DescriptorRangeDesc, DescriptorSetLayoutBinding,
        DescriptorSetWrite, DescriptorType, Element, ShaderStageFlags, VertexBufferDesc,
    },
    pso::{
        BlendState, ColorBlendDesc, ColorMask, EntryPoint, GraphicsPipelineDesc, GraphicsShaderSet,
        PipelineStage, Rasterizer, Rect, Viewport,
    },
    Backbuffer, Backend, DescriptorPool, Device, FrameSync, Graphics, Instance, MemoryType,
    PhysicalDevice, Primitive, QueueGroup, Submission, Surface, SwapImageIndex, Swapchain,
    SwapchainConfig,
};

use gfx_hal::IndexType;
use std::borrow::Borrow;

pub mod asset_load;
pub mod buffer_util;
pub mod context;
pub mod factory;

/// A three-dimensional vertex
/// with a color.
#[derive(Clone, Copy)]
pub struct Vertex {
    pub a_position: Vec3,
    pub a_color: Vec3,
}

impl Vertex {
    /// Produces a vertex with the specified
    /// positions and a randomized color.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vertex {
            a_position: vec3(x, y, z),
            a_color: vec3(rand::random(), rand::random(), rand::random()),
        }
    }
}

/// Uniform
#[derive(Clone, Copy)]
struct MatrixBlock {
    /// The full MVP matrix
    matrix: Mat4,
}

pub fn create_context() -> RenderContext<back::Backend> {
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

    let color_attr = AttributeDesc {
        location: 1,
        binding: 0,
        element: Element {
            format: Format::Rgb32Float,
            offset: 12,
        },
    };

    let builder = RenderBuilder::new()
        .with_title("Luminite")
        .with_vertex_shader(include_bytes!("../../assets/shaders/model.vert.spv"))
        .with_fragment_shader(include_bytes!("../../assets/shaders/model.frag.spv"))
        .with_pipeline(&pipeline_layout)
        .with_vertex_attr(vertex_desc, vec![position_attr, color_attr]);

    let mut ctx = builder.build();
    asset_load::upload_models(&mut ctx);
    ctx
}

pub fn render(ctx: &mut RenderContext<back::Backend>, world: &World) {
    let device = &ctx.device;
    let image_views = &ctx.image_views;
    let frame_buffers = &ctx.frame_buffers;
    let (frame_fence, frame_semaphore) = (&ctx.frame_fence, &ctx.frame_semaphore);

    device.reset_fence(&frame_fence);
    ctx.command_pool.reset();

    let frame_index: SwapImageIndex = ctx
        .swapchain
        .acquire_image(0, FrameSync::Semaphore(frame_semaphore))
        .unwrap();

    let finished_command_buffer = {
        let mut command_buffer = ctx.command_pool.acquire_command_buffer(false);

        let viewport = viewport(&ctx.extent);
        command_buffer.set_viewports(0, &[viewport.clone()]);
        command_buffer.set_scissors(0, &[viewport.rect]);

        command_buffer.bind_graphics_pipeline(&ctx.pipeline);
        command_buffer.bind_vertex_buffers(0, vec![(&ctx.models[0].vertices.buffer, 0)]);
        command_buffer.bind_graphics_descriptor_sets(&ctx.pipeline_layout, 0, vec![&ctx.desc_set], &[]);

        let index_buffer_view = IndexBufferView {
            buffer: &ctx.models[0].indices.buffer,
            offset: 0,
            index_type: IndexType::U32,
        };
        command_buffer.bind_index_buffer(index_buffer_view);
        {
            let mut encoder = command_buffer.begin_render_pass_inline(
                &ctx.render_pass,
                &frame_buffers[frame_index as usize],
                viewport.rect,
                &[ClearValue::Color(ClearColor::Float([0.0, 0.0, 0.0, 1.0]))],
            );

            // Draw each object in the world
            // TODO distance checks, instanced rendering
            for object in world.get_objs().values() {
                render_obj(
                    object,
                    &mut encoder,
                    &ctx.device,
                    &ctx.memory_types,
                    &ctx.models,
                    &mut ctx.uniform_buffer,
                    &ctx.pipeline,
                    &ctx.pipeline_layout,
                    &ctx.desc_set,
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

fn render_obj<B: Backend>(
    object: &world::Object,
    encoder: &mut RenderPassInlineEncoder<B, Primary>,
    device: &B::Device,
    memory_types: &Vec<MemoryType>,
    models: &Vec<context::ModelBuffer<B>>,
    uniform_buffer: &mut context::BufferMem<B>,
    pipeline: &B::GraphicsPipeline,
    pipeline_layout: &B::PipelineLayout,
    desc_set: &B::DescriptorSet,
) {
    let model_buffer = &models[object.model_index];

    let matrix = mvp_matrix(object);

    buffer_util::fill_buffer::<B, MatrixBlock>(
        device,
        &mut uniform_buffer.memory,
        &[MatrixBlock { matrix }],
    );

    println!("{}", model_buffer.indices.element_count);

    encoder.draw(0..(model_buffer.vertices.element_count as u32), 0..1);
}

/// Produces a model-view-projection matrix
/// for the specified object.
fn mvp_matrix(object: &world::Object) -> Mat4 {
    use glm::ext::*;
    let translation = translate(&num::one(), object.location.to_vec());
    // TODO rotation
    let rotation: Mat4 = num::one();
    let scale: Mat4 = num::one();
    let model = translation * rotation * scale;

    // TODO moving camera
    let view = look_at(
        vec3(4.0, 3.0, 3.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
    );

    // TODO view distance, custom aspect ratio
    let projection = perspective(45.0f32, 4.0 / 3.0, 0.1, 100.0);
    model * view * projection
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
