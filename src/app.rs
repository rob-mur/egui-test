use crate::widgets::board::BoardWidget;
use egui_extras::{Size, StripBuilder};

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
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for AppData {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .sizes(Size::remainder(), 2)
                .vertical(|mut strip| {
                    strip.strip(|builder| {
                        builder.sizes(Size::remainder(), 2).horizontal(|mut strip| {
                            strip.cell(|ui| {
                                let available_size = ui.available_size();
                                let length = f32::min(available_size.x, available_size.y);
                                ui.add_sized([length, length], BoardWidget);
                            });
                            strip.empty();
                        });
                    });
                    strip.empty();
                });
        });
    }
}
