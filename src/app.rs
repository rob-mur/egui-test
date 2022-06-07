use egui::{Color32, pos2, Rect, Sense};
use egui_extras::{Size, StripBuilder};
use crate::widgets::board::board;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct AppData {
    phantom_state: String,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            phantom_state: "nothing".to_string(),
        }
    }
}

impl AppData {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_debug_on_hover(true);
        Default::default()
    }
}

impl eframe::App for AppData {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            board(ui);
        });
    }
}
