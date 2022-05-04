use eframe::egui;
use tinyfiledialogs::open_file_dialog;

use crate::ui;

pub fn show(app: &mut ui::MainApp, ctx: &egui::Context) {
    egui::Window::new("File").show(ctx, |ui| {
        ui.label(format!("Current file: {}", app.selected_file));
        if ui.button("Select file").clicked() {
            app.selected_file = if let Some(file) = open_file_dialog("Open", "~", None) {
                file
            } else {
                "".to_owned()
            };
        }
    });
}

