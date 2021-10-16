use std::fmt;
use std::cmp::Ordering;

// years, days, hours, minutes and seconds.

const SECS_PER_MINUTE: i64 = 60;
/// The number of seconds in an hour.
const SECS_PER_HOUR: i64 = 3600;
/// The number of (non-leap) seconds in days.
const SECS_PER_DAY: i64 = 86400;
/// The number of (non-leap) seconds in a year.
const SECS_PER_YEAR: i64 = 31536000;

struct HumanDuration {
    num_years: i64,
    num_days: i64,
    num_hours: i64,
    num_minutes: i64,
    num_seconds: i64,
}

// todo 多种写法
impl fmt::Display for HumanDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut v: Vec<String> = Vec::new();
        match self.num_years.cmp(&1) {
            Ordering::Greater => v.push(format!("{} years", self.num_years)),
            Ordering::Equal => v.push(format!("{} year", self.num_years)),
            _ => (),
        }
        match self.num_days.cmp(&1) {
            Ordering::Greater => v.push(format!("{} days", self.num_days)),
            Ordering::Equal => v.push(format!("{} day", self.num_days)),
            _ => (),
        }
        match self.num_hours.cmp(&1) {
            Ordering::Greater => v.push(format!("{} hours", self.num_hours)),
            Ordering::Equal => v.push(format!("{} hour", self.num_hours)),
            _ => (),
        }
        match self.num_minutes.cmp(&1) {
            Ordering::Greater => v.push(format!("{} minutes", self.num_minutes)),
            Ordering::Equal => v.push(format!("{} minute", self.num_minutes)),
            _ => (),
        }
        match self.num_seconds.cmp(&1) {
            Ordering::Greater => v.push(format!("{} seconds", self.num_seconds)),
            Ordering::Equal => v.push(format!("{} second", self.num_seconds)),
            _ => (),
        }
        match v.len() {
            1 => write!(f, "{}", v[0]),
            2 => write!(f, "{} and {}", v[0], v[1]),
            3 => write!(f, "{}, {} and {}", v[0], v[1], v[2]),
            4 => write!(f, "{}, {}, {} and {}", v[0], v[1], v[2], v[3]),
            5 => write!(f, "{}, {}, {}, {} and {}", v[0], v[1], v[2], v[3], v[4]),
            _ => write!(f, "now")
        }
    }
}

impl HumanDuration {
    fn new(seconds: u64) -> HumanDuration {
        let mut left_seconds = seconds as i64;

        let num_years = left_seconds / SECS_PER_YEAR;
        left_seconds %= SECS_PER_YEAR;

        let num_days = left_seconds / SECS_PER_DAY;
        left_seconds %= SECS_PER_DAY;

        let num_hours = left_seconds / SECS_PER_HOUR;
        left_seconds %= SECS_PER_HOUR;

        let num_minutes = left_seconds / SECS_PER_MINUTE;
        left_seconds %= SECS_PER_MINUTE;

        HumanDuration {
            num_years: num_years,
            num_days: num_days,
            num_hours: num_hours,
            num_minutes: num_minutes,
            num_seconds: left_seconds,
        }
    }
}

pub fn format_duration(seconds: u64) -> String {
    format!("{}", HumanDuration::new(seconds))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(format_duration(1), "1 second");
        assert_eq!(format_duration(62), "1 minute and 2 seconds");
        assert_eq!(format_duration(120), "2 minutes");
        assert_eq!(format_duration(3600), "1 hour");
        assert_eq!(format_duration(3662), "1 hour, 1 minute and 2 seconds");
    }
}