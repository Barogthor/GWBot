use std::cmp::Ordering;

use chrono::{DateTime, TimeZone};

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