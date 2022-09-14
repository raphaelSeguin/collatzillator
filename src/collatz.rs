use std::time::Duration;
use std::io::Write;
use rodio::Source;

// TO DO
// set_pitch
// interpolation entre les points (linéaire, quad..) pour les pitches plus petit que 0
// autres modes de conversion en f32 que modulo u16 ?
// autres modes d'exploration: 
//      - maximum + 1
//      - count
// autres suites/séquences, voir OEIS (Recaman)

pub struct Collatz {
    init_value: u128,
    value: u128,
    pub exploration_step: u128,
    pub repeats: u128,
    maximum: u128,
    count_samples: u128, // pour contraindre une longueur de table ?
    count_repeats: u128,
    next_value: u128,
    cursor: f32,
    increment: f32,
}
impl Collatz {
    pub fn new(init_value: u128) -> Collatz {
        Collatz { 
            init_value,
            value: init_value,
            exploration_step: 1,
            maximum: 0,
            repeats: 1,
            count_samples: 0,
            count_repeats: 0,
            next_value: 0,
            cursor: 0.0,
            increment: 1.0,
        }
    }
    pub fn set_pitch(&mut self, pitch: f32) {
        self.increment = pitch;
    }
    fn next_value(&mut self) {
        self.value = if self.value % 2 == 0 { 
            self.value / 2 
        } else {
            if self.value == 1 {
                self.count_repeats += 1;
                if self.count_repeats == self.repeats {
                    self.count_repeats = 0;
                    self.init_value += self.exploration_step;
                }
                self.init_value
            } else {
                3 * self.value + 1 
            }
        };
        self.maximum = if self.value > self.maximum { self.value } else { self.maximum };
        self.count_samples += 1;
    }
    pub fn interpolate(&mut self) -> f32 {
        self.cursor += self.increment;
        if self.cursor >= 1.0 {
            self.next_value();
            self.cursor %= 1.0;
        }

        (self.value % u16::MAX as u128) as f32 / u16::MAX as f32
    }
}
impl Iterator for Collatz {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.interpolate())
    }
}
impl Source for Collatz {
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { 44100 }
    fn current_frame_len(&self) -> Option<usize> { None }
    fn total_duration(&self) -> Option<Duration> { None }
}


