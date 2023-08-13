use chrono::{DateTime, Local};
use crate::now;
use super::RepetitionPattern;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct FixedTiming {
    #[serde(default = "now")]
    pub start: DateTime<Local>,
    #[serde(default = "now")]
    pub end: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct InflexibleEvent {
    #[serde(flatten, default)]
    timing: FixedTiming,
    #[serde(default)]
    pub repetition: Option<EventRepetition>,
}

impl std::fmt::Display for FixedTiming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{start} -> {end}", start = self.start, end = self.end)
    }
}

impl std::fmt::Display for InflexibleEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: Repetition {:?}", self.timing, self.repetition)
    }
}

#[derive(Debug)]
pub struct InflexibleEventIterator(Option<InflexibleEvent>);

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq)]
pub struct EventRepetition {
    #[serde(default = "now")]
    pub repeat_start: DateTime<Local>,
    #[serde(default)]
    pub repeat_end: Option<DateTime<Local>>,
    #[serde(default)]
    pub pattern: RepetitionPattern,
}

impl Default for EventRepetition {
    fn default() -> Self {
        Self {
            repeat_start: now(),
            repeat_end: None,
            pattern: Default::default(),
        }
    }
}

impl Iterator for InflexibleEventIterator {
    type Item = FixedTiming;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.0 {
            Some(e) => match e.repetition {
                Some(r) => {
                    let one_off = e.timing;
                    match r.pattern {
                        RepetitionPattern::Daily => {
                            e.timing.start = e.timing.start + chrono::Duration::days(1);
                            e.timing.end = e.timing.end + chrono::Duration::days(1);
                        },
                        RepetitionPattern::Weekly => {
                            e.timing.start = e.timing.start + chrono::Duration::weeks(1);
                            e.timing.end = e.timing.end + chrono::Duration::weeks(1);
                        },
                    }
                    if let Some(end) = r.repeat_end {
                        if e.timing.start > end {
                            self.0 = None;
                        }
                    }
                    return Some(one_off);
                },
                None => {
                    let one_off = e.timing;
                    self.0 = None;
                    return Some(one_off);
                }
            },
            None => None
        }
    }
}

impl IntoIterator for InflexibleEvent {
    type Item = FixedTiming;
    type IntoIter = InflexibleEventIterator;

    fn into_iter(self) -> Self::IntoIter {
        InflexibleEventIterator(Some(self))
    }
}