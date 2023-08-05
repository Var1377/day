use day_core::event::Event;

use crate::config::Configurable;

impl Configurable for Event {
    fn run_configurator(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}