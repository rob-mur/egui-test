use egui::{Color32, Widget};

pub struct SquareWidget<'a> {
    pub clicked: &'a mut bool,
}

/// Required as a lower bound for fonts to avoid painter crash
const FONT_MIN_SIZE: f32 = 4.0;

impl<'a> Widget for SquareWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let font_size = ui.available_height();
        let font_id = ui
            .style_mut()
            .text_styles
            .get_mut(&egui::style::TextStyle::Button)
            .expect("button style should always exist");
        font_id.size = f32::max(FONT_MIN_SIZE, font_size);

        let text = match self.clicked {
            true => "X",
            false => "",
        };

        let response = ui.add_sized(
            ui.available_size(),
            egui::Button::new(text).fill(Color32::WHITE),
        );

        if response.clicked() {
            *self.clicked = !*self.clicked;
        }

        response
    }
}
