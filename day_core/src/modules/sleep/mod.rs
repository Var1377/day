mod monophasic;

use std::fmt::Display;
use serde::{Deserialize, Serialize};
use enum_iterator::Sequence;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct SleepConfig {
    #[serde(default)]
    pub schedule: SleepSchedule,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SleepSchedule {
    Monophasic(monophasic::Monophasic),
    Biphasic,
    Everyman,
    Uberman,
    Dymaxion,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Sequence, Default)]
pub enum SleepScheduleType {
    #[default]
    Monophasic,
    Biphasic,
    Everyman,
    Uberman,
    Dymaxion,
}

impl From<SleepScheduleType> for SleepSchedule {
    fn from(schedule_type: SleepScheduleType) -> Self {
        match schedule_type {
            SleepScheduleType::Monophasic => SleepSchedule::Monophasic(Default::default()),
            SleepScheduleType::Biphasic => SleepSchedule::Biphasic,
            SleepScheduleType::Everyman => SleepSchedule::Everyman,
            SleepScheduleType::Uberman => SleepSchedule::Uberman,
            SleepScheduleType::Dymaxion => SleepSchedule::Dymaxion,
        }
    }
}

impl Default for SleepSchedule {
    fn default() -> Self {
        SleepSchedule::Monophasic(Default::default())
    }
}

impl Display for SleepSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SleepSchedule::Monophasic(_m) => write!(f, "🌚 Monophasic"),
            SleepSchedule::Biphasic => write!(f, "🌞 Biphasic"),
            SleepSchedule::Everyman => write!(f, "🤺 Everyman"),
            SleepSchedule::Uberman => write!(f, "🦾 Uberman"),
            // extra space needed here for some reason for correct terminal formatting, not sure why
            SleepSchedule::Dymaxion => write!(f, "👁️  Dymaxion"),
        }
    }
}
