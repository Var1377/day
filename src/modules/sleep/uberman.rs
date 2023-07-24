use crate::config::Configurable;
use super::SleepSchedule;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Uberman {}

impl SleepSchedule for Uberman {}

impl Configurable for Uberman {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}