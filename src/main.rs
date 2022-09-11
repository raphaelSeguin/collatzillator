use std::thread;
use std::time::Duration;
use std::fs::File;
use std::path::Path;
use rodio::{OutputStream, Source};

mod collatz;
use collatz::Collatz;

// TO DO
// wav: sauvegarde vers wav file
// use clap: parse cli arguments pitch, duration, start value, explo_step etc.

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut collatz = Collatz::new(55555555555555555);
    collatz.exploration_step = 1;
    let _result = stream_handle.play_raw(collatz.convert_samples().amplify(0.1).speed(0.5));
    // .filter(1000)
    thread::sleep(Duration::from_millis(60000));
}