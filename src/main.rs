mod circle;
mod term;
mod time;
use crossterm::style::Stylize;
use std::{thread, time::Duration};

use circle::Circle;
use term::Term;
use time::Time;

/// How long should the loop wait before it should repeat
const DELAY: u64 = 1;

/// How many degrees in a rad, and change to and from them
const DEG_TO_RAD: f64 = 57.29577;

/// Function used to draw the second-hand
///
/// ## Arguments
/// * term - The terminal to draw the window to
/// * circle - The circle the clock is in
/// * second - the current time in seconds
fn draw_seconds(term: &mut Term, circle: &Circle, second: usize) {
    let angle = (second * 6) as f64;
    let angle_d = angle / DEG_TO_RAD;
    let point = circle.get_point(angle_d, |radius, angle| -> f64 {
        angle * (radius - 1) as f64
    });
    term.draw_line(&circle.center, &point, &"s".dark_green());
}

/// Function used to draw the minute-hand
///
/// ## Arguments
/// * term - The terminal to draw the window to
/// * circle - The circle the clock is in
/// * minutes - the current time in minutes
fn draw_minutes(term: &mut Term, circle: &Circle, minutes: usize) {
    let angle = (minutes * 6) as f64;
    let angle_d = angle / DEG_TO_RAD;
    let point = circle.get_point(angle_d, |radius, angle| -> f64 {
        angle * (radius - 4) as f64
    });
    term.draw_line(&circle.center, &point, &"m".yellow());
}

/// Function used to draw the hour-hand
///
/// ## Arguments
/// * term - The terminal to draw the window to
/// * circle - The circle the clock is in
/// * hours - the current time in hours
/// * minutes - the current time in minutes
fn draw_hours(term: &mut Term, circle: &Circle, hours: usize, minutes: usize) {
    let angle = ((hours * 30) + (minutes / 12) * 6) as f64;
    let angle_d = angle / DEG_TO_RAD;
    let point = circle.get_point(angle_d, |radius, angle| -> f64 {
        angle * (radius / 2) as f64
    });
    term.draw_line(&circle.center, &point, &"h".blue());
}

/// Function to write the stubs along the clock-rim
///
/// ## Arguments
///
/// * term - The terminal to write to
/// * circle - The circle used for the clock-face
fn draw_major_stubs(term: &mut Term, circle: &Circle) {
    for outside_angle in (0..360).step_by(30) {
        let angle_d = outside_angle as f64 / DEG_TO_RAD;

        let point_1 = circle.get_point(angle_d, |radius, angle| -> f64 { angle * (radius) as f64 });

        let point_2 = circle.get_point(angle_d, |radius, angle| -> f64 {
            angle * (radius - 2) as f64
        });

        term.draw_line(&point_1, &point_2, &"*".red());
    }
}

/// Function to write the stubs along the clock-rim
///
/// ## Arguments
///
/// * term - The terminal to write to
/// * circle - The circle used for the clock-face
fn draw_minor_stubs(term: &mut Term, circle: &Circle) {
    for outside_angle in (0..360).step_by(6) {
        let angle_d = outside_angle as f64 / DEG_TO_RAD;

        let point_1 = circle.get_point(angle_d, |radius, angle| -> f64 { angle * (radius) as f64 });

        let point_2 = circle.get_point(angle_d, |radius, angle| -> f64 {
            angle * (radius - 1) as f64
        });

        term.draw_line(&point_1, &point_2, &"*".blue());
    }
}

fn main() {
    let mut term = Term::new();
    let width = 30;
    let heigth = 30;
    let circle = Circle::new(width, heigth);

    loop {
        let time = Time::get_current_time();
        let seconds = Time::format_seconds(&time);
        let minutes = Time::format_minutes(&time);
        let hours = Time::format_hours(&time);

        term.draw(&mut |term| {
            term.draw_circle(&circle);
            if circle.radius >= 29 { draw_minor_stubs(term, &circle); }
            //draw_major_stubs(term, &circle);
            draw_seconds(term, &circle, seconds);
            //draw_minutes(term, &circle, minutes);
            //draw_hours(term, &circle, hours, minutes);
        });

        thread::sleep(Duration::from_secs(DELAY));
    }
}
