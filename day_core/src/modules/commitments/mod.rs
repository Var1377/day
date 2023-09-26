mod state;
pub use state::CommitmentState;

use crate::event::{InflexibleEvent, FlexibleEvent, EventRepetition, EventDetails};
use std::{fmt::Display, str::FromStr};
use enum_iterator::Sequence;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Commitment {
    #[serde(default, flatten)]
    pub inner: CustomEventInner,
    #[serde(default, flatten)]
    pub details: EventDetails,
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
