use crate::event::FixedTiming;
use chrono::{DateTime, Local};

use super::{Schedule, ScheduleSlot, SlotCandidateTiming};

pub struct ScheduleSolver<'a> {
    modules: Vec<Box<dyn super::Module + 'a>>,
}

impl<'a> ScheduleSolver<'a> {
    pub fn solve(mut self, limit: DateTime<Local>) -> Schedule {
        let mut schedule = Schedule::default();
        loop {
            match self
                .modules
                .iter_mut()
                .find_map(|m| m.next_candidate(&schedule))
            {
                Some(slot) => match slot.timing {
                    SlotCandidateTiming::Fixed(timing) => {
                        schedule.events.push(ScheduleSlot {
                            event: slot.event,
                            timing,
                        });
                    }
                    SlotCandidateTiming::FlexibleFixedDuration {
                        min_start,
                        max_start,
                        minutes,
                    } => {
                        let mut timing = FixedTiming {
                            start: min_start,
                            end: min_start + chrono::Duration::minutes(minutes as i64),
                        };
                        schedule.events.push(ScheduleSlot {
                            event: slot.event,
                            timing: timing,
                        });
                    }
                    SlotCandidateTiming::Flexible {
                        min_start,
                        max_start,
                        min_duration,
                        max_duration,
                    } => {
                        let mut timing = FixedTiming {
                            start: min_start,
                            end: min_start + chrono::Duration::minutes(min_duration as i64),
                        };
                        schedule.events.push(ScheduleSlot {
                            event: slot.event,
                            timing: timing,
                        });
                    }
                },
                None => break,
            }
        }
        schedule
    }
}
