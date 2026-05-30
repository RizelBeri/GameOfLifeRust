#![allow(unused_variables)]
#![allow(dead_code)]

use eframe::egui::{Color32, Panel, Pos2, Rect};
use egui::{Sense, Vec2, containers::menu};
use std::time::{Duration, Instant};

use crate::grid;

const CELL_SIZE: f32 = 15.0;

pub struct MyApp {
    grid: grid::Grid,

    gen_count: u64,

    last_tick: Instant,
    simulation_status: bool,

    camera: Vec2,

    scene_rect: Rect,
    simulation_speed: f32,
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let grid = grid::Grid::default();
        Self {
            grid,
            gen_count: 0,
            last_tick: Instant::now(),
            simulation_status: false,
            scene_rect: Rect::NOTHING,
            camera: Vec2::ZERO,
            simulation_speed: 250.0,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Generation simulation delay
        if self.simulation_status {
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_millis(
                    self.simulation_speed as u64,
                ));
            if self.last_tick.elapsed() >= Duration::from_millis(self.simulation_speed as u64) {
                self.grid = grid::tick(&self.grid);
                self.gen_count += 1;

                self.last_tick = Instant::now();
            }
        }

        Panel::top("menu").show_inside(ui, |ui| {
            menu::MenuBar::new().ui(ui, |ui| {
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
            let panel_size = ui.min_size();

            // FRAME
            // let frame = egui::Frame::default().fill(Color32::WHITE);
            let frame = egui::Frame::default();

            frame.show(ui, |ui| {
                ui.set_max_height(panel_size.y * 0.7);

                // Recieve aviable space
                let available = ui.available_size();

                // Creating area for painting
                let (response, painter) = ui.allocate_painter(available, Sense::click_and_drag());
                let rect = response.rect;

                // ======================================
                // CAMERA MOVEMENT
                // ======================================

                if response.dragged() {
                    self.camera += response.drag_delta();
                }

                // GRID LINES

                let stroke = egui::Stroke::new(1.0, Color32::DARK_GRAY);

                let cols = (rect.width() / CELL_SIZE) as i32 + 2;
                let rows = (rect.height() / CELL_SIZE) as i32 + 2;

                let offset_x = self.camera.x.rem_euclid(CELL_SIZE);
                let offset_y = self.camera.y.rem_euclid(CELL_SIZE);

                // vertical
                for x in -1..cols {
                    let x_pos = rect.min.x + x as f32 * CELL_SIZE + offset_x;

                    painter.line_segment(
                        [
                            Pos2::new(x_pos, rect.top()),
                            Pos2::new(x_pos, rect.bottom()),
                        ],
                        stroke,
                    );
                }

                // horizontal
                for x in -1..rows {
                    let y_pos = rect.min.y + x as f32 * CELL_SIZE + offset_y;

                    painter.line_segment(
                        [
                            Pos2::new(rect.left(), y_pos),
                            Pos2::new(rect.right(), y_pos),
                        ],
                        stroke,
                    );
                }

                // DRAW CELLS

                for &(x, y) in &self.grid.cells {
                    let screen_x = rect.min.x + x as f32 * CELL_SIZE + self.camera.x;

                    let screen_y = rect.min.y + y as f32 * CELL_SIZE + self.camera.y;

                    let cell_rect =
                        Rect::from_min_size(Pos2::new(screen_x, screen_y), Vec2::splat(CELL_SIZE));

                    painter.rect_filled(cell_rect, 0.0, Color32::LIGHT_GREEN);
                }

                // ======================================
                // MOUSE -> GRID COORD
                // ======================================

                let mouse_pos = ui.input(|i| i.pointer.hover_pos());

                if let Some(mouse) = mouse_pos {
                    // position inside area
                    let local = mouse - rect.min.to_vec2();

                    // position inside world
                    let world = local - self.camera;

                    let cell_x = (world.x / CELL_SIZE).floor() as i32;
                    let cell_y = (world.y / CELL_SIZE).floor() as i32;

                    if response.clicked() && !self.simulation_status {
                        self.grid.toggle(cell_x, cell_y);
                    }
                }
            });

            egui::MenuBar::new().ui(ui, |ui| {
                if ui.button("Start/Stop").clicked() {
                    self.simulation_status = !self.simulation_status;
                }
                if ui.button("Reset").clicked() {
                    self.grid.clear();
                    self.gen_count = 0;
                    self.simulation_status = false;
                    self.simulation_speed = 250.0;
                }
                if ui.button("Speed Up").clicked() {
                    self.simulation_speed = (self.simulation_speed - 25.0).max(50.0);
                }
                if ui.button("Slow Down").clicked() {
                    self.simulation_speed = (self.simulation_speed + 25.0).min(2000.0);
                }
            });

            ui.label(format!("Current generation: {}", self.gen_count));
            ui.label(format!("Simulation status: {}", self.simulation_status));

            ui.label(format!(
                "Simulation speed: {:.0}%",
                250.0 / self.simulation_speed * 100.0
            ));
        });
    }
}
