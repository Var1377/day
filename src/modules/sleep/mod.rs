use std::fmt::Display;

use self::{
    biphasic::Biphasic, dymaxion::Dymaxion, everyman::Everyman, monophasic::Monophasic,
    uberman::Uberman,
};
use serde::{Deserialize, Serialize};

mod biphasic;
pub mod cli;
mod dymaxion;
mod everyman;
mod monophasic;
mod uberman;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Sleep {
    #[serde(default)]
    monophasic: Monophasic,
    #[serde(default)]
    biphasic: Biphasic,
    #[serde(default)]
    everyman: Everyman,
    #[serde(default)]
    uberman: Uberman,
    #[serde(default)]
    dymaxion: Dymaxion,

    #[serde(default)]
    pub sleep_schedule: SleepScheduleType,
}

pub trait SleepSchedule {}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SleepScheduleType {
    Monophasic,
    Biphasic,
    Everyman,
    Uberman,
    Dymaxion,
}

const SLEEP_SCHEDULES: [SleepScheduleType; 5] = [
    SleepScheduleType::Monophasic,
    SleepScheduleType::Biphasic,
    SleepScheduleType::Everyman,
    SleepScheduleType::Uberman,
    SleepScheduleType::Dymaxion,
];

impl Default for SleepScheduleType {
    fn default() -> Self {
        SleepScheduleType::Monophasic
    }
}

impl Display for SleepScheduleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            SleepScheduleType::Monophasic => "🌚 Monophasic",
            SleepScheduleType::Biphasic => "🌞 Biphasic",
            SleepScheduleType::Everyman => "🤺 Everyman",
            SleepScheduleType::Uberman => "🦾 Uberman",
            // extra space needed here for some reason for correct terminal formatting, not sure why
            SleepScheduleType::Dymaxion => "👁️  Dymaxion",
        };
        write!(f, "{}", s)
    }
}
