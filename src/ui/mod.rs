pub use std::sync::Arc;
pub use vulkano::{
    command_buffer::SecondaryAutoCommandBuffer, device::Queue, format::Format,
    pipeline::graphics::viewport::Viewport, render_pass::Subpass, swapchain::Surface,
};
pub use winit::{event::WindowEvent, event_loop::EventLoopWindowTarget};

#[cfg(feature = "egui")]
pub mod egui_backend;

/// Represents an arbitrary immediate mode UI implementation such as imgui-rs or egui
pub trait UIBackend {
    /// Immediate mode UI state
    type Context;

    fn context(&self) -> Self::Context;

    fn new(
        target: &EventLoopWindowTarget<()>,
        surface: Arc<Surface>,
        graphics_queue: Arc<Queue>,
        subpass: Subpass,
        format: Format,
    ) -> Self;

    fn draw(&mut self, dimensions: [f64; 2]) -> Arc<SecondaryAutoCommandBuffer>;

    /// Rebuild the UI
    fn immediate_ui(&mut self, ui: impl FnOnce(&mut Self::Context));

    // Pass winit event to the UI
    fn update(&mut self, event: &WindowEvent) -> bool;

    // Return the leftover area, or the whole screen if not supported
    fn viewport(&self) -> Option<AvailableRectangle> {
        None
    }
}

/// Dimensions of the area not covered by panels (in logical pixels)
pub struct AvailableRectangle {
    origin: [f32; 2],
    dimensions: [f32; 2],
}

impl AvailableRectangle {
    pub fn to_viewport(&self, scale_factor: f64) -> Viewport {
        let scale_factor = scale_factor as f32;
        Viewport {
            offset: [self.origin[0] * scale_factor, self.origin[1] * scale_factor],
            extent: [
                self.dimensions[0] * scale_factor,
                self.dimensions[1] * scale_factor,
            ],
            depth_range: 0.0..=1.0,
        }
    }
}

impl Default for AvailableRectangle {
    fn default() -> Self {
        Self {
            dimensions: [1.0; 2],
            origin: [0.0; 2],
        }
    }
}
