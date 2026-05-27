mod app;
mod grid;
use app::MyApp;
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 720.0]), // Set your custom default resolution
        ..Default::default()
    };

    eframe::run_native(
        "Game of Life",
        // eframe::NativeOptions::default(),
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(_cc)))),
    )
}
