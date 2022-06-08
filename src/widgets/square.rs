use egui::{Color32, Widget};
use super::game::Player;


pub struct SquareWidget<'a> {
    pub clicked: &'a mut Option<Player>,
    pub next_player: &'a mut Player
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

        let text = match &*self.clicked {
            Some(x) => format!("{}",x),
            None => String::new()
        };
       
        if self.clicked.is_some() {
            ui.set_enabled(false);
        }

        let response = ui.add_sized(
            ui.available_size(),
            egui::Button::new(text).fill(Color32::WHITE),
        );

        if response.clicked() {
            *self.clicked = Some(*self.next_player); 
            *self.next_player = match self.next_player {
                Player::O => Player::X,
                Player::X => Player::O,
            };
        }

        response
    }
}
