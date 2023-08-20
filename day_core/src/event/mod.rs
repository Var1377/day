mod flexible;
mod inflexible;
pub use flexible::*;
pub use inflexible::*;
use uuid::Uuid;

use crate::time::HourMinute;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Copy, PartialEq, Eq)]
pub enum RepetitionPattern {
    #[default]
    Daily, 
    Weekly,
}

#[serde_inline_default]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Event {
    #[serde_inline_default("Task Title".into())]
    pub title: String,
    #[serde(default = "Uuid::now_v7")]
    pub id: Uuid,
    #[serde(default)]
    pub notes: String,
    #[serde_inline_default(HourMinute(0, 30))]
    pub duration: HourMinute,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            title: "Task Title".into(),
            id: Uuid::now_v7(),
            notes: "".into(),
            duration: HourMinute(0, 30)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct TimedEvent {
    #[serde(default)]
    pub inner: Event,
    #[serde(default)]
    pub timing: EventTiming,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventTiming {
    Flexible(FlexibleEvent),
    Inflexible(InflexibleEvent),
}

impl Default for EventTiming {
    fn default() -> Self {
        Self::Inflexible(Default::default())
    }
}

impl std::fmt::Display for EventTiming {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventTiming::Flexible(event) => write!(f, "{}", event),
            EventTiming::Inflexible(event) => write!(f, "{}", event),
        }
    }
}