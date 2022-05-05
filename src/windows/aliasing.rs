use std::time::Duration;

use eframe::egui; 

use crate::ui::MainApp;

pub fn show(app: &mut MainApp, ctx: &egui::Context) {
    egui::Window::new("Aliasing").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Aliasing factor:");
            ui.add(egui::Slider::new(&mut app.aliasing_params.factor, 1..=1000));
        });
        ui.horizontal(|ui| {
            ui.label("Factor variation:");
            ui.add(egui::Slider::new(&mut app.aliasing_params.factor_variation, 0..=1000));
        });
        ui.horizontal(|ui| {
            ui.label("Target output duration (secs):");
            let mut target_duration_secs = app.aliasing_params.target_duration.as_secs_f32();
            ui.add(egui::Slider::new(&mut target_duration_secs, 0.1..=150.0));
            app.aliasing_params.target_duration = Duration::from_secs_f32(target_duration_secs);
        });
    });
}

