use day_core::event::{InflexibleEvent, Event};

use crate::{config::Configurable, table::{TableFmt, DurationTableFmt}};

impl Configurable for InflexibleEvent {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

impl TableFmt for Event {
    fn headers() -> Vec<&'static str> {
        ["Name", "Notes", "Duration", "Time"].into()
    }

    fn row(self) -> comfy_table::Row {
        vec![
            self.name.into(),
            self.notes.into(),
            self.duration.to_cell_duration(),
            self.timing.to_string().into(),
        ].into()
    }
}