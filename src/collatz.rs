use std::time::Duration;
use rodio::Source;

// TO DO
// set_pitch
// interpolation entre les points (linéaire, quad..) pour les pitches plus petit que 0
// autres modes de conversion en f32 que modulo u16 ?
// autres modes d'exploration: 
//      - maximum + 1
//      - count
// autres suites/séquences, voir OEIS (Recaman)
// variantes 

pub struct Collatz {
    init_value: u128,
    min_value: u128,
    pub max_value: u128,
    value: u128,
    pub exploration_step: u128,
    pub repeats: u128,
    maximum: u128,
    count_samples: u128, // pour contraindre une longueur de table ?
    count_repeats: u128,
    cursor: f32,
    increment: f32,
    pub altern_phase: bool,
    pub looping: bool,
    pub drift: u128,
    pub interpolation: bool,
}
impl Collatz {
    pub fn new(init_value: u128) -> Collatz {
        Collatz { 
            init_value,
            min_value: init_value,
            max_value: u128::max_value(),
            looping: false,
            value: init_value,
            exploration_step: 1,
            maximum: 0,
            repeats: 1,
            count_samples: 0,
            count_repeats: 0,
            cursor: 0.0,
            increment: 1.0,
            altern_phase: true,
            drift: 0,
            interpolation: false,
        }
    }
    pub fn set_octave(&mut self, octave: f32) {
        self.increment = 2.0f32.powf(octave);
    }
    fn next_value(&mut self) {
        self.value = if self.value == 1 {
            self.count_repeats += 1;
            if self.count_repeats == self.repeats {
                self.count_repeats = 0;
                self.init_value += self.exploration_step;
                if self.looping && self.init_value >= self.max_value {
                    self.min_value += self.drift;
                    self.max_value += self.drift;
                    self.init_value = self.min_value;
                }
            }
            self.init_value
        } else {
            if self.value % 2 == 0 { 
                self.value / 2 
            } else {
                3 * self.value + 1
            }
        };
        self.maximum = if self.value > self.maximum { self.value } else { self.maximum };
        self.count_samples += 1;
    }
    pub fn interpolate(&mut self) -> f32 {
        self.cursor += self.increment;
        while self.cursor >= 1.0 {
            self.next_value();
            self.cursor += -1.0
        }
        if self.altern_phase {
            ((self.value % u16::MAX as u128) as f32 / u16::MAX as f32) * (((self.count_repeats % 2) * 2) as f32 - 1.0)
        } else {
            (self.value % u16::MAX as u128) as f32 / u16::MAX as f32
        }
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


