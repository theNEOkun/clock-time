mod circle;
mod term;
mod time;
use crossterm::{
    style::{self, Stylize},
    terminal,
};
use std::{thread, time::Duration};

use circle::{Circle, Point};
use term::Term;
use time::Time;

const DELAY: u64 = 1;
const DEG_TO_RAD: f64 = 57.29577;

/**
 * Function used to draw the second-hand
 *
 * ## Arguments
 * * term - The terminal to draw the window to
 * * circle - The circle the clock is in
 * * second - the current time in seconds
 */
fn draw_seconds(term: &mut Term, circle: &Circle, second: usize) {
    let angle = (second * 6) as f64;
    let angle_d = angle / DEG_TO_RAD;
    let point = circle.get_point(angle_d, |radius, angle| -> f64 {
        angle * (radius - 1) as f64
    });
    term.draw_line(&circle.center, &point, &"s".dark_green());
}

/**
 * Function used to draw the minute-hand
 *
 * ## Arguments
 * * term - The terminal to draw the window to
 * * circle - The circle the clock is in
 * * minutes - the current time in minutes
 */
fn draw_minutes(term: &mut Term, circle: &Circle, minutes: usize) {
    let angle = (minutes * 6) as f64;
    let angle_d = angle / DEG_TO_RAD;
    let point = circle.get_point(angle_d, |radius, angle| -> f64 {
        angle * (radius - 3) as f64
    });
    term.draw_line(&circle.center, &point, &"m".yellow());
}

/**
 * Function used to draw the hour-hand
 *
 * ## Arguments
 * * term - The terminal to draw the window to
 * * circle - The circle the clock is in
 * * hours - the current time in hours
 * * minutes - the current time in minutes
 */
fn draw_hours(term: &mut Term, circle: &Circle, hours: usize, minutes: usize) {
    let angle = ((hours * 30) + (minutes / 12) * 6) as f64;
    let angle_d = angle / DEG_TO_RAD;
    let point = circle.get_point(angle_d, |radius, angle| -> f64 {
        angle * (radius / 2) as f64
    });
    term.draw_line(&circle.center, &point, &"h".blue());
}

fn draw_stubs(term: &mut Term, circle: &Circle) {
    for angle in (0..360).step_by(120) {
        let angle_d = angle as f64 / DEG_TO_RAD;

        let point_1 = circle.get_point(angle_d, |radius, angle| -> f64 {
            angle * (radius) as f64
        });

        let point_2 = circle.get_point(angle_d, |radius, angle| -> f64 {
            angle * (radius - 5) as f64
        });

        term.draw_line(&point_1, &point_2, &"*".red());
    }
}

fn main() {
    let mut term = Term::new();
    let circle = Circle::new(40, 60);

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

        term.draw_clock(&circle);

        draw_stubs(&mut term, &circle);
        draw_seconds(&mut term, &circle, seconds);
        draw_minutes(&mut term, &circle, minutes);
        draw_hours(&mut term, &circle, hours, minutes);

        term.flush();

        thread::sleep(Duration::from_secs(DELAY));
    }
}
