mod circle;
mod time;

use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand, Result,
};
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use circle::Circle;
use time::Time;

const DELAY: u64 = 1;

fn draw_seconds(put_pixel: &mut dyn FnMut(i16, i16, &str), circle: &Circle, second: usize) {
    let angle = (second / 6) as usize;
    let point = circle.get_point(angle, |radius, angle| -> usize {
        radius * (if angle > 0 { angle - 1 } else { angle })
    });
}

fn draw_minutes(put_pixel: &mut dyn FnMut(i16, i16, &str), circle: &Circle, minutes: usize) {
    let angle = (minutes / 6) as usize;
    let point = circle.get_point(angle, |radius, angle| -> usize {
        radius * (if angle > 0 { angle / 2 } else { angle })
    });
}

fn draw_hours(put_pixel: &mut dyn FnMut(i16, i16, &str), circle: &Circle, hours: usize) {
    let angle = (hours / 6) as usize;
    let point = circle.get_point(angle, |radius, angle| -> usize {
        radius * (if angle > 0 { angle - 1 } else { angle })
    });
}

fn draw_clock(put_pixel: &mut dyn FnMut(i16, i16, &str), circle: &Circle) {
    let radius = circle.radius as i16;
    let diam = (radius << 1) as i16;

    let mut x: i16 = radius - 1;
    let mut y: i16 = 0;
    let mut dx: i16 = 1;
    let mut dy: i16 = 1;
    let mut err: i16 = dx - diam;

    let x0 = circle.center.x as i16;
    let y0 = circle.center.y as i16;

    let what = "*";

    while x >= y {
        put_pixel(x0 + x, y0 + y, what);
        put_pixel(x0 + y, y0 + x, what);
        put_pixel(x0 - y, y0 + x, what);
        put_pixel(x0 - x, y0 + y, what);
        put_pixel(x0 + y, y0 - x, what);
        put_pixel(x0 + x, y0 - y, what);
        put_pixel(x0 - x, y0 - y, what);
        put_pixel(x0 - y, y0 - x, what);

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
}

fn main() -> Result<()> {
    let mut stdout = stdout();

    loop {
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        let time = Time::get_current_time();
        let seconds = Time::format_seconds(&time);
        let minutes = Time::format_minutes(&time);
        let hours = Time::format_hours(&time);

        let circle = Circle::new(20, 20);

        let mut put_pixel = |x: i16, y: i16, what: &str| {
            let x = x << 2;
            stdout
                .queue(cursor::MoveTo(x as u16, y as u16))
                .expect("Something went wrong when drawing the circle")
                .queue(style::PrintStyledContent(what.magenta()))
                .expect("Something went wrong with the coloring");
        };

        draw_clock(&mut put_pixel, &circle);

        draw_seconds(&mut put_pixel, &circle, seconds);
        draw_minutes(&mut put_pixel, &circle, minutes);
        draw_hours(&mut put_pixel, &circle, hours);

        stdout.flush()?;

        thread::sleep(Duration::from_secs(DELAY));
    }

    //stdout.execute(terminal::Clear(terminal::ClearType::All))?;
}
