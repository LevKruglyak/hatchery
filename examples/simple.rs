use hatchery::{EguiBackend, Engine, EngineLauncher, UIBackend};

pub struct SimpleEngine;

impl Engine for SimpleEngine {
    type UI = EguiBackend;

    fn init(_: &hatchery::EngineContext) -> Self {
        Self
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
}

fn main() {
    EngineLauncher::<SimpleEngine>::run(Default::default());
}
