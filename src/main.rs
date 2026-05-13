#![allow(unused_variables)]
#![allow(dead_code)]

use std::time::{Duration, Instant};

use eframe::egui::{self, Color32, Pos2, Rect};

mod grid;

const CELL_SIZE: f32 = 10.0;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Game of Life",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
}

struct MyApp {
    grid: grid::Grid,
    gen_count: u64,
    last_tick: Instant,
}

impl MyApp {
    pub fn new() -> Self {
        let mut grid = grid::Grid::default();
        grid.insert(1, 0);
        grid.insert(2, 1);
        grid.insert(0, 2);
        grid.insert(1, 2);
        grid.insert(2, 2);

        Self {
            grid,
            gen_count: 0,
            last_tick: Instant::now(),
        }
    }
}

// draw new generation
fn paint_grid(painter: &egui::Painter, grid: &grid::Grid) {
    for (x, y) in &grid.cells {
        painter.rect_filled(
            Rect::from_min_size(
                Pos2::new(*x as f32 * CELL_SIZE, *y as f32 * CELL_SIZE),
                egui::Vec2::splat(CELL_SIZE),
            ),
            0.0,
            Color32::WHITE,
        );
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_secs(1));

        if self.last_tick.elapsed() >= Duration::from_secs(1) {
            self.grid = grid::tick(&self.grid);
            self.gen_count += 1;

            self.last_tick = Instant::now();

            println!("Generation count: {}", self.gen_count);
        }

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let painter = ui.painter();
            paint_grid(&painter, &self.grid);
        });
    }
}
