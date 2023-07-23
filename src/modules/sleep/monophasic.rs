use crate::modules::Configurable;

use super::SleepSchedule;

pub struct Monophasic {}

impl SleepSchedule for Monophasic {}

impl Configurable for Monophasic {
    fn run_config(&mut self) -> anyhow::Result<()> {
        todo!()
    }
}