use std::time::{SystemTime, UNIX_EPOCH, Duration};

const HOURS_PER_DAY: usize = 24;
const MINUTES_PER_HOUR: usize = 60;
const SECONDS_PER_MINUTE: usize = 60;

pub struct Time {
    unix_time: Duration,
}

impl Time {

    /**
     * Used to format the time given in seconds
     * Time presumed to be in milliseconds
     *
     * ## Arguments
     * * time - Time in milliseconds given as unix_time
     */
    pub fn get_seconds(time: &Duration) -> usize {
        time.as_secs() as usize
    }

    /**
     * Used to format the time given in minutes
     * Time presumed to be in milliseconds
     *
     * ## Arguments
     * * time - Time in milliseconds given as unix_time
     */
    pub fn get_minutes(time: &Duration) -> usize {
        Self::get_seconds(time) * SECONDS_PER_MINUTE
    }

    /**
     * Used to format the time given in hours
     * Time presumed to be in milliseconds
     *
     * ## Arguments
     * * time - Time in milliseconds given as unix_time
     */
    pub fn get_hours(time: &Duration) -> usize {
        Self::get_minutes(time) * MINUTES_PER_HOUR
    }
}

#[cfg(test)]
mod time_test {
    use super::*;

    #[test]
    fn test_seconds() {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards");

        let seconds = Time::get_seconds(&current_time);

        assert_eq!(seconds, current_time.as_secs() as usize);
    }

    #[test]
    fn test_minutes() {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards");

        let seconds = Time::get_minutes(&current_time);

        assert_eq!(seconds, current_time.as_secs() as usize * SECONDS_PER_MINUTE);
    }

    #[test]
    fn test_hours() {
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("time went backwards");

        let seconds = Time::get_hours(&current_time);

        assert_eq!(seconds, current_time.as_secs() as usize * SECONDS_PER_MINUTE * MINUTES_PER_HOUR);
    }
}
