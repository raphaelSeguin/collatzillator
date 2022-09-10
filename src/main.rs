use std::thread;
use std::time::Duration;
use rodio::{OutputStream, Source};
use rand;


struct MySource {
    sample_rate: u32,
}
impl MySource {
    fn new(sample_rate: u32) -> MySource {
        MySource { 
            sample_rate,
        }
    }
}
impl Source for MySource {
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn current_frame_len(&self) -> Option<usize> { None }
    fn total_duration(&self) -> Option<Duration> { None }
}
impl Iterator for MySource {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        Some( rand::random())
    }
}


fn main() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let mySource = MySource::new(44100);
    let _result = stream_handle.play_raw(mySource.convert_samples());
    thread::sleep(Duration::from_millis(1500));
}