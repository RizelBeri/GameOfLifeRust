#![allow(dead_code)]

use eframe::egui;
use egui::{Color32, Pos2, Rect, Sense, Vec2};
use std::collections::HashSet;
use std::time::{Duration, Instant};

const CELL_SIZE: f32 = 20.0;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Conway",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

#[derive(Default, Clone)]
struct Grid {
    cells: HashSet<(i32, i32)>,
}

impl Grid {
    fn insert(&mut self, x: i32, y: i32) {
        self.cells.insert((x, y));
    }

    fn remove(&mut self, x: i32, y: i32) {
        self.cells.remove(&(x, y));
    }

    fn toggle(&mut self, x: i32, y: i32) {
        if self.cells.contains(&(x, y)) {
            self.remove(x, y);
        } else {
            self.insert(x, y);
        }
    }
}

pub struct MyApp {
    grid: Grid,

    generation: u64,

    running: bool,
    last_tick: Instant,

    // Камера
    camera: Vec2,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut grid = Grid::default();

        // glider
        grid.insert(1, 0);
        grid.insert(2, 1);
        grid.insert(0, 2);
        grid.insert(1, 2);
        grid.insert(2, 2);

        Self {
            grid,
            generation: 0,
            running: false,
            last_tick: Instant::now(),
            camera: Vec2::ZERO,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // simulation
        if self.running {
            ui.request_repaint_after(Duration::from_millis(150));

            if self.last_tick.elapsed() >= Duration::from_millis(150) {
                self.grid = tick(&self.grid);
                self.generation += 1;
                self.last_tick = Instant::now();
            }
        }

        egui::TopBottomPanel::top("top_panel").show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Start / Stop").clicked() {
                    self.running = !self.running;
                }

                if ui.button("Dark").clicked() {
                    ui.set_visuals(egui::Visuals::dark());
                }

                if ui.button("Light").clicked() {
                    ui.set_visuals(egui::Visuals::light());
                }

                ui.label(format!("Generation: {}", self.generation));
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            // Получаем всю доступную область
            let available = ui.available_size();

            // Создаем область для рисования
            let (response, painter) = ui.allocate_painter(available, Sense::click_and_drag());

            let rect = response.rect;

            // =========================
            // CAMERA MOVEMENT
            // =========================

            // drag мышкой
            if response.dragged() {
                self.camera += response.drag_delta();
            }

            // WASD
            if ui.input(|i| i.key_down(egui::Key::W)) {
                self.camera.y += 10.0;
            }

            if ui.input(|i| i.key_down(egui::Key::S)) {
                self.camera.y -= 10.0;
            }

            if ui.input(|i| i.key_down(egui::Key::A)) {
                self.camera.x += 10.0;
            }

            if ui.input(|i| i.key_down(egui::Key::D)) {
                self.camera.x -= 10.0;
            }

            // =========================
            // MOUSE -> CELL
            // =========================

            let mouse_pos = ui.input(|i| i.pointer.hover_pos());

            if let Some(mouse) = mouse_pos {
                // позиция внутри поля
                let local = mouse - rect.min.to_vec2();

                // мировые координаты
                let world = local - self.camera;

                // клетка
                let cell_x = (world.x / CELL_SIZE).floor() as i32;
                let cell_y = (world.y / CELL_SIZE).floor() as i32;

                // debug text
                painter.text(
                    rect.min + Vec2::new(10.0, 10.0),
                    egui::Align2::LEFT_TOP,
                    format!("Cell: {}, {}", cell_x, cell_y),
                    egui::FontId::monospace(18.0),
                    Color32::WHITE,
                );

                // click
                if response.clicked() {
                    self.grid.toggle(cell_x, cell_y);
                }
            }

            // =========================
            // GRID LINES
            // =========================

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
            for y in -1..rows {
                let y_pos = rect.min.y + y as f32 * CELL_SIZE + offset_y;

                painter.line_segment(
                    [
                        Pos2::new(rect.left(), y_pos),
                        Pos2::new(rect.right(), y_pos),
                    ],
                    stroke,
                );
            }

            // =========================
            // DRAW CELLS
            // =========================

            for &(x, y) in &self.grid.cells {
                let screen_x = rect.min.x + x as f32 * CELL_SIZE + self.camera.x;

                let screen_y = rect.min.y + y as f32 * CELL_SIZE + self.camera.y;

                let cell_rect =
                    Rect::from_min_size(Pos2::new(screen_x, screen_y), Vec2::splat(CELL_SIZE));

                painter.rect_filled(cell_rect, 0.0, Color32::LIGHT_GREEN);
            }
        });
    }
}

// =========================================
// GAME OF LIFE
// =========================================

fn tick(grid: &Grid) -> Grid {
    let mut new_grid = Grid::default();

    let mut neighbor_counts = std::collections::HashMap::<(i32, i32), i32>::new();

    for &(x, y) in &grid.cells {
        for ny in -1..=1 {
            for nx in -1..=1 {
                if nx == 0 && ny == 0 {
                    continue;
                }

                *neighbor_counts.entry((x + nx, y + ny)).or_insert(0) += 1;
            }
        }
    }

    for (cell, count) in neighbor_counts {
        if count == 3 || (count == 2 && grid.cells.contains(&cell)) {
            new_grid.insert(cell.0, cell.1);
        }
    }

    new_grid
}
