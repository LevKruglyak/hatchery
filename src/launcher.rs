use std::sync::Arc;

use crate::{render_pass::FinalRenderPass, ui::UIBackend, Engine};
use vulkano::{
    device::{DeviceExtensions, Features},
    instance::{InstanceCreateFlags, InstanceCreateInfo, InstanceExtensions},
    render_pass::Subpass,
    swapchain::Surface,
};
use vulkano_util::{
    context::{VulkanoConfig, VulkanoContext},
    window::{VulkanoWindows, WindowDescriptor},
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    event_loop::EventLoop,
};

/// Display options for the winit window
#[derive(Debug, Clone, Copy)]
pub struct WindowOptions {
    pub title: &'static str,
    pub dimensions: [u32; 2],
}

/// Overall options for the engine,
#[derive(Default, Clone, Copy)]
pub struct EngineOptions {
    pub window: WindowOptions,
    pub instance_extensions: InstanceExtensions,
    pub device_extensions: DeviceExtensions,
    pub features: Features,
}

/// Wrapper struct for engine methods
pub struct EngineLauncher<E> {
    _pd: std::marker::PhantomData<E>,
}

/// Contains input system, performance, some graphics objects
pub struct EngineContext {
    pub vulkan: VulkanoContext,
    pub windows: VulkanoWindows,
    pub surface: Arc<Surface>,
    pub viewport_subpass: Subpass,
}

impl<E> EngineLauncher<E>
where
    E: Engine + 'static,
{
    /// Start the engine loop, open the window, initialize all of the graphics contexts
    pub fn run(mut options: EngineOptions) {
        // Ensure khr_swapchain is enabled
        options.device_extensions.khr_swapchain = true;

        // Create Vulkano context
        let vulkano_config = VulkanoConfig {
            instance_create_info: InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                max_api_version: E::VERSION,
                enabled_extensions: options.instance_extensions,
                ..InstanceCreateInfo::default()
            },
            device_features: options.features,
            device_extensions: options.device_extensions,
            print_device_name: true,
            ..VulkanoConfig::default()
        };
        let vulkan = VulkanoContext::new(vulkano_config);

        // Create window and surface
        let event_loop = EventLoop::new();
        let mut windows = VulkanoWindows::default();
        let _ = windows.create_window(
            &event_loop,
            &vulkan,
            &WindowDescriptor {
                width: options.window.dimensions[0] as f32,
                height: options.window.dimensions[1] as f32,
                title: options.window.title.to_string(),
                ..Default::default()
            },
            |swapchain_create_info| swapchain_create_info.image_format = E::FORMAT,
        );
        let surface = windows.get_primary_renderer().unwrap().surface();

        // Create render pass
        let render_pass = FinalRenderPass::new(&vulkan, E::FORMAT, E::SAMPLES);

        // Pack everything into EngineContext struct
        let mut context = EngineContext {
            vulkan,
            windows,
            surface: surface.clone(),
            viewport_subpass: render_pass.viewport_subpass(),
        };

        // Create UI
        let mut ui = E::UI::new(
            &event_loop,
            surface,
            context.vulkan.graphics_queue().clone(),
            render_pass.ui_subpass(),
            E::FORMAT,
        );

        // Create engine
        let mut engine = E::init(&context);

        engine.start(&context);
        event_loop.run(move |event, _, control_flow| match event {
            Event::WindowEvent { event, .. } => {
                if !ui.update(&event) {
                    engine.on_winit_event(&event, &mut context);
                }

                // Handle resize and exit events
                match event {
                    WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. } => {
                        context.windows.get_primary_renderer_mut().unwrap().resize();
                    }
                    WindowEvent::CloseRequested => {
                        engine.stop(&mut context);
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => (),
                }
            }
            Event::RedrawRequested(_) => {
                // Rebuild ui
                ui.immediate_ui(|ctx| {
                    engine.immediate_ui(ctx, &mut context);
                });

                render_pass.draw(&mut ui, &mut engine, &mut context.windows);
            }
            Event::MainEventsCleared => {
                context
                    .windows
                    .get_primary_window()
                    .unwrap()
                    .request_redraw();
            }
            _ => {}
        });
    }
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            title: "Hatchery App",
            dimensions: [300, 300],
        }
    }
}
