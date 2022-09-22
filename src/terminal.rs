use crossterm::{
    cursor,
    style::{PrintStyledContent, Stylize},
    terminal::{self, size},
    ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Stdout, Write};

pub struct Term {
    stdout: Stdout,
    pub width: u16,
    pub height: u16,
}

impl Term {
    pub fn new() -> Self {
        let stdout = stdout();
        let (column, row) = terminal::size().unwrap();
        Self {
            stdout,
            width: column,
            height: row,
        }
    }

    fn clear(&mut self) {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .expect("Could not clear the screen");
    }

    fn flush(&mut self) {
        self.stdout.flush().expect("Could not flush the screen");
    }

    pub fn draw(&mut self, function: &mut dyn FnMut(&mut Self)) -> () {
        self.clear();
        function(self);
        self.flush();
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, what: &str) {
        self.stdout
            .queue(cursor::MoveTo(x as u16, y as u16))
            .expect("Something went wrong when drawing the circle")
            .queue(cursor::Hide)
            .expect("Could not hide the cursor")
            .queue(PrintStyledContent(what.red()))
            .expect("Something went wrong with the coloring");
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        todo!();
    }
}

