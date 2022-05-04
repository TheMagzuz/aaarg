use std::sync::{Mutex, Arc, mpsc::Receiver};
use rodio::{Sink, OutputStreamHandle, OutputStream};
use eframe::{egui, epi};

use libaaarg::AliasingParams;
use crate::windows;

pub fn run() {
    let app = MainApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

pub struct MainApp {
    pub sink: Arc<Mutex<Sink>>,
    // These need to be part of the struct in order to keep the stream alive
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    pub selected_file: String,
    pub export_file: String,
    pub finished_writing_receiver: Option<Receiver<()>>,
    pub factor: usize,
    pub factor_variation: usize,
    pub target_duration_secs: f32,
}

impl MainApp {
    pub fn get_aliasing_params(&self) -> AliasingParams {
        AliasingParams::from_secs(self.target_duration_secs, self.factor, self.factor_variation)
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
            factor_variation: 0,
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
            windows::file::show(self, ctx);
            windows::aliasing::show(self, ctx);
            windows::playback::show(self, ctx);
            windows::export::show(self, ctx);
        });
    }
}
