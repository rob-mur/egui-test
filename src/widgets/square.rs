use egui::{Color32, Widget};

pub struct SquareWidget<'a> {
    pub clicked: &'a mut bool,
}

impl<'a> Widget for SquareWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let font_id = ui
            .style_mut()
            .text_styles
            .get_mut(&egui::style::TextStyle::Button)
            .expect("button style should always exist");
        font_id.size = 40.0;

        let response = match self.clicked {
            true => ui.add_sized(
                ui.available_size(),
                egui::Button::new("X").fill(Color32::WHITE),
            ),
            false => ui.add_sized(
                ui.available_size(),
                egui::Button::new("").fill(Color32::WHITE),
            ),
        };

        if response.clicked() {
            *self.clicked = !*self.clicked;
            println!("updating click");
        }

        response
    }
}
