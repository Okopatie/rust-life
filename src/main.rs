#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use life::{Board, Cell};
use std::{thread, time};

use eframe::{egui::{self, Slider, Button}, epaint::{Color32, Vec2}};

struct LifeApp {
    board: Board,
}

impl Default for LifeApp {
    fn default() -> Self {
        Self {
            board: Board::new(10, 10),
        }
    }
}

impl LifeApp {
    fn draw_cell(&self, x: usize, y: usize) -> egui::Button {
        let cell = self.board.get_cell(x, y).unwrap();
        let colour = if cell.is_alive() {Color32::RED} else {Color32::WHITE};
        egui::Button::new("")
            .fill(colour)
            .min_size(Vec2::new(50.0, 50.0))
    }
}

impl eframe::App for LifeApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.0);
        egui::TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            ui.heading("GAME OF LIFE");
            ui.add(egui::Slider::new(&mut self.board.y_dim, 1..=20).text("Rows"));
            ui.add(egui::Slider::new(&mut self.board.x_dim, 1..=20).text("Columns"));
            ui.horizontal(|ui| {
                if ui.button("NEXT GENERATION").clicked() {
                    self.board.update_board();
                }
                if ui.button("RESET").clicked() {
                    self.board.reset();
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("Life Grid").show(ui, |ui| {
                while self.board.board.len() < self.board.x_dim * self.board.y_dim {
                    self.board.board.push(Cell::new_dead());
                }
                for y in 0..self.board.y_dim {
                    for x in 0..self.board.x_dim {
                        if ui.add(self.draw_cell(x, y)).clicked() {
                            self.board.toggle_cell(x, y);
                        }
                    }
                    ui.end_row();
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size((800.0, 800.0))
            .with_active(true),
        ..Default::default()
    };
    eframe::run_native("Game of life",
                       options,
                      Box::new(|_| Box::<LifeApp>::default())
                       )
}
