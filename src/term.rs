use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};

use crate::circle::Point;

pub struct Term {
    stdout: Stdout,
}

impl Term {
    pub fn new() -> Self {
        Self { stdout: stdout() }
    }

    pub fn clear(&mut self) {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .expect("Could not clear the screen");
    }

    pub fn flush(&mut self) {
        self.stdout.flush().expect("Could not flush the screen");
    }

    pub fn draw_line(&mut self, start: &Point, end: &Point, what: &str) {
        for each_y in start.y..=end.y {
            for each_x in start.x..=end.x {
                println!("{each_y}:{each_x}");
                let each_x = (each_y-start.y)*(end.x-start.x)/((end.y-start.y) + start.x);
                self.put_pixel(each_x as i16, each_y as i16, what);
            }
        }
    }

    pub fn put_pixel(&mut self, x: i16, y: i16, what: &str) {
        self.stdout
            .queue(cursor::MoveTo(x as u16, y as u16))
            .expect("Something went wrong when drawing the circle")
            .queue(style::PrintStyledContent(what.magenta()))
            .expect("Something went wrong with the coloring");
    }
}
