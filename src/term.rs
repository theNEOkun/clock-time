use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor,
    style::{self, PrintStyledContent, StyledContent},
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

    pub fn draw_line(&mut self, start: &Point, end: &Point, what: &StyledContent<&str>) {
        let (start_x, end_x) = if start.x < end.x {
            (start.x, end.x)
        } else {
            (end.x, start.x)
        };
        let (start_y, end_y) = if start.y < end.y {
            (start.y, end.y)
        } else {
            (end.y, start.y)
        };
        for each_y in start_y..=end_y {
            for each_x in start_x..=end_x {
                //let each_x = (each_y - start_y) * (end_x - start_x) / (end_y - start_y) + start_x;
                self.put_pixel(each_x as i16, each_y as i16, what);
            }
        }
    }

    pub fn put_pixel(&mut self, x: i16, y: i16, what: &StyledContent<&str>) {
        let x = x << 1;
        self.stdout
            .queue(cursor::MoveTo(x as u16, y as u16))
            .expect("Something went wrong when drawing the circle")
            .queue(PrintStyledContent(*what))
            .expect("Something went wrong with the coloring");
    }
}
