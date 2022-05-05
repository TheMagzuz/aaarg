// TODO: Maybe consolidate these into a trait?
// I don't really see a reason for doing it right now,
// but it might come in handy later.
pub mod file;
pub mod aliasing;
pub mod playback;
pub mod export;
pub mod stutter;

use std::ops::RangeInclusive;
use eframe::egui;

pub fn range_to_sliders<I>(ui: &mut egui::Ui, label: &str, range: &mut RangeInclusive<I>, bounds: &RangeInclusive<I>)
where I: egui::emath::Numeric {
    ui.label(label);


    let (mut min, mut max) = range.clone().into_inner();

    ui.horizontal(|ui| {
        ui.label("Min: ");
        ui.add(egui::Slider::new(&mut min, bounds.clone()));
    });

    ui.horizontal(|ui| {
        ui.label("Max: ");
        ui.add(egui::Slider::new(&mut max, bounds.clone()));
    });

    *range = min..=max;
}
