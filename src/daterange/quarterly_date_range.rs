use crate::daterange::DateRange;
use crate::daterange::DefaultDateRange;
use crate::dateutils::{add_months, first_day_of_month, last_day_of_month, subtract_months};
use chrono::NaiveDate;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct QuarterlyDateRange(DefaultDateRange);

impl QuarterlyDateRange {
    /// Creates a quarterly range starting at the given start_date.
    pub fn with_start_date(start_date: NaiveDate) -> Self {
        let start = first_day_of_month(start_date);
        let end = add_months(first_day_of_month(start_date), 2);
        Self(DefaultDateRange::new(start, end))
    }

    /// Creates a quarterly range ending at the given end_date.
    pub fn with_end_date(end_date: NaiveDate) -> Self {
        let start = subtract_months(first_day_of_month(end_date), 2);
        let end = last_day_of_month(end_date);
        Self(DefaultDateRange::new(start, end))
    }

    /// Returns the previous quarter.
    pub fn prior(&self) -> Self {
        let start = subtract_months(self.0.start_date(), 3);
        let end = last_day_of_month(subtract_months(first_day_of_month(self.0.end_date()), 3));
        Self(DefaultDateRange::new(start, end))
    }

    /// Returns the next quarter.
    pub fn next(&self) -> Self {
        let start = add_months(self.0.start_date(), 3);
        let end = last_day_of_month(add_months(first_day_of_month(self.0.end_date()), 3));
        Self(DefaultDateRange::new(start, end))
    }
}

impl DateRange for QuarterlyDateRange {
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
        QuarterlyDateRange(DefaultDateRange::new(start, end))
    }
}
