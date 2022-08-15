mod time;

use time::Time;

fn main() {
    let time = Time::get_current_time();
    println!("{}:{}:{}", Time::format_hours(&time), Time::format_minutes(&time), Time::format_seconds(&time));
    println!("Hello, world!");
}
