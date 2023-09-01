mod state;
pub use state::CommitmentState;

use crate::event::{InflexibleEvent, FlexibleEvent, EventRepetition};
use std::{fmt::Display, str::FromStr};
use enum_iterator::Sequence;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Commitment {
    Event(CustomEvent),
    Ical(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomEvent {
    #[serde(default, flatten)]
    pub inner: CustomEventInner,
    #[serde(default)]
    pub repetition: Option<EventRepetition>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CustomEventInner {
    Flexible(FlexibleEvent),
    Inflexible(InflexibleEvent)
}

impl Default for CustomEventInner {
    fn default() -> Self {
        Self::Inflexible(Default::default())
    }
}

impl CustomEventInner {
    pub fn is_flexible(&self) -> bool {
        match self {
            Self::Flexible(_) => true,
            Self::Inflexible(_) => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Sequence)]
pub enum CommitmentType {
    #[default]
    Event,
    Ical,
}

impl Display for CommitmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommitmentType::Ical => write!(f, "Ical Calendar"),
            CommitmentType::Event => write!(f, "Custom Event"),
        }
    }
}

impl From<CommitmentType> for Commitment {
    fn from(commitment_type: CommitmentType) -> Self {
        match commitment_type {
            CommitmentType::Ical => Commitment::Ical(String::default()),
            CommitmentType::Event => Commitment::Event(Default::default()),
        }
    }
}

impl FromStr for CommitmentType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Ical Calendar" => Ok(CommitmentType::Ical),
            "Custom Event" => Ok(CommitmentType::Event),
            _ => Err(()),
        }
    }
}

impl Default for Commitment {
    fn default() -> Self {
        Self::Event(Default::default())
    }
}
