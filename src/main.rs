use std::thread;
use std::time::Duration;
use std::fs::File;
use std::path::Path;
use rodio::{OutputStream, Source};
use clap::{Arg, App};

mod collatz;
use collatz::Collatz;

// TO DO
// wav: sauvegarde vers wav file
// use clap: parse cli arguments pitch, duration, start value, explo_step etc.

fn main() {
    let matches = App::new("Collatzillator")
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
            .help("Initial value "))
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

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut collatz = Collatz::new(init_value);
    collatz.exploration_step = 1;
    let _result = stream_handle.play_raw(collatz.convert_samples().amplify(0.1).speed(0.5));
    // .filter(1000)
    thread::sleep(duration);
}