use std::sync::Arc;

use hatchery::*;

use egui_demo_lib::DemoWindows;

#[derive(Default)]
pub struct EguiDemoEngine {
    windows: DemoWindows,
}

impl Engine for EguiDemoEngine {
    type UI = EguiBackend;

    fn init(_: &hatchery::EngineContext) -> Self {
        Default::default()
    }

    fn immediate_ui(
        &mut self,
        ctx: &<Self::UI as UIBackend>::Context,
        _: &hatchery::EngineContext,
    ) {
        self.windows.ui(ctx);
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
    EngineLauncher::<EguiDemoEngine>::run(Default::default());
}
