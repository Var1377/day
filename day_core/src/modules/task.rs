use std::{fmt::Display, cmp::Ordering};

use chrono::{NaiveDate, DateTime, Local};

use crate::event::Event;

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Task {
    #[serde(default)]
    pub urgency: u8,
    #[serde(default)]
    pub deadline: Option<Deadline>,
    #[serde(default, flatten)]
    pub event: Event,
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.urgency.cmp(&other.urgency).reverse().then(deadline_cmp(&self.deadline, &other.deadline).reverse()).then(self.event.id.cmp(&other.event.id))
    }
}

fn deadline_cmp(a: &Option<Deadline>, b: &Option<Deadline>) -> Ordering {
    match (a, b) {
        (Some(a), Some(b)) => a.partial_cmp(&b).unwrap_or(Ordering::Equal).reverse(),
        (Some(_), None) => Ordering::Greater,
        (None, Some(_)) => Ordering::Less,
        (None, None) => Ordering::Equal,
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Ord)]
pub enum Deadline {
    Date(NaiveDate),
    DateTime(DateTime<Local>),
}

impl Display for Deadline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Deadline::Date(date) => write!(f, "{}", date),
            Deadline::DateTime(date_time) => write!(f, "{}", date_time),
        }
    }
}

impl PartialOrd for Deadline {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Deadline::Date(a), Deadline::Date(b)) => a.partial_cmp(b),
            (Deadline::DateTime(a), Deadline::DateTime(b)) => a.partial_cmp(b),
            (Deadline::Date(a), Deadline::DateTime(b)) => a.and_hms_opt(0, 0, 0)?.and_local_timezone(Local).earliest()?.partial_cmp(b),
            (Deadline::DateTime(a), Deadline::Date(b)) => a.partial_cmp(&b.and_hms_opt(0, 0, 0)?.and_local_timezone(Local).earliest()?),
        }
    }
}

impl Deadline {
    pub fn get_naive_date(&self) -> NaiveDate {
        match self {
            Deadline::Date(date) => *date,
            Deadline::DateTime(date_time) => date_time.date_naive(),
        }
    }

    pub fn get_date_time(&self) -> Option<DateTime<Local>> {
        match self {
            Deadline::Date(date) => date.and_hms_opt(0, 0, 0)?.and_local_timezone(Local).earliest(),
            Deadline::DateTime(date_time) => Some(*date_time),
        }
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            deadline: None,
            urgency: 0,
            event: Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct CompletedTask {
    pub completed_at: DateTime<Local>,
    pub task: Task,
}

impl PartialOrd for CompletedTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CompletedTask {
    fn cmp(&self, other: &Self) -> Ordering {
        self.completed_at.cmp(&other.completed_at).reverse().then(self.task.cmp(&other.task))
    }
}