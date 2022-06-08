use super::board::BoardWidget;
use egui::{Response, Ui, Widget};
use egui_extras::{Size, StripBuilder};

pub struct GameWidget<'a> {
    pub board: &'a mut [Option<Player>; 9],
    pub next_player: &'a mut Player,
}

impl<'a> Widget for GameWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
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
                                    board: self.board,
                                    next_player: self.next_player,
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

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::X
    }
}
