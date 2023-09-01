use day_core::{
    event::{Event, EventRepetition, FixedTiming, InflexibleEvent, RepetitionPattern, FlexibleEvent, Constraints},
    modules::commitments::{CustomEvent, CustomEventInner},
    time::HourMinute, now,
};

use crate::{
    config::Configurable,
    table::TableFmt, cli::DatetimeConfig,
};

impl Configurable for InflexibleEvent {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        self.timing.run_configurator()?;
        Ok(())
    }
}

impl Configurable for FlexibleEvent {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        self.constraints.run_configurator()?;
        Ok(())
    }
}

impl Configurable for Constraints {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

impl Configurable for CustomEventInner {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        if inquire::Confirm::new("Is this event flexible?")
            .with_default(self.is_flexible())
            .prompt()?
        {
            let mut event = FlexibleEvent::default();
            event.run_configurator()?;
            *self = Self::Flexible(event);
        } else {
            let mut event = InflexibleEvent::default();
            event.run_configurator()?;
            *self = Self::Inflexible(event);
        }

        Ok(())
    }
}

impl Configurable for CustomEvent {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        
        self.inner.run_configurator()?;

        if inquire::Confirm::new("Does this event repeat?")
            .with_default(self.repetition.is_some())
            .prompt()?
        {
            let mut repitition = self.repetition.unwrap_or_default();
            repitition.run_configurator()?;
            self.repetition = Some(repitition);
        }


        Ok(())
    }
}

impl Configurable for FixedTiming {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        println!("Start:");
        self.start.run_configurator("Start")?;
        println!("End:");
        self.end.run_configurator("End")?;
        Ok(())
    }
}

impl Configurable for EventRepetition {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        self.pattern.run_configurator()?;
        if inquire::Confirm::new("Does this repetition have an endpoint?")
            .with_default(self.repeat_end.is_some())
            .prompt()?
        {
            let mut repitition = self.repeat_end.unwrap_or(now());
            repitition.run_configurator("Repeat until")?;
            self.repeat_end = Some(repitition);
        }

        Ok(())
    }
}

impl Configurable for RepetitionPattern {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        *self = inquire::Select::new("", enum_iterator::all().collect()).prompt()?;
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
        ]
        .into()
    }
}
