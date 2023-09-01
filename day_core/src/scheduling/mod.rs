pub mod solver;

use chrono::{DateTime, Local};
use crate::event::{EventDetails, FixedTiming};


#[derive(Debug, Clone, Default)]
pub struct Schedule {
    pub events: Vec<ScheduleSlot>,
}

#[derive(Debug, Clone)]
pub struct ScheduleSlot {
    pub event: EventDetails,
    pub timing: FixedTiming,
}

pub trait Module {
    fn next_candidate(&mut self, schedule: &Schedule) -> Option<SlotCandidate>;
}

pub struct SlotCandidate {
    pub event: EventDetails,
    pub timing: SlotCandidateTiming,
}

pub enum SlotCandidateTiming {
    Fixed(FixedTiming),
    FlexibleFixedDuration {
        min_start: DateTime<Local>,
        max_start: DateTime<Local>,
        minutes: u32,
    },
    Flexible {
        min_start: DateTime<Local>,
        max_start: DateTime<Local>,
        min_duration: u32,
        max_duration: u32,
    },
}
