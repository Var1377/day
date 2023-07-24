use crate::config::Configurable;

use super::SleepSchedule;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Everyman {}

impl SleepSchedule for Everyman {}

impl Configurable for Everyman {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}