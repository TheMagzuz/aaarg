use std::sync::mpsc::Receiver;
use std::thread;
use std::sync::{Mutex, Arc, mpsc};
use std::io::BufReader;
use std::fs::File;
use rodio::source::SamplesConverter;
use rodio::{source::Source, Decoder, Sink};

use libaaarg::{AliasingParams, self};

pub fn load_samples(path: &String) -> SamplesConverter<Decoder<BufReader<File>>, f32> {
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();
        source.convert_samples::<f32>()
}

pub fn export_file(from: &String, to: &String, params: &AliasingParams) -> Receiver<()> {
    let from = from.clone();
    let to = to.clone();
    let params = (*params).clone();

    let (tx, rx) = mpsc::channel::<()>();

    thread::spawn(move || {
        let samples = load_samples(&from);
        let mut aliased = libaaarg::alias(samples, &params);

        libaaarg::encoding::write_audio(&to, &mut aliased);
        tx.send(()).unwrap();
    });
    rx
}

pub fn preview_aliasing(selected_file: &String, params: &AliasingParams, sink: Arc<Mutex<Sink>>) {
    let selected_file = selected_file.clone();
    let params = (*params).clone();
    thread::spawn(move || {
        let samples = load_samples(&selected_file);
        let sink = sink.lock().unwrap();

        let buf = libaaarg::alias(samples, &params);

        sink.append(buf);

    });
}

