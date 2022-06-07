use egui::{Color32, Direction, Layout, Response, Stroke, Ui, Widget};
use egui_extras::{Size, Strip, StripBuilder};

pub struct BoardWidget;

impl Widget for BoardWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        egui::CentralPanel::default()
            .show_inside(ui, |ui| {
                egui::CentralPanel::default().show_inside(ui, |ui| {
                    ui.label("Next player: X");
                    board(ui);
                });
            })
            .response
    }
}

fn board(ui: &mut Ui) {
    ui.style_mut().spacing.item_spacing.x = 0.0;
    ui.style_mut().spacing.item_spacing.y = 0.0;

    let mut idx = 0;
    StripBuilder::new(ui)
        .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
        .sizes(Size::remainder(), 3)
        .vertical(|mut strip| {
            fill_strip(&mut strip, &mut idx);
            fill_strip(&mut strip, &mut idx);
            fill_strip(&mut strip, &mut idx);
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
        ui.painter().rect_stroke(
            ui.available_rect_before_wrap(),
            0.0,
            Stroke {
                width: 5.0,
                color: Color32::BLACK,
            },
        );
        ui.label(format!("{}", *idx));
        *idx += 1;
    });
}
