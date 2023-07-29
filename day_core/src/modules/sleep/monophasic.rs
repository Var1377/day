use crate::time::HourMinute;
use super::SleepSchedule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Monophasic {
    #[serde(default)]
    pub time_before_sleep_max: HourMinute,
    #[serde(default)]
    pub time_before_sleep_min: HourMinute,
}

impl Default for Monophasic {
    fn default() -> Self {
        Self {
            time_before_sleep_max: HourMinute(1, 0),
            time_before_sleep_min: HourMinute(0, 30),
        }
    }
}

impl SleepSchedule for Monophasic {}