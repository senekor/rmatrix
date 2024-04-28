use crate::matrix::get_random_char;

#[derive(Debug, PartialEq)]
pub struct HotChar {
    c: char,
    x: usize,
    y: f64,
    /// cells per tick
    speed: f64,
    tail_len: usize,
    ticks_since_last_char_change: usize,
}

// Why can we implement Eq for HotChar, even though it holds floats?
// We only generate a maximum of one HotChar per tick, so the only
// way for two of them to have the same y value is by having different
// speeds. This means that they are not equal.
impl Eq for HotChar {}

impl Ord for HotChar {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.y != other.y {
            return self.y.partial_cmp(&other.y).unwrap();
        }
        // identical y? speed must be different, since we only generate
        // one HotChar per tick.
        self.speed.partial_cmp(&other.speed).unwrap()
    }
}
impl PartialOrd for HotChar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl HotChar {
    pub fn new(x: usize, speed: f64, max_len: usize) -> Self {
        debug_assert!(speed > 0.0);

        Self {
            c: get_random_char(),
            y: 0.0,
            x,
            tail_len: max_len,
            speed,
            ticks_since_last_char_change: 0,
        }
    }

    /// Returns a new fading character if an integer boundary was crossed.
    pub fn tick(&mut self) -> Option<(usize, usize, f64)> {
        let old_y = self.y;
        let new_y = old_y + self.speed;
        self.y = new_y;

        self.ticks_since_last_char_change += 1;
        if self.ticks_since_last_char_change >= 6 {
            self.c = get_random_char();
            self.ticks_since_last_char_change = 0;
        }

        // if we cross an integer boundary, we have to update the tail
        if old_y.floor() != new_y.floor() {
            let ticks_per_cell = 1.0 / self.speed;
            let ticks_to_live = ticks_per_cell * self.tail_len as f64;
            return Some((self.x, old_y.floor() as usize, ticks_to_live));
        }
        None
    }

    pub fn smaller_than(&self, height: f64) -> bool {
        self.y < height
    }

    pub fn get_x(&self) -> usize {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_char(&self) -> char {
        self.c
    }
}
