use std::cmp::Ordering;
use chrono::{NaiveDate};
use crate::daterange::DateRange;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DefaultDateRange {
    start_date: NaiveDate,
    end_date: NaiveDate,
    len: usize,
}

impl DefaultDateRange {
    pub fn new(start_date: NaiveDate, end_date: NaiveDate) -> Self {
        let days = (end_date - start_date).num_days() as usize + 1;
        Self {
            start_date,
            end_date,
            len: days,
        }
    }
}

impl PartialOrd for DefaultDateRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DefaultDateRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_date.cmp(&other.start_date)
    }
}

impl DateRange for DefaultDateRange {
    fn start_date(&self) -> NaiveDate {
        self.start_date
    }

    fn end_date(&self) -> NaiveDate {
        self.end_date
    }

    fn len(&self) -> usize {
        self.len
    }

    fn create_new_date_range(&self, start: NaiveDate, end: NaiveDate) -> Self {
        DefaultDateRange::new(start, end)
    }
}
