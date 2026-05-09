use eframe::egui;
mod grid;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Game of Life",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    count: i32,
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Game of Life");

        if ui.button("Click me").clicked() {
            self.count += 1;
        }

        ui.label(format!("Count: {}", self.count));
    }
}
