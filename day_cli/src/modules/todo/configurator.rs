use chrono::Local;
use day_core::{modules::todos::{TodoConfig, Todo, Deadline}, time::{TimeOfDay, HourMinute}};

use crate::config::Configurable;


impl Configurable for TodoConfig {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Configurable for Todo {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        self.name = inquire::Text::new("Todo Name:")
            .with_default(&self.name)
            .prompt()?;

        let mut desc = inquire::Text::new("Todo Description:");

        self.urgency = inquire::CustomType::<u8>::new("Urgency from 0 to 7: ")
            .with_default(self.urgency)
            .prompt()?;

        if !&self.notes.is_empty() {
            desc = desc.with_default(&self.notes);
        }

        self.notes = desc.prompt()?;

        if inquire::Confirm::new("Does this todo have a deadline?")
            .with_default(false)
            .prompt()?
        {
            let mut date_picker = inquire::DateSelect::new("Todo Deadline:");
            if let Some(date) = &self.deadline {
                date_picker = date_picker.with_default(date.get_naive_date());
            }
            let chosen_date = Some(date_picker.prompt()?);
            if inquire::Confirm::new("Does this deadline have a specific time?")
                .with_default(false)
                .prompt()?
            {
                let time = inquire::CustomType::<TimeOfDay>::new("Deadline time: ")
                    .with_default(
                        self.deadline
                            .clone()
                            .map(|e| e.get_date_time())
                            .flatten()
                            .unwrap_or_default()
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

        self.duration = inquire::CustomType::<HourMinute>::new("Estimated Duration: ")
            .with_default(self.duration)
            .prompt()?
            .into();

        Ok(())
    }
}
