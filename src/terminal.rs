use crossterm::{
    cursor,
    style::{PrintStyledContent, Stylize},
    terminal,
    ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Stdout, Write};

/// Struct that handles the terminal
/// * stdout - is the standard output
/// * width - is the width of the terminal
/// * height - is the height of the terminal
pub struct Term {
    stdout: Stdout,
    pub width: u16,
    pub height: u16,
}

impl Term {
    /// Creates a new terminal, with everything set
    pub fn new() -> Self {
        const RATIO: f32 = 480.0/640.0;
        const RATIO_H: f32 = 640.0/480.0;
        const WIDTH: u16 = 160;// u32::min(f32::ceil(term.width as f32 * ratio) as u32, 640);
        const HEIGHT: u16 = 120;// u32::min(term.height.into(), 480);
        let stdout = stdout();
        let (width, height) = terminal::size().unwrap();
        println!("{}: {}", width, height);
        let width = f32::ceil(height as f32 * RATIO_H) as u16;
        println!("{}: {}", width, height);
        Self {
            stdout,
            width,
            height,
        }
    }

    /// Private function to clear the screen
    fn clear(&mut self) {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .expect("Could not clear the screen");
    }

    /// Flushes the screen
    fn flush(&mut self) {
        self.stdout.flush().expect("Could not flush the screen");
    }

    /// Function to draw to the sreen
    ///
    /// * function - is a function which takes the terminal and draws to it, clears and flushes
    /// before and after
    pub fn draw(&mut self, function: &mut dyn FnMut(&mut Self)) -> () {
        self.clear();
        function(self);
        self.flush();
    }

    /// Function which puts a string at an x and y
    pub fn put_pixel(&mut self, x: u32, y: u32, what: &str) {
        self.stdout
            .queue(cursor::MoveTo(x as u16, y as u16))
            .expect("Something went wrong when drawing the circle")
            .queue(cursor::Hide)
            .expect("Could not hide the cursor")
            .queue(PrintStyledContent(what.white()))
            .expect("Something went wrong with the coloring");
    }
}

