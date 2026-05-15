#![allow(unused_variables)]
#![allow(dead_code)]

use eframe::egui::{self, Color32, Panel, Pos2, Rect};
use egui::containers::menu;
use std::time::{Duration, Instant};

use crate::grid;

const CELL_SIZE: f32 = 10.0;

pub struct MyApp {
    grid: grid::Grid,
    gen_count: u64,
    last_tick: Instant,
    simulation_status: bool,
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
            simulation_status: false,
        }
    }
}

// draw new generation
fn paint_grid(painter: &egui::Painter, grid: &grid::Grid) {
    for (x, y) in &grid.cells {
        painter.rect_filled(
            Rect::from_center_size(
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
        // Generation simulation delay
        if self.simulation_status {
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(250));
            if self.last_tick.elapsed() >= Duration::from_millis(250) {
                self.grid = grid::tick(&self.grid);
                self.gen_count += 1;

                self.last_tick = Instant::now();

                println!("Generation count: {}", self.gen_count);
            }
        }

        Panel::top("menu").show_inside(ui, |ui| {
            menu::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open").clicked() {
                        println!("Open clicked");
                    }
                    if ui.button("Save").clicked() {
                        println!("Save clicked");
                    }
                });
                if ui.button("Start/Stop simulation").clicked() {
                    self.simulation_status = !self.simulation_status;
                }

                if ui.button("🌙 Dark").clicked() {
                    ui.set_visuals(egui::Visuals::dark());
                }
                if ui.button("🌙 Light").clicked() {
                    ui.set_visuals(egui::Visuals::light());
                }
            });
            // ui.horizontal(|ui| {});
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let painter = ui.painter();
            paint_grid(&painter, &self.grid);
        });
    }
}
