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
    pub timing: FixedTiming,
}

impl std::fmt::Display for FixedTiming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{start} -> {end}", start = self.start, end = self.end)
    }
}

impl std::fmt::Display for InflexibleEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.timing)
    }
}

#[derive(Debug)]
pub struct InflexibleEventIterator(Option<InflexibleEvent>);

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Default)]
pub struct EventRepetition {
    #[serde(default)]
    pub repeat_end: Option<DateTime<Local>>,
    #[serde(default)]
    pub pattern: RepetitionPattern,
}
