extern crate gnuplot;

use std::path::Path;
use gnuplot::{Figure, Graph, Caption, AxesCommon};
use onset_detection::extractor::Music;
use onset_detection::utils::get_path;
use onset_detection::detector::Detector;
use onset_detection::printtm;

fn main() {
    printtm!("Starting processing file");

    let path_str = get_path();
    let path = Path::new(&path_str);
    let music = Music::from_file(&path)
        .expect("Error occured during parsing");

    printtm!("Parsing finished");

    let samples = music.frames.samples();

    printtm!("Got a samples");

    let fft = samples.fft();

    printtm!("Got a fft");

    let mut x = Vec::new();
    let mut y = Vec::new();

    for i in 1..fft.len() {
        x.push(i);
        y.push(*fft.get(i).unwrap() as f32);
    }

    let mut fg = Figure::new();

    fg.axes2d()
        .set_title("Перетворення Фур'є", &[])
        .set_legend(Graph(0.2), Graph(0.2), &[], &[])
        .set_x_label("x - крок", &[])
        .set_y_label("y - частотний діапазон", &[])
        .lines(&x, &y, &[Caption(&music.name)]);

    fg.show();
}
