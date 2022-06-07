use egui::{Color32, Direction, Frame, Layout, Response, Stroke, Ui, Widget};
use egui_extras::{Size, Strip, StripBuilder};

pub struct BoardWidget;

impl Widget for BoardWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        egui::CentralPanel::default()
            .show_inside(ui, |ui| {
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    ui.heading("Next player: X");
                    ui.label("");
                    board(ui);
                });
            })
            .response
    }
}

fn board(ui: &mut Ui) {
    ui.style_mut().spacing.item_spacing.x = 2.5;
    ui.style_mut().spacing.item_spacing.y = 2.5;

    egui::CentralPanel::default()
        .frame(Frame {
            inner_margin: Default::default(),
            outer_margin: Default::default(),
            rounding: Default::default(),
            shadow: Default::default(),
            fill: Color32::BLACK,
            stroke: Stroke {
                width: 5.0,
                color: Color32::BLACK,
            },
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
        ui.painter()
            .rect_filled(ui.available_rect_before_wrap(), 0.0, Color32::WHITE);
        ui.label(format!("{}", *idx));
        *idx += 1;
    });
}
