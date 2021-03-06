extern crate serde_json;

use std::path::Path;
use onset_detection::extractor::Music;
use onset_detection::utils::get_path;

fn main() {
    let path_str = get_path();
    let path = Path::new(&path_str);
    let music = Music::from_file(&path)
        .expect("Error occured during parsing");

    let samples = music.frames.samples();

    println!("{}", serde_json::to_string(&samples).unwrap());
}
