mod circle;
mod term;
mod time;
use std::{thread, time::Duration};
use crossterm::style::{self, Stylize};

use circle::{Circle, Point};
use term::Term;
use time::Time;

const DELAY: u64 = 1;
const DEG_TO_RAD: f64 = 57.29577;

fn draw_seconds(term: &mut Term, circle: &Circle, second: usize) {
    let angle = (second * 6) as f64;
    let angle_d = angle / DEG_TO_RAD;
    let point = circle.get_point(angle_d, |radius, angle| -> f64 {
        angle * (radius - 1) as f64
    });
    term.draw_line(&circle.center, &point, &"-".dark_green());
}

fn draw_minutes(term: &mut Term, circle: &Circle, minutes: usize) {
    let angle = (minutes * 6) as f64;
    let angle_d = angle / DEG_TO_RAD;
    let point = circle.get_point(angle_d, |radius, angle| -> f64 {
        angle * (radius - 3) as f64
    });
    term.draw_line(&circle.center, &point, &",".yellow());
}

fn draw_hours(term: &mut Term, circle: &Circle, hours: usize, minutes: usize) {
    let angle = ((hours * 30) + (minutes / 12) * 6) as f64;
    let angle_d = angle / DEG_TO_RAD;
    let point = circle.get_point(angle_d, |radius, angle| -> f64 {
        angle * (radius / 2) as f64
    });
    term.draw_line(&circle.center, &point, &"+".yellow());
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

    let what = "*".magenta();

    while x >= y {
        term.put_pixel(x0 + x, y0 + y, &what);
        term.put_pixel(x0 + y, y0 + x, &what);
        term.put_pixel(x0 - y, y0 + x, &what);
        term.put_pixel(x0 - x, y0 + y, &what);
        term.put_pixel(x0 + y, y0 - x, &what);
        term.put_pixel(x0 + x, y0 - y, &what);
        term.put_pixel(x0 - x, y0 - y, &what);
        term.put_pixel(x0 - y, y0 - x, &what);

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
/*
    term.clear();
    term.draw_line(&Point { x: 0, y: 0 }, &Point { x: 10, y: 5 }, &"*".magenta());

    term.flush();
*/
    loop {

        term.clear();

        let time = Time::get_current_time();
        let seconds = Time::format_seconds(&time);
        let minutes = Time::format_minutes(&time);
        let hours = Time::format_hours(&time);

        let circle = Circle::new(30, 30);

        draw_clock(&mut term, &circle);

        draw_seconds(&mut term, &circle, seconds);
        draw_minutes(&mut term, &circle, minutes);
        draw_hours(&mut term, &circle, hours, minutes);

        term.flush();

        thread::sleep(Duration::from_secs(DELAY));
    }

    //stdout.execute(terminal::Clear(terminal::ClearType::All))?;
}
