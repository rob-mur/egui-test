use super::board::BoardWidget;
use egui::{Response, Ui, Widget};
use egui_extras::{Size, StripBuilder};

pub struct GameWidget<'a> {
    pub clicks: &'a mut [bool; 9],
}

impl<'a> Widget for GameWidget<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        StripBuilder::new(ui)
            .sizes(Size::remainder(), 2)
            .vertical(|mut strip| {
                strip.strip(|builder| {
                    builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                        strip.cell(|ui| {
                            let available_size = ui.available_size();
                            let length = f32::min(available_size.x, available_size.y);
                            ui.add_sized(
                                [length, length],
                                BoardWidget {
                                    clicks: &mut self.clicks,
                                },
                            );
                        });
                        strip.empty();
                    });
                });
                strip.empty();
            })
    }
}
