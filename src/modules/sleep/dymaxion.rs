use crate::config::Configurable;

use super::SleepSchedule;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Dymaxion {}

impl SleepSchedule for Dymaxion {}

impl Configurable for Dymaxion {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}