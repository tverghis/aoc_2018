use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Activity {
    BeginShift(u32),
    FallAsleep(u8),
    WakeUp(u8),
}

pub type ActivityParseResult<T> = Result<T, Box<Error>>;

impl Activity {
    fn from_pieces(minute: u8, activity_str: &str) -> ActivityParseResult<Activity> {
        if activity_str.contains("begins shift") {
            return Activity::begin_shift(activity_str);
        }

        if activity_str == "falls asleep" {
            return Activity::fall_asleep(minute);
        }

        if activity_str == "wakes up" {
            return Activity::wake_up(minute);
        }

        Err(Box::from("Unknown activity type"))
    }

    fn begin_shift(activity_str: &str) -> ActivityParseResult<Activity> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^Guard \#(?P<id>\d+) begins shift$").unwrap();
        }

        let captures = match RE.captures(&activity_str) {
            Some(caps) => caps,
            None => return Err(Box::from("Could not parse activity_str for BeginResult")),
        };

        let id = captures["id"].parse()?;

        Ok(Activity::BeginShift(id))
    }

    fn fall_asleep(minute: u8) -> ActivityParseResult<Activity> {
        Ok(Activity::FallAsleep(minute))
    }

    fn wake_up(minute: u8) -> ActivityParseResult<Activity> {
        Ok(Activity::WakeUp(minute))
    }
}

impl FromStr for Activity {
    type Err = Box<Error>;

    fn from_str(s: &str) -> ActivityParseResult<Activity> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^\[\d{4}-\d{2}-\d{2} \d{2}:(?P<minute>\d{2})\] (?P<activity>[\w\s#]+)$"
            )
            .unwrap();
        }

        let captures = match RE.captures(s) {
            Some(caps) => caps,
            None => return Err(Box::from("Input does not match expected format!")),
        };

        let minute: u8 = captures["minute"].parse()?;
        let activity_string = captures["activity"].to_string();

        Activity::from_pieces(minute, &activity_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_begin_shift_line() {
        parses(
            "[1518-06-12 00:00] Guard #3359 begins shift",
            Activity::BeginShift(3359),
        );
    }

    #[test]
    fn parses_fall_asleep_line() {
        parses("[1518-06-07 00:26] falls asleep", Activity::FallAsleep(26));
    }

    #[test]
    fn parses_wake_up_line() {
        parses("[1518-10-08 00:19] wakes up", Activity::WakeUp(19));
    }

    fn parses(s: &str, expected: Activity) {
        let parsed: Activity = s.parse().unwrap();
        assert_eq!(parsed, expected);
    }
}
