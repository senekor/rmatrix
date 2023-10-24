//! This module contains the [`Matrix`] struct and its associated methods.

mod fading_character;
mod hot_character;

use std::vec;

use rand::Rng;
use ratatui::{
    style::{Color, Style},
    widgets::Widget,
};

use self::{fading_character::FadingChar, hot_character::HotChar};

/// Encapsulates the logic of the iconc matrix animation.
/// Holds both the "current state" as well as configuration.
#[derive(Debug)]
pub struct Matrix {
    fading_chars: Vec<Vec<Option<FadingChar>>>,
    hot_chars: Vec<HotChar>,

    width: usize,
    height: usize,
    // TODO configuration (speed, density, ...)
}

impl Matrix {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            fading_chars: vec![vec![None; height]; width],
            hot_chars: Vec::new(),
            width,
            height,
        }
    }

    pub fn tick(&mut self) {
        self.fading_chars
            .iter_mut()
            .flat_map(|col| col.iter_mut())
            .filter_map(|fading_char| fading_char.as_mut())
            .for_each(FadingChar::tick);

        for fading_char in self.fading_chars.iter_mut().flat_map(|col| col.iter_mut()) {
            if fading_char.as_ref().map_or(false, FadingChar::has_faded) {
                *fading_char = None;
            }
        }

        for hot_char in self.hot_chars.iter_mut() {
            if let Some((x, y, ticks_to_live)) = hot_char.tick() {
                if x < self.width && y < self.height {
                    self.fading_chars[x][y] = Some(FadingChar::new(ticks_to_live));
                }
            }
        }

        // TODO tune probabilities, distributions and make configurable
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.1) {
            let x = rng.gen_range(0..self.width);
            let speed = rng.gen_range(0.05..0.5);
            let max_len = rng.gen_range(1..=self.height.saturating_sub(8) + 1);
            self.hot_chars.push(HotChar::new(x, speed, max_len));
        }

        self.hot_chars
            .retain(|hot_char| hot_char.smaller_than(self.height as f64));
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        if self.fading_chars.len() > width {
            self.fading_chars.truncate(width);
        } else {
            self.fading_chars.resize(width, vec![None; height]);
        }
        for col in self.fading_chars.iter_mut() {
            if col.len() > height {
                col.truncate(height);
            } else {
                col.resize(height, None);
            }
        }
    }
}

impl Widget for &Matrix {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        buf.set_style(area, Style::default().bg(Color::Rgb(0, 0, 0)));

        let fading_chars = self.fading_chars.iter().enumerate().flat_map(|(x, col)| {
            col.iter()
                .enumerate()
                .flat_map(move |(y, f)| f.as_ref().map(|f| (x as u16, y as u16, f)))
        });
        for (x, y, fading_char) in fading_chars {
            if x >= area.width || y >= area.height {
                continue;
            }
            let x = x + area.x;
            let y = y + area.y;

            let brightness = fading_char.get_brightness();

            buf.get_mut(x, y)
                .set_char(fading_char.get_char())
                .set_fg(Color::Rgb(0, brightness, 0));
            // .set_style(ratatui::style::Style::default().fg(Color::Rgb(0, brightness, 0)));
        }
        for hot_char in &self.hot_chars {
            let x = hot_char.get_x() as u16 + area.x;
            let y = hot_char.get_y() as u16 + area.y;
            if x >= area.width || y >= area.height {
                continue;
            }
            buf.get_mut(x, y)
                .set_char(hot_char.get_char())
                .set_fg(Color::Rgb(u8::MAX, u8::MAX, u8::MAX));
        }
    }
}

fn get_random_char() -> char {
    rand::thread_rng().gen_range('ァ'..='ヺ')
}
