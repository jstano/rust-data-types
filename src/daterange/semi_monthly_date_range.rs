use crate::daterange::default_date_range::DefaultDateRange;
use crate::daterange::DateRange;
use crate::util::date_utils::last_day_of_month;
use chrono::{Datelike, Duration, NaiveDate};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemiMonthlyDateRange(DefaultDateRange);

const FIFTEENTH_OF_MONTH: u32 = 15;

impl SemiMonthlyDateRange {
    pub fn with_end_date(end_date: NaiveDate) -> Self {
        let start = calculate_start_date_from_end_date(end_date);
        Self(DefaultDateRange::new(start, end_date))
    }

    fn prior(&self) -> Self {
        let end_date = self.0.start_date() - Duration::days(1);
        let start_date = if self.0.start_date().day() == FIFTEENTH_OF_MONTH {
            // prior is 1st → 15th
            NaiveDate::from_ymd_opt(end_date.year(), end_date.month(), 1).unwrap()
        } else {
            // prior is 16th → last day of the previous month
            NaiveDate::from_ymd_opt(end_date.year(), end_date.month(), FIFTEENTH_OF_MONTH).unwrap()
        };

        Self(DefaultDateRange::new(start_date, end_date))
    }

    fn next(&self) -> Self {
        let start_date = if self.0.end_date().day() == FIFTEENTH_OF_MONTH {
            // next is the 16th → last day of the month
            NaiveDate::from_ymd_opt(
                self.0.end_date().year(),
                self.0.end_date().month(),
                FIFTEENTH_OF_MONTH + 1,
            ).unwrap()
        } else {
            // next is 1st → 15th of next month
            let next_month = self.0.end_date().month() % 12 + 1;
            let year = if next_month == 1 {
                self.0.end_date().year() + 1
            } else {
                self.0.end_date().year()
            };
            NaiveDate::from_ymd_opt(year, next_month, 1).unwrap()
        };

        let end_date = if start_date.day() == 1 {
            NaiveDate::from_ymd_opt(start_date.year(), start_date.month(), FIFTEENTH_OF_MONTH).unwrap()
        } else {
            last_day_of_month(start_date)
        };

        Self(DefaultDateRange::new(start_date, end_date))
    }
}

impl DateRange for SemiMonthlyDateRange {
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
        SemiMonthlyDateRange(DefaultDateRange::new(start, end))
    }
}

/// Calculate the start date given an end date.
///
/// Valid end dates are either the 15th of the month or the last day of the month.
fn calculate_start_date_from_end_date(end_date: NaiveDate) -> NaiveDate {
    if end_date.day() == FIFTEENTH_OF_MONTH {
        NaiveDate::from_ymd_opt(end_date.year(), end_date.month(), 1).unwrap()
    } else {
        NaiveDate::from_ymd_opt(end_date.year(), end_date.month(), FIFTEENTH_OF_MONTH).unwrap()
    }
}
