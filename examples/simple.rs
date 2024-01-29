use std::sync::Arc;

use hatchery::{EguiBackend, Engine, EngineContext, EngineLauncher, UIBackend};
use vulkano::{command_buffer::SecondaryAutoCommandBuffer, pipeline::graphics::viewport::Viewport};

pub struct SimpleEngine {}

impl Engine for SimpleEngine {
    type UI = EguiBackend;

    fn init(_: &hatchery::EngineContext) -> Self {
        Self {}
    }

    fn immediate_ui(
        &mut self,
        ctx: &<Self::UI as UIBackend>::Context,
        _: &hatchery::EngineContext,
    ) {
        egui::Window::new("Hello, World!").show(ctx, |ui| {
            ui.label("This is a window!");
        });
    }

    fn draw_viewport(
        &mut self,
        _: Viewport,
        _: &EngineContext,
    ) -> Option<Arc<SecondaryAutoCommandBuffer>> {
        None
    }
}

fn main() {
    EngineLauncher::<SimpleEngine>::run(Default::default());
}
