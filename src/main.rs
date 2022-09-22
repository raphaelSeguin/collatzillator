use std::thread;
use std::time::Duration;
use std::fs::File;
use std::path::Path;
use std::io::Error;
use rodio::{OutputStream};
use clap::{Parser};
use wav;

mod collatz;
use collatz::Collatz;

// TO DO

// Utils
// mtof ftom
// params in audio file output name
// atodb dbtoa
// ramp si plusieurs arguments
// loop -> valeur -end

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, allow_hyphen_values = true)]
struct Args {
   #[clap(short, long, value_parser, default_value_t = 1000)]
   duration: u64,
   #[clap(short, long, value_parser, default_value_t = 1)]
   init: u128,
   #[clap(short, long, value_parser, default_value_t = 1000)]
   nsteps: u128,
   #[clap(long, value_parser, default_value_t = 0)]
   drift: u128,
   #[clap(short, long, value_parser, default_value_t = 1)]
   step: u128,
   #[clap(short, long, value_parser, default_value_t = 1)]
   repeats: u128,
   #[clap(short, long, value_parser, default_value_t = String::from(""))]
   file: String,
//    #[clap(short, long, value_parser, default_value_t = 1.0)]
//    pitch: f32,
   #[clap(short, long, value_parser, default_value_t = 0.0)]
   octave: f32,
   #[clap(short, long, value_parser, default_value_t = false)]
   altern_phase: bool,
   #[clap(short, long, value_parser, default_value_t = false)]
   looping: bool,
//    #[clap(short, long, value_parser)]
//    list: Vec<f32>
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    let mut collatz = Collatz::new(args.init);
    collatz.exploration_step = args.step;
    collatz.repeats = args.repeats;
    collatz.altern_phase = args.altern_phase;
    collatz.set_octave(args.octave);
    collatz.max_value = args.init + args.nsteps;
    collatz.looping = args.looping;
    collatz.drift = args.drift;

    if args.file.len() > 0 {
        let mut audio: Vec<f32> = Vec::new();
        for _n in 0..(args.duration as f32* 44.1) as usize {
            audio.push(collatz.next().unwrap())
        }
        let header = wav::Header::new(
            wav::header::WAV_FORMAT_PCM,
            1,
            44_100,
            32
        );
        let data = wav::BitDepth::ThirtyTwoFloat(audio);

        let file_name = String::from(args.file) + ".wav";
        let mut out_file = File::create(Path::new(&file_name))?;
        wav::write(header, &data, &mut out_file)?;
    } else {
        // let source = collatz.buffered().convert_samples().amplify(1.0);
        let _result = stream_handle.play_raw(collatz);
        // .speed(args.pitch)
        // .reverb(Duration::from_millis(100), 0.7)
        // .filter(1000)
        thread::sleep(Duration::from_millis(args.duration));
    }
    Ok(())
}