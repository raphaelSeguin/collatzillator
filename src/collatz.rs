use std::time::Duration;
use std::io::Write;
use rodio::Source;

// TO DO
// set_pitch
// interpolation entre les points (linéaire, quad..) pour les pitches plus petit que 0
// autres suites/séquences, voir OEIS (Recaman)

pub struct Collatz {
    init_value: u128,
    value: u128,
    pub exploration_step: u128
}
impl Collatz {
    pub fn new(init_value: u128) -> Collatz {
        Collatz { 
            init_value,
            value: init_value,
            exploration_step: 1
        }
    }
}
impl Iterator for Collatz {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        self.value = if self.value % 2 == 0 { 
            self.value / 2 
        } else {
            if self.value == 1 {
                self.init_value += self.exploration_step;
                self.init_value
            } else {
                3 * self.value + 1 
            }
        };
        Some((self.value % u16::MAX as u128) as f32 / u16::MAX as f32)
        // Some(self.value as f32 / u32::MAX as f32)
    }
}
impl Source for Collatz {
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { 44100 }
    fn current_frame_len(&self) -> Option<usize> { None }
    fn total_duration(&self) -> Option<Duration> { None }
}


