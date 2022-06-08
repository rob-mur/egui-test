use super::game::Player;
use super::square::SquareWidget;
use egui::style::Margin;
use egui::{Color32, Direction, Frame, Layout, Response, Stroke, Ui, Vec2, Widget};
use egui_extras::{Size, Strip, StripBuilder};
use std::default::Default;

pub struct BoardWidget<'a> {
    pub board: &'a mut [Option<Player>; 9],
    pub next_player: &'a mut Player,
}

impl<'a> Widget for BoardWidget<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        egui::CentralPanel::default()
            .show_inside(ui, |ui| {
                if let Some(winner) = calculate_winner(self.board) {
                    ui.heading(format!("Winner: {}\n", winner));
                    ui.set_enabled(false);
                } else {
                    ui.heading(format!("Next player: {}\n", self.next_player));
                }
                self.board(ui);
            })
            .response
    }
}

fn calculate_winner(board: &[Option<Player>; 9]) -> Option<Player> {
    const LINES: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for [a, b, c] in LINES {
        if board[a].is_some() && board[a] == board[b] && board[a] == board[c] {
            return board[a];
        }
    }
    None
}

impl<'a> BoardWidget<'a> {
    fn board(mut self, ui: &mut Ui) {
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
                let mut idx: usize = 0;
                StripBuilder::new(ui)
                    .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
                    .sizes(Size::remainder(), 3)
                    .vertical(|mut strip| {
                        self.fill_strip(&mut strip, &mut idx);
                        self.fill_strip(&mut strip, &mut idx);
                        self.fill_strip(&mut strip, &mut idx);
                    });
            });
    }

    fn fill_strip(&mut self, strip: &mut Strip<'_, '_>, idx: &mut usize) {
        strip.strip(|builder| {
            builder.sizes(Size::remainder(), 3).horizontal(|mut strip| {
                self.fill_cell(&mut strip, idx);
                self.fill_cell(&mut strip, idx);
                self.fill_cell(&mut strip, idx);
            });
        });
    }

    fn fill_cell(&mut self, strip: &mut Strip<'_, '_>, idx: &mut usize) {
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
                        SquareWidget {
                            clicked: self.board.get_mut(*idx).expect("id should always exist"),
                            next_player: self.next_player,
                        },
                    );
                });
            *idx += 1;
        });
    }
}
