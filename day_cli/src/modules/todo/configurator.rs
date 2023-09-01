use chrono::{Local, NaiveDateTime};
use day_core::{
    modules::{
        task::{Deadline, Task},
        todos::TodoConfig,
    },
    now,
    time::TimeOfDay,
};

use crate::{cli::CustomConfigurable, config::Configurable};

impl Configurable for TodoConfig {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Configurable for Task {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        self.event.run_configurator()?;

        self.urgency = inquire::CustomType::<u8>::new("Urgency from 0 to 7: ")
            .with_default(self.urgency)
            .prompt()?;

        self.duration.run_configurator("How long will this last")?;

        if inquire::Confirm::new("Does this todo have a deadline?")
            .with_default(self.deadline.is_some())
            .prompt()?
        {
            let mut new_deadline = self.deadline.unwrap_or_default();
            new_deadline.run_configurator()?;
            self.deadline = Some(new_deadline);
        }
        Ok(())
    }
}

impl Configurable for Deadline {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        let date = inquire::DateSelect::new("Deadline:")
            .with_default(self.get_naive_date())
            .prompt()?;

        if inquire::Confirm::new("Does this deadline have a specific time?")
            .with_default(self.has_time())
            .prompt()?
        {
            let mut time: TimeOfDay = now().time().into();
            time.run_configurator("Deadline Time")?;
            *self = Self::DateTime(
                NaiveDateTime::new(date, time.into())
                    .and_local_timezone(Local)
                    .earliest().expect("Time conversion failed"),
            )
        } else {
            *self = Self::Date(date);
        }

        Ok(())
    }
}
