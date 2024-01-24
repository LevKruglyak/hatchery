use crate::ui::UIBackend;

/// An implementation of the engine stages, contains input processing and render information
pub trait HatcheryEngine {
    type UI: UIBackend;

    // /// Called right after the vulkano context is created
    // fn init(context: &mut EngineContext<Self::Gui>) -> Self;
    //
    // /// Called after initialization
    // fn start(&mut self, api: &mut EngineApi) {}
    //
    // /// Called before a close is requested
    // fn stop(&mut self, api: &mut EngineApi) {}
    //
    // /// Called any time a winit event occurs within the viewport
    // fn on_winit_event(&mut self, event: &WindowEvent, api: &mut EngineApi) {}

    /// All the ui code goes here
    fn immediate(
        &mut self,
        // context: &mut <<Self as HatcheryEngine>::UI as UIBackend>::Context,
        context: &mut Self::UI, // api: &mut EngineApi,
    ) {
    }

    // /// Viewport rendering code goes here
    // fn render(
    //     &mut self,
    //     command_buffer: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    //     subpass: Subpass,
    //     viewport: Viewport,
    //     api: &mut EngineApi,
    // ) {
    // }
}
