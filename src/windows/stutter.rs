use std::time::Duration;

use eframe::egui;

use crate::ui::MainApp;

pub fn show(app: &mut MainApp, ctx: &egui::Context) {
    egui::Window::new("Stutter").show(ctx, |ui| {
        crate::windows::range_to_sliders(ui, "Stutter count", &mut app.aliasing_params.stutter_count, &(0..=20u16));

        let app_stutter_duration = app.aliasing_params.stutter_duration.clone();
        let mut stutter_duration = app_stutter_duration.start().as_secs_f32()..=app_stutter_duration.end().as_secs_f32();
        crate::windows::range_to_sliders(ui, "Stutter duration", &mut stutter_duration, &(0f32..=2f32));
        app.aliasing_params.stutter_duration = Duration::from_secs_f32(*stutter_duration.start())..=Duration::from_secs_f32(*stutter_duration.end());

        let app_stutter_piece_length = app.aliasing_params.stutter_piece_length.clone();
        let mut stutter_piece_length = app_stutter_piece_length.start().as_secs_f32()..=app_stutter_piece_length.end().as_secs_f32();
        crate::windows::range_to_sliders(ui, "Stutter piece length", &mut stutter_piece_length, &(0f32..=2f32));
        app.aliasing_params.stutter_piece_length = Duration::from_secs_f32(*stutter_piece_length.start())..=Duration::from_secs_f32(*stutter_piece_length.end());
    });
}
