use crate::daterange::DefaultDateRange;
use crate::daterange::DateRange;
use crate::dateutils::{add_years, subtract_years};
use chrono::{Duration, NaiveDate};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AnnualDateRange(DefaultDateRange);

impl AnnualDateRange {
    pub fn with_start_date(start_date: NaiveDate) -> Self {
        let end = add_years(start_date, 1) - Duration::days(1);
        Self(DefaultDateRange::new(start_date, end))
    }

    pub fn with_end_date(end_date: NaiveDate) -> Self {
        let start = subtract_years(end_date, 1) + Duration::days(1);
        Self(DefaultDateRange::new(start, end_date))
    }

    /// Returns the previous year.
    pub fn prior(&self) -> Self {
        let start = subtract_years(self.0.start_date(), 1);
        let end = subtract_years(self.0.end_date(), 1);
        Self(DefaultDateRange::new(start, end))
    }

    /// Returns the next year.
    pub fn next(&self) -> Self {
        let start = add_years(self.0.start_date(), 1);
        let end = add_years(self.0.end_date(), 1);
        Self(DefaultDateRange::new(start, end))
    }
}

impl DateRange for AnnualDateRange {
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
        AnnualDateRange(DefaultDateRange::new(start, end))
    }
}
