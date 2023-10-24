use ratatui::{style::Style, Frame};

struct FallingLine {
    x: u16,
    y: u16,
    // speed: u8,
}

pub struct Matrix {
    falling_lines: Vec<FallingLine>,
}

impl Matrix {
    pub fn draw(&self, frame: &mut Frame) {
        // let area = frame.size();
        let buffer = frame.buffer_mut();
        for line in &self.falling_lines {
            for y in line.y.saturating_sub(3)..line.y {
                buffer.set_string(line.x, y, "x", Style::default())
            }
        }
    }
}

pub fn example() -> Matrix {
    Matrix {
        falling_lines: vec![FallingLine { x: 10, y: 20 }, FallingLine { x: 15, y: 5 }],
    }
}
