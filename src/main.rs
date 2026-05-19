mod app;
mod grid;
use app::MyApp;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Game of Life",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::new(_cc)))),
    )
}
