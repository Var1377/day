use chrono::Local;
use day_core::{
    modules::{
        task::{Deadline, Task},
        todos::TodoConfig,
    },
    time::TimeOfDay, now,
};

use crate::config::Configurable;

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

        if inquire::Confirm::new("Does this todo have a deadline?")
            .with_default(self.deadline.is_some())
            .prompt()?
        {
            let mut date_picker = inquire::DateSelect::new("Deadline:").with_default(
                self.deadline.as_ref().map(|e| e.get_naive_date()).unwrap_or(now().date_naive())
            );
            if let Some(date) = &self.deadline {
                date_picker = date_picker.with_default(date.get_naive_date());
            }
            let chosen_date = Some(date_picker.prompt()?);
            if inquire::Confirm::new("Does this deadline have a specific time?")
                .with_default(self.deadline.as_ref().map(|e| e.has_time()).unwrap_or(false))
                .prompt()?
            {
                let time = inquire::CustomType::<TimeOfDay>::new("Deadline time: ")
                    .with_default(
                        self.deadline
                            .clone()
                            .map(|e| e.get_date_time())
                            .flatten()
                            .unwrap_or(now())
                            .into(),
                    )
                    .prompt()?;
                self.deadline = Some(Deadline::DateTime(
                    chosen_date
                        .unwrap()
                        .and_hms_opt(time.0, time.1, 0)
                        .unwrap()
                        .and_local_timezone(Local)
                        .earliest()
                        .unwrap(),
                ));
            } else {
                self.deadline = Some(Deadline::Date(chosen_date.unwrap()));
            }
        }
        Ok(())
    }
}
