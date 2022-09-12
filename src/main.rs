use std::thread;
use std::time::Duration;
use std::fs::File;
use std::path::Path;
use rodio::{OutputStream, Source};
use clap::{Arg, Command};
use wav;

mod collatz;
use collatz::Collatz;

// TO DO
// wav: sauvegarde vers wav file
// use clap: parse cli arguments pitch, duration, start value, explo_step etc.

// Utils
// mtof ftom
// atodb dbtoa

fn main() {
    let matches = Command::new("Collatzillator")
        .version("0.1.0")
        .author("Raphaël Seguin")
        .about("Produces sound from syracuse/collatz suite")
        .arg(Arg::with_name("duration")
            .short('d')
            .long("duration")
            .takes_value(true)
            .help("Duration of the sound in ms"))
        .arg(Arg::with_name("init-value")
            .short('i')
            .long("init-value")
            .takes_value(true)
            .help("Initial value"))
        .arg(Arg::with_name("step")
            .short('s')
            .long("step")
            .takes_value(true)
            .help("Step value"))
        .arg(Arg::with_name("repeats")
            .short('r')
            .long("repeats")
            .takes_value(true)
            .help("Number of repetitions of a flight"))
        .arg(Arg::with_name("output-file")
            .short('o')
            .long("output")
            .takes_value(true)
            .help("Output file"))
        .get_matches();

    let default_duration: u64 = 1000;
    let mut duration: Duration = Duration::from_millis(default_duration);
    match matches.value_of("duration") {
        None => println!("Durée par défaut: {default_duration}msec"),
        Some(number) => {
            match number.parse::<u64>() {
                Err(_) => println!("Erreur: {number} n'est pas un nombre, durée par défaut: {default_duration}msec"),
                Ok(n) => duration = Duration::from_millis(n)
            }
        }
    };
    let default_init_value = 1;
    let mut init_value = default_init_value;
    match matches.value_of("init-value") {
        None => println!("Valeur initiale par défaut: {default_init_value}"),
        Some(number) => match number.parse::<u128>() {
            Err(_) => println!("Erreur: {number} n'est pas un nombre, valaue initiale par défaut: {default_init_value}"),
            Ok(number) => init_value = number
        }
    }
    let default_step_value = 0;
    let mut step_value = default_step_value;
    match matches.value_of("step") {
        None => println!("Pas par défaut: {default_step_value}"),
        Some(number) => match number.parse::<u128>() {
            Err(_) => println!("Erreur: {number} n'est pas un nombre, valaue initiale par défaut: {default_step_value}"),
            Ok(number) => step_value = number
        }
    }
    let default_repeats = 1;
    let mut repeats = default_repeats;
    match matches.value_of("repeats") {
        None => println!("Répétitions par défaut: {default_repeats}"),
        Some(number) => match number.parse::<u128>() {
            Err(_) => println!("Erreur: {number} n'est pas un nombre, valaue initiale par défaut: {default_repeats}"),
            Ok(number) => repeats = number
        }
    }


    let output_file = matches.value_of("output-file").unwrap_or("");

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
    
    
    
    let mut collatz = Collatz::new(init_value);
    collatz.exploration_step = step_value;
    collatz.repeats = repeats;

    if output_file.len() > 0 {
        println!("Génération du fichier {output_file}");
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
        let _result = stream_handle.play_raw(collatz.convert_samples().amplify(0.1).speed(1.));
        // .reverb()
        // .filter(1000)
        thread::sleep(duration);
    }
}