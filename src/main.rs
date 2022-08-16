mod circle;
mod time;
mod term;
use std::{
    thread,
    time::Duration,
};

use circle::Circle;
use time::Time;
use term::Term;

const DELAY: u64 = 1;

fn draw_seconds(term: &mut Term, circle: &Circle, second: usize) {
    let angle = (second / 6) as usize;
    let point = circle.get_point(angle, |radius, angle| -> usize {
        radius * (if angle > 0 { angle - 1 } else { 1 })
    });
}

fn draw_minutes(term: &mut Term, circle: &Circle, minutes: usize) {
    let angle = (minutes / 6) as usize;
    let point = circle.get_point(angle, |radius, angle| -> usize {
        radius * (if angle > 0 { angle / 2 } else { 1 })
    });
}

fn draw_hours(term: &mut Term, circle: &Circle, hours: usize) {
    let angle = (hours / 6) as usize;
    let point = circle.get_point(angle, |radius, angle| -> usize {
        radius * (if angle > 0 { angle - 1 } else { 1 })
    });
    term.draw_line(&circle.center, &point, "-");
}

fn draw_clock(term: &mut Term, circle: &Circle) {
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
        term.put_pixel(x0 + x, y0 + y, what);
        term.put_pixel(x0 + y, y0 + x, what);
        term.put_pixel(x0 - y, y0 + x, what);
        term.put_pixel(x0 - x, y0 + y, what);
        term.put_pixel(x0 + y, y0 - x, what);
        term.put_pixel(x0 + x, y0 - y, what);
        term.put_pixel(x0 - x, y0 - y, what);
        term.put_pixel(x0 - y, y0 - x, what);

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

fn main() {

    let mut term = Term::new();

    loop {

        term.clear();

        let time = Time::get_current_time();
        let seconds = Time::format_seconds(&time);
        let minutes = Time::format_minutes(&time);
        let hours = Time::format_hours(&time);

        let circle = Circle::new(20, 20);

        draw_clock(&mut term, &circle);

        draw_seconds(&mut term, &circle, seconds);
        draw_minutes(&mut term, &circle, minutes);
        draw_hours(&mut term, &circle, hours);

        term.flush();


        thread::sleep(Duration::from_secs(DELAY));
    }

    //stdout.execute(terminal::Clear(terminal::ClearType::All))?;
}
