use chrono::{Datelike, Duration, NaiveDate, Weekday};

/// Represents a range of dates.
pub trait DateRange: Sized + Clone + Copy {
    /// Get the starting date in the range.
    fn start_date(&self) -> NaiveDate;

    /// Get the ending date in the range.
    fn end_date(&self) -> NaiveDate;

    /// Get the dates contained in the range in a vec.
    fn dates(&self) -> Vec<NaiveDate> {
        let mut dates = Vec::with_capacity(self.len());
        let mut current = self.start_date();
        while current <= self.end_date() {
            dates.push(current);
            current += Duration::days(1);
        }
        dates
    }

    /// Get the date at the specified index. If the index is outside the bounds
    //  an error will be returned.
    fn date_at(&self, index: usize) -> Option<NaiveDate> {
        self.dates().get(index).copied()
    }

    /// Get a list of dates from the range that match the specified DayOfWeek
    fn dates_for_day(&self, day: Weekday) -> Vec<NaiveDate> {
        self.dates()
            .into_iter()
            .filter(|d| d.weekday() == day)
            .collect()
    }

    /// Check if a date is contained in the range.
    fn contains_date(&self, date: NaiveDate) -> bool {
        date >= self.start_date() && date <= self.end_date()
    }

    /// Check if a date range is fully contained in the range.
    fn contains_range<T: DateRange>(&self, date_range: &T) -> bool {
        date_range.start_date() >= self.start_date() && date_range.end_date() <= self.end_date()
    }

    /// Check if a date range is partially contained in the range.
    fn overlaps<T: DateRange>(&self, date_range: &T) -> bool {
        self.start_date() <= date_range.end_date() && self.end_date() >= date_range.start_date()
    }

    /// Check if a date range is partially contained in a list of date ranges.
    fn overlaps_any<T: DateRange>(&self, date_ranges: &[T]) -> bool {
        date_ranges.iter().any(|range| self.overlaps(range))
    }

    /// Get the number of days in the range.
    fn len(&self) -> usize {
        (self.end_date() - self.start_date()).num_days() as usize + 1
    }

    /// Get the DateRange that contains the specified date.
    fn range_containing_date(&self, date: NaiveDate) -> Self {
        let mut range = self.create_new_date_range(self.start_date(), self.end_date());
        while !range.contains_date(date) {
            if date > range.end_date() {
                range = range.next();
            } else {
                range = range.prior();
            }
        }
        range
    }

    /// Get a DateRange that represents the prior range to this dateRange.
    fn prior(&self) -> Self {
        self.create_new_date_range(
            self.start_date() - Duration::days(self.len() as i64),
            self.end_date() - Duration::days(self.len() as i64),
        )
    }

    /// Get the DateRange that represents a date range that is N ranges prior to the current DateRange.
    fn prior_n(&self, number: usize) -> Self {
        let mut range = self.prior();
        for _ in 1..number {
            range = range.prior();
        }
        range
    }

    /// Get a DateRange that represents the next range to this dateRange.
    fn next(&self) -> Self {
        self.create_new_date_range(
            self.start_date() + Duration::days(self.len() as i64),
            self.end_date() + Duration::days(self.len() as i64),
        )
    }

    /// Get the DateRange that represents a date range that is N ranges after the current DateRange.
    fn next_n(&self, number: usize) -> Self {
        let mut range = self.next();
        for _ in 1..number {
            range = range.next();
        }
        range
    }

    // Get a list of N DateRanges before this DateRange, not including this DateRange.
    fn ranges_before(&self, number: usize) -> Vec<Self> {
        self.ranges_before_impl(number, false)
    }

    // Get a list of N DateRanges before this DateRange, including this DateRange.
    fn ranges_before_inclusive(&self, number: usize) -> Vec<Self> {
        self.ranges_before_impl(number, true)
    }

    // Get a list of N DateRanges after this DateRange, not including this DateRange.
    fn ranges_after(&self, number: usize) -> Vec<Self> {
        self.ranges_after_impl(number, false)
    }

    // Get a list of N DateRanges after this DateRange, including this DateRange.
    fn ranges_after_inclusive(&self, number: usize) -> Vec<Self> {
        self.ranges_after_impl(number, true)
    }

    // Get a list of DateRanges that includes the current DateRange, N DateRanges before
    // this DateRange and N DateRanges after this date range.
    fn ranges_window(&self, before: usize, after: usize) -> Vec<Self> {
        let mut ranges = Vec::with_capacity(before + after + 1);

        // Add prior ranges
        ranges.extend(self.ranges_before_impl(before, true)); // includes self

        // Add after ranges, skip self since already included
        let mut after_ranges = self.ranges_after_impl(after, false);
        ranges.append(&mut after_ranges);

        ranges
    }

    /// Get a list of DateRanges that contain the specified dates.
    fn ranges_containing_span(&self, from_date: NaiveDate, to_date: NaiveDate) -> Vec<Self> {
        // assert!(from != null);
        // assert!(from <= to);
        // assert!(from <= to);

        let mut ranges = Vec::new();
        let mut range = self.range_containing_date(from_date);
        ranges.push(range.clone());

        while range.end_date() < to_date {
            range = range.next();
            ranges.push(range.clone());
        }

        ranges.sort_by_key(|r| r.start_date());
        ranges
    }

    fn create_new_date_range(&self, start: NaiveDate, end: NaiveDate) -> Self;

    fn ranges_before_impl(&self, number: usize, include_self: bool) -> Vec<Self> {
        let mut ranges = Vec::with_capacity(number + 1);
        if include_self {
            ranges.push(self.create_new_date_range(self.start_date(), self.end_date()));
        }
        let mut current = self.create_new_date_range(self.start_date(), self.end_date());
        for _ in 0..number {
            current = current.prior();
            ranges.push(current.create_new_date_range(current.start_date(), current.end_date()));
        }
        ranges.reverse(); // to match Java order
        ranges
    }

    fn ranges_after_impl(&self, number: usize, include_self: bool) -> Vec<Self> {
        let mut ranges = Vec::with_capacity(number + 1);
        if include_self {
            ranges.push(self.create_new_date_range(self.start_date(), self.end_date()));
        }
        let mut current = self.create_new_date_range(self.start_date(), self.end_date());
        for _ in 0..number {
            current = current.next();
            ranges.push(current.create_new_date_range(current.start_date(), current.end_date()));
        }
        ranges
    }
}
