use crate::config::Configurable;


use super::SleepSchedule;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Biphasic {}

impl SleepSchedule for Biphasic {}

impl Configurable for Biphasic {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}