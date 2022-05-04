use eframe::egui; 

use crate::ui::MainApp;

pub fn show(app: &mut MainApp, ctx: &egui::Context) {
    egui::Window::new("Aliasing").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Aliasing factor:");
            ui.add(egui::Slider::new(&mut app.factor, 1..=1000));
        });
        ui.horizontal(|ui| {
            ui.label("Factor variation:");
            ui.add(egui::Slider::new(&mut app.factor_variation, 0..=1000));
        });
        ui.horizontal(|ui| {
            ui.label("Target output duration (secs):");
            ui.add(egui::Slider::new(&mut app.target_duration_secs, 0.1..=150.0));
        });
    });
}

