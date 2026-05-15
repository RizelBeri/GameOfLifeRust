#![allow(unused_variables)]
#![allow(dead_code)]

use eframe::egui::{Color32, Panel, Pos2, Rect};
use egui::{containers::menu, response};
use std::time::{Duration, Instant};

use crate::grid;

const CELL_SIZE: f32 = 10.0;

pub struct MyApp {
    grid: grid::Grid,
    gen_count: u64,
    last_tick: Instant,
    simulation_status: bool,
    scene_rect: Rect,
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
            scene_rect: Rect::ZERO,
        }
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
            egui::Frame::default().show(ui, |ui| {
                let scene = egui::Scene::new().max_inner_size([350.0, 350.0]);

                let response = scene.show(ui, &mut self.scene_rect, |ui| {
                    let panel_rect = ui.max_rect();
                    let painter = ui.painter_at(panel_rect);

                    paint_gridlines(&painter, panel_rect.min, panel_rect.size());
                    paint_grid(&painter, &self.grid, panel_rect.min);
                });
            });

            // let panel_rect = ui.max_rect();
            // let painter = ui.painter_at(panel_rect); // clips to panel automatically
        });
    }
}

// draw new generation
fn paint_grid(painter: &egui::Painter, grid: &grid::Grid, origin: egui::Pos2) {
    for (x, y) in &grid.cells {
        painter.rect_filled(
            Rect::from_center_size(
                Pos2::new(
                    origin.x + *x as f32 * CELL_SIZE,
                    origin.y + *y as f32 * CELL_SIZE,
                ),
                egui::Vec2::splat(CELL_SIZE),
            ),
            0.0,
            Color32::DARK_GRAY,
        );
    }
}

fn paint_gridlines(painter: &egui::Painter, origin: egui::Pos2, screen: egui::Vec2) {
    let stroke = egui::Stroke::new(0.5, Color32::DARK_GRAY);

    // vertical lines
    for x in 0..=(screen.x / CELL_SIZE) as i32 {
        let x_pos = origin.x + x as f32 * CELL_SIZE;
        painter.line_segment(
            [
                Pos2::new(x_pos, origin.y),
                Pos2::new(x_pos, origin.y + screen.y),
            ],
            stroke,
        );
    }

    // horizontal lines
    for y in 0..=(screen.y / CELL_SIZE) as i32 {
        let y_pos = origin.y + y as f32 * CELL_SIZE;
        painter.line_segment(
            [
                Pos2::new(origin.x, y_pos),
                Pos2::new(origin.x + screen.x, y_pos),
            ],
            stroke,
        );
    }
}
