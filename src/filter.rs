struct DelayLine {
    buffer: Vec<f32>,
    index: usize
}

impl DelayLine {
    pub fn new(size: usize) -> DelayLine {
        DelayLine {
            buffer: vec![0.0; size],
            index: 0
        }
    }
    pub fn read(&self) -> f32 {
        self.buffer[self.index]
    }
    pb write_and_advance(&mut self, input: f32) {
        self.buffer[self.index] = input;
        self.index = if self.index < len(buffer) - 1 {
            self.index + 1
        } else {
            0
        }
    }
}

struct OnePole {
    delayLine: DelayLine,
}

impl OnePole {
    pub fn new() -> OnePole {
        OnePole {
            delayLine(1)
        }
    }
    pub fn tick(input: f32) -> f32 {
        let delayed
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}