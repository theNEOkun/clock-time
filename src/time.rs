use std::time::{Duration, SystemTime, UNIX_EPOCH};

const HOURS_PER_DAY: usize = 24;
const MINUTES_PER_HOUR: usize = 60;
const SECONDS_PER_MINUTE: usize = 60;
const MY_TIMEZONE: usize = 2 * MINUTES_PER_HOUR * SECONDS_PER_MINUTE;

pub struct Time;

/// Used to format time in different formats
impl Time {

    /**
     * Used to get the current time in Duration from UNIX_EPOCH
     */
    pub fn get_current_time() -> Duration {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            + Duration::from_secs(MY_TIMEZONE as u64)
    }

    /**
     * Used to format the time given in seconds
     * Time presumed to be in milliseconds
     *
     * ## Example
     *
     * Given 1000 milliseconds, will return 1 second
     *
     * ## Arguments
     * * time - Time in milliseconds given as unix_time
     *
     * ## Returns
     * The given time in seconds of the day
     */
    pub fn get_seconds(time: &Duration) -> usize {
        time.as_secs() as usize
    }

    /**
     * Used to get the time in minutes, formated in 60s
     * Time presumed to be in milliseconds
     *
     * ## Arguments
     * * time - Time in milliseconds given as unix_time
     *
     * ## Returns
     * The given time in hours of the day
     */
    pub fn format_seconds(time: &Duration) -> usize {
        Self::get_seconds(time) % SECONDS_PER_MINUTE
    }

    /**
     * Used to get the given time in minutes
     * Time presumed to be in milliseconds
     *
     * ## Example
     *
     * Given 1000 milliseconds, will return 0 minutes
     *
     * ## Arguments
     * * time - Time in milliseconds given as unix_time
     *
     * ## Returns
     * The given time in minutes of the day
     */
    pub fn get_minutes(time: &Duration) -> usize {
        Self::get_seconds(time) / SECONDS_PER_MINUTE
    }

    /**
     * Used to get the time in minutes, formated in 60m
     * Time presumed to be in milliseconds
     *
     * ## Arguments
     * * time - Time in milliseconds given as unix_time
     *
     * ## Returns
     * The given time in hours of the day
     */
    pub fn format_minutes(time: &Duration) -> usize {
        Self::get_minutes(time) % MINUTES_PER_HOUR
    }

    /**
     * Used to get the given time in hours
     * Time presumed to be in milliseconds
     *
     * ## Arguments
     * * time - Time in milliseconds given as unix_time
     *
     * ## Returns
     * The given time in hours of the day
     */
    pub fn get_hours(time: &Duration) -> usize {
        Self::get_minutes(time) / MINUTES_PER_HOUR
    }

    /**
     * Used to get the time in hours, formatted in 24h
     * Time presumed to be in milliseconds
     *
     * ## Arguments
     * * time - Time in milliseconds given as unix_time
     *
     * ## Returns
     * The given time in hours of the day
     */
    pub fn format_hours(time: &Duration) -> usize {
        Self::get_hours(time) % HOURS_PER_DAY
    }
}

#[cfg(test)]
mod time_test {
    use super::*;

    #[test]
    fn test_seconds() {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards");

        let seconds = Time::get_seconds(&current_time);

        assert_eq!(seconds, current_time.as_secs() as usize);
    }

    #[test]
    fn test_minutes() {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards");

        let seconds = Time::get_minutes(&current_time);

        assert_eq!(
            seconds,
            current_time.as_secs() as usize / SECONDS_PER_MINUTE
        );
    }

    #[test]
    fn test_hours() {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards");

        let seconds = Time::get_hours(&current_time);

        assert_eq!(
            seconds,
            current_time.as_secs() as usize / SECONDS_PER_MINUTE / MINUTES_PER_HOUR
        );
    }
}
