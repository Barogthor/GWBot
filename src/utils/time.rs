use std::cmp::Ordering;
use std::ops::Add;

use chrono::{Date, Datelike, DateTime, Duration, Timelike, TimeZone, Utc};

#[derive(Clone, PartialOrd, PartialEq, Eq, Ord)]
pub enum DateTimeRangeComparison {
    Before = 0,
    Within = 1,
    After = 2,
}

#[derive(Debug)]
pub struct DateTimeRange<T: TimeZone>(pub DateTime<T>, pub DateTime<T>);

impl<T: TimeZone> DateTimeRange<T> {
    pub fn new(start: DateTime<T>, end: DateTime<T>) -> Self {
        Self(start, end)
    }
    #[inline]
    fn within(&self, dt: &DateTime<T>) -> bool {
        self.0.cmp(&dt).le(&Ordering::Equal) && self.1.cmp(&dt).ge(&Ordering::Equal)
    }
    pub fn compare(&self, dt: &DateTime<T>) -> DateTimeRangeComparison {
        if self.0.cmp(dt).eq(&Ordering::Greater) {
            DateTimeRangeComparison::After
        } else if self.within(dt) {
            DateTimeRangeComparison::Within
        } else {
            DateTimeRangeComparison::Before
        }
    }
}

pub fn get_time_left(from: DateTime<Utc>, since: DateTime<Utc>) -> (i64, i64, i64, i64) {
    let time_left = from.signed_duration_since(since);
    let days_left = time_left.num_days();
    let hours_left = time_left.num_hours() % 24;
    let mins_left = time_left.num_minutes() % 60;
    let secs_left = time_left.num_seconds() % 60;
    (days_left, hours_left, mins_left, secs_left)
}

pub fn get_next_week(now: DateTime<Utc>) -> Date<Utc> {
    let weekday = now.weekday();
    if weekday.num_days_from_monday() == 0 {
        now.date()
    } else {
        now.add(Duration::days((7 - weekday.num_days_from_monday()) as i64)).date()
    }
}

pub fn get_next_day(now: DateTime<Utc>, reference_hour: u32) -> Date<Utc> {
    if now.hour() < reference_hour {
        now.date()
    } else {
        now.add(Duration::days(1)).date()
    }
}

pub fn get_utc_start(date: Date<Utc>) -> DateTime<Utc> {
    date.and_hms(16, 0, 0)
}
