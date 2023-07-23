use crate::modules::Configurable;

use super::SleepSchedule;

pub struct Everyman {}

impl SleepSchedule for Everyman {}

impl Configurable for Everyman {
    fn run_config(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}