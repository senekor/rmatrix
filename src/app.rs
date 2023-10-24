use crate::matrix::Matrix;

#[derive(Debug)]
pub struct App {
    matrix: Matrix,
    should_quit: bool,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(width: u16, height: u16) -> Self {
        let (width, height) = term_to_matrix_size(width, height);
        Self {
            matrix: Matrix::new(width, height),
            should_quit: false,
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.matrix.tick()
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        let (width, height) = term_to_matrix_size(width, height);
        self.matrix.resize(width, height);
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn get_matrix(&self) -> &Matrix {
        &self.matrix
    }
}

fn term_to_matrix_size(width: u16, height: u16) -> (usize, usize) {
    let extra_width = width as usize * 2;
    let extra_height = height as usize * 2;
    (extra_width, extra_height)
}
