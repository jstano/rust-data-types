use chrono::{Datelike, Duration, NaiveDate, Weekday};
use crate::daterange::DateRange;
use crate::daterange::DefaultDateRange;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WeeklyDateRange(DefaultDateRange);

impl WeeklyDateRange {
    pub fn with_start_date(start_date: NaiveDate) -> Self {
        let end = start_date + Duration::days(6);
        Self(DefaultDateRange::new(start_date, end))
    }

    pub fn with_end_date(end_date: NaiveDate) -> Self {
        let start = end_date - Duration::days(6);
        Self(DefaultDateRange::new(start, end_date))
    }

    pub fn with_target_date(target: NaiveDate, end_day: Weekday) -> Self {
        let offset = calculate_day_of_week_offset(target, end_day);
        let end = target + Duration::days(offset as i64);
        let start = end - Duration::days(6);
        Self(DefaultDateRange::new(start, end))
    }
}

impl DateRange for WeeklyDateRange {
    fn start_date(&self) -> NaiveDate {
        self.0.start_date()
    }

    fn end_date(&self) -> NaiveDate {
        self.0.end_date()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn create_new_date_range(&self, start: NaiveDate, end: NaiveDate) -> Self {
        WeeklyDateRange(DefaultDateRange::new(start, end))
    }
}

fn calculate_day_of_week_offset(date: NaiveDate, end_day: Weekday) -> i64 {
    let mut offset = end_day.num_days_from_monday() as i64 - date.weekday().num_days_from_monday() as i64;
    if offset < 0 {
        offset += 7;
    }
    offset
}
