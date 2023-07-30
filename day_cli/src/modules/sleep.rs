use crate::{cli::Runnable, config::Configurable};
use clap::Args;
use day_core::modules::sleep::{SleepConfig, SleepSchedule};
use day_core::state::State;

impl Configurable for SleepConfig {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        self.schedule.run_configurator()?;

        Ok(())
    }
}

#[derive(Debug, Args)]
pub struct SleepArgs;

impl Runnable for SleepArgs {
    type Args = ();
    fn run(&self, _: &(), state: &mut State) -> anyhow::Result<()> {
        state.config.sleep.run_configurator()
    }
}

impl Configurable for SleepSchedule {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        match self {
            SleepSchedule::Monophasic(m) => {
                m.cycles = inquire::CustomType::new("How many 90 minute sleep cycles would you like to sleep?")
                    .with_default(5)
                    .prompt()?;
            }
            _ => unimplemented!("Sleep schedule configurator not implemented"),
        }

        Ok(())
    }
}
