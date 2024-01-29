use std::sync::Arc;

use crate::ui::*;
use vulkano::image::SampleCount;

pub struct EguiBackend {
    renderer: egui_winit_vulkano::Gui,
}

impl UIBackend for EguiBackend {
    type Context = egui::Context;

    fn new(
        target: &EventLoopWindowTarget<()>,
        surface: Arc<Surface>,
        graphics_queue: Arc<Queue>,
        subpass: Subpass,
        format: Format,
    ) -> Self {
        Self {
            renderer: egui_winit_vulkano::Gui::new_with_subpass(
                target,
                surface,
                graphics_queue,
                subpass,
                format,
                egui_winit_vulkano::GuiConfig {
                    // TODO: figure out where to move this to
                    allow_srgb_render_target: true,
                    is_overlay: false,
                    samples: SampleCount::Sample4,
                },
            ),
        }
    }

    fn context(&self) -> Self::Context {
        self.renderer.context()
    }

    fn viewport(&self) -> Option<AvailableRectangle> {
        let context = self.context();

        let origin = context.available_rect().left_top();
        let dimensions = context.available_rect().right_bottom() - origin;

        Some(AvailableRectangle {
            origin: [origin.x, origin.y],
            dimensions: [dimensions.x, dimensions.y],
        })
    }

    fn draw(&mut self, dimensions: [f64; 2]) -> Arc<SecondaryAutoCommandBuffer> {
        self.renderer
            .draw_on_subpass_image([dimensions[0] as u32, dimensions[1] as u32])
    }

    fn immediate_ui(&mut self, ui: impl FnOnce(&mut Self::Context)) {
        self.renderer.immediate_ui(|context| {
            ui(&mut context.context());
        });
    }

    fn update(&mut self, event: &WindowEvent) -> bool {
        self.renderer.update(event)
    }
}
