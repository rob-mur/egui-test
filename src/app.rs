use crate::widgets::game::{GameWidget, Player};

#[derive(Default)]
pub struct AppData {
    board: [Option<Player>; 9],
    next_player: Player,
}

impl AppData {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for AppData {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(GameWidget {
                board: &mut self.board,
                next_player: &mut self.next_player,
            });
        });
    }
}
