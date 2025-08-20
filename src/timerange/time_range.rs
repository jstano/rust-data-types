use chrono::{NaiveTime, Duration};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct TimeRange {
    start: NaiveTime,
    end: NaiveTime,
}

impl TimeRange {
    pub fn of(start: NaiveTime, end: NaiveTime) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> NaiveTime {
        self.start
    }

    pub fn end(&self) -> NaiveTime {
        self.end
    }

    pub fn duration(&self) -> Duration {
        self.end - self.start
    }

    pub fn overlaps(&self, other: &TimeRange) -> bool {
        let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

        if self.end == midnight && other.end == midnight {
            return true;
        }

        if self.end == midnight {
            return other.end >= self.start;
        }

        if other.end == midnight {
            return other.start <= self.end;
        }

        self.start <= other.end && self.end >= other.start
    }
}

impl PartialEq for TimeRange {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Eq for TimeRange {}

impl Hash for TimeRange {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.end.hash(state);
    }
}

impl PartialOrd for TimeRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TimeRange {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.start.cmp(&other.start) {
            Ordering::Equal => self.end.cmp(&other.end),
            ord => ord,
        }
    }
}
