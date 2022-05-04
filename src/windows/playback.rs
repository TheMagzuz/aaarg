use eframe::egui;

use crate::ui::MainApp;

pub fn show(app: &mut MainApp, ctx: &egui::Context) {
    egui::Window::new("Playback").show(ctx, |ui| {
        let lock = app.sink.try_lock(); 
        let lock_acquired = lock.is_ok();
        let stopped = if let Ok(s) = lock {
            s.empty()
        } else {
            true
        };
        if stopped {
            if ui.add_enabled(app.selected_file != "" && lock_acquired, egui::Button::new("Play")).clicked() {
                let params = app.get_aliasing_params();
                crate::audio::preview_aliasing(
                    &app.selected_file,
                    &params,
                    app.sink.clone()
                );
            }
        } else {
            if ui.button("Stop").clicked() {
                app.sink.lock().as_ref().unwrap().stop();
            }
        }
    });
}

