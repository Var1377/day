use std::fmt::Display;

pub use self::{
    biphasic::Biphasic, dymaxion::Dymaxion, everyman::Everyman, monophasic::Monophasic,
    uberman::Uberman,
};
use serde::{Deserialize, Serialize};
use enum_iterator::Sequence;

mod biphasic;
mod dymaxion;
mod everyman;
mod monophasic;
mod uberman;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Sleep {
    #[serde(default)]
    pub monophasic: Monophasic,
    #[serde(default)]
    pub biphasic: Biphasic,
    #[serde(default)]
    pub everyman: Everyman,
    #[serde(default)]
    pub uberman: Uberman,
    #[serde(default)]
    pub dymaxion: Dymaxion,

    #[serde(default)]
    pub sleep_schedule: SleepScheduleType,
}

pub trait SleepSchedule {}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Sequence)]
pub enum SleepScheduleType {
    Monophasic,
    Biphasic,
    Everyman,
    Uberman,
    Dymaxion,
}

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
