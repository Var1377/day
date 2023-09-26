use std::num::NonZeroU32;

use super::RepetitionPattern;
use crate::now;
use chrono::{DateTime, Local};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct FixedTiming {
    #[serde(default = "now")]
    pub start: DateTime<Local>,
    #[serde(default = "now")]
    pub end: DateTime<Local>,
}

impl Default for FixedTiming {
    fn default() -> Self {
        Self {
            start: now(),
            end: now(),
        }
    }
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

#[serde_inline_default]
#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq)]
pub struct EventRepetition {
    #[serde(default)]
    pub repeat_end: Option<DateTime<Local>>,
    #[serde_inline_default(NonZeroU32::new(1).unwrap())]
    pub interval: NonZeroU32,
    #[serde(default)]
    pub pattern: RepetitionPattern,
}

impl Default for EventRepetition {
    fn default() -> Self {
        Self {
            repeat_end: None,
            interval: NonZeroU32::new(1).unwrap(),
            pattern: Default::default()
        }
    }
}