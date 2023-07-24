use std::str::FromStr;

use super::SleepSchedule;
use crate::{
    config::Configurable,
    time::HourMinute,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Monophasic {
    #[serde(default)]
    time_before_sleep_max: HourMinute,
    #[serde(default)]
    time_before_sleep_min: HourMinute,
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

impl Configurable for Monophasic {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        let max = HourMinute::prompt_with_default("How much free time do you need before sleep?", HourMinute(1, 0))?;

        let min: HourMinute = inquire::Text::new("What's the minimum amount of free time you need before sleep?")
            .with_validator(move |input: &str| {
                let hm = HourMinute::from_str(input)?;
                if hm < max {
                    Ok(inquire::validator::Validation::Valid)
                } else {
                    Ok(inquire::validator::Validation::Invalid("Minimum time before sleep must be less than maximum time before sleep".into()))
                }
            })
            .with_default(&self.time_before_sleep_min.to_string())
            .prompt()?.parse().unwrap(); // unwrap is safe because we validated the input

        self.time_before_sleep_max = max;
        self.time_before_sleep_min = min;

        Ok(())
    }
}
