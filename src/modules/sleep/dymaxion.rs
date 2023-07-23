use crate::modules::Configurable;

use super::SleepSchedule;

pub struct Dymaxion {}

impl SleepSchedule for Dymaxion {}

impl Configurable for Dymaxion {
    fn run_config(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}