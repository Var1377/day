mod flexible;
mod inflexible;
use enum_iterator::Sequence;
pub use flexible::*;
pub use inflexible::*;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Copy, PartialEq, Eq, Sequence)]
pub enum RepetitionPattern {
    #[default]
    Daily, 
    Weekly,
}

impl std::fmt::Display for RepetitionPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Daily => "Daily",
            Self::Weekly => "Weekly", 
        })
    }
}

#[serde_inline_default]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventDetails {
    #[serde_inline_default("Title".into())]
    pub title: String,
    #[serde(default = "Uuid::now_v7")]
    pub id: Uuid,
    #[serde(default)]
    pub notes: String,
}

impl Default for EventDetails {
    fn default() -> Self {
        Self {
            title: "Title".into(),
            id: Uuid::now_v7(),
            notes: "".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct TimedEvent {
    #[serde(default)]
    pub inner: EventDetails,
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