use rand::Rng;

use super::get_random_char;

#[derive(Debug, Clone)]
pub struct FadingChar {
    c: char,
    elapsed_ticks: usize,
    ticks_to_live: f64,
}

impl FadingChar {
    pub fn new(ticks_to_live: f64) -> Self {
        Self {
            c: get_random_char(),
            elapsed_ticks: 0,
            ticks_to_live,
        }
    }

    pub fn tick(&mut self) {
        self.elapsed_ticks += 1;

        // Very rarely change the character of a fading char.
        // This is what happens in the original movie, screensavers etc.
        // but not in the oriiginal cmatrix, which makes it look a bit static.
        if rand::thread_rng().gen_bool(0.002) {
            self.c = get_random_char();
        }
    }

    pub fn get_char(&self) -> char {
        self.c
    }

    pub fn has_faded(&self) -> bool {
        self.elapsed_ticks as f64 > self.ticks_to_live
    }

    pub fn get_brightness(&self) -> u8 {
        (((self.ticks_to_live - self.elapsed_ticks as f64) / self.ticks_to_live) * u8::MAX as f64)
            .floor() as u8
    }
}
