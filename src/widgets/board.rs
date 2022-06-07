use egui::{Color32, Ui};
use egui_extras::{Size, Strip, StripBuilder};

pub fn board(ui: &mut Ui) {
    let mut idx = 0;
    StripBuilder::new(ui)
        .sizes(Size::remainder(), 3)
        .vertical(|mut strip| {
            fill_strip(&mut strip, &mut idx);
            fill_strip(&mut strip, &mut idx);
            fill_strip(&mut strip, &mut idx);
        });
}

fn fill_strip(strip: &mut Strip, idx: &mut i32) {
    strip.strip(|builder| {
        builder.sizes(Size::remainder(), 3).horizontal(|mut strip| {
            fill_cell(&mut strip,  idx);
            fill_cell(&mut strip,  idx);
            fill_cell(&mut strip,  idx);
        });
    });
}

fn fill_cell(strip: &mut Strip<'_, '_>, idx: &mut i32) {
    strip.cell(|ui| {
        ui.painter().rect_filled(
            ui.available_rect_before_wrap(),
            0.0,
            Color32::from_gray((*idx as f32 * 255.0 / 9.0) as u8),
        );
        ui.label(format!("{}", *idx));
        *idx += 1;
    });
}

