use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

fn main() {
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("assets/success.mp3").unwrap());

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = Decoder::new(file).unwrap();
    _ = stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
}
