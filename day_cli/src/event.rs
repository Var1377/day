use day_core::{event::{InflexibleEvent, Event}, time::HourMinute};

use crate::{config::Configurable, table::{TableFmt, DurationTableFmt}};

impl Configurable for InflexibleEvent {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}


impl Configurable for Event {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        self.title = inquire::Text::new("Name:")
            .with_default(&self.title)
            .prompt()?;

        let mut desc = inquire::Text::new("Description:");

        if !&self.notes.is_empty() {
            desc = desc.with_default(&self.notes);
        }

        self.notes = desc.prompt()?;

        self.duration = inquire::CustomType::<HourMinute>::new("Estimated Duration: ")
            .with_default(self.duration)
            .prompt()?
            .into();

        Ok(())
    }
}

impl TableFmt for Event {
    fn headers() -> Vec<&'static str> {
        ["Name", "Notes", "Duration"].into()
    }

    fn row(self) -> comfy_table::Row {
        vec![
            self.title.into(),
            self.notes.into(),
            self.duration.to_cell_duration(),
        ].into()
    }
}