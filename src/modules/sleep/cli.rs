use crate::modules::{Configurable, sleep::SleepScheduleType};
use inquire::Select;
use super::Sleep;

impl Configurable for Sleep {
    fn run_config(&mut self) -> anyhow::Result<()> {
        match Select::new("What is your preferred sleep schedule?", SleepScheduleType::iter_variants().collect()).prompt() {
            Ok(sleep_schedule) => {
                self.sleep_schedule = sleep_schedule;
                todo!()
            },
            Err(_) => todo!(),
        };
    }
}