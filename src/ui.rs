use std::sync::{Mutex, Arc, mpsc::Receiver};
use rodio::{Sink, OutputStreamHandle, OutputStream};
use eframe::{egui, epi};
use tinyfiledialogs::{open_file_dialog, save_file_dialog};

use crate::audio::{AliasingParams, self};

pub fn run() {
    let app = MainApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

struct MainApp {
    sink: Arc<Mutex<Sink>>,
    // These need to be part of the struct in order to keep the stream alive
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    selected_file: String,
    export_file: String,
    finished_writing_receiver: Option<Receiver<()>>,
    factor: usize,
    target_duration_secs: f32,
}

impl MainApp {

    fn get_aliasing_params(&self) -> AliasingParams {
        AliasingParams::from_secs(self.target_duration_secs, self.factor)
    }

    fn show_file_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("File").show(ctx, |ui| {
            ui.label(format!("Current file: {}", self.selected_file));
            if ui.button("Select file").clicked() {
                self.selected_file = if let Some(file) = open_file_dialog("Open", "~", None) {
                    file
                } else {
                    "".to_owned()
                };
            }
        });
    }

    fn show_playback_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Playback").show(ctx, |ui| {
            let lock = self.sink.try_lock(); 
            let lock_acquired = lock.is_ok();
            let stopped = if let Ok(s) = lock {
                s.empty()
            } else {
                true
            };
            if stopped {
                if ui.add_enabled(self.selected_file != "" && lock_acquired, egui::Button::new("Play")).clicked() {
                    let params = self.get_aliasing_params();
                    crate::audio::preview_aliasing(
                        &self.selected_file,
                        &params,
                        self.sink.clone()
                    );
                }
            } else {
                if ui.button("Stop").clicked() {
                    self.sink.lock().as_ref().unwrap().stop();
                }
            }
        });
    }

    fn show_aliasing_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Aliasing").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Aliasing factor:");
                ui.add(egui::Slider::new(&mut self.factor, 1..=1000));
            });
            ui.horizontal(|ui| {
                ui.label("Target output duration (secs):");
                ui.add(egui::Slider::new(&mut self.target_duration_secs, 0.1..=150.0));
            });
        });
    }

    fn show_export_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Export").show(ctx, |ui| {
            ui.label(format!("Current file: {}", self.export_file));
            if ui.button("Select file").clicked() {
                self.export_file = if let Some(file) = save_file_dialog("Open", "output.wav") {
                    file
                } else {
                    "".to_owned()
                };
            }

            ui.separator();

            let exporting = if let Some(rx) = &self.finished_writing_receiver {
                if rx.try_recv().is_ok() {
                    self.finished_writing_receiver = None;
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

            let button_enabled = !exporting && self.export_file != "" && self.selected_file != "";
            
            if ui.add_enabled(button_enabled, egui::Button::new(exporting_text)).clicked() {
                let params = self.get_aliasing_params();
                self.finished_writing_receiver = Some(audio::export_file(&self.selected_file, &self.export_file, &params));
            }
        });
    }
}

impl Default for MainApp {
    fn default() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            sink: Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap())),
            _stream: stream,
            _stream_handle: stream_handle,
            selected_file: "".to_owned(),
            export_file: "".to_owned(),
            finished_writing_receiver: None,
            factor: 100,
            target_duration_secs: 5.0,
        }
    }
}

impl epi::App for MainApp {
    fn name(&self) -> &str {
        "aaarg v0.1"
    }

    fn setup(&mut self, ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        ctx.set_visuals(egui::Visuals::dark());
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |_ui| {
            self.show_file_window(ctx);
            self.show_aliasing_window(ctx);
            self.show_playback_window(ctx);
            self.show_export_window(ctx);
        });
    }
}
