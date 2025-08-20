use crate::daterange::DefaultDateRange;
use crate::daterange::DateRange;
use crate::dateutils::{add_months, subtract_months};
use chrono::{Duration, NaiveDate};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemiAnnualDateRange(DefaultDateRange);

impl SemiAnnualDateRange {
    pub fn with_start_date(start_date: NaiveDate) -> Self {
        let end = add_months(start_date, 6) - Duration::days(1);
        Self(DefaultDateRange::new(start_date, end))
    }

    pub fn with_end_date(end_date: NaiveDate) -> Self {
        let start = subtract_months(end_date, 6) + Duration::days(1);
        Self(DefaultDateRange::new(start, end_date))
    }

    /// Returns the prior range.
    pub fn prior(&self) -> Self {
        let start = subtract_months(self.0.start_date(), 6);
        let end = subtract_months(self.0.end_date(), 6);
        Self(DefaultDateRange::new(start, end))
    }

    /// Returns the next range.
    pub fn next(&self) -> Self {
        let start = add_months(self.0.start_date(), 6);
        let end = add_months(self.0.end_date(), 6);
        Self(DefaultDateRange::new(start, end))
    }
}

impl DateRange for SemiAnnualDateRange {
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
        SemiAnnualDateRange(DefaultDateRange::new(start, end))
    }
}
