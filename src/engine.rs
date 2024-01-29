use std::sync::Arc;

use crate::{ui::UIBackend, EngineContext};
use vulkano::{
    command_buffer::SecondaryAutoCommandBuffer,
    format::{ClearValue, Format},
    image::SampleCount,
    pipeline::graphics::viewport::Viewport,
    Version,
};
use winit::event::WindowEvent;

/// An implementation of the engine stages, contains input processing and render information
#[allow(unused)]
pub trait Engine {
    type UI: UIBackend;

    /// Called right after the vulkano context is created
    fn init(context: &EngineContext) -> Self;

    /// Called after initialization
    fn start(&mut self, context: &EngineContext) {}

    /// Called before a close is requested
    fn stop(&mut self, context: &EngineContext) {}

    /// Called any time a winit event occurs within the viewport
    fn on_winit_event(&mut self, event: &WindowEvent, context: &EngineContext) {}

    /// All the UI code goes here
    fn immediate_ui(&mut self, ctx: &<Self::UI as UIBackend>::Context, context: &EngineContext) {}

    /// Viewport rendering code goes here
    fn draw_viewport(
        &mut self,
        viewport: Viewport,
        context: &EngineContext,
    ) -> Option<Arc<SecondaryAutoCommandBuffer>> {
        None
    }

    const FORMAT: Format = Format::B8G8R8A8_SRGB;
    const SAMPLES: SampleCount = SampleCount::Sample1;
    const VERSION: Option<Version> = None;

    fn background_color(&self) -> ClearValue {
        [0.0, 0.0, 0.0, 1.0].into()
    }
}
