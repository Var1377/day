use super::{Sleep, SleepScheduleType, SLEEP_SCHEDULES};
use crate::cli::Runnable;
use crate::config::Configurable;
use clap::Args;
use inquire::Select;

impl Configurable for Sleep {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        let sleep_schedule = Select::new(
            "What is your preferred sleep schedule?",
            SLEEP_SCHEDULES.into(),
        ).prompt()?;

        self.sleep_schedule = sleep_schedule;
        match sleep_schedule {
            SleepScheduleType::Monophasic => self.monophasic.run_configurator()?,
            SleepScheduleType::Biphasic => self.biphasic.run_configurator()?,
            SleepScheduleType::Everyman => self.everyman.run_configurator()?,
            SleepScheduleType::Uberman => self.uberman.run_configurator()?,
            SleepScheduleType::Dymaxion => self.dymaxion.run_configurator()?,
        }
        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct SleepArgs;

impl Runnable for SleepArgs {
    type Args = ();
    fn run(&self, _: &(), state: &mut crate::state::State) -> anyhow::Result<()> {
        state.config.sleep.run_configurator()
    }
}
