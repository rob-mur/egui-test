use egui::style::Margin;
use egui::{Color32, Direction, Frame, Layout, Response, Stroke, Ui, Vec2, Widget};
use egui_extras::{Size, Strip, StripBuilder};
use std::default::Default;

pub struct BoardWidget;

impl Widget for BoardWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        egui::CentralPanel::default()
            .show_inside(ui, |ui| {
                ui.heading("Next player: X\n");
                board(ui);
            })
            .response
    }
}

fn board(ui: &mut Ui) {
    ui.style_mut().spacing.item_spacing.x = 2.5;
    ui.style_mut().spacing.item_spacing.y = 2.5;

    egui::CentralPanel::default()
        .frame(Frame {
            fill: Color32::BLACK,
            stroke: Stroke {
                width: 5.0,
                color: Color32::BLACK,
            },
            ..std::default::Default::default()
        })
        .show_inside(ui, |ui| {
            let mut idx = 0;
            StripBuilder::new(ui)
                .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
                .sizes(Size::remainder(), 3)
                .vertical(|mut strip| {
                    fill_strip(&mut strip, &mut idx);
                    fill_strip(&mut strip, &mut idx);
                    fill_strip(&mut strip, &mut idx);
                });
        });
}

fn fill_strip(strip: &mut Strip<'_, '_>, idx: &mut i32) {
    strip.strip(|builder| {
        builder.sizes(Size::remainder(), 3).horizontal(|mut strip| {
            fill_cell(&mut strip, idx);
            fill_cell(&mut strip, idx);
            fill_cell(&mut strip, idx);
        });
    });
}

fn fill_cell(strip: &mut Strip<'_, '_>, idx: &mut i32) {
    strip.cell(|ui| {
        ui.spacing_mut().item_spacing = Vec2 { x: 20.0, y: 20.0 };
        egui::CentralPanel::default()
            .frame(Frame {
                inner_margin: Margin::same(5.0),
                fill: Color32::WHITE,
                ..Default::default()
            })
            .show_inside(ui, |ui| {
                ui.add_sized(
                    ui.available_size(),
                    egui::Button::new(format!("{}", *idx)).fill(Color32::WHITE),
                );
            });
        *idx += 1;
    });
}
