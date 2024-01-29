use std::sync::Arc;
use vulkano::{
    command_buffer::{
        allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
        AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo,
        SubpassContents, SubpassEndInfo,
    },
    device::{Device, Queue},
    format::Format,
    image::SampleCount,
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    sync::GpuFuture,
};
use vulkano_util::{context::VulkanoContext, window::VulkanoWindows};

use crate::{ui::UIBackend, Engine};

pub struct FinalRenderPass {
    graphics_queue: Arc<Queue>,
    render_pass: Arc<RenderPass>,
    allocator: StandardCommandBufferAllocator,
}

impl FinalRenderPass {
    pub fn new(context: &VulkanoContext, format: Format, samples: SampleCount) -> Self {
        let render_pass = Self::create_render_pass(context.device().clone(), format, samples);

        let allocator = StandardCommandBufferAllocator::new(
            context.device().clone(),
            StandardCommandBufferAllocatorCreateInfo::default(),
        );

        Self {
            graphics_queue: context.graphics_queue().clone(),
            render_pass,
            allocator,
        }
    }

    pub fn viewport_subpass(&self) -> Subpass {
        Subpass::from(self.render_pass.clone(), 0).unwrap()
    }

    pub fn ui_subpass(&self) -> Subpass {
        Subpass::from(self.render_pass.clone(), 1).unwrap()
    }

    fn create_render_pass(
        device: Arc<Device>,
        format: Format,
        samples: SampleCount,
    ) -> Arc<RenderPass> {
        vulkano::ordered_passes_renderpass!(
            device,
            attachments: {
                color: {
                    format: format,
                    samples: samples,
                    load_op: Clear,
                    store_op: Store,
                }
            },
            passes: [
                { color: [color], depth_stencil: {}, input: [] }, // Viewport subpass
                { color: [color], depth_stencil: {}, input: [] }  // UI subpass
            ]
        )
        .expect("Error creating render pass!")
    }

    pub fn draw<E>(&self, ui: &mut E::UI, engine: &mut E, windows: &mut VulkanoWindows)
    where
        E: Engine + 'static,
    {
        // Get target information from windows
        let scale_factor = windows.get_primary_window().unwrap().scale_factor();
        let renderer = windows.get_primary_renderer_mut().unwrap();

        let before_future = renderer.acquire().unwrap();
        let target = renderer.swapchain_image_view();

        // Get dimensions
        let image_dimensions = target.image().extent();

        // Create framebuffer (must be in same order as render pass description in `new`
        let framebuffer = Framebuffer::new(
            self.render_pass.clone(),
            FramebufferCreateInfo {
                attachments: vec![target],
                ..Default::default()
            },
        )
        .unwrap();

        // Create primary command buffer builder
        let mut primary_builder = AutoCommandBufferBuilder::primary(
            &self.allocator,
            self.graphics_queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        // Begin render pass
        primary_builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some(engine.background_color())],
                    ..RenderPassBeginInfo::framebuffer(framebuffer)
                },
                SubpassBeginInfo {
                    contents: SubpassContents::SecondaryCommandBuffers,
                    ..Default::default()
                },
            )
            .unwrap();

        // Get region for us to render to
        let viewport = ui
            .viewport()
            .map(|v| v.to_viewport(scale_factor))
            .unwrap_or_default();

        if let Some(cb) = engine.draw_viewport(viewport) {
            primary_builder.execute_commands(cb).unwrap();
        }

        // Render gui
        primary_builder
            .next_subpass(
                SubpassEndInfo {
                    ..Default::default()
                },
                SubpassBeginInfo {
                    contents: SubpassContents::SecondaryCommandBuffers,
                    ..Default::default()
                },
            )
            .unwrap();

        let cb = ui.draw([
            image_dimensions[0] as f64 * scale_factor,
            image_dimensions[1] as f64 * scale_factor,
        ]);
        primary_builder.execute_commands(cb).unwrap();

        // End render pass
        let _ = primary_builder
            .end_render_pass(SubpassEndInfo {
                ..Default::default()
            })
            .unwrap();

        // Build command buffer
        let command_buffer = primary_builder.build().unwrap();

        // Execute primary command buffer
        let after_future = before_future
            .then_execute(self.graphics_queue.clone(), command_buffer)
            .unwrap();

        renderer.present(after_future.boxed(), true);
    }
}
