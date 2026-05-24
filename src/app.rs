#![allow(unused_variables)]
#![allow(dead_code)]

use eframe::egui::{Color32, Panel, Pos2, Rect};
use egui::{Sense, Vec2, containers::menu};
use std::time::{Duration, Instant};

use crate::grid;

const CELL_SIZE: f32 = 10.0;

pub struct MyApp {
    grid: grid::Grid,

    gen_count: u64,

    last_tick: Instant,
    simulation_status: bool,

    camera: Vec2,

    scene_rect: Rect,
}


impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
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
            scene_rect: Rect::NOTHING,
            camera: Vec2::ZERO,
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
            let panel_size = ui.min_size();

            // FRAME
            let frame = egui::Frame::default().fill(Color32::WHITE);
            frame.show(ui, |ui| {
                ui.set_max_height(400.0);

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

                    painter.text(
                        rect.min + Vec2::new(10.0, 10.0),
                        egui::Align2::LEFT_TOP,
                        format!("Cell: {}, {}", cell_x, cell_y),
                        egui::FontId::monospace(18.0),
                        Color32::WHITE,
                    );

                    if responce.clicked() {
                        self.grid.toggle(cell_x, cell_y);
                    }
                }


            ui.label(format!("Current generation: {}", self.gen_count));
        });
    }
}
