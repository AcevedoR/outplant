use std::fs::File;
use std::io::Read;

fn main() {
    let mut data = String::new();
    let mut file = File::open("events.json").unwrap();
    file.read_to_string(&mut data).expect("could not read events file");

    println!("cargo:rustc-env=UNNAMED_GAME_EVENTS_JSON={}", data.replace('\n', ""));
    // TODO find a way to pass the file with line breaks
}