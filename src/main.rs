mod circle;
mod time;

use std::{thread, time::Duration};

use circle::Circle;
use time::Time;

const DELAY: u64 = 1;

fn draw_seconds(circle: &Circle, second: usize) {
    let angle = (second / 6) as f64;
    let point = circle.get_point(angle, |radius, angle| -> f64 { radius * (angle - 1.0) });
}

fn draw_minutes(circle: &Circle, minutes: usize) {
    let angle = (minutes / 6) as f64;
    let point = circle.get_point(angle, |radius, angle| -> f64 { radius * (angle / 2.0) });
}

fn draw_hours(circle: &Circle, hours: usize) {
    let angle = (hours / 6) as f64;
    let point = circle.get_point(angle, |radius, angle| -> f64 { radius * (angle - 1.0) });
}

fn main() {
    loop {
        let time = Time::get_current_time();
        let seconds = Time::format_seconds(&time);
        let minutes = Time::format_minutes(&time);
        let hours = Time::format_hours(&time);

        println!("{}:{}:{}", hours, minutes, seconds);

        let circle = Circle::new(10.0, 10.0);
        draw_seconds(&circle, seconds);
        draw_minutes(&circle, minutes);
        draw_hours(&circle, hours);

        thread::sleep(Duration::from_secs(DELAY));
    }
}
