use crate::modules::Configurable;

use super::SleepSchedule;

pub struct Biphasic {}

impl SleepSchedule for Biphasic {}

impl Configurable for Biphasic {
    fn run_config(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}