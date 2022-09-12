use std::thread;
use std::time::Duration;
use std::fs::File;
use std::path::Path;
use rodio::{OutputStream, Source};
use clap::{Command, Parser};
use wav;

mod collatz;
use collatz::Collatz;

// TO DO
// wav: sauvegarde vers wav file
// use clap: parse cli arguments pitch, duration, start value, explo_step etc.

// Utils
// mtof ftom
// atodb dbtoa

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
   #[clap(short, long, value_parser, default_value_t = 1000)]
   duration: u64,
   #[clap(short, long, value_parser, default_value_t = 1)]
   init: u128,
   #[clap(short, long, value_parser, default_value_t = 1)]
   step: u128,
   #[clap(short, long, value_parser, default_value_t = 1)]
   repeats: u128,
   #[clap(short, long, value_parser, default_value_t = String::from(""))]
   output: String,
   #[clap(short, long, value_parser, default_value_t = 1.0)]
   pitch: f32,
}

fn main() {
    let args = Args::parse();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    let mut collatz = Collatz::new(args.init);
    collatz.exploration_step = args.step;
    collatz.repeats = args.repeats;

    if args.output.len() > 0 {

        // let mut out_file = File::create(Path::new("{output_file}.wav"));
        // // audio format, channel count, sampling rate, bit depth
        // let header = wav::Header::new(wav::header::WAV_FORMAT_PCM, 1, 44_100, 16);

        // // on  a un iterator en f32 et on veut un Vec<i16>
        // let data = wav::bit_depth::BitDepth::ThirtyTwoFloat(Vec::<f32>::from_iter(collatz.take(44100)));
        // let file_result = wav::write(header, &data, &mut out_file);
        // match file_result {
        //     Ok(()) => println!("OK"),
        //     Err(_) => println!("Erreur de génération de fichier")
        // };
    } else {
        let _result = stream_handle.play_raw(collatz.convert_samples().amplify(0.1).speed(args.pitch));
        // .reverb()
        // .filter(1000)
        thread::sleep(Duration::from_millis(args.duration));
    }
}