use eframe::egui;
use tinyfiledialogs::save_file_dialog;

use crate::ui::MainApp;
use crate::audio;

pub fn show(app: &mut MainApp, ctx: &egui::Context) {
        egui::Window::new("Export").show(ctx, |ui| {
            ui.label(format!("Current file: {}", app.export_file));
            if ui.button("Select file").clicked() {
                app.export_file = if let Some(file) = save_file_dialog("Open", "output.wav") {
                    file
                } else {
                    "".to_owned()
                };
            }

            ui.separator();

            let exporting = if let Some(rx) = &app.finished_writing_receiver {
                if rx.try_recv().is_ok() {
                    app.finished_writing_receiver = None;
                    false
                } else {
                    true
                }
            } else {
                false
            };

            let exporting_text = if exporting {
                "Exporting"
            } else {
                "Export"
            };

            let button_enabled = !exporting && app.export_file != "" && app.selected_file != "";
            
            if ui.add_enabled(button_enabled, egui::Button::new(exporting_text)).clicked() {
                let params = app.get_aliasing_params();
                app.finished_writing_receiver = Some(audio::export_file(&app.selected_file, &app.export_file, &params));
            }
        });
    }
