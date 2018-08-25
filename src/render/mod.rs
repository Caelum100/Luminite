use gfx_hal::{
    Backbuffer,
    Backend,
    command::{ClearColor, ClearValue},
    Device,
    format::{Aspects, ChannelType, Format, Swizzle},
    FrameSync,
    Graphics,
    image::{Access, Layout, SubresourceRange, ViewKind, Extent}, Instance, pass::{
        Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDependency,
        SubpassDesc, SubpassRef,
    }, pool::{CommandPool, CommandPoolCreateFlags}, Primitive, pso::{
        BlendState, ColorBlendDesc, ColorMask, EntryPoint, GraphicsPipelineDesc, GraphicsShaderSet,
        PipelineStage, Rasterizer, Rect, Viewport,
    }, pso::{DescriptorSetLayoutBinding, DescriptorType, ShaderStageFlags},
    Surface, Swapchain, SwapchainConfig, SwapImageIndex, QueueGroup, Submission,
};
use self::context::RenderContext;
use self::factory::RenderBuilder;
use super::*;
use super::back;
use super::winit;

use std::borrow::Borrow;

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
    let (frame_fence, frame_semaphore) =
        (&mut ctx.frame_fence, &mut ctx.frame_semaphore);
    let command_pool = &mut ctx.command_pool;

    device.reset_fence(&frame_fence);
    command_pool.reset();

    let frame_index: SwapImageIndex = swapchain
        .acquire_image(FrameSync::Semaphore(frame_semaphore))
        .unwrap();

    let finished_command_buffer = {
        let mut command_buffer =
            command_pool.acquire_command_buffer(false);

        let viewport = viewport(&ctx.extent);
        command_buffer.set_viewports(0, &[viewport.clone()]);
        command_buffer.set_scissors(0, &[viewport.rect]);

        command_buffer.bind_graphics_pipeline(&ctx.pipeline);

        {
            let mut encoder =
                command_buffer.begin_render_pass_inline(
                    &ctx.render_pass,
                    &frame_buffers[frame_index as usize],
                    viewport.rect,
                    &[ClearValue::Color(ClearColor::Float([0.0, 0.0, 0.0, 1.0]))],
                );
        }

        command_buffer.finish()
    };

    let submission = Submission::new()
        .wait_on(&[(frame_semaphore.borrow(), PipelineStage::BOTTOM_OF_PIPE)])
        .submit(vec!(finished_command_buffer));

    ctx.queue_group.queues[0].submit(submission, Some(&frame_fence));

    device.wait_for_fence(&frame_fence, !0);

    swapchain.present(
        &mut ctx.queue_group.queues[0],
        frame_index,
        &[])
        .unwrap();
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