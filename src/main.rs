#![windows_subsystem = "windows"]

mod app;
mod grid;
mod widgets;

use app::MyApp;
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 720.0]), // Set your custom default resolution
        ..Default::default()
    };

    eframe::run_native(
        concat!("Game of Life v", env!("CARGO_PKG_VERSION")),
        options,
        Box::new(|_cc| {
            // Apply the dark theme on startup
            _cc.egui_ctx.set_visuals(egui::Visuals::dark());

            Ok(Box::new(MyApp::new(_cc)))
        }),
    )
}
