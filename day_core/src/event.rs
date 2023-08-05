use chrono::{DateTime, Local};


#[derive(Debug, Clone, Serialize, Deserialize, Default)]

pub struct Event {
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub repetition: Option<EventRepetition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRepetition {
    pub repeat_start: DateTime<Local>,
    pub repeat_end: Option<DateTime<Local>>,
    pub pattern: RepetitionPattern,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum RepetitionPattern {
    #[default]
    Daily,
    Weekly,
    Monthly,
}