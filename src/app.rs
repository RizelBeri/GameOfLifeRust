#![allow(unused_variables)]
#![allow(dead_code)]

use crate::grid;
use crate::widgets::ViewMode;
use crate::widgets::toggle;
use eframe::egui::{Color32, Panel, PointerButton, Pos2, Rect};
use egui::{Key, Sense, Vec2, containers::menu};
use std::time::{Duration, Instant};

const MIN_CELL_SIZE: f32 = 4.0;
const MAX_CELL_SIZE: f32 = 64.0;

pub struct MyApp {
    grid: grid::Grid,

    gen_count: u64,

    last_tick: Instant,
    simulation_status: bool,
    simulation_speed: f32,

    view_mode: ViewMode,
    camera: Vec2,
    zoom: f32,
    cell_color: Color32,

    scene_rect: Rect,
    last_drawn: Option<(i32, i32)>,
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
            zoom: 15.0,
            cell_color: Color32::LIGHT_BLUE,
            view_mode: ViewMode::Flat,
            simulation_speed: 250.0,
            last_drawn: None,
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

                // GRID LINES

                let stroke = egui::Stroke::new(1.0, Color32::DARK_GRAY);

                let cols = (rect.width() / self.zoom) as i32 + 2;
                let rows = (rect.height() / self.zoom) as i32 + 2;

                let offset_x = self.camera.x.rem_euclid(self.zoom);
                let offset_y = self.camera.y.rem_euclid(self.zoom);

                // vertical
                for x in -1..cols {
                    let x_pos = rect.min.x + x as f32 * self.zoom + offset_x;

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
                    let y_pos = rect.min.y + x as f32 * self.zoom + offset_y;

                    painter.line_segment(
                        [
                            Pos2::new(rect.left(), y_pos),
                            Pos2::new(rect.right(), y_pos),
                        ],
                        stroke,
                    );
                }

                // DRAW CELLS
                draw_cell(self, &rect, &painter);

                // ======================================
                // ZOOM
                // ======================================

                let scroll_delta = ui.input(|i| i.smooth_scroll_delta.y);
                if scroll_delta != 0.0 {
                    let zoom_factor = 1.0 + scroll_delta * 0.01;
                    let new_zoom = (self.zoom * zoom_factor).clamp(MIN_CELL_SIZE, MAX_CELL_SIZE);

                    // Zoom toward the mouse cursor so it stays fixed on screen
                    if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                        // Where is the mouse in world space before zoom?
                        let local = mouse_pos - rect.min.to_vec2();
                        let world_before = local - self.camera;

                        // After zoom, that same world point must still be at local
                        // local = world_before * (new_zoom / old_zoom) + new_camera
                        // => new_camera = local - world_before * scale
                        let scale = new_zoom / self.zoom;
                        self.camera = local - world_before * scale;
                    }

                    self.zoom = new_zoom;
                }

                // ======================================
                // CAMERA MOVEMENT (right click to drag)
                // ======================================

                if response.dragged_by(PointerButton::Secondary) {
                    self.camera += response.drag_delta();
                }

                // ======================================
                // MOUSE -> GRID COORD
                // DRAG TO DRAW
                // ======================================

                let mouse_pos = ui.input(|i| i.pointer.hover_pos());

                if let Some(mouse) = mouse_pos {
                    // position inside area
                    let local = mouse - rect.min.to_vec2();

                    // position inside world
                    let world = local - self.camera;

                    let cell_x = (world.x / self.zoom).floor() as i32;
                    let cell_y = (world.y / self.zoom).floor() as i32;

                    if response.clicked_by(PointerButton::Primary) && !self.simulation_status {
                        self.grid.toggle(cell_x, cell_y);
                        self.last_drawn = Some((cell_x, cell_y));
                    }

                    if response.dragged_by(PointerButton::Primary) && !self.simulation_status {
                        let new_cell = self.last_drawn != Some((cell_x, cell_y));
                        if new_cell {
                            self.grid.insert(cell_x, cell_y);
                            self.last_drawn = Some((cell_x, cell_y));
                        }
                    }
                }

                // Reset draw trackin posiiton when button is released
                if ui.input(|i| i.pointer.primary_released()) {
                    self.last_drawn = None;
                }
            });

            egui::MenuBar::new().ui(ui, |ui| {
                if ui.button("Start/Stop").clicked() {
                    self.simulation_status = !self.simulation_status;
                }
                if ui.input(|i| i.key_pressed(Key::Space)) {
                    self.simulation_status = !self.simulation_status;
                }
                if ui.button("Reset").clicked() {
                    self.grid.clear();
                    self.gen_count = 0;
                    self.simulation_status = false;
                    self.simulation_speed = 250.0;
                }
                if ui.input(|i| i.key_pressed(Key::R)) {
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

            ui.horizontal(|ui| {
                ui.label("Cell color: ");
                ui.color_edit_button_srgba(&mut self.cell_color);
                if ui.button("Reset").clicked() {
                    self.cell_color = Color32::LIGHT_BLUE;
                }
            });

            ui.add(toggle(&mut self.view_mode));
        });
    }
}

// DRAW CELLS
pub fn draw_cell(ui: &mut MyApp, rect: &Rect, painter: &egui::Painter) {
    for &(x, y) in &ui.grid.cells {
        let screen_x = rect.min.x + x as f32 * ui.zoom + ui.camera.x;

        let screen_y = rect.min.y + y as f32 * ui.zoom + ui.camera.y;

        let cell_rect = Rect::from_min_size(Pos2::new(screen_x, screen_y), Vec2::splat(ui.zoom));

        painter.rect_filled(cell_rect, 0.0, ui.cell_color);
    }
}
