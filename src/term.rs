mod bresenham;
mod dda;

use std::io::{stdout, Stdout, Write};

use crossterm::{
    cursor,
    style::{PrintStyledContent, StyledContent, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};

use crate::circle::{Circle, Point};

/// Struct that abstracts where the information is written
pub struct Term {
    stdout: Stdout,
}

impl Term {
    pub fn new() -> Self {
        Self { stdout: stdout() }
    }

    /// Method to clear the screen
    pub fn clear(&mut self) {
        self.stdout
            .execute(terminal::Clear(terminal::ClearType::All))
            .expect("Could not clear the screen");
    }

    /// Method to flush the input
    pub fn flush(&mut self) {
        self.stdout.flush().expect("Could not flush the screen");
    }

    /// Method to draw to the screen
    /// Clears the screen first, and after flushes it
    ///
    /// ## Arguments
    ///
    /// * func - Closure where the terminal is used to write to
    pub fn draw(&mut self, func: &mut dyn FnMut(&mut Self)) {
        self.clear();
        func(self);
        self.flush();
    }

    /// Function to draw the circle to the screen
    /// It uses some algorithm that I cannot use right now
    ///
    /// ## Arguments
    ///
    /// * circle - The circle to draw
    pub fn draw_circle(&mut self, circle: &Circle) {
        let mut d: i16 = 3 - (2 * circle.radius as i16);
        let mut x: i16 = 0;
        let mut y = circle.radius as i16;

        let x0 = circle.center.x as i16;
        let y0 = circle.center.y as i16;

        let what = "*".magenta();
        while x <= y {
            self.put_pixel(x0 + x, y0 + y, &what);
            self.put_pixel(x0 + x, y0 - y, &what);
            self.put_pixel(x0 - x, y0 + y, &what);
            self.put_pixel(x0 - x, y0 - y, &what);
            self.put_pixel(x0 + y, y0 + x, &what);
            self.put_pixel(x0 + y, y0 - x, &what);
            self.put_pixel(x0 - y, y0 + x, &what);
            self.put_pixel(x0 - y, y0 - x, &what);

            if d < 0 {
                d = d + (4 * x) + 6;
            } else {
                d = d + 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
        /*
        let radius = circle.radius as i16;
        let diam = (radius << 1) as i16;

        let mut x: i16 = radius - 1;
        let mut y: i16 = 0;
        let mut dx: i16 = 1;
        let mut dy: i16 = 1;
        let mut err: i16 = dx - diam;

        let x0 = circle.center.x as i16;
        let y0 = circle.center.y as i16;

        let what = "*".magenta();

        while x >= y {
            self.put_pixel(x0 + x, y0 + y, &what);
            self.put_pixel(x0 + y, y0 + x, &what);
            self.put_pixel(x0 - y, y0 + x, &what);
            self.put_pixel(x0 - x, y0 + y, &what);
            self.put_pixel(x0 + y, y0 - x, &what);
            self.put_pixel(x0 + x, y0 - y, &what);
            self.put_pixel(x0 - x, y0 - y, &what);
            self.put_pixel(x0 - y, y0 - x, &what);

            if err <= 0 {
                y += 1;
                err += dy;
                dy += 2;
            }

            if err > 0 {
                x -= 1;
                dx += 2;
                err += dx - diam;
            }
        }
        *
        */
    }

    /// Method to draw a line to the terminal
    ///
    /// ## Arguments
    /// * start - The point to start the line from
    /// * end - The point to end the line
    ///
    /// Both use the Point-struct
    ///
    /// * what - What to write on the lines. Uses StyledContent
    pub fn draw_line(&mut self, start: &Point, end: &Point, what: &StyledContent<&str>) {
        dda::draw_line_float(self, start, end, what);
    }

    /// Method to put a pixel at the x and y coordinates
    ///
    /// ## Arguments
    /// * x - The x-position to write to
    /// * y - the y-position to write to
    /// * what - What to write at that position
    pub fn put_pixel(&mut self, x: i16, y: i16, what: &StyledContent<&str>) {
        let x = x << 1;
        self.stdout
            .queue(cursor::MoveTo(x as u16, y as u16))
            .expect("Something went wrong when drawing the circle")
            .queue(cursor::Hide)
            .expect("Could not hide the cursor")
            .queue(PrintStyledContent(*what))
            .expect("Something went wrong with the coloring");
    }
}
