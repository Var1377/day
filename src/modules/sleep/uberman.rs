use crate::modules::Configurable;

use super::SleepSchedule;

pub struct Uberman {}

impl SleepSchedule for Uberman {}

impl Configurable for Uberman {
    fn run_config(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}