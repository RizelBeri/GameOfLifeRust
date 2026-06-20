#![allow(unused_variables)]
#![allow(dead_code)]

use crate::grid;
use crate::widgets::ViewMode;
use crate::widgets::toggle;
use eframe::egui::{Color32, PointerButton, Pos2, Rect, Shape, Stroke};
use egui::{Key, Sense, Vec2};
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
    sidebar_open: bool,
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
            sidebar_open: true,
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

        // ================================
        // KEYBOARD CONTROLS
        // ================================
        if ui.input(|i| i.key_pressed(Key::Space)) {
            self.simulation_status = !self.simulation_status;
        }
        if ui.input(|i| i.key_pressed(Key::R)) {
            self.grid.clear();
            self.gen_count = 0;
            self.simulation_status = false;
            self.simulation_speed = 250.0;
        }
        if ui.input(|i| i.key_pressed(Key::ArrowRight)) {
            self.simulation_speed = self.simulation_speed.max(50.0);
            self.simulation_speed *= 0.9;
        }
        if ui.input(|i| i.key_pressed(Key::ArrowLeft)) {
            self.simulation_speed *= 1.1;
            self.simulation_speed = self.simulation_speed.min(2000.0);
        }
        // ================================
        // CONROL UI
        // ================================
        if self.sidebar_open {
            egui::Panel::right("Controls")
                .resizable(true)
                .min_size(180.0)
                .default_size(220.0)
                .show_inside(ui, |ui| {
                    if ui.small_button("▶").clicked() {
                        self.sidebar_open = false;
                    }
                    // Simulation control in collapsing pannel
                    egui::CollapsingHeader::new("⚙ Simulation")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.add_space(4.0);

                            let btn_label = if self.simulation_status {
                                "⏸ Pause  [SPACE]"
                            } else {
                                "▶ Start  [SPACE]"
                            };

                            if ui
                                .add_sized(
                                    [ui.available_width(), 26.0],
                                    egui::Button::new(btn_label),
                                )
                                .clicked()
                            {
                                self.simulation_status = !self.simulation_status;
                            }

                            ui.add_space(4.0);

                            if ui
                                .add_sized(
                                    [ui.available_width(), 28.0],
                                    egui::Button::new("↺ Reset  [R]"),
                                )
                                .clicked()
                            {
                                self.grid.clear();
                                self.gen_count = 0;
                                self.simulation_status = false;
                                self.simulation_speed = 250.0;
                            }

                            // Speed
                            ui.label(format!(
                                "Speed: {:.0}%",
                                250.0 / self.simulation_speed * 100.0
                            ));
                            ui.add(
                                egui::Slider::new(&mut self.simulation_speed, 50.0..=2000.0)
                                    .logarithmic(true)
                                    .show_value(false),
                            );
                        });

                    ui.add_space(6.0);

                    // --- Stats ---
                    egui::CollapsingHeader::new("📊 Stats")
                        .default_open(true)
                        .show(ui, |ui| {
                            ui.add_space(4.0);
                            ui.label(format!("Generation: {}", self.gen_count));
                            ui.add_space(4.0);
                            ui.label(format!("Cells alive: {}", self.grid.cells.len()));
                        });

                    ui.add_space(6.0);

                    egui::CollapsingHeader::new("🎨 Appearance")
                        .default_open(true)
                        .show(ui, |ui| {
                            // Color button
                            ui.horizontal(|ui| {
                                ui.label("Cell color: ");
                                ui.color_edit_button_srgba(&mut self.cell_color);
                                if ui.small_button("↺").clicked() {
                                    self.cell_color = Color32::LIGHT_BLUE;
                                }
                            });

                            ui.add_space(6.0);

                            // View mode
                            ui.horizontal(|ui| {
                                ui.label("2D");
                                ui.add(toggle(&mut self.view_mode));
                                ui.label("3D");
                            });

                            ui.add_space(6.0);

                            if ui
                                .add_sized(
                                    [ui.available_width(), 28.0],
                                    egui::Button::new("🌙 Dark / ☀ Light"),
                                )
                                .clicked()
                            {
                                if ui.visuals().dark_mode {
                                    ui.set_visuals(egui::Visuals::light());
                                } else {
                                    ui.set_visuals(egui::Visuals::dark());
                                }
                            }

                            ui.add_space(4.0);
                        });
                });
        } else {
            // thin strip on the right with just a toggle button
            egui::Panel::right("Controls")
                .resizable(false)
                .exact_size(40.0)
                .show_inside(ui, |ui| {
                    ui.add_space(4.0);
                    if ui.small_button("◀").clicked() {
                        self.sidebar_open = true;
                    }
                });
        }

        // ================================
        // MAIN SIMULATION
        // ================================
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let panel_size = ui.min_size();

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
    }
}

// DRAW CELLS
pub fn draw_cell(ui: &mut MyApp, rect: &Rect, painter: &egui::Painter) {
    let d: f32 = ui.zoom * 0.3;
    let base = ui.cell_color;

    // Change color for top and left faces
    let top_color = Color32::from_rgb(
        (base.r() as f32 * 1.3).min(255.0) as u8,
        (base.g() as f32 * 1.3).min(255.0) as u8,
        (base.b() as f32 * 1.3).min(255.0) as u8,
    );
    let left_color = Color32::from_rgb(
        (base.r() as f32 * 0.6) as u8,
        (base.g() as f32 * 0.6) as u8,
        (base.b() as f32 * 0.6) as u8,
    );

    let mut sorted_cells: Vec<(i32, i32)> = ui.grid.cells.iter().cloned().collect();

    if ui.view_mode == ViewMode::Flat {
        for &(x, y) in &sorted_cells {
            let screen_x = rect.min.x + x as f32 * ui.zoom + ui.camera.x;
            let screen_y = rect.min.y + y as f32 * ui.zoom + ui.camera.y;
            let cell_rect =
                Rect::from_min_size(Pos2::new(screen_x, screen_y), Vec2::splat(ui.zoom));
            painter.rect_filled(cell_rect, 0.0, base);
        }
    } else {
        // Left face
        sorted_cells.sort_by(|a, b| a.1.cmp(&b.1).then(b.0.cmp(&a.0)));
        for &(x, y) in &sorted_cells {
            let screen_x = rect.min.x + x as f32 * ui.zoom + ui.camera.x;
            let screen_y = rect.min.y + y as f32 * ui.zoom + ui.camera.y;
            let s = ui.zoom;

            painter.add(Shape::convex_polygon(
                vec![
                    Pos2::new(screen_x - d, screen_y - d),
                    Pos2::new(screen_x, screen_y),
                    Pos2::new(screen_x, screen_y + s),
                    Pos2::new(screen_x - d, screen_y + s - d),
                ],
                left_color,
                Stroke::NONE,
            ));
        }

        // Top face
        sorted_cells.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));
        for &(x, y) in &sorted_cells {
            if !ui.grid.cells.contains(&(x, y - 1)) {
                // draw top face

                let screen_x = rect.min.x + x as f32 * ui.zoom + ui.camera.x;
                let screen_y = rect.min.y + y as f32 * ui.zoom + ui.camera.y;
                let s = ui.zoom;

                painter.add(Shape::convex_polygon(
                    vec![
                        Pos2::new(screen_x - d, screen_y - d),
                        Pos2::new(screen_x + s - d, screen_y - d),
                        Pos2::new(screen_x + s, screen_y),
                        Pos2::new(screen_x, screen_y),
                    ],
                    top_color,
                    Stroke::NONE,
                ));
            }
        }

        // Front face
        for &(x, y) in &sorted_cells {
            let screen_x = rect.min.x + x as f32 * ui.zoom + ui.camera.x;
            let screen_y = rect.min.y + y as f32 * ui.zoom + ui.camera.y;
            let s = ui.zoom;

            painter.rect_filled(
                Rect::from_min_size(Pos2::new(screen_x, screen_y), Vec2::splat(s)),
                0.0,
                base,
            );
        }
    }
}
